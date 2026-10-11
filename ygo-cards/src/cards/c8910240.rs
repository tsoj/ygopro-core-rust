//! Rose Papillon
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 8910240;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Additional Tribute Summon of a Level 7 or higher monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e1, LOCATION_HAND, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsLevelAbove,7))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_level_above(d, c, 7))));
    // e1:SetValue(0x1)
    effect::set_value(d, e1, 0x1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can attack directly while you control another Insect monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
    // e2:SetCondition(s.atkcond)
    effect::set_condition(d, e2, Some(condition(atkcond)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkcond(e)`
fn atkcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_INSECT),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,e:GetHandler())
    let p = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_INSECT)),
        p,
        LOCATION_MZONE,
        0,
        1,
        h.into(),
    )
}
