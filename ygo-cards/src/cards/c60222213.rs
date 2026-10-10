//! Raphion, the Timelord
use super::prelude::*;

pub const ID: u32 = 60222213;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_range(d, e1, LOCATION_DECK);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    card::register_effect(d, c, e2, false);
    // indes
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
    let e4 = effect::clone(d, e3);
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
    card::register_effect(d, c, e4, false);
    // damage val
    let e5 = effect::clone(d, e3);
    effect::set_code(d, e5, EFFECT_AVOID_BATTLE_DAMAGE);
    card::register_effect(d, c, e5, false);
    // LP
    let e6 = effect::create_effect(d, c);
    effect::set_category(d, e6, CATEGORY_DAMAGE);
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e6, EVENT_PHASE | PHASE_BATTLE);
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    effect::set_range(d, e6, LOCATION_MZONE);
    effect::set_condition(d, e6, Some(condition(damcon)));
    effect::set_target(d, e6, Some(cost_target(damtg)));
    effect::set_operation(d, e6, Some(operation(damop)));
    card::register_effect(d, c, e6, false);
    // to deck
    let e7 = effect::create_effect(d, c);
    effect::set_description(d, e7, aux::stringid(ID, 1));
    effect::set_category(d, e7, CATEGORY_TODECK);
    effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e7, EVENT_PHASE | PHASE_STANDBY);
    effect::set_property(d, e7, EFFECT_FLAG_REPEAT, 0);
    effect::set_count_limit(d, e7, 1, 0, 0, 0);
    effect::set_range(d, e7, LOCATION_MZONE);
    effect::set_condition(d, e7, Some(condition(tdcon)));
    effect::set_target(d, e7, Some(cost_target(tdtg)));
    effect::set_operation(d, e7, Some(operation(tdop)));
    card::register_effect(d, c, e7, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else {
        return true;
    };
    let tp = card::get_controler(d, c);
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.damfilter(c,e)`
fn damfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    let h = effect::get_handler(d, e).nil();
    card::is_faceup(d, c) && group::is_contains(&card::get_battled_group(d, h), c)
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_battled_group_count(d, h) > 0
}
fn damtg(_d: &mut Duel, _a: &EffArgs) -> bool {
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let e = a.e;
    let mut g = duel::get_matching_group(d, Some(&|d, x| damfilter(d, x, e)), tp, 0, LOCATION_MZONE, NO_EX);
    if g.len() > 1 {
        g = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    }
    let tc = group::get_first(d, &g);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, c, a.e) {
            let atk = card::get_attack(d, tc);
            duel::damage(d, 1 - tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::is_able_to_deck(d, c) {
        duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
