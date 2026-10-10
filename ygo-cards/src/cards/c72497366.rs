//! Sinister Yorishiro
use super::prelude::*;

pub const ID: u32 = 72497366;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // decrease tribute
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_HAND, 0);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(lua_fn(ntcon)));
    effect::set_target(d, e2, Some(aux::field_summon_proc_tg(Some(effect_card_fn(nttg)), None)));
    card::register_effect(d, c, e2, false);
    // destroy replace
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_target(d, e3, Some(cost_target(reptg)));
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
}
/// `s.ntcon(e,c,minc)`
fn ntcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return true };
    let minc = a.int(2);
    minc == 0 && duel::get_location_count(d, card::get_controler(d, c), LOCATION_MZONE as u8, None, None, None).0 > 0
}
fn nttg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_level_above(d, c, 5) && card::is_race(d, c, RACE_FIEND)
}
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg);
    if a.chk == Some(0) {
        return eg.len() == 1
            && card::is_faceup(d, tc.nil())
            && card::is_location(d, tc.nil(), LOCATION_MZONE)
            && card::is_race(d, tc.nil(), RACE_FIEND)
            && card::is_level_above(d, tc.nil(), 5)
            && card::is_normal_summoned(d, tc.nil())
            && !card::is_reason(d, tc.nil(), REASON_REPLACE);
    }
    let c = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        duel::send_to_grave(d, c, REASON_EFFECT, None, None);
        return true;
    }
    false
}
