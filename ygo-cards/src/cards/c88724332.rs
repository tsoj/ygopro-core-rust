//! Night Dragolich
use super::prelude::*;

pub const ID: u32 = 88724332;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change all non-Wyrm Attack Position monsters that were Special Summoned from the Main or Extra Deck to Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SET_POSITION)
    effect::set_code(d, e1, EFFECT_SET_POSITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(function(e,c) return c:IsFaceup() and not c:IsRace(RACE_WYRM) and c:IsSpecialSummoned() and c:IsSummonLocation(LOCATION_DECK|LOCATION_EXTRA) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| target_fn(d, c))));
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetValue(POS_FACEUP_DEFENSE)
    effect::set_value(d, e1, POS_FACEUP_DEFENSE as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // All non-Wyrm monsters that were Special Summoned from the Main or Extra Deck lose DEF equal to their original DEF
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // e2:SetValue(function(e,c) return -c:GetBaseDefense() end)
    effect::set_value_fn(d, e2, effect_card_value(|d, _e, c| -(card::get_base_defense(d, c) as i64)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e,c) of e1:SetTarget
fn target_fn(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and not c:IsRace(RACE_WYRM) and c:IsSpecialSummoned() and c:IsSummonLocation(LOCATION_DECK|LOCATION_EXTRA)
    card::is_faceup(d, c)
        && !card::is_race(d, c, RACE_WYRM)
        && card::is_special_summoned(d, c)
        && card::is_summon_location(d, c, LOCATION_DECK | LOCATION_EXTRA)
}
