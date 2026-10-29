//! Crusadia Regulex
use super::prelude::*;

pub const ID: u32 = 9617996;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CRUSADIA as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Link Summon Procedure
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_type_ctx(d, x, TYPE_EFFECT, sc, st, Some(tp)))),
        2,
        Some(2),
        Some(group_check(|d, g, lc, st, tp| lcheck(d, g, lc, st, tp))),
        None,
        None,
    );
    // Gains ATK equal to the ATK of the monsters it points to
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_value(atkval));
    card::register_effect(d, c, e1, false);
    // Monsters this card points to cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(atklimit)));
    card::register_effect(d, c, e2, false);
    // Search 1 "Crusadia" Spell/Trap
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_condition(d, e3, Some(aux::zptcon(Some(zpt_filter))));
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
}
/// `aux.FilterBoolFunction(Card.IsType,TYPE_EFFECT)` of the zone point condition
fn zpt_filter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    card::is_type(d, c, TYPE_EFFECT)
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    group::is_exists(d, g, &|d, c| card::is_set_card_ctx(d, c, &[SET_CRUSADIA as u16], Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    let h = effect::get_handler(d, e).nil();
    let lg = card::get_linked_group(d, h);
    let g = group::filter(d, &lg, &|d, c| card::is_faceup(d, c), NO_EX);
    group::get_sum(d, &g, &|d, x| card::get_base_attack(d, x) as i64)
}
/// `s.atklimit(e,c)`
fn atklimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let h = effect::get_handler(d, e).nil();
    card::get_linked_group(d, h).contains(c)
}
/// `s.thfilter(c,tp)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_CRUSADIA as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g1 = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g1.is_empty() {
        duel::send_to_hand(d, &g1, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g1);
    }
}
