//! Scrounging Goblin
use super::prelude::*;

pub const ID: u32 = 12152769;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOBLIN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // search
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep != a.tp
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_GOBLIN as u16]) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let hg = duel::get_field_group(d, 1 - tp, LOCATION_HAND, 0);
    if !hg.is_empty() && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 1)) {
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, aux::stringid(ID, 2));
        let sg = group::select(d, &hg, 1 - tp, 1, 1, false, NO_EX).nil();
        duel::send_to_hand(d, &sg, Some(tp), REASON_EFFECT, None);
        if duel::is_chain_disablable(d, 0) {
            duel::negate_effect(d, 0);
            return;
        }
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
