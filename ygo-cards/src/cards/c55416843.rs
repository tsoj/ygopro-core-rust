//! Feast of the Wild LV5
use super::prelude::*;

pub const ID: u32 = 55416843;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special summon up to 2 level 5 warrior monsters from hand or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:GetLevel()==5 and c:IsRace(RACE_WARRIOR) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::get_level(d, c) == 5
        && card::is_race(d, c, RACE_WARRIOR)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| filter(d, c, a.e, a.tp)),
                a.tp,
                LOCATION_HAND | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if ft>2 then ft=2 end
    if ft > 2 {
        ft = 2;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,ft,nil,e,tp)
    let nv = aux::necro_valley_filter(|d, c| filter(d, c, a.e, a.tp));
    let g = duel::select_matching_card(d, a.tp, Some(&nv), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, ft as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // local tc=g:GetFirst()
        let _ = group::get_first(d, &g);
        // for tc in aux.Next(g) do
        for tc in group::iter(d, &g) {
            // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            // Negate their effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // Cannot attack this turn
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetDescription(3206)
            effect::set_description(d, e3, 3206);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e3, EFFECT_CANNOT_ATTACK);
            // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e3:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc, e3, false);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
