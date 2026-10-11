//! Beast Rage
//! ビーストレイジ
use super::prelude::*;

pub const ID: u32 = 9999961;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_BEAST|RACE_WINGEDBEAST),tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(
                d,
                Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_BEAST | RACE_WINGEDBEAST))),
                a.tp,
                LOCATION_REMOVED,
                0,
                1,
                NO_EX,
            );
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local atk=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_BEAST|RACE_WINGEDBEAST),tp,LOCATION_REMOVED,0,nil)*200
    let atk = duel::get_matching_group_count(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_BEAST | RACE_WINGEDBEAST))),
        a.tp,
        LOCATION_REMOVED,
        0,
        NO_EX,
    ) as i64
        * 200;
    // if #g==0 or atk==0 then return end
    if g.is_empty() || atk == 0 {
        return;
    }
    // local tc=g:GetFirst()
    let _tc = group::get_first(d, &g);
    let handler = effect::get_handler(d, a.e).nil();
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, handler);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(atk)
        effect::set_value(d, e1, atk);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
