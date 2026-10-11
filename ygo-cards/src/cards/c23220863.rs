//! Magical Cavalry of Cxulub
use super::prelude::*;

pub const ID: u32 = 23220863;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // immune
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetValue(s.efilter)
    effect::set_value_fn(d, e1, lua_fn(|d, a| efilter(d, a.e(0), a.e(1))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, _e: EffectId, te: EffectId) -> bool {
    // return te:IsMonsterEffect() and te:IsActivated() and not te:GetOwner():IsType(TYPE_PENDULUM)
    effect::is_monster_effect(d, te)
        && effect::is_activated(d, te)
        && !card::is_type(d, effect::get_owner(d, te).nil(), TYPE_PENDULUM)
}
