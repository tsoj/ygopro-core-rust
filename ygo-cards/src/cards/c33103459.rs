//! Performapal Corn
use super::prelude::*;

pub const ID: u32 = 33103459;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_PERFORMAPAL, SET_ODD_EYES], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change both this card and 1 other "Perfomapal" monster you control to Defense Position, and if you do, add 1 "Odd-Eyes" monster from your Deck to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_POSITION | CATEGORY_TOHAND | CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_status(d, h, STATUS_SUMMON_TURN) || card::is_status(d, h, STATUS_SPSUMMON_TURN)
    })));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Gain 500 LP
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_RECOVER);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    effect::set_cost(d, e2, Some(cost_target(reccost)));
    effect::set_target(d, e2, Some(cost_target(rectg)));
    effect::set_operation(d, e2, Some(operation(recop)));
    card::register_effect(d, c, e2, false);
}
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_PERFORMAPAL as u16])
        && card::is_attack_below(d, c, 1000)
        && card::is_position(d, c, POS_FACEUP_ATTACK as u32)
        && card::is_can_change_position(d, c)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ODD_EYES as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, c) = (a.tp, effect::get_handler(d, a.e).nil());
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && posfilter(d, chkc) && chkc != c;
    }
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX)
            && card::is_position(d, c, POS_FACEUP_ATTACK as u32)
            && card::is_can_change_position(d, c)
            && duel::is_existing_target(d, Some(&posfilter), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_POSCHANGE as u64);
    let g = duel::select_target(d, tp, Some(&posfilter), tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    let mut both = g.nil();
    group::add_card(&mut both, &CardSet::from_card(c));
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(both), 2, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let (tp, c) = (a.tp, effect::get_handler(d, a.e).nil());
    if card::is_relate_to_effect(d, c, a.e) && card::is_position(d, c, POS_FACEUP_ATTACK as u32) && card::is_controler(d, c, tp) {
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::is_position(d, tc, POS_FACEUP_ATTACK as u32) && card::is_controler(d, tc, tp) {
            let mut both = CardSet::from_card(c);
            group::add_card(&mut both, &CardSet::from_card(tc));
            if duel::change_position(d, both, POS_FACEUP_DEFENSE as u8, None, None, None, false) == 2 {
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
                let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
                if let Some(g) = g {
                    if !g.is_empty() {
                        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
                        duel::confirm_cards(d, 1 - tp, g);
                    }
                }
            }
        }
    }
}
fn cfilter(d: &mut Duel, c: CardId, id: u32) -> bool {
    card::is_set_card(d, c, &[SET_PERFORMAPAL as u16])
        && card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, true, false)
        && !card::is_code(d, c, &[id])
}
fn reccost(d: &mut Duel, a: &EffArgs) -> bool {
    let (tp, c) = (a.tp, effect::get_handler(d, a.e).nil());
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, ID)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let mut g = duel::select_matching_card(d, tp, Some(&|d, x| cfilter(d, x, ID)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    group::add_card(&mut g, &CardSet::from_card(c));
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn rectg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, tp);
    duel::set_target_param(d, 500);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, 500);
    true
}
fn recop(d: &mut Duel, _a: &EffArgs) {
    let v = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::recover(d, v[0].int() as u8, v[1].int() as i64, REASON_EFFECT, false, None);
}
