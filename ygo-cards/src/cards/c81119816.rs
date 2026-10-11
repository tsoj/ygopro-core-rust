//! Prank-Kids Fansies
use super::prelude::*;

pub const ID: u32 = 81119816;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PRANK_KIDS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e1, EVENT_BE_MATERIAL);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.tgcon)
    effect::set_condition(d, e1, Some(condition(tgcon)));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tgcon(e,tp,eg,ep,ev,re,r,rp)
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c);
    // return c:IsLocation(LOCATION_GRAVE) and rc:IsSetCard(SET_PRANK_KIDS) and r & REASON_FUSION+REASON_LINK ~=0
    card::is_location(d, c, LOCATION_GRAVE)
        && card::is_set_card(d, rc.nil(), &[SET_PRANK_KIDS as u16])
        && (a.r & (REASON_FUSION + REASON_LINK)) != 0
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PRANK_KIDS) and c:IsAbleToGrave() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_PRANK_KIDS as u16]) && card::is_able_to_grave(d, c, None) && !card::is_code(d, c, &[ID])
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_PRANK_KIDS) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_PRANK_KIDS as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && !card::is_code(d, c, &[ID])
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // local g2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK|LOCATION_HAND,0,g:GetFirst(),e,tp)
    let first = group::get_first(d, &g);
    let g2 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_HAND, 0, first.into());
    // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)~=0 and g:GetFirst():IsLocation(LOCATION_GRAVE)
    //     and #g2>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty()
        && duel::send_to_grave(d, &g, REASON_EFFECT, None, None) != 0
        && card::is_location(d, group::get_first(d, &g).nil(), LOCATION_GRAVE)
        && !g2.is_empty()
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g2:Select(tp,1,1,nil)
        let sg = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, &sg, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
