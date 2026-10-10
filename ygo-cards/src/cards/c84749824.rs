//! Solemn Warning
use super::prelude::*;

pub const ID: u32 = 84749824;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate(summon)
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_SUMMON);
    effect::set_condition(d, e1, Some(condition(cond1)));
    effect::set_cost(d, e1, Some(cost_target(cost1)));
    effect::set_target(d, e1, Some(cost_target(target1)));
    effect::set_operation(d, e1, Some(operation(activate1)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_FLIP_SUMMON);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EVENT_SPSUMMON);
    card::register_effect(d, c, e3, false);
    // Activate(effect)
    let e4 = effect::create_effect(d, c);
    effect::set_category(d, e4, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_type(d, e4, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e4, EVENT_CHAINING);
    effect::set_condition(d, e4, Some(condition(cond2)));
    effect::set_cost(d, e4, Some(cost_target(cost2)));
    effect::set_target(d, e4, Some(cost_target(target2)));
    effect::set_operation(d, e4, Some(operation(activate2)));
    card::register_effect(d, c, e4, false);
}
fn cond1(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)==0
    duel::get_current_chain(d, true) == 0
}
fn cost1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckLPCost(tp,2000)
    // else Duel.PayLPCost(tp,2000) end
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, a.tp, 2000);
    }
    duel::pay_lp_cost(d, a.tp, 2000);
    true
}
fn target1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,0,0)
    let eg = a.eg.clone().nil();
    let n = eg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg), n, 0, 0);
    true
}
fn activate1(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, a.eg());
    // Duel.Destroy(eg,REASON_EFFECT)
    duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
}
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    // if not Duel.IsChainNegatable(ev) then return false end
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    // if not re or (not re:IsMonsterEffect() and not re:IsHasType(EFFECT_TYPE_ACTIVATE)) then return false end
    let Some(re) = a.re else { return false };
    if !effect::is_monster_effect(d, re) && !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    // return re:IsHasCategory(CATEGORY_SPECIAL_SUMMON)
    effect::is_has_category(d, re, CATEGORY_SPECIAL_SUMMON)
}
fn cost2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckLPCost(tp,2000)
    // else Duel.PayLPCost(tp,2000) end
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, a.tp, 2000);
    }
    duel::pay_lp_cost(d, a.tp, 2000);
    true
}
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    if card::is_destructable(d, h, None) && card::is_relate_to_effect(d, h, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn activate2(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let h = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, h, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
