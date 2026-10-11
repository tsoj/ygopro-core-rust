//! Timestar Magician
use super::prelude::*;

pub const ID: u32 = 47349116;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,s.matfilter,4,2,nil,nil,nil,nil,true)
    xyz::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| matfilter(d, x, sc, st, tp))), Some(4), 2, None, None, None, None, true, None);
    // spsummon condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, lua_fn(splimit));
    card::register_effect(d, c, e1, false);
    // Search
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // destroy replace
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_target(d, e3, Some(cost_target(reptg)));
    effect::set_value_fn(d, e3, effect_card_fn(repval));
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter(c,xyz,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, xyz: Option<CardId>, st: u64, tp: u8) -> bool {
    card::is_set_card_ctx(d, c, &[SET_MAGICIAN as u16], xyz, st, Some(tp)) && card::is_type_ctx(d, c, TYPE_PENDULUM, xyz, st, Some(tp))
}
/// `s.splimit(e,se,sp,st)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let se = a.v(1).as_effect();
    let st = a.int(3) as u64;
    let h = effect::get_handler(d, e).nil();
    !card::is_location(d, h, LOCATION_EXTRA) || (st & SUMMON_TYPE_XYZ as u64) == SUMMON_TYPE_XYZ as u64 && se.is_none()
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_SPELLCASTER as u64)
        && card::is_attribute(d, c, ATTRIBUTE_DARK)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_EXTRA))
        && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK | LOCATION_GRAVE | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, x| aux::necro_valley_filter(thfilter)(d, x)),
        tp,
        LOCATION_DECK | LOCATION_GRAVE | LOCATION_EXTRA,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let tc = group::get_first(d, &g);
    if tc.is_none() {
        return;
    }
    let tc = tc.nil();
    duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    duel::confirm_cards(d, 1 - tp, tc);
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_MZONE | LOCATION_PZONE)
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_SPELLCASTER as u64) && card::is_able_to_grave(d, c, None)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return group::is_exists(d, &eg, &|d, x| repfilter(d, x, tp), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        let sg = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
        return true;
    }
    false
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
