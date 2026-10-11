//! Cataclysmic Cryonic Coldo
use super::prelude::*;

pub const ID: u32 = 19420830;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP | EFFECT_FLAG_DELAY, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c,tp)
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        // and c:IsPreviousControler(tp) and c:GetPreviousAttributeOnField() & ATTRIBUTE_WIND ~=0
        && card::is_previous_controler(d, c, tp)
        && (card::get_previous_attribute_on_field(d, c) & ATTRIBUTE_WIND) != 0
        // and c:IsReason(REASON_DESTROY) and ((c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp) or c:IsReason(REASON_BATTLE))
        && card::is_reason(d, c, REASON_DESTROY)
        && ((card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp) || card::is_reason(d, c, REASON_BATTLE))
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp)
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| cfilter(d, c, a.tp), 1, NO_EX)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            // and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,0,PLAYER_ALL,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    true
}
// function s.tdfilter(c)
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToDeck()
    card::is_spell_trap(d, c) && card::is_able_to_deck(d, c)
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
        // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&tdfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
            // local sg=g:Select(tp,1,1,nil)
            let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoDeck(sg,nil,SEQ_DECKTOP,REASON_EFFECT)
            duel::send_to_deck(d, sg, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
        }
    }
}
