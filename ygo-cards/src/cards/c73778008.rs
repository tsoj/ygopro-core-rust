//! Rage of the Deep Sea
use super::prelude::*;

pub const ID: u32 = 73778008;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(s.atkup)
    effect::set_value_fn(d, e1, effect_card_value(atkup));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkup(e,c)`
fn atkup(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsRace,c:GetControler(),LOCATION_GRAVE,0,nil,RACE_FISH|RACE_SEASERPENT|RACE_AQUA)*500
    let tp = card::get_controler(d, c);
    let ct = duel::get_matching_group_count(
        d,
        Some(&|d, x| card::is_race(d, x, RACE_FISH | RACE_SEASERPENT | RACE_AQUA)),
        tp,
        LOCATION_GRAVE,
        0,
        NO_EX,
    );
    ct as i64 * 500
}
