//! Pixie Ring
use super::prelude::*;

pub const ID: u32 = 46502013;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // cannot be battle target
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(con)));
    effect::set_value_fn(d, e2, effect_card_fn(atlimit));
    card::register_effect(d, c, e2, false);
}
fn tfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // c:IsFaceup() and c:GetAttack()<atk
    card::is_faceup(d, c) && card::get_attack(d, c) < atk
}
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // Duel.IsExistingMatchingCard(Card.IsPosition,e:GetHandlerPlayer(),LOCATION_MZONE,0,2,nil,POS_FACEUP_ATTACK)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_position(d, c, POS_FACEUP_ATTACK)), p, LOCATION_MZONE, 0, 2, NO_EX)
}
fn atlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and not Duel.IsExistingMatchingCard(s.tfilter,c:GetControler(),LOCATION_MZONE,0,1,c,c:GetAttack())
    if !card::is_faceup(d, c) {
        return false;
    }
    let p = card::get_controler(d, c);
    let atk = card::get_attack(d, c);
    !duel::is_existing_matching_card(d, Some(&|d, cc| tfilter(d, cc, atk)), p, LOCATION_MZONE, 0, 1, c.into())
}
