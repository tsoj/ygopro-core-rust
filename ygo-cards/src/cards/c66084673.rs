//! Brotherhood of the Fire Fist - Rhino
use super::prelude::*;

pub const ID: u32 = 66084673;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FIRE_FIST, SET_FIRE_FORMATION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetCost(s.atkcost)
    effect::set_cost(d, e1, Some(cost_target(atkcost)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return (Duel.GetAttacker():IsControler(tp) and Duel.GetAttacker():IsSetCard(SET_FIRE_FIST))
    //     or (Duel.GetAttackTarget() and Duel.GetAttackTarget():IsControler(tp) and Duel.GetAttackTarget():IsSetCard(SET_FIRE_FIST))
    let at = duel::get_attacker(d).nil();
    if card::is_controler(d, at, tp) && card::is_set_card(d, at, &[SET_FIRE_FIST as u16]) {
        return true;
    }
    match duel::get_attack_target(d) {
        Some(t) => card::is_controler(d, t, tp) && card::is_set_card(d, t, &[SET_FIRE_FIST as u16]),
        None => false,
    }
}
/// `s.filter1(c)`
fn filter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_FIRE_FORMATION) and c:IsAbleToGraveAsCost()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_FIRE_FORMATION as u16]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FIRE_FIST) and c:GetBaseAttack()>0 and c:IsAbleToGraveAsCost()
    card::is_set_card(d, c, &[SET_FIRE_FIST as u16]) && card::get_base_attack(d, c) > 0 && card::is_able_to_grave_as_cost(d, c)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    // if chk==0 then return e:GetHandler():GetFlagEffect(id)==0
    //     and Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_ONFIELD,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return card::get_flag_effect(d, h, ID) == 0
            && duel::is_existing_matching_card(d, Some(&filter1), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&filter2), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_ONFIELD,0,1,1,nil)
    let mut g1 = duel::select_matching_card(d, tp, Some(&filter1), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g2=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_HAND,0,1,1,nil)
    let g2 = duel::select_matching_card(d, tp, Some(&filter2), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabel(g2:GetFirst():GetBaseAttack())
    let tc2 = group::get_first(d, &g2).nil();
    let base = card::get_base_attack(d, tc2);
    effect::set_label(d, e, &[base as i64]);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SendtoGrave(g1,REASON_COST)
    duel::send_to_grave(d, g1, REASON_COST, None, None);
    // e:GetHandler():RegisterFlagEffect(id,RESET_PHASE|PHASE_DAMAGE_CAL,0,1)
    card::register_flag_effect(d, h, ID, RESET_PHASE | PHASE_DAMAGE_CAL, 0, 1, 0, 0);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=Duel.GetAttacker()
    let mut c = duel::get_attacker(d).nil();
    // if c:IsControler(1-tp) then c=Duel.GetAttackTarget() end
    if card::is_controler(d, c, 1 - tp) {
        c = duel::get_attack_target(d).nil();
    }
    let h = effect::get_handler(d, e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE_CAL)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
    // e1:SetValue(e:GetLabel())
    effect::set_value(d, e1, effect::get_label(d, e));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
