//! Nanobreaker
use super::prelude::*;

pub const ID: u32 = 70948327;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLE_START);
    effect::set_target(d, e1, Some(cost_target(destg)));
    effect::set_operation(d, e1, Some(operation(desop)));
    card::register_effect(d, c, e1, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tgt = duel::get_attack_target(d);
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_attacker(d) == Some(h)
            && tgt.map_or(false, |t| card::is_faceup(d, t) && card::is_level_below(d, t, 3) && card::is_relate_to_battle(d, t));
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tgt.map(CardSet::from_card), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, _a: &EffArgs) {
    let t = duel::get_attack_target(d).nil();
    if card::is_relate_to_battle(d, t) && card::is_faceup(d, t) && card::is_level_below(d, t, 3) {
        duel::destroy(d, t, REASON_EFFECT, None, None);
    }
}
