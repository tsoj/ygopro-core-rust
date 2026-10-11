//! Aquamirror Illusion
use super::prelude::*;

pub const ID: u32 = 64437633;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GISHKI],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Gishki" Ritual Monster from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GISHKI) and c:IsRitualMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,true)
    card::is_set_card(d, c, &[SET_GISHKI as u16]) && card::is_ritual_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, true, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,true,POS_FACEUP)>0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, tp, tp, false, true, POS_FACEUP as u8, None) > 0 {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // --Cannot attack
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(aux.Stringid(id,1))
            effect::set_description(d, e1, aux::stringid(ID, 1));
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1,true)
            card::register_effect(d, sc, e1, true);
            // --Return it to hand during end phase
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e2:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
            // e2:SetRange(LOCATION_MZONE)
            effect::set_range(d, e2, LOCATION_MZONE);
            // e2:SetCountLimit(1)
            effect::set_count_limit(d, e2, 1, 0, 0, 0);
            // e2:SetOperation(function(e) Duel.SendtoHand(e:GetHandler(),nil,REASON_EFFECT) end)
            effect::set_operation(
                d,
                e2,
                Some(operation(|d, a| {
                    let h = effect::get_handler(d, a.e).nil();
                    duel::send_to_hand(d, h, None, REASON_EFFECT, None);
                })),
            );
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e2,true)
            card::register_effect(d, sc, e2, true);
        }
    }
}
