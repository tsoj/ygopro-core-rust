//! Artmage Academic Arcane Arts Acropolis
use super::prelude::*;

pub const ID: u32 = 74733322;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_MEDIUS_THE_PURE], listed_series: &[SET_ARTMAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // During your Main Phase, you can Normal Summon 1 "Medius the Pure" in addition to your Normal Summon/Set
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    effect::set_range(d, e1, LOCATION_FZONE);
    effect::set_target_range(d, e1, LOCATION_HAND | LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_code(d, c, &[CARD_MEDIUS_THE_PURE]))));
    card::register_effect(d, c, e1, false);
    // Add 1 declared "Artmage" monster from your Deck to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_cost(d, e2, Some(cost::discard(Some(std::sync::Arc::new(|d, c, _e, _tp| card::is_spell_trap(d, c))), false, None, None, None)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    // Track names declared to activate e2
    aux::global_check(d, ID, |d| {
        aux::add_values_reset(d, reset_declared);
    });
}
fn reset_declared(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Nil);
    set_s_var(d, ID, 1, Value::Nil);
}
/// `s.declared_names[tp]`: the codes declared so far this turn.
fn declared_names(d: &Duel, tp: u8) -> Vec<u32> {
    match s_var(d, ID, tp as u64) {
        Value::Multi(v) => v.iter().map(|x| x.int() as u32).collect(),
        _ => Vec::new(),
    }
}
fn declfilter(d: &mut Duel, c: CardId, exc1: &[u32], exc2: &[u32]) -> bool {
    card::is_set_card(d, c, &[SET_ARTMAGE as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
        && (exc1.is_empty() || !card::is_code(d, c, exc1))
        && (exc2.is_empty() || !card::is_code(d, c, exc2))
}
/// `g:GetClass(Card.GetCode)`: the distinct codes of the group, ascending (the core's std::set order).
fn get_class_codes(d: &mut Duel, g: &CardSet) -> Vec<u32> {
    let mut codes: Vec<u32> = group::iter(d, g).into_iter().map(|c| card::get_code(d, c)[0]).collect();
    codes.sort_unstable();
    codes.dedup();
    codes
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let faceup_artmage = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ARTMAGE as u16])), a.tp, LOCATION_MZONE, 0, NO_EX);
    let fcs = get_class_codes(d, &faceup_artmage);
    let declared = declared_names(d, a.tp);
    let g = duel::get_matching_group(d, Some(&|d, c| declfilter(d, c, &fcs, &declared)), a.tp, LOCATION_DECK, 0, NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let mut announce_filter: Vec<u64> = Vec::new();
    for code in get_class_codes(d, &g) {
        if announce_filter.is_empty() {
            announce_filter.push(code as u64);
            announce_filter.push(OPCODE_ISCODE);
        } else {
            announce_filter.push(code as u64);
            announce_filter.push(OPCODE_ISCODE);
            announce_filter.push(OPCODE_OR);
        }
    }
    crate::class::set_announce_filter(d, ID, &announce_filter);
    let ac = duel::announce_card(d, a.tp, &announce_filter) as u32;
    let mut names = declared;
    names.push(ac);
    set_s_var(d, ID, a.tp as u64, Value::Multi(names.iter().map(|&x| Value::Int(x as i64)).collect()));
    duel::set_target_param(d, ac);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, a.tp, ANNOUNCE_CARD_FILTER as i32);
    true
}
fn thfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    card::is_code(d, c, &[code]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let code = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, code)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
    if !duel::has_flag_effect(d, a.tp, ID, None) {
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Cannot Special Summon from outside the Extra Deck for the rest of this turn, except "Artmage" monsters and "Medius the Pure"
        let c = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, aux::stringid(ID, 2));
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        effect::set_target_range(d, e1, 1, 0);
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| {
            !(card::is_set_card(d, c, &[SET_ARTMAGE as u16]) || card::is_code(d, c, &[CARD_MEDIUS_THE_PURE]) || card::is_location(d, c, LOCATION_EXTRA))
        })));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, a.tp);
    }
}
