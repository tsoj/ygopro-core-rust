//! Mad Hacker
use super::prelude::*;

pub const ID: u32 = 86993168;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Take control
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_CONTROL);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(ctcost)));
    effect::set_target(d, e2, Some(cost_target(cttg)));
    effect::set_operation(d, e2, Some(operation(ctop)));
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsControler,1,nil,1-tp) end`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, cc| card::is_controler(d, cc, 1 - tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.ctcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ctcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None);
    }
    duel::remove(d, c, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil):GetMinGroup(Card.GetAttack)`
fn min_attack_group(d: &mut Duel, tp: u8) -> Option<CardSet> {
    let all = duel::get_matching_group(d, Some(&|d, cc| card::is_faceup(d, cc)), tp, 0, LOCATION_MZONE, NO_EX);
    group::get_min_group(d, &all, &|d, cc| card::get_attack(d, cc) as i64).map(|x| x.0)
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let g = min_attack_group(d, a.tp);
    if a.chk == Some(0) {
        return match &g {
            Some(g) => group::is_exists(d, g, &|d, cc| card::is_able_to_change_controler(d, cc), 1, NO_EX),
            None => false,
        };
    }
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_CONTROL), None).0 < 1 {
        return;
    }
    let Some(mut g) = min_attack_group(d, tp).filter(|g| !g.is_empty()) else {
        return;
    };
    if group::filter(d, &g, &|d, cc| card::is_able_to_change_controler(d, cc), NO_EX).len() < 1 {
        return;
    }
    if g.len() > 1 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
        g = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    }
    if g.is_empty() {
        return;
    }
    duel::hint_selection(d, g.clone(), true);
    // if Duel.GetControl(g,tp) then: the core returns a number, which is always true in Lua
    let _ = duel::get_control(d, g.clone(), tp, 0, 0, None, None);
    {
        let c = effect::get_handler(d, a.e).nil();
        let tc = group::get_first(d, &g).nil();
        // Cannot activate its effects
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3302);
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CONTROL, 1);
        card::register_effect(d, tc, e1, true);
        // Cannot Special Summon non-Link monsters from Extra Deck
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
        effect::set_range(d, e2, LOCATION_MZONE);
        effect::set_absolute_range(d, e2, tp, 1, 0);
        effect::set_target(d, e2, Some(effect_card_fn(|d, _e, cc| card::is_location(d, cc, LOCATION_EXTRA) && !card::is_type(d, cc, TYPE_LINK))));
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_CONTROL, 1);
        card::register_effect(d, tc, e2, true);
        // Lizard check
        let e3 = aux::create_continuous_lizard_check(d, c, LOCATION_MZONE, Some(effect_card_fn(|d, _e, cc| card::is_original_type(d, cc, TYPE_LINK))), None, None);
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD | RESET_CONTROL, 1);
        card::register_effect(d, tc, e3, true);
    }
}
