//! Smoke Mosquito
use super::prelude::*;

pub const ID: u32 = 28427869;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Chain Level
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    effect::set_operation(d, e2, Some(operation(lvop)));
    card::register_effect(d, c, e2, false);
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let attacker = duel::get_attacker(d).nil();
    card::is_controler(d, attacker, 1 - a.tp)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::get_battle_damage(d, a.tp) > 0;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_target_range(d, e1, 1, 0);
        effect::set_value(d, e1, HALF_DAMAGE as i64);
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
        duel::register_effect(d, e1, a.tp);
        let turn_player = duel::get_turn_player(d);
        duel::skip_phase(d, turn_player, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
    }
}
fn lvfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    card::is_faceup(d, c) && !card::is_level(d, c, &[lv]) && card::is_level_above(d, c, 1)
}
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let lv = card::get_level(d, c);
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && lvfilter(d, chkc, lv);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| lvfilter(d, x, lv)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    duel::select_target(d, a.tp, Some(&|d, x| lvfilter(d, x, lv)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn lvop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    let c = effect::get_handler(d, a.e).nil();
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            let lv = card::get_level(d, tc);
            effect::set_value(d, e1, lv as i64);
            effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
            card::register_effect(d, c, e1, false);
        }
    }
}
