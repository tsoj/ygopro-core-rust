//! Rinyan, Lightsworn Rogue
use super::prelude::*;

pub const ID: u32 = 38815069;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // flip
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_DRAW);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]) && card::is_monster(d, c) && card::is_able_to_deck(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return true;
    }
    if duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
        let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
        duel::set_operation_info(d, 0, CATEGORY_TODECK, g, 1, 0, 0);
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        let e = a.e;
        if card::is_relate_to_effect(d, tc, e) {
            if duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) > 0
                && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA)
            {
                if card::is_location(d, tc, LOCATION_DECK) {
                    duel::shuffle_deck(d, a.tp);
                }
                duel::draw(d, a.tp, 1, REASON_EFFECT);
            }
        }
    }
}
