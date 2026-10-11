//! Extra Pendulum
use super::prelude::*;

pub const ID: u32 = 58308221;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // During your Main Phase this turn, you can conduct 1 Pendulum Summon of a monster(s) from your Extra Deck in addition to your Pendulum Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(function(e,tp) return Pendulum.PlayerCanGainAdditionalPendulumSummon(tp,id) end)
    effect::set_condition(d, e1, Some(condition(|d, a| pendulum::player_can_gain_additional_pendulum_summon(d, a.tp, ID))));
    // e1:SetOperation(function(e,tp) Pendulum.GrantAdditionalPendulumSummon(e:GetHandler(),nil,tp,LOCATION_EXTRA,aux.Stringid(id,1),aux.Stringid(id,2),id) end)
    effect::set_operation(
        d,
        e1,
        Some(operation(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            pendulum::grant_additional_pendulum_summon(d, h, None, a.tp, LOCATION_EXTRA, Some(aux::stringid(ID, 1)), Some(aux::stringid(ID, 2)), ID, None, None);
        })),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
