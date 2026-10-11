//! Gigantic Thundercross
//! ギガンティック・サンダークロス
use super::prelude::*;

pub const ID: u32 = 34047456;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish monster on the field/GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove()
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && filter(d, chkc);
    }
    // local ct=math.abs(Duel.GetFieldGroupCount(tp,LOCATION_REMOVED,0)-Duel.GetFieldGroupCount(tp,0,LOCATION_REMOVED))
    let ct = (duel::get_field_group_count(d, tp, LOCATION_REMOVED, 0) - duel::get_field_group_count(d, tp, 0, LOCATION_REMOVED)).abs();
    // if chk==0 then return ct>0 and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,ct,nil) end
    if a.chk == Some(0) {
        return ct > 0
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, ct as u16, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE,ct,ct,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE, ct as u16, ct as u16, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,1-tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 1 - tp, LOCATION_DECK as i32);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local rg=Duel.GetTargetCards(e)
    let rg = duel::get_target_cards(d, e).nil();
    // if #rg==0 then return end
    if rg.is_empty() {
        return;
    }
    // if Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)>0 and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,1-tp,LOCATION_DECK,0,1,nil,e,1-tp)
    //     and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
    if duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, 1 - tp)), 1 - tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(1-tp,s.spfilter,1-tp,LOCATION_DECK,0,1,1,nil,e,1-tp)
        let sg = duel::select_matching_card(d, 1 - tp, Some(&|d, c| spfilter(d, c, e, 1 - tp)), 1 - tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SpecialSummon(sg,0,1-tp,1-tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, 1 - tp, 1 - tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
