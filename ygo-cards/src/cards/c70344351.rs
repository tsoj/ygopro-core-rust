//! Riryoku Field
use super::prelude::*;

pub const ID: u32 = 70344351;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_NEGATE | CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
    match tg {
        Some(tg) => {
            tg.len() == 1
                && card::is_location(d, group::get_first(d, &tg).nil(), LOCATION_MZONE)
                && effect::is_spell_effect(d, re)
                && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
                && duel::is_chain_negatable(d, a.ev as u8)
        }
        None => false,
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let re = a.re.nil();
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let h = effect::get_handler(d, re).nil();
    if card::is_destructable(d, h, None) && card::is_relate_to_effect(d, h, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    if duel::negate_activation(d, a.ev as u8) {
        let h = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, h, re) {
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
