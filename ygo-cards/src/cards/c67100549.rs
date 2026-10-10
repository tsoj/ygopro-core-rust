//! Screams of the Branded
use super::prelude::*;

pub const ID: u32 = 67100549;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 of your Fusion Monsters that is banished or in GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set itself from GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Check for Fusion Monsters sent to the GY
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_TO_GRAVE)
        effect::set_code(d, ge1, EVENT_TO_GRAVE);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
    // end)
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // for tc in aux.Next(eg) do
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        // if tc:IsType(TYPE_FUSION) then
        if card::is_type(d, tc, TYPE_FUSION) {
            // Duel.RegisterFlagEffect(tc:GetControler(),id,RESET_PHASE|PHASE_END,0,1)
            let p = card::get_controler(d, tc);
            duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(tp,id)>0
    duel::get_flag_effect(d, a.tp, ID) > 0
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsFaceup() or c:IsLocation(LOCATION_GRAVE)) and c:IsType(TYPE_FUSION) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_GRAVE))
        && card::is_type(d, c, TYPE_FUSION)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            // and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil,e,tp) end
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(|d, c| filter(d, c, a.e, a.tp))),
        a.tp,
        LOCATION_GRAVE | LOCATION_REMOVED,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsReason(REASON_COST) and re and re:IsActivated() and re:IsMonsterEffect()
    //     and re:GetHandler():IsCode(CARD_ALBAZ) then
    if let Some(re) = a.re {
        if card::is_reason(d, c, REASON_COST) && effect::is_activated(d, re) && effect::is_monster_effect(d, re) {
            let re_h = effect::get_handler(d, re).nil();
            if card::is_code(d, re_h, &[CARD_ALBAZ]) {
                // local e1=Effect.CreateEffect(c)
                let e1 = effect::create_effect(d, c);
                // e1:SetDescription(aux.Stringid(id,1))
                effect::set_description(d, e1, aux::stringid(ID, 1));
                // e1:SetCategory(CATEGORY_SET)
                effect::set_category(d, e1, CATEGORY_SET);
                // e1:SetType(EFFECT_TYPE_QUICK_O)
                effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
                // e1:SetCode(EVENT_FREE_CHAIN)
                effect::set_code(d, e1, EVENT_FREE_CHAIN);
                // e1:SetRange(LOCATION_GRAVE)
                effect::set_range(d, e1, LOCATION_GRAVE);
                // e1:SetCountLimit(1,id)
                effect::set_count_limit(d, e1, 1, ID, 0, 0);
                // e1:SetHintTiming(TIMING_END_PHASE)
                effect::set_hint_timing(d, e1, TIMING_END_PHASE, None);
                // e1:SetCondition(s.setcon)
                effect::set_condition(d, e1, Some(condition(setcon)));
                // e1:SetTarget(s.settg)
                effect::set_target(d, e1, Some(cost_target(settg)));
                // e1:SetOperation(s.setop)
                effect::set_operation(d, e1, Some(operation(setop)));
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // c:RegisterEffect(e1)
                card::register_effect(d, c, e1, false);
            }
        }
    }
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_END)
    duel::is_phase(d, PHASE_END)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsSSetable() end
    if a.chk == Some(0) {
        return card::is_s_setable(d, h, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsSSetable() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None) {
        // Duel.SSet(tp,c)
        duel::s_set(d, a.tp, CardSet::from_card(c), None, true);
    }
}
