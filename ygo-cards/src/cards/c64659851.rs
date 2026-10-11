//! Law of the Cosmos
use super::prelude::*;

pub const ID: u32 = 64659851;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_JINZO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Your opponent can Set 1 Trap from their hand or Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON | CATEGORY_TOHAND | CATEGORY_SEARCH | CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,e,tp,ft)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    // return (ft>0 and s.spfilter(c,e,tp)) or s.thfilter(c)
    (ft > 0 && spfilter(d, c, e, tp)) || thfilter(d, c)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_JINZO) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_JINZO]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsCode(CARD_JINZO) or (c:IsMonster() and c:ListsCode(CARD_JINZO))) and c:IsAbleToHand()
    (card::is_code(d, c, &[CARD_JINZO]) || (card::is_monster(d, c) && card::lists_code(d, c, &[CARD_JINZO]))) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)) end
    if a.chk == Some(0) {
        let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        let (e, tp) = (a.e, a.tp);
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp, ft)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.setfilter(c,tp)
fn setfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsTrap() and c:IsSSetable(false,1-tp)
    card::is_trap(d, c) && card::is_s_setable(d, c, false, Some(1 - tp))
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local summon=false
    let mut summon = false;
    // local g=Duel.GetMatchingGroup(s.setfilter,tp,0,LOCATION_HAND|LOCATION_DECK,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| setfilter(d, c, a.tp)), a.tp, 0, LOCATION_HAND | LOCATION_DECK, NO_EX);
    // if Duel.GetLocationCount(1-tp,LOCATION_SZONE,1-tp,LOCATION_REASON_TOFIELD)>0 and #g>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,1)) then
    if duel::get_location_count(d, 1 - a.tp, LOCATION_SZONE as u8, Some(1 - a.tp), Some(LOCATION_REASON_TOFIELD), None).0 > 0
        && !g.is_empty()
        && duel::select_yes_no(d, 1 - a.tp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_SET)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_SET as u64);
        // local sg=g:Select(1-tp,1,1,nil)
        let sg = group::select(d, &g, 1 - a.tp, 1, 1, false, NO_EX).nil();
        // summon=Duel.SSet(1-tp,sg)>0
        summon = duel::s_set(d, 1 - a.tp, sg, None, true) > 0;
    }
    // if summon and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if summon && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
        let (e, tp) = (a.e, a.tp);
        let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, e, tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local hg=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let hg = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #hg>0 then
        if !hg.is_empty() {
            // Duel.SendtoHand(hg,nil,REASON_EFFECT)
            duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,hg)
            duel::confirm_cards(d, 1 - a.tp, hg);
        }
    }
}
