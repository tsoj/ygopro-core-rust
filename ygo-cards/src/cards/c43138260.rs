//! Xyz Remora
use super::prelude::*;

pub const ID: u32 = 43138260;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon itself from hand
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // If summoned with above effect, special summon 2 level 4 fish monsters from GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e2, Some(condition(spcon2)));
    effect::set_target(d, e2, Some(cost_target(sptg2)));
    effect::set_operation(d, e2, Some(operation(spop2)));
    card::register_effect(d, c, e2, false);
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return true };
    let p = card::get_controler(d, c);
    duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::check_remove_overlay_card(d, p, 1, 0, 2, REASON_COST, None)
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args) {
    let tp = a.p(1);
    duel::remove_overlay_card(d, tp, 1, 0, 2, 2, REASON_COST, None);
}
// function s.spcon2(e,tp,eg,ep,ev,re,r,rp)
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_summon_type(d, h) == SUMMON_TYPE_SPECIAL + 1
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_race(d, c, RACE_FISH) && card::get_level(d, c) == 4 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// function s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && spfilter(d, chkc, a.e, a.tp);
    }
    if a.chk == Some(0) {
        return !duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 2, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 2, 2, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 2, 0, 0);
    true
}
// function s.xyzlimit(e,c)
fn xyzlimit(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return false };
    !card::is_attribute(d, c, ATTRIBUTE_WATER)
}
// function s.spop2(e,tp,eg,ep,ev,re,r,rp)
fn spop2(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_target_cards(d, a.e).nil();
    if g.is_empty() {
        return;
    }
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < g.len() as i32
        || (g.len() > 1 && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some())
    {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    let _ = group::get_first(d, &g);
    for tc in group::iter(d, &g) {
        duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        // Cannot attack
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3206);
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
        // Negate their effects
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e2, EFFECT_DISABLE);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e2, false);
        let e3 = effect::create_effect(d, c);
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e3, false);
        // Cannot change their battle positions
        let e4 = effect::create_effect(d, c);
        effect::set_description(d, e4, 3313);
        effect::set_property(d, e4, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e4, EFFECT_CANNOT_CHANGE_POSITION);
        effect::set_reset(d, e4, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e4, false);
        // Cannot be used as Xyz material, except for a WATER monster
        let e5 = effect::create_effect(d, c);
        effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e5, EFFECT_CANNOT_BE_XYZ_MATERIAL);
        effect::set_value_fn(d, e5, lua_fn(xyzlimit));
        effect::set_reset(d, e5, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e5, false);
    }
    duel::special_summon_complete(d);
}
