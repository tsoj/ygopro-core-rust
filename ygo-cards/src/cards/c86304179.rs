//! Tempura of Fortune - EBI
use super::prelude::*;

pub const ID: u32 = 86304179;
pub const INFO: ScriptInfo = ScriptInfo { toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon procedure
    pendulum::add_procedure(d, c, None, None);
    // Toss a coin, and Special Summon or destroy 1 card in your Pendulum Zone
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_COIN + CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(cointg)));
    effect::set_operation(d, e1, Some(operation(coinop)));
    card::register_effect(d, c, e1, false);
}
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_PZONE) && card::is_controler(d, chkc, tp);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_PZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, tp, None, tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, tp, 1);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g.clone(), 1, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    true
}
fn coinop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        let res = duel::toss_coin(d, tp, 1)[0];
        if res == COIN_HEADS as i32 {
            duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
        } else if res == COIN_TAILS as i32 {
            if duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) > 0 {
                let lp = duel::get_lp(d, tp);
                let sc = card::get_scale(d, tc) as i32;
                duel::set_lp(d, tp, lp - sc * 300);
            }
        }
    }
}
