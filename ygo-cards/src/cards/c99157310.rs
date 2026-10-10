//! Tindangle Delaunay
use super::prelude::*;

pub const ID: u32 = 99157310;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[75119040], listed_series: &[SET_TINDANGLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy attacking monster, special summon 1 "Tindangle Acute Cerberus" from extra deck
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Special summon 3 "Tindangle" monsters with different names from GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_TINDANGLE as u16]) && card::is_monster(d, c)
}
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    a.ep == tp && {
        let at = duel::get_attacker(d).nil();
        card::is_controler(d, at, 1 - tp)
    } && duel::is_existing_matching_card(d, Some(&cfilter1), tp, LOCATION_GRAVE, 0, 3, NO_EX)
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[75119040])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let tg = duel::get_attacker(d);
    let g = duel::get_matching_group(d, Some(&cfilter1), tp, LOCATION_GRAVE, 0, NO_EX);
    if a.chk == Some(0) {
        let tg = tg.nil();
        return card::is_on_field(d, tg)
            && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) > 2
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    let tg = tg.nil();
    duel::set_target_card(d, tg);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tg)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_relate_to_battle(d, tc) && duel::destroy(d, tc, REASON_EFFECT, None, None) != 0 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        if g.len() > 0 {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    card::get_sequence(d, c) >= 5
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    !duel::is_existing_matching_card(d, Some(&cfilter2), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_TINDANGLE as u16])
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chkc().is_some() {
        return false;
    }
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 2
            && group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) > 2;
    }
    let tg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(3), Some(3), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    let n = tg.len() as u8;
    duel::set_target_card(d, tg.clone());
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), n, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let g = duel::get_target_cards(d, a.e).unwrap_or_default();
    if ft <= 0 || g.len() == 0 || (g.len() > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()) {
        return;
    }
    if g.len() as i32 <= ft {
        duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
        duel::confirm_cards(d, 1 - tp, g);
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, tp, ft as u16, ft as u16, false, NO_EX).nil();
        duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
