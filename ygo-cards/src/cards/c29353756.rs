//! ZW - Eagle Claw
//! ZW－荒鷲激神爪
use super::prelude::*;

pub const ID: u32 = 29353756;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UTOPIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Special Summon itself from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a| spcon(d, a))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equip itself to an "Utopia" monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.eqcon)
    effect::set_condition(d, e2, Some(condition(eqcon)));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.AddZWEquipLimit(c,s.eqcon,function(tc,c,tp) return s.filter(tc) and tc:IsControler(tp) end,s.equipop,e2)
    aux::add_zw_equip_limit(
        d,
        c,
        Some(condition(eqcon)),
        lua_fn(|d, a| {
            let tc = a.c(0);
            let tp = a.p(2);
            filter(d, tc) && card::is_controler(d, tc, tp)
        }),
        lua_fn(|d, a| {
            equipop(d, a.c(0), a.e(1), a.p(2), a.c(3));
            Value::Nil
        }),
        e2,
        None,
        None,
        None,
    );
    // Negate the effect of an opponent's Trap card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.negcon)
    effect::set_condition(d, e3, Some(condition(negcon)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // aux.DoubleSnareValidity(c,LOCATION_SZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_SZONE, None);
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args<'_>) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLP(tp)<=Duel.GetLP(1-tp)-2000 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    duel::get_lp(d, tp) <= duel::get_lp(d, 1 - tp) - 2000 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
// function s.eqcon(e)
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():CheckUniqueOnField(e:GetHandlerPlayer())
    let h = effect::get_handler(d, a.e).nil();
    let p = effect::get_handler_player(d, a.e);
    card::check_unique_on_field(d, h, p, None, None)
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_UTOPIA)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_UTOPIA as u16])
}
// function s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.eqop(e,tp,eg,ep,ev,re,r,rp)
fn eqop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, e) || card::is_facedown(d, c) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc_opt = duel::first_target(d);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or not tc:IsRelateToEffect(e) or tc:IsFacedown()
    //   or tc:GetControler()~=tp or not c:CheckUniqueOnField(tp) then
    let fail = if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        true
    } else {
        let tc = tc_opt.nil();
        !card::is_relate_to_effect(d, tc, e) || card::is_facedown(d, tc) || card::get_controler(d, tc) != tp || !card::check_unique_on_field(d, c, tp, None, None)
    };
    if fail {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        return;
    }
    // s.equipop(c,e,tp,tc)
    equipop(d, c, e, tp, tc_opt.nil());
}
// function s.equipop(c,e,tp,tc)
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // if not aux.EquipAndLimitRegister(c,e,tp,tc) then return end
    if !aux::equip_and_limit_register(d, c, e, tp, tc, None, None) {
        return;
    }
    // Increase the ATK of the equipped monster by 2000
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(2000)
    effect::set_value(d, e1, 2000);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.negcon(e,tp,eg,ep,ev,re,r,rp)
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and e:GetHandler():GetEquipTarget()
    //   and Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)==LOCATION_SZONE
    //   and re:IsTrapEffect() and Duel.IsChainDisablable(ev)
    if a.rp != 1 - a.tp {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if card::get_equip_target(d, h).is_none() {
        return false;
    }
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION]);
    if loc[0].int() != LOCATION_SZONE as i64 {
        return false;
    }
    effect::is_trap_effect(d, a.re.nil()) && duel::is_chain_disablable(d, a.ev as u8)
}
// function s.negop(e,tp,eg,ep,ev,re,r,rp)
fn negop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
