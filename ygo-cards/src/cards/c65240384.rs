//! Big Shield Gardna
use super::prelude::*;

pub const ID: u32 = 65240384;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change this card to face-up Defense Position, and if you do, negate the activation
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_NEGATE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_F);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL + EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(negcon)));
    effect::set_target(d, e1, Some(cost_target(negtg)));
    effect::set_operation(d, e1, Some(operation(negop)));
    card::register_effect(d, c, e1, false);
    // If this card is attacked, change it to Attack Position at the end End of the Damage Step
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    effect::set_operation(d, e2, Some(operation(posop)));
    card::register_effect(d, c, e2, false);
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    if !(effect::is_spell_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0)) {
        return false;
    }
    let c = effect::get_handler(d, a.e).nil();
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    match tg {
        Some(tg) => tg.len() == 1 && group::get_first(d, &tg) == Some(c) && card::is_facedown(d, c),
        None => false,
    }
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(c)), 1, a.tp, POS_FACEUP_DEFENSE as i32);
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.ChangePosition returns a number, which is always true in Lua
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
        duel::negate_activation(d, a.ev as u8);
    }
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if Some(c) == duel::get_attack_target(d) && card::is_defense_pos(d, c) && card::is_relate_to_battle(d, c) {
        duel::change_position(d, c, POS_FACEUP_ATTACK as u8, None, None, None, false);
    }
}
