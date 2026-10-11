//! Dewdark of the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 90311614;

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
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return c:IsFaceup() and (lv==0 or c:GetLevel()>2)
    card::is_faceup(d, c) && (lv == 0 || card::get_level(d, c) > 2)
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandler():GetControler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = card::get_controler(d, c);
    // return not Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
