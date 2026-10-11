//! Madolche Fresh Sistart
use super::prelude::*;

pub const ID: u32 = 96150936;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MADOLCHE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_MADOLCHE),2,2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_MADOLCHE as u16], sc, st, Some(tp)))),
        2,
        Some(2),
        None,
        None,
        None,
    );
    // destroy replace
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target(d, e1, Some(cost_target(desreptg)));
    effect::set_operation(d, e1, Some(operation(desrepop)));
    card::register_effect(d, c, e1, false);
    // protect
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_ONFIELD, 0);
    effect::set_condition(d, e2, Some(condition(indescon)));
    effect::set_target(d, e2, Some(effect_card_fn(indestg)));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    card::register_effect(d, c, e3, false);
}
fn desrepfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_MADOLCHE as u16]) && card::is_monster(d, c) && card::is_able_to_deck(d, c) && aux::nvfilter(d, c)
}
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_RULE) && duel::is_existing_matching_card(d, Some(&desrepfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::select_effect_yes_no(d, tp, c, Some(96))
}
fn desrepop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    let g = duel::select_matching_card(d, tp, Some(&desrepfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT | REASON_REPLACE, None);
}
fn linkfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MADOLCHE as u16]) && card::is_monster(d, c)
}
fn indescon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let g = card::get_linked_group(d, c);
    group::is_exists(d, &g, &|d, x| linkfilter(d, x), 1, NO_EX)
}
fn indestg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MADOLCHE as u16]) && card::is_spell_trap(d, c)
}
