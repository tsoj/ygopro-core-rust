//! Forgotten Temple of the Deep
use super::prelude::*;

pub const ID: u32 = 43889633;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_UMI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // This card's name becomes "Umi"
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_value(d, e1, CARD_UMI as i64);
    card::register_effect(d, c, e1, false);
    // Banish 1 Level 4 or lower Fish, Sea Serpent, or Aqua monster you control
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_hint_timing(d, e2, TIMING_END_PHASE, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    effect::set_operation(d, e2, Some(operation(rmop)));
    card::register_effect(d, c, e2, false);
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level_below(d, c, 4)
        && card::is_race(d, c, RACE_FISH | RACE_SEASERPENT | RACE_AQUA)
        && card::is_faceup(d, c)
        && card::is_able_to_remove(d, c, None, None, None)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && rmfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, a.tp, Some(&rmfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, a.tp, 0);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_controler(d, tc, a.tp)
        && rmfilter(d, tc)
        && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && card::is_location(d, tc, LOCATION_REMOVED)
    {
        let c = effect::get_handler(d, a.e).nil();
        let fid = card::get_field_id(d, c) as i64;
        card::register_flag_effect(d, tc, ID, RESET_EVENT + RESETS_STANDARD, 0, 1, fid, 0);
        if duel::is_existing_matching_card(
            d,
            Some(&|d, x| card::get_flag_effect_label(d, x, ID).first() == Some(&fid)),
            a.tp,
            LOCATION_REMOVED,
            LOCATION_REMOVED,
            1,
            tc.into(),
        ) {
            return;
        }
        // Special Summon the monster(s) banished by this card's effect
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, aux::stringid(ID, 1));
        effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        effect::set_range(d, e1, LOCATION_SZONE);
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        effect::set_condition(d, e1, Some(condition(spcon)));
        effect::set_target(d, e1, Some(cost_target(sptg)));
        effect::set_operation(d, e1, Some(operation(spop)));
        effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let fid = card::get_field_id(d, c) as i64;
    duel::is_turn_player(d, a.tp)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, x| card::get_flag_effect_label(d, x, ID).first() == Some(&fid)),
            a.tp,
            LOCATION_REMOVED,
            LOCATION_REMOVED,
            1,
            NO_EX,
        )
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    let fid = card::get_field_id(d, c) as i64;
    let g = duel::get_matching_group(
        d,
        Some(&|d, x| card::get_flag_effect_label(d, x, ID).first() == Some(&fid)),
        a.tp,
        LOCATION_REMOVED,
        LOCATION_REMOVED,
        NO_EX,
    );
    let n = g.len() as u8;
    duel::set_target_card(d, g.clone());
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), n, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).nil();
    if !tg.is_empty() {
        duel::special_summon(d, tg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        let c = effect::get_handler(d, a.e).nil();
        let fid = card::get_field_id(d, c) as i64;
        if duel::is_existing_matching_card(
            d,
            Some(&|d, x| card::get_flag_effect_label(d, x, ID).first() == Some(&fid)),
            a.tp,
            LOCATION_REMOVED,
            LOCATION_REMOVED,
            1,
            NO_EX,
        ) {
            return;
        }
        effect::reset(d, a.e);
    }
}
