//! Cat Shark
use super::prelude::*;

pub const ID: u32 = 84224627;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 2 monsters
    // Xyz.AddProcedure(c,nil,2,2)
    xyz::add_procedure(d, c, None, Some(2), 2, None, None, None, None, false, None);
    // Cannot be destroyed by battle while it has a material attached that was originally WATER
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_condition(d, e1, Some(condition(indcon)));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // Double the ATK/DEF of 1 Rank 4 or lower Xyz Monster you control
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, None);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(stat_change_damage_step_condition)));
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetOverlayGroup():IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_WATER)
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_overlay_group(d, h);
    group::is_exists(d, &g, &|d, c| card::is_attribute(d, c, ATTRIBUTE_WATER), 1, NO_EX)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsRankBelow(4)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_rank_below(d, c, 4)
}
/// `aux.StatChangeDamageStepCondition()` used as a condition
fn stat_change_damage_step_condition(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        // return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc)
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        // return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil)
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // local tc=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,tc,1,tp,tc:GetBaseAttack()*2)
    let atk = card::get_base_attack(d, tc) * 2;
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(tc)), 1, a.tp, atk);
    // Duel.SetOperationInfo(0,CATEGORY_DEFCHANGE,tc,1,tp,tc:GetBaseDefense()*2)
    let def = card::get_base_defense(d, tc) * 2;
    duel::set_operation_info(d, 0, CATEGORY_DEFCHANGE, Some(CardSet::from_card(tc)), 1, a.tp, def);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(tc:GetBaseAttack()*2)
        let atk = card::get_base_attack(d, tc) * 2;
        effect::set_value(d, e1, atk as i64);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // e2:SetValue(tc:GetBaseDefense()*2)
        let def = card::get_base_defense(d, tc) * 2;
        effect::set_value(d, e2, def as i64);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
