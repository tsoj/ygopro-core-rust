//! Black Tyranno
use super::prelude::*;

pub const ID: u32 = 38670435;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // direct attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    effect::set_condition(d, e1, Some(condition(dircon)));
    card::register_effect(d, c, e1, false);
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandler():GetControler()
    let h = effect::get_handler(d, a.e).nil();
    let tp = card::get_controler(d, h);
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_SZONE)==0 and not Duel.IsExistingMatchingCard(Card.IsAttackPos,tp,0,LOCATION_MZONE,1,nil)
    duel::get_field_group_count(d, tp, 0, LOCATION_SZONE) == 0
        && !duel::is_existing_matching_card(d, Some(&|d, c| card::is_attack_pos(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
