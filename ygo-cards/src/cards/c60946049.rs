//! Artmage Graflare
use super::prelude::*;

pub const ID: u32 = 60946049;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARTMAGE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // The first time this card would be destroyed by battle each turn, it is not destroyed
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_value_fn(d, e1, lua_fn(indes_value));
    card::register_effect(d, c, e1, false);
    // Special Summon this card from your hand, then you can Set 1 "Artmage" Spell from your Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Destroy 1 Spell/Trap your opponent controls
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(e3_cond)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
    // This is a Quick Effect if you control 3 or more Monster Types
    let e3a = effect::clone(d, e3);
    effect::set_type(d, e3a, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3a, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e3a, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_condition(d, e3a, Some(condition(quickdescon)));
    card::register_effect(d, c, e3a, false);
}
/// `function(e,re,r) return (r&REASON_BATTLE)==REASON_BATTLE end`
fn indes_value(_d: &mut Duel, a: Args) -> bool {
    (a.int(2) as u32 & REASON_BATTLE) == REASON_BATTLE
}
fn is_artmage_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ARTMAGE as u16])
}
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&is_artmage_faceup), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ARTMAGE as u16]) && card::is_spell(d, c) && card::is_s_setable(d, c, false, None)
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    let g = duel::get_matching_group(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    if g.is_empty() || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    if !sg.is_empty() {
        duel::break_effect(d);
        duel::s_set(d, a.tp, sg, None, true);
    }
}
fn e3_cond(d: &mut Duel, a: &EffArgs) -> bool {
    !quickdescon(d, a)
}
fn quickdescon(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX);
    group::get_bin_class_count(d, &g, &|d, c| card::get_race(d, c) as i64) >= 3
}
fn spelltrap(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && card::is_spell_trap(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&spelltrap), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, Some(&spelltrap), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
