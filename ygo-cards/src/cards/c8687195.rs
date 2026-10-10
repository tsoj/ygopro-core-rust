//! Dreamsprite
use super::prelude::*;

pub const ID: u32 = 8687195;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // change attack target
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_BATTLE_CONFIRM);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    duel::get_attack_target(d) == Some(c) && card::is_controler(d, duel::get_attacker(d).nil(), 1 - a.tp)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        let c = effect::get_handler(d, a.e).nil();
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && chkc != c;
    }
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let c = effect::get_handler(d, a.e).nil();
    duel::select_target(d, tp, None, tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d);
    let at = duel::get_attacker(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            let at = at.nil();
            if card::can_attack(d, at) && !card::is_immune_to_effect(d, at, a.e) {
                // Duel.CalculateDamage(a,tc)
                duel::calculate_damage(d, at, Some(tc), false);
            }
        }
    }
}
