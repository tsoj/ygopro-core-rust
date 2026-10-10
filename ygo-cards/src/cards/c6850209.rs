//! A Deal with Dark Ruler
use super::prelude::*;

pub const ID: u32 = 6850209;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[85605684], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // s[0]=false
        set_s_var(d, ID, 0, Value::Bool(false));
        // s[1]=false
        set_s_var(d, ID, 1, Value::Bool(false));
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
        // aux.AddValuesReset(function() s[0]=false s[1]=false end)
        aux::add_values_reset(d, reset_values);
    });
}
/// `function() s[0]=false s[1]=false end` (run by `aux.AddValuesReset`)
fn reset_values(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Bool(false));
    set_s_var(d, ID, 1, Value::Bool(false));
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local tc=eg:GetFirst()
    let _ = group::get_first(d, &a.eg.clone().nil());
    // for tc in aux.Next(eg) do
    let eg = a.eg();
    for tc in group::iter(d, &eg) {
        // if tc:IsLevelAbove(8) and tc:IsPreviousLocation(LOCATION_MZONE) then
        if card::is_level_above(d, tc, 8) && card::is_previous_location(d, tc, LOCATION_MZONE) {
            // s[tc:GetPreviousControler()]=true
            let p = card::get_previous_controler(d, tc);
            set_s_var(d, ID, p as u64, Value::Bool(true));
        }
    }
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(85605684) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    card::is_code(d, c, &[85605684]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return s[tp] and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and
    //     Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return s_var(d, ID, a.tp as u64).truthy()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_HAND) as i32);
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
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    }
}
