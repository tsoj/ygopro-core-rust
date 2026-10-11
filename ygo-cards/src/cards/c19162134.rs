//! Dueltaining
use super::prelude::*;

pub const ID: u32 = 19162134;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Draw 2 cards (Special Summon 5 monsters)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e2, LOCATION_FZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(|d, a| spcon(d, a, 0))));
    effect::set_operation(d, e2, Some(operation(|d, a| draw(d, a.tp ^ 0))));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_condition(d, e3, Some(condition(|d, a| spcon(d, a, 1))));
    effect::set_operation(d, e3, Some(operation(|d, a| draw(d, a.tp ^ 1))));
    card::register_effect(d, c, e3, false);
    // Draw 2 cards (Battled 5 times)
    let e4 = effect::clone(d, e2);
    effect::set_code(d, e4, EVENT_BATTLED);
    effect::set_condition(d, e4, Some(condition(|d, a| btcon(d, a, 0))));
    card::register_effect(d, c, e4, false);
    let e5 = effect::clone(d, e3);
    effect::set_code(d, e5, EVENT_BATTLED);
    effect::set_condition(d, e5, Some(condition(|d, a| btcon(d, a, 1))));
    card::register_effect(d, c, e5, false);
    // Draw 2 cards (Chain Link 5 or higher)
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e0, EVENT_CHAINING);
    effect::set_range(d, e0, LOCATION_FZONE);
    effect::set_operation(d, e0, Some(aux::chainreg()));
    card::register_effect(d, c, e0, false);
    let e6 = effect::clone(d, e2);
    effect::set_code(d, e6, EVENT_CHAIN_SOLVING);
    effect::set_condition(d, e6, Some(condition(|d, a| chcon(d, a, 0))));
    card::register_effect(d, c, e6, false);
    let e7 = effect::clone(d, e3);
    effect::set_code(d, e7, EVENT_CHAIN_SOLVING);
    effect::set_condition(d, e7, Some(condition(|d, a| chcon(d, a, 1))));
    card::register_effect(d, c, e7, false);
    // Draw 2 cards (Coin/dice tossed 5 times)
    let e8 = effect::create_effect(d, c);
    effect::set_type(d, e8, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e8, EVENT_TOSS_COIN);
    effect::set_property(d, e8, EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e8, LOCATION_FZONE);
    effect::set_operation(d, e8, Some(operation(tossop)));
    card::register_effect(d, c, e8, false);
    let e9 = effect::clone(d, e8);
    effect::set_code(d, e9, EVENT_TOSS_DICE);
    effect::set_operation(d, e9, Some(operation(diceop)));
    card::register_effect(d, c, e9, false);
    // Draw 2 cards (Damage made LP 500 or lower)
    let ea = effect::clone(d, e2);
    effect::set_code(d, ea, EVENT_DAMAGE);
    effect::set_condition(d, ea, Some(condition(|d, a| damcon(d, a, 0))));
    card::register_effect(d, c, ea, false);
    let eb = effect::clone(d, e3);
    effect::set_code(d, eb, EVENT_DAMAGE);
    effect::set_condition(d, eb, Some(condition(|d, a| damcon(d, a, 1))));
    card::register_effect(d, c, eb, false);
}
fn draw(d: &mut Duel, tp: u8) {
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    duel::draw(d, tp, 2, REASON_EFFECT);
}
fn spcon(d: &mut Duel, a: &EffArgs, p: u8) -> bool {
    let eg = a.eg.clone().nil();
    let tp = a.tp ^ p;
    eg.len() == 5
        && group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, tp), 1, NO_EX)
        && group::get_class_count(d, &eg, &|d, c| card::get_level(d, c) as i64) == 5
}
fn btcon(d: &mut Duel, a: &EffArgs, p: u8) -> bool {
    let (battle_monster, _) = duel::get_battle_monster(d, a.tp ^ p);
    if let Some(bm) = battle_monster {
        card::register_flag_effect(d, bm, ID, RESET_EVENT | RESETS_STANDARD | RESET_CONTROL | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
        card::get_flag_effect(d, bm, ID) == 5
    } else {
        false
    }
}
fn chcon(d: &mut Duel, a: &EffArgs, p: u8) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    a.rp == (a.tp ^ p) && duel::get_current_chain(d, false) >= 5 && card::get_flag_effect(d, c, 1) > 0
}
fn register_flag(d: &mut Duel, c: CardId, code: u32) {
    card::register_flag_effect(d, c, code, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
}
fn tossop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let count = aux::get_coin_count_from_ev(a.ev);
    let flag_id = ID + if a.ep == a.tp { 1 } else { 2 };
    for _ in 0..count {
        register_flag(d, c, flag_id);
    }
    if card::get_flag_effect(d, c, flag_id) >= 5 && card::get_flag_effect(d, c, flag_id + 2) == 0 {
        draw(d, a.ep);
        register_flag(d, c, flag_id + 2);
    }
}
fn diceop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ct1 = aux::get_dice_count_self_from_ev(a.ev);
    let ct2 = aux::get_dice_count_oppo_from_ev(a.ev);
    if a.ep == a.tp {
        for _ in 0..ct1 {
            register_flag(d, c, ID + 1);
        }
        for _ in 0..ct2 {
            register_flag(d, c, ID + 2);
        }
    } else {
        for _ in 0..ct2 {
            register_flag(d, c, ID + 1);
        }
        for _ in 0..ct1 {
            register_flag(d, c, ID + 2);
        }
    }
    if card::get_flag_effect(d, c, ID + 1) >= 5 && card::get_flag_effect(d, c, ID + 3) == 0 {
        draw(d, a.tp);
        register_flag(d, c, ID + 3);
    }
    if card::get_flag_effect(d, c, ID + 2) >= 5 && card::get_flag_effect(d, c, ID + 4) == 0 {
        draw(d, a.tp ^ 1);
        register_flag(d, c, ID + 4);
    }
}
fn damcon(d: &mut Duel, a: &EffArgs, p: u8) -> bool {
    let tp = a.tp ^ p;
    a.ep == tp && duel::get_lp(d, tp) <= 500 && duel::get_lp(d, tp) > 0
}
