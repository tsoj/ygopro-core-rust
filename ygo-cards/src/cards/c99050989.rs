//! Drillago
use super::prelude::*;

pub const ID: u32 = 99050989;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // direct attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetCondition(s.dircon)
    effect::set_condition(d, e1, Some(condition(dircon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetAttack()<1600 or c:IsFacedown()
    card::get_attack(d, c) < 1600 || card::is_facedown(d, c)
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_SZONE)==0
    //     and not Duel.IsExistingMatchingCard(s.filter,tp,0,LOCATION_MZONE,1,nil)
    duel::get_field_group_count(d, tp, 0, LOCATION_SZONE) == 0 && !duel::is_existing_matching_card(d, Some(&filter), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
