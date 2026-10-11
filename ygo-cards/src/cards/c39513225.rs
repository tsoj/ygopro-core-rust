//! Seventh Barian's
use super::prelude::*;

pub const ID: u32 = 39513225;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER, SET_UMBRAL_HORROR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Each player can only Special Summon non-"Number" monsters from the Extra Deck twice per turn while this card is face-up on the field
    let e1a = effect::create_effect(d, c);
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1a, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1a, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_range(d, e1a, LOCATION_FZONE);
    effect::set_target_range(d, e1a, 1, 1);
    effect::set_target(d, e1a, Some(lua_fn(e1a_target)));
    card::register_effect(d, c, e1a, false);
    let e1b = effect::create_effect(d, c);
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e1b, LOCATION_FZONE);
    effect::set_operation(d, e1b, Some(operation(e1b_op)));
    card::register_effect(d, c, e1b, false);
    let e1c = effect::create_effect(d, c);
    effect::set_type(d, e1c, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1c, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1c, EFFECT_LEFT_SPSUMMON_COUNT);
    effect::set_range(d, e1c, LOCATION_FZONE);
    effect::set_target_range(d, e1c, 1, 1);
    effect::set_value_fn(d, e1c, lua_fn(e1c_value));
    card::register_effect(d, c, e1c, false);
    // During your Main Phase: You can add 1 "Umbral Horror" monster from your Deck to your hand, then discard 1 card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_HANDES);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // During the End Phase: Each player takes 400 damage for each Xyz Monster on the field
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e3, LOCATION_FZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_target(d, e3, Some(cost_target(e3_target)));
    effect::set_operation(d, e3, Some(operation(e3_op)));
    card::register_effect(d, c, e3, false);
}
/// `function(e,c,summon_player,sumtype,sumpos,targetp,se)`
fn e1a_target(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let summon_player = a.int(2) as u32;
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_location(d, c, LOCATION_EXTRA) && !card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::has_flag_effect(d, h, ID + summon_player, Some(2))
}
fn e1b_op(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg();
    let h = effect::get_handler(d, a.e).nil();
    for sc in group::iter(d, &eg) {
        if card::is_summon_location(d, sc, LOCATION_EXTRA) && (!card::is_set_card(d, sc, &[SET_NUMBER as u16]) || card::is_facedown(d, sc)) {
            let summon_player = card::get_summon_player(d, sc) as u32;
            card::register_flag_effect(d, h, ID + summon_player, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        }
    }
}
/// `function(e,re,tp) return math.max(2-e:GetHandler():GetFlagEffect(id+tp),0) end`
fn e1c_value(d: &mut Duel, a: Args) -> i64 {
    let tp = a.p(2) as u32;
    let h = effect::get_handler(d, a.e(0)).nil();
    (2 - card::get_flag_effect(d, h, ID + tp) as i64).max(0)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_UMBRAL_HORROR as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() || duel::send_to_hand(d, &g, None, REASON_EFFECT, None) == 0 {
        return;
    }
    duel::confirm_cards(d, 1 - tp, &g);
    duel::shuffle_hand(d, tp);
    if duel::get_field_group_count(d, tp, LOCATION_HAND, 0) > 0 {
        duel::break_effect(d);
        duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
fn e3_target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let dam = 400 * duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_xyz_monster(d, c))), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as i32;
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, PLAYER_ALL as u8, dam);
    true
}
fn e3_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let dam = 400 * duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, c| card::is_xyz_monster(d, c))), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as i64;
    if dam > 0 {
        duel::damage(d, tp, dam, REASON_EFFECT, true, None);
        duel::damage(d, 1 - tp, dam, REASON_EFFECT, true, None);
        duel::rd_complete(d);
    }
}
