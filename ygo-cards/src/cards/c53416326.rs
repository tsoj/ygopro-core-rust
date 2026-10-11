//! Fairyant the Circular Sorcerer
use super::prelude::*;

pub const ID: u32 = 53416326;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| spcon(d, a))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be used as a non-Tuner for the Synchro Summon of a Plant/Insect monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_NONTUNER)
    effect::set_code(d, e2, EFFECT_NONTUNER);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(function(e,sc) return sc:IsRace(RACE_PLANT|RACE_INSECT) end)
    effect::set_value_fn(d, e2, effect_card_fn(|d, _e, sc| card::is_race(d, sc, RACE_PLANT | RACE_INSECT)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    let e = a.e(0);
    // return Duel.IsExistingMatchingCard(Card.IsRace,0,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,RACE_PLANT|RACE_INSECT)
    //     and Duel.GetLocationCount(e:GetHandlerPlayer(),LOCATION_MZONE)>0
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_race(d, c, RACE_PLANT | RACE_INSECT)), 0, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX)
        && duel::get_location_count(d, effect::get_handler_player(d, e), LOCATION_MZONE as u8, None, None, None).0 > 0
}
