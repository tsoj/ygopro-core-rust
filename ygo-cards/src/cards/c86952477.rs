//! Shine Knight
use super::prelude::*;

pub const ID: u32 = 86952477;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // lvchange
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.lvcon)
    effect::set_condition(d, e1, Some(condition(lvcon)));
    // e1:SetValue(4)
    effect::set_value(d, e1, 4);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.lvcon(e)
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsDefensePos()
    let h = effect::get_handler(d, a.e).nil();
    card::is_defense_pos(d, h)
}
