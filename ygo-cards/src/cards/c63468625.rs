//! Meklord Astro Mekanikle
use super::prelude::*;

pub const ID: u32 = 63468625;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MEKLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon condition
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(0)
    effect::set_value(d, e1, 0);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon procedure
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // equip
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e3, CATEGORY_EQUIP);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.eqtg)
    effect::set_target(d, e3, Some(cost_target(eqtg)));
    // e3:SetOperation(s.eqop)
    effect::set_operation(d, e3, Some(operation(eqop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e3)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| eqval(d, a.c(0), a.p(2))),
        lua_fn(|d, a| {
            equipop(d, a.c(0), a.e(1), a.p(2), a.c(3));
            Value::Nil
        }),
        e3,
        None,
        None,
        None,
    );
    // damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_DAMAGE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e4:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e4, EVENT_PHASE | PHASE_STANDBY);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.damcon)
    effect::set_condition(d, e4, Some(condition(damcon)));
    // e4:SetCost(s.damcost)
    effect::set_cost(d, e4, Some(cost_target(damcost)));
    // e4:SetTarget(s.damtg)
    effect::set_target(d, e4, Some(cost_target(damtg)));
    // e4:SetOperation(s.damop)
    effect::set_operation(d, e4, Some(operation(damop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, ec: CardId, tp: u8) -> bool {
    // return ec:IsType(TYPE_SYNCHRO) and ec:IsControler(1-tp)
    card::is_type(d, ec, TYPE_SYNCHRO) && card::is_controler(d, ec, 1 - tp)
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_MEKLORD) and c:IsAbleToGraveAsCost()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_MEKLORD as u16]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_HAND, 0, h.into());
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and #rg>2 and aux.SelectUnselectGroup(rg,e,tp,3,3,nil,0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && rg.len() > 2
        && matches!(
            aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), None, 0, tp, None, None, None, false),
            Err(true)
        )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local rg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,c)
    let rg = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_HAND, 0, c.into());
    // local g=aux.SelectUnselectGroup(rg,e,tp,3,3,nil,1,tp,HINTMSG_TOGRAVE,nil,nil,true)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), None, 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true)
        .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    if let Value::Group(g) = effect::get_label_object(d, e) {
        // Duel.SendtoGrave(g,REASON_COST)
        duel::send_to_grave(d, g, REASON_COST, None, None);
    }
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_SYNCHRO) and c:IsAbleToChangeControler()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_able_to_change_controler(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.eqfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && eqfilter(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.eqfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&eqfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.eqfilter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&eqfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(g), 1, 0, 0);
    true
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // local atk=tc:GetTextAttack()
    let mut atk = card::get_text_attack(d, tc);
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc,id) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), None) {
        return;
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // e2:SetValue(atk)
    effect::set_value(d, e2, atk as i64);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsMonster() then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) && card::is_monster(d, tc) {
        // s.equipop(c,e,tp,tc)
        equipop(d, c, e, tp, tc);
    }
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.dcfilter(c)`
fn dcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetFlagEffect(id)~=0 and c:IsAbleToGraveAsCost()
    card::get_flag_effect(d, c, ID) != 0 && card::is_able_to_grave_as_cost(d, c)
}
/// `s.damcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return e:GetHandler():GetEquipGroup():IsExists(s.dcfilter,1,nil) end
    if a.chk == Some(0) {
        let eq = card::get_equip_group(d, h);
        return group::is_exists(d, &eq, &|d, c| dcfilter(d, c), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=e:GetHandler():GetEquipGroup():FilterSelect(tp,s.dcfilter,1,1,nil)
    let eq = card::get_equip_group(d, h);
    let g = group::filter_select(d, &eq, tp, &|d, c| dcfilter(d, c), 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g.clone(), REASON_COST, None, None);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local atk=g:GetFirst():GetTextAttack()
    let first = group::get_first(d, &g).nil();
    let mut atk = card::get_text_attack(d, first);
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // e:SetLabel(atk)
    effect::set_label(d, e, &[atk as i64]);
    true
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let label = effect::get_label(d, a.e);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - a.tp);
    // Duel.SetTargetParam(e:GetLabel())
    duel::set_target_param(d, label as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,e:GetLabel())
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, label as i32);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let (p, dam) = (info[0].int() as u8, info[1].int());
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, p, dam as i64, REASON_EFFECT, false, None);
}
