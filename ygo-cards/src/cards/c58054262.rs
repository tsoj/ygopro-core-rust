//! Machina Force
use super::prelude::*;

pub const ID: u32 = 58054262;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[22666164, 60999392, 23782705, 96384007], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Must be Special Summoned by its own condition
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // Must pay 1000 LP to declare attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_ATTACK_COST);
    effect::set_cost(d, e2, Some(lua_fn(atcost)));
    effect::set_operation(d, e2, Some(operation(atop)));
    card::register_effect(d, c, e2, false);
    // Special Summon 3 monsters from the GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_cost(d, e3, Some(cost::self_to_grave()));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
/// `function(e,c,tp) return Duel.CheckLPCost(tp,1000) end`
fn atcost(d: &mut Duel, a: Args) -> bool {
    duel::check_lp_cost(d, a.p(2), 1000)
}
/// `function(e,tp,eg,ep,ev,re,r,rp) if Duel.IsAttackCostPaid()~=2 and e:GetHandler():IsLocation(LOCATION_MZONE) then ... end end`
fn atop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if duel::is_attack_cost_paid(d) != 2 && card::is_location(d, h, LOCATION_MZONE) {
        duel::pay_lp_cost(d, a.tp, 1000);
        duel::attack_cost_paid(d, None);
    }
}
fn filter(d: &mut Duel, c: CardId, code: u32, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chkc().is_some() {
        return false;
    }
    if a.chk == Some(0) {
        return !duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, 60999392, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, 23782705, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, 96384007, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g1 = duel::select_target(d, tp, Some(&|d, c| filter(d, c, 60999392, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    let mut g1 = g1.nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g2 = duel::select_target(d, tp, Some(&|d, c| filter(d, c, 23782705, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g3 = duel::select_target(d, tp, Some(&|d, c| filter(d, c, 96384007, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    group::add_card(&mut g1, &g2);
    group::add_card(&mut g1, &g3);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g1), 3, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 3 {
        return;
    }
    let g = duel::get_target_cards(d, a.e).nil();
    if group::get_count(&g) == 3 {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
