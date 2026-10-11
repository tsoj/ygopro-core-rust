//! Star Staring Starling
use super::prelude::*;

pub const ID: u32 = 34244455;

/// `s.levels`
const LEVELS: [i64; 5] = [4, 1, 2, 1, 3];

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_LEVEL)
    effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
    // e1:SetCondition(s.lvcon)
    effect::set_condition(d, e1, Some(condition(lvcon)));
    // e1:SetValue(s.lvval)
    effect::set_value_fn(d, e1, lua_fn(lvval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.lvcon(e,tp,eg,ep,ev,re,r,rp)`: `return e:GetHandler():GetSequence()<5`
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_sequence(d, c) < 5
}
/// `s.lvval(e,tp,eg,ep,ev,re,r,rp)`: `return s.levels[e:GetHandler():GetSequence()+1]`
/// (a sequence beyond the table gives `nil`, which the core reads as 0)
fn lvval(d: &mut Duel, a: Args) -> i64 {
    let c = effect::get_handler(d, a.e(0)).nil();
    let seq = card::get_sequence(d, c) as usize;
    LEVELS.get(seq).copied().unwrap_or(0)
}
