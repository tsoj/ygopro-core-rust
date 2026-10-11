//! True King Bahrastos, the Fathomer
use super::prelude::*;

pub const ID: u32 = 82321037;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 2 monsters and Special Summon itself from the hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 non-WATER Wyrm from the Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_DESTROYED);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(cond2)));
    effect::set_target(d, e2, Some(cost_target(sptg2)));
    effect::set_operation(d, e2, Some(operation(spop2)));
    card::register_effect(d, c, e2, false);
}
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_reason(d, c, REASON_EFFECT)
}
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && ((card::is_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)) || card::is_location(d, c, LOCATION_HAND))
}
/// `function(c,sg,e,tp,mg)` of `s.rescon`: `aux.ChkfMMZ(1)(sg,e,tp,mg) and sg:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_WATER)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    let mmz = aux::chkf_mmz(1)(d, sg, e, tp, mg, c).0;
    let ok = mmz && group::is_exists(d, sg, &|d, cc| card::is_attribute(d, cc, ATTRIBUTE_WATER), 1, NO_EX);
    (ok, false)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let loc = if ft < 0 { LOCATION_MZONE } else { LOCATION_MZONE | LOCATION_HAND };
    let loc2 = if duel::is_player_affected_by_effect(d, tp, 88581108).is_some() { LOCATION_MZONE } else { 0 };
    let g = duel::get_matching_group(d, Some(&desfilter), tp, LOCATION_MZONE | LOCATION_HAND, loc2, c.into());
    if a.chk == Some(0) {
        return ft > -2
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && g.len() >= 2
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon), 0, tp, None, None, None, false), Err(true));
    }
    if (g.len() == 2 && group::filter_count(d, &g, &|d, cc| card::is_location(d, cc, LOCATION_HAND), NO_EX) == 1)
        || !group::is_exists(d, &g, &|d, cc| card::is_location(d, cc, LOCATION_HAND), 1, NO_EX)
    {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, 0, 0);
    } else {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 2, tp, loc as i32);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, (LOCATION_SZONE | LOCATION_GRAVE) as i32);
    true
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let loc2 = if duel::is_player_affected_by_effect(d, tp, 88581108).is_some() { LOCATION_MZONE } else { 0 };
    let g = duel::get_matching_group(d, Some(&desfilter), tp, LOCATION_MZONE | LOCATION_HAND, loc2, c.into());
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_DESTROY as u64), None, None, false).unwrap_or_default();
    let rm = group::is_exists(d, &sg, &|d, cc| card::is_attribute(d, cc, ATTRIBUTE_WATER), 2, NO_EX);
    if duel::destroy(d, sg, REASON_EFFECT, None, None) == 2 {
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        if duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
            return;
        }
        let rg = duel::get_matching_group(d, Some(&rmfilter), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
        if rm && !rg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
            let tg = group::select(d, &rg, tp, 1, 2, false, NO_EX).nil();
            duel::hint_selection(d, tg.clone(), true);
            duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
fn thfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_attribute_except(d, c, ATTRIBUTE_WATER, None, 0, None)
        && card::is_race(d, c, RACE_WYRM)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, cc| thfilter(d, cc, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, LOCATION_DECK as i32);
    true
}
fn spop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| thfilter(d, cc, a.e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
