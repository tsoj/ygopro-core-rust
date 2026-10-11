//! Dark Guardian
//! 闇の守護神－ダーク・ガーディアン
use super::prelude::*;

pub const ID: u32 = 26746975;

const CARD_DARK_ELEMENT: u32 = 53194323;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_SANGA_OF_THE_THUNDER, CARD_KAZEJIN, CARD_SUIJIN, CARD_DARK_ELEMENT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Must be Special Summoned with "Dark Element" or its own procedure
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e0, false);
    // Special Summon procedure
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    effect::set_condition(d, e1, Some(lua_fn(spproccon)));
    effect::set_target(d, e1, Some(cost_target(spproctg)));
    effect::set_operation(d, e1, Some(operation(spprocop)));
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by battle
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    // Register if it's Special Summoned with "Dark Element"
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_operation(d, e3, Some(operation(regop)));
    card::register_effect(d, c, e3, false);
    // Unaffected by other monsters' effects and your opponent's activated Spell effects
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e4, EFFECT_IMMUNE_EFFECT);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_condition(d, e4, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::has_flag_effect(d, h, ID, None)
    })));
    effect::set_value_fn(d, e4, lua_fn(|d, a: Args| immvalue(d, a.e(0), a.e(1))));
    card::register_effect(d, c, e4, false);
}
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_code(d, c, CARDS_SANGA_KAZEJIN_SUIJIN)
        && card::is_able_to_deck_as_cost(d, c)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND | LOCATION_GRAVE))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let ok = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0
        && group::is_exists(d, sg, &|d, c| card::is_code(d, c, &[CARD_SANGA_OF_THE_THUNDER]), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_code(d, c, &[CARD_KAZEJIN]), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_code(d, c, &[CARD_SUIJIN]), 1, NO_EX);
    (ok, false)
}
/// `s.spproccon(e,c)`
fn spproccon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    if a.card(1).is_none() {
        return true;
    }
    let tp = effect::get_handler_player(d, e);
    let h = effect::get_handler(d, e).nil();
    let rg = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_ONFIELD | LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED, 0, h.into());
    matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
/// `s.spproctg(e,tp,eg,ep,ev,re,r,rp,c)`: the Lua parameter `c` is the number 1 here.
fn spproctg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    let rg = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_ONFIELD | LOCATION_HAND | LOCATION_GRAVE | LOCATION_REMOVED, 0, h.into());
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(3), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_TODECK as u64), None, None, true).unwrap_or_default();
    if !g.is_empty() {
        // g:KeepAlive() does nothing in Rust (groups are values)
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    false
}
/// `s.spprocop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spprocop(d: &mut Duel, a: &EffArgs) {
    let Some(g) = effect::get_label_object(d, a.e).as_group().cloned() else { return };
    duel::hint_selection(d, g.clone(), true);
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    // g:DeleteGroup() does nothing in Rust (groups are values)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_code(d, rh, &[CARD_DARK_ELEMENT]) {
        let h = effect::get_handler(d, a.e).nil();
        card::register_flag_effect(d, h, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TEMP_REMOVE), EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 1));
    }
}
/// `s.immvalue(e,te)`
fn immvalue(d: &mut Duel, e: EffectId, te: EffectId) -> bool {
    (effect::is_monster_effect(d, te) && effect::get_owner(d, te) != effect::get_owner(d, e))
        || (effect::is_activated(d, te)
            && effect::is_spell_effect(d, te)
            && effect::get_owner_player(d, te) != effect::get_handler_player(d, e))
}
