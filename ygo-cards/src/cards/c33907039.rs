//! Piri Reis Map
use super::prelude::*;

pub const ID: u32 = 33907039;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Search 1 monster with 0 ATK
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_phase(d, PHASE_MAIN1) && !duel::check_phase_activity(d)
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_attack(d, c, &[0]) && card::is_able_to_hand(d, c, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g.clone());
        let tc = group::get_first(d, &g).nil();
        if card::is_location(d, tc, LOCATION_HAND) {
            let lp = duel::get_lp(d, tp);
            duel::set_lp(d, tp, (lp as f64 / 2.0).ceil() as i32);
            let c = effect::get_handler(d, a.e).nil();
            //Check Normal Summon for matching name
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
            let code = card::get_code(d, tc)[0];
            effect::set_label(d, e1, &[code as i64]);
            effect::set_operation(d, e1, Some(operation(checkop)));
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
            duel::register_effect(d, e1, tp);
            //Cannot activate effects of monsters with the same name
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_FIELD);
            effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
            effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
            effect::set_target_range(d, e2, 1, 0);
            effect::set_value_fn(d, e2, lua_fn(aclimit));
            effect::set_label_object(d, e2, Value::Effect(e1));
            effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 2);
            duel::register_effect(d, e2, tp);
        }
    }
}
fn checkop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    let sc = group::get_first(d, &eg).nil();
    if card::is_summon_player(d, sc, 1 - a.tp) {
        return;
    }
    let label = effect::get_label(d, a.e);
    if card::is_code(d, sc, &[label as u32]) {
        effect::set_label(d, a.e, &[-1]);
    }
}
fn aclimit(d: &mut Duel, a: Args) -> bool {
    let lo = effect::get_label_object(d, a.e(0)).as_effect().nil();
    let label = effect::get_label(d, lo);
    let h = effect::get_handler(d, a.e(1)).nil();
    card::is_code(d, h, &[label as u32])
}
