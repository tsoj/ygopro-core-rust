//! Chemicritter Oxy Ox
//! 化合獣オキシン・オックス
use super::prelude::*;

pub const ID: u32 = 18993198;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gemini.AddProcedure(c)
    gemini::add_procedure(d, c);
    // Special Summon 1 Gemini monster and change levels to that monster's level
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(Gemini.EffectStatusCondition)
    effect::set_condition(d, e1, Some(gemini::effect_status_condition()));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_GEMINI) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_GEMINI) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.gmfilter(c,lv)`
fn gmfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_GEMINI) and c:HasLevel() and not c:IsLevel(lv)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_GEMINI) && card::has_level(d, c) && !card::is_level(d, c, &[lv])
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINT_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 or Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if g.is_empty() || duel::special_summon(d, g.clone(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local lv=g:GetFirst():GetOriginalLevel()
    let first = group::get_first(d, &g).nil();
    let lv = card::get_original_level(d, first);
    // local tg=Duel.GetMatchingGroup(s.gmfilter,tp,LOCATION_MZONE,0,nil,lv)
    let tg = duel::get_matching_group(d, Some(&|d, c| gmfilter(d, c, lv)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // for tc in tg:Iter() do
    for tc in group::iter(d, &tg) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_CHANGE_LEVEL_FINAL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL_FINAL);
        // e1:SetValue(lv)
        effect::set_value(d, e1, lv as i64);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
