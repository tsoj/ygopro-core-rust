//! Morphtronic Magnen
use super::prelude::*;

pub const ID: u32 = 29947751;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atk
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_condition(d, e1, Some(condition(cona)));
    effect::set_value_fn(d, e1, effect_card_fn(vala));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_CANNOT_DIRECT_ATTACK);
    effect::set_condition(d, e2, Some(condition(cona)));
    card::register_effect(d, c, e2, false);
    // def
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_condition(d, e3, Some(condition(cond)));
    effect::set_value_fn(d, e3, effect_card_fn(atlimit));
    card::register_effect(d, c, e3, false);
}
fn cona(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsAttackPos() and Duel.IsExistingMatchingCard(Card.IsFaceup,e:GetHandler():GetControler(),0,LOCATION_MZONE,1,nil)
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_attack_pos(d, h) {
        return false;
    }
    let p = card::get_controler(d, h);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), p, 0, LOCATION_MZONE, 1, NO_EX)
}
fn filter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:GetAttack()>atk
    card::is_faceup(d, c) && card::get_attack(d, c) > atk
}
/// `s.vala(e,c)`
fn vala(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // if c:IsFaceup() then
    if card::is_faceup(d, c) {
        // return Duel.IsExistingMatchingCard(s.filter,c:GetControler(),LOCATION_MZONE,0,1,c,c:GetAttack())
        let p = card::get_controler(d, c);
        let atk = card::get_attack(d, c);
        duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, atk)), p, LOCATION_MZONE, 0, 1, c.into())
    // else return true end
    } else {
        true
    }
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsDefensePos()
    let h = effect::get_handler(d, a.e).nil();
    card::is_defense_pos(d, h)
}
/// `s.atlimit(e,c)`
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler()
    c != effect::get_handler(d, e).nil()
}
