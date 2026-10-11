//! Spright Red
use super::prelude::*;

pub const ID: u32 = 75922381;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon self
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    card::register_effect(d, c, e1, false);
    // Negate monster effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(discon)));
    effect::set_cost(d, e2, Some(cost_target(discost)));
    effect::set_target(d, e2, Some(cost_target(distg)));
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
}
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && (card::is_level(d, c, &[2]) || card::is_link(d, c, &[2]))
}
/// `function(e,c)`: the core checks the effect alone with no card
fn spcon(d: &mut Duel, a: Args) -> bool {
    let Some(_c) = a.card(1) else {
        return true;
    };
    let tp = effect::get_handler_player(d, a.e(0));
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && duel::is_chain_disablable(d, a.ev as u8)
}
fn discostfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_level(d, c, &[2]) || card::is_rank(d, c, &[2]) || card::is_link(d, c, &[2])
}
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&discostfilter), 1, 1, false, None, c.into());
    }
    let g = duel::select_release_group_cost(d, a.tp, Some(&discostfilter), 1, 1, false, None, c.into());
    let rc = group::get_first(d, &g).nil();
    let level = card::get_level(d, rc) as i64;
    effect::set_label(d, a.e, &[level]);
    duel::release(d, rc, REASON_COST, None);
    true
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        return !card::is_disabled(d, rc);
    }
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    if duel::negate_effect(d, a.ev as u8) {
        let re = a.re.nil();
        let h = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, h, re)
            && effect::get_label(d, a.e) == 0
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
        {
            duel::break_effect(d);
            duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
        }
    }
}
