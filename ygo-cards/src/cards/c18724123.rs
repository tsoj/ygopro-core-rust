//! Infernity Archer
//! インフェルニティ・アーチャー
use super::prelude::*;

pub const ID: u32 = 18724123;

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
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_HAND,0)==0
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, LOCATION_HAND, 0) == 0
}
