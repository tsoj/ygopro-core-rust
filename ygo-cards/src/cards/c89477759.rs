//! Number C1000: Numerounius
use super::prelude::*;

pub const ID: u32 = 89477759;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[15862758], xyz_number: Some(1000), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz summon
    // Xyz.AddProcedure(c,nil,12,5)
    xyz::add_procedure(d, c, None, Some(12), 5, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy and Special Summon (Battle Phase)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_BATTLE);
    // e2:SetTarget(s.bptg)
    effect::set_target(d, e2, Some(cost_target(bptg)));
    // e2:SetOperation(s.bpop)
    effect::set_operation(d, e2, Some(operation(bpop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon "Number iC1000: Numerounius Numerounia"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, h.into());
    }
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, h.into()).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.bptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn bptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    // local sg=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let sg = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.bpop(e,tp,eg,ep,ev,re,r,rp)`
fn bpop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    // local dg=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let dg = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, h.into());
    // local ct=Duel.Destroy(dg,REASON_EFFECT)
    let ct = duel::destroy(d, dg, REASON_EFFECT, None, None);
    // if ct==0 or Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if ct == 0 || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_GRAVE,nil,e,tp)
    let tp = a.tp;
    let e = a.e;
    let g = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e, tp)), tp, 0, LOCATION_GRAVE, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 3)) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_DESTROY) and rp~=tp and c:IsReason(REASON_EFFECT)
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    //     and c:GetOverlayCount()>0
    card::is_reason(d, c, REASON_DESTROY)
        && a.rp != a.tp
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, a.tp)
        && card::get_overlay_count(d, c) > 0
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(15862758) and c:IsCanBeSpecialSummoned(e,0x53b,tp,false,false) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    card::is_code(d, c, &[15862758])
        && card::is_can_be_special_summoned(d, c, e, 0x53b, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummon(tc,0x53b,tp,tp,false,false,POS_FACEUP)>0 and e:GetHandler():IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0x53b, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            let h = effect::get_handler(d, a.e).nil();
            if card::is_relate_to_effect(d, h, a.e) {
                // Duel.Overlay(tc,e:GetHandler())
                duel::overlay(d, tc, h, false);
            }
        }
    }
}
