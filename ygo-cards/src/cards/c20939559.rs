//! Shadowslayer
use super::prelude::*;

pub const ID: u32 = 20939559;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // direct attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(condition(con)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.con(e)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(Card.IsAttackPos,e:GetHandler():GetControler(),0,LOCATION_MZONE,1,nil)
    let c = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, c);
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_attack_pos(d, c)), p, 0, LOCATION_MZONE, 1, NO_EX)
}
