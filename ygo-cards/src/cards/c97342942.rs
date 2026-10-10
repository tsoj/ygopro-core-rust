//! Ectoplasmer
use super::prelude::*;

pub const ID: u32 = 97342942;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // The turn player must Tribute 1 face-up monster, and if they do, inflict damage to their opponent equal to half the original ATK of the Tributed monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_RELEASE | CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(tribtg)));
    effect::set_operation(d, e1, Some(operation(tribop)));
    card::register_effect(d, c, e1, false);
}
fn tribtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let turn_player = duel::get_turn_player(d);
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, None, 1, turn_player, LOCATION_MZONE as i32);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - turn_player, 0);
    true
}
fn tribop(d: &mut Duel, _a: &EffArgs) {
    let turn_player = duel::get_turn_player(d);
    let f = aux::faceup_filter(|d, c| card::is_releasable_by_effect(d, c, None, None));
    let g = duel::select_release_group(d, turn_player, Some(&f), 1, 1, duel::ReleaseArgs::default(), NO_EX, false).nil();
    let Some(sc) = group::get_first(d, &g) else {
        return;
    };
    duel::hint_selection(d, sc, true);
    if duel::release(d, sc, REASON_EFFECT, None) > 0 {
        let atk = card::get_text_attack(d, sc) as f64 / 2.0;
        if atk > 0.0 {
            duel::damage(d, 1 - turn_player, lua_int(atk), REASON_EFFECT, false, None);
        }
    }
}
