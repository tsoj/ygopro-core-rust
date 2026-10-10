//! Blaze Cannon
//! Logical Nonsense
use super::prelude::*;

pub const ID: u32 = 4059313;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Grant multiple effects to 1 "The Winged Dragon of Ra"
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_INACTIVATE + EFFECT_FLAG_CANNOT_NEGATE, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, TIMING_STANDBY_PHASE | TIMING_BATTLE_START, None);
    card::register_effect(d, c, e1, false);
}
/// `s.selfilter(c)`: face-up "The Winged Dragon of Ra" without this card's flag
fn selfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_RA]) && card::get_flag_effect(d, c, ID) == 0
}
/// Activation legality
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&selfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// Grant multiple effects to 1 "The Winged Dragon of Ra"
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_APPLYTO as u64);
    let sg = duel::select_matching_card(d, tp, Some(&selfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = match group::get_first(d, &sg) {
        Some(tc) => tc,
        None => return,
    };
    duel::hint_selection(d, tc, true);
    if card::get_flag_effect(d, tc, ID) == 0 {
        card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 0));
        // Unaffected by opponent's card effects
        let e1 = effect::create_effect(d, tc);
        effect::set_description(d, e1, 3110);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_value_fn(d, e1, lua_fn(efilter));
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        // Tribute any number of other monsters; gain ATK equal to the combined ATK of the tributed monsters
        let e2 = effect::create_effect(d, tc);
        effect::set_category(d, e2, CATEGORY_ATKCHANGE);
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
        effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
        effect::set_range(d, e2, LOCATION_MZONE);
        effect::set_condition(d, e2, Some(condition(atkcon)));
        effect::set_cost(d, e2, Some(cost_target(atkcost)));
        effect::set_operation(d, e2, Some(operation(atkop)));
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
        // After damage calculation, send all opponent's monsters to GY
        let e3 = effect::create_effect(d, tc);
        effect::set_category(d, e3, CATEGORY_TOGRAVE);
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
        effect::set_code(d, e3, EVENT_BATTLED);
        effect::set_condition(d, e3, Some(condition(sendcon)));
        effect::set_target(d, e3, Some(cost_target(sendtg)));
        effect::set_operation(d, e3, Some(operation(sendop)));
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e3, false);
    }
}
/// Unaffected by opponent's card effects: `function(e,re) return e:GetOwnerPlayer()==1-re:GetOwnerPlayer() end`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let (e, re) = (a.e(0), a.e(1));
    effect::get_owner_player(d, e) == 1 - effect::get_owner_player(d, re)
}
/// Check that an attack involving "The Winged Dragon of Ra" was declared
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(c) || (duel::get_attack_target(d).is_some() && duel::get_attack_target(d) == Some(c))
}
/// Check for monsters that did not attack
fn atkfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::get_attack_announced_count(d, c) == 0 && card::get_text_attack(d, c) > 0 && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
/// Tribute cost
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| atkfilter(d, c, tp)), 1, 1, false, None, h.into());
    }
    let g = duel::select_release_group_cost(d, tp, Some(&|d, c| atkfilter(d, c, tp)), 1, 99, false, None, h.into());
    let _ct = duel::release(d, g.clone(), REASON_COST, None);
    effect::set_label_object(d, e, Value::Group(g));
    true
}
/// Gain ATK equal to the combined ATK of the tributed monsters
fn atkop(d: &mut Duel, a: &EffArgs) {
    let g = match effect::get_label_object(d, a.e).as_group().cloned() {
        Some(g) => g,
        None => return,
    };
    let mut atk: i64 = 0;
    for tc in group::iter(d, &g) {
        let batk = card::get_text_attack(d, tc);
        if batk > 0 {
            atk += batk as i64;
        }
    }
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, atk);
    effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
    card::register_effect(d, h, e1, false);
}
/// If this card attacked
fn sendcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(h)
}
/// Activation legality
fn sendtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, 0, 0);
    true
}
/// After damage calculation, send all opponent's monsters to GY
fn sendop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    duel::send_to_grave(d, &g, REASON_EFFECT, None, None);
}
