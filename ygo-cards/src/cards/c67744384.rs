//! Tindangle Protector
use super::prelude::*;

pub const ID: u32 = 67744384;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TINDANGLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change monsters to face-up Defense position and search "Tindangle" cards
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_facedown(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_POSITION, None, 1, 0, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_TINDANGLE as u16]) && card::is_able_to_hand(d, c, None)
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_MZONE, 0, 1, 3, false, NO_EX).nil();
    if g.len() > 0 {
        let ct = duel::change_position(d, g.clone(), POS_FACEUP_DEFENSE as u8, None, None, None, false);
        if ct as usize == g.len() && group::is_exists(d, &g, &|d, c| card::is_set_card(d, c, &[SET_TINDANGLE as u16]), ct as u16, NO_EX) {
            let sg = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
            if sg.len() > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
                duel::break_effect(d);
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
                let tg = group::select(d, &sg, tp, 1, ct as u16, false, NO_EX).nil();
                duel::send_to_hand(d, tg.clone(), Some(tp), REASON_EFFECT, None);
                duel::confirm_cards(d, 1 - tp, tg);
            }
        }
    }
}
