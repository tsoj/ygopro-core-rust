//! Eternal Soul
use super::prelude::*;

pub const ID: u32 = 48680970;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN, 63391643, 2314238], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE as u32));
    card::register_effect(d, c, e1, false);
    // immune effect
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_IMMUNE_EFFECT);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_target(d, e2, Some(effect_card_fn(etarget)));
    effect::set_value_fn(d, e2, lua_fn(efilter));
    card::register_effect(d, c, e2, false);
    // destroy when leaving the field
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e3, Some(condition(descon)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
    // search
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_hint_timing(d, e4, 0, Some(TIMING_END_PHASE as u32));
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    effect::set_target(d, e4, Some(cost_target(thtg)));
    effect::set_operation(d, e4, Some(operation(thop)));
    card::register_effect(d, c, e4, false);
    // special summon
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_type(d, e5, EFFECT_TYPE_QUICK_O);
    effect::set_category(d, e5, CATEGORY_SPECIAL_SUMMON);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_code(d, e5, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e5, 0, Some(TIMING_END_PHASE as u32));
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    effect::set_target(d, e5, Some(cost_target(sptg)));
    effect::set_operation(d, e5, Some(operation(spop)));
    card::register_effect(d, c, e5, false);
}
fn etarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_code(d, c, &[CARD_DARK_MAGICIAN])
}
/// `function(e,re)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let (e, re) = (a.e(0), a.e(1));
    effect::get_owner_player(d, re) != effect::get_handler_player(d, e)
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_position(d, c, POS_FACEUP) && !card::is_location(d, c, LOCATION_DECK)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, 0, NO_EX);
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, &[2314238, 63391643]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_DARK_MAGICIAN]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, desc);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let e = a.e;
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp))), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
