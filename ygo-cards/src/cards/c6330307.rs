//! DZW - Chimera Clad
use super::prelude::*;

pub const ID: u32 = 6330307;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NUMBER_C39],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetRange(LOCATION_HAND|LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_MZONE);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddZWEquipLimit(c,nil,function(tc,c,tp) return s.filter(tc) and tc:IsControler(tp) end,aux.EquipAndLimitRegister,e1)
    aux::add_zw_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a: Args| {
            let (tc, tp) = (a.c(0), a.p(2));
            filter(d, tc) && card::is_controler(d, tc, tp)
        }),
        lua_fn(|d, a: Args| {
            aux::equip_and_limit_register(d, a.c(0), a.e(1), a.p(2), a.c(3), None, None);
        }),
        e1,
        None,
        None,
        None,
    );
    // destroy sub
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // chain attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e3, EVENT_DAMAGE_STEP_END);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_NUMBER_C39)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NUMBER_C39 as u16])
}
// function s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.eqop(e,tp,eg,ep,ev,re,r,rp)
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if c:IsLocation(LOCATION_MZONE) and c:IsFacedown() then return end
    if card::is_location(d, c, LOCATION_MZONE) && card::is_facedown(d, c) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or not tc:IsRelateToEffect(e) or tc:IsFacedown()
    //     or tc:GetControler()~=tp or not c:CheckUniqueOnField(tp) then
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0
        || !card::is_relate_to_effect(d, tc.nil(), a.e)
        || card::is_facedown(d, tc.nil())
        || card::get_controler(d, tc.nil()) != tp
        || !card::check_unique_on_field(d, c, tp, None, None)
    {
        // Duel.SendtoGrave(c,REASON_EFFECT)
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        // return
        return;
    }
    // aux.EquipAndLimitRegister(c,e,tp,tc)
    aux::equip_and_limit_register(d, c, a.e, tp, tc.nil(), None, None);
}
// function s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let at_attacker = duel::get_attacker(d);
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return at and a==e:GetHandler():GetEquipTarget() and at:IsRelateToBattle() and at:GetAttack()>0 and a:CanChainAttack()
    let Some(at) = at else { return false };
    let c = effect::get_handler(d, a.e).nil();
    if at_attacker != card::get_equip_target(d, c) {
        return false;
    }
    card::is_relate_to_battle(d, at)
        && card::get_attack(d, at) > 0
        && card::can_chain_attack(d, at_attacker.nil(), None, false)
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d).nil();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    let _ec = card::get_equip_target(d, c);
    // if at:IsRelateToBattle() and not at:IsImmuneToEffect(e) and at:GetAttack()>0 then
    if card::is_relate_to_battle(d, at) && !card::is_immune_to_effect(d, at, a.e) && card::get_attack(d, at) > 0 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // at:RegisterEffect(e1)
        card::register_effect(d, at, e1, false);
        // Duel.ChainAttack(at)
        duel::chain_attack(d, Some(at));
    }
}
