//! Hero Counterattack
use super::prelude::*;

pub const ID: u32 = 19024706;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTAL_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]) && card::get_previous_controler(d, c) == tp
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16])), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    let sg = group::random_select(d, &g, 1 - a.tp, 1);
    let tc = group::get_first(d, &sg);
    if let Some(tc) = tc {
        duel::confirm_cards(d, 1 - a.tp, CardSet::from_card(tc));
        if card::is_set_card(d, tc, &[SET_ELEMENTAL_HERO as u16]) && card::is_monster(d, tc) {
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
            let dg = duel::select_matching_card(d, a.tp, None, a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
            duel::destroy(d, dg, REASON_EFFECT, None, None);
            if duel::special_summon(d, CardSet::from_card(tc), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
                duel::shuffle_hand(d, a.tp);
            }
        } else {
            duel::shuffle_hand(d, a.tp);
        }
    }
}
