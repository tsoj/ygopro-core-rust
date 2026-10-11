//! Spirit Charmers
use super::prelude::*;

pub const ID: u32 = 91530236;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CHARMER, SET_POSSESSED, SET_FAMILIAR_POSSESSED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Take 2 cards from the deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.filter2(c,e,tp,mft,sft,code)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mft: i32, sft: i32, code: u32) -> bool {
    // return not c:IsCode(code)
    //     and ((mft>0 and c:IsMonster() and c:IsSetCard({SET_CHARMER,SET_FAMILIAR_POSSESSED})
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE))
    //     or (sft>0 and c:IsSpellTrap() and c:IsSetCard(SET_POSSESSED) and c:IsSSetable()))
    if card::is_code(d, c, &[code]) {
        return false;
    }
    (mft > 0
        && card::is_monster(d, c)
        && card::is_set_card(d, c, &[SET_CHARMER as u16, SET_FAMILIAR_POSSESSED as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None))
        || (sft > 0 && card::is_spell_trap(d, c) && card::is_set_card(d, c, &[SET_POSSESSED as u16]) && card::is_s_setable(d, c, false, None))
}
/// `s.filter(c,e,tp,mft,sft)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mft: i32, sft: i32) -> bool {
    // return c:IsAbleToHand()
    //     and ((c:IsMonster() and c:IsSetCard({SET_CHARMER,SET_FAMILIAR_POSSESSED})) or (c:IsSpellTrap() and c:IsSetCard(SET_POSSESSED)))
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_DECK,0,1,c,e,tp,mft,sft,c:GetCode())
    if !card::is_able_to_hand(d, c, None) {
        return false;
    }
    let kind = (card::is_monster(d, c) && card::is_set_card(d, c, &[SET_CHARMER as u16, SET_FAMILIAR_POSSESSED as u16]))
        || (card::is_spell_trap(d, c) && card::is_set_card(d, c, &[SET_POSSESSED as u16]));
    if !kind {
        return false;
    }
    // c:GetCode() (the first code)
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, e, tp, mft, sft, code)), tp, LOCATION_DECK, 0, 1, c.into())
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local mft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local sft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut sft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then sft=sft-1 end
    let h = effect::get_handler(d, e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        sft -= 1;
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,mft,sft) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp, mft, sft)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local mft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local sft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let sft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if not Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,mft,sft) then return end
    if !duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp, mft, sft)), tp, LOCATION_DECK, 0, 1, NO_EX) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local c1=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,e,tp,mft,sft)
    let c1 = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp, mft, sft)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local c2=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_DECK,0,1,1,nil,e,tp,mft,sft,c1:GetFirst():GetCode())
    let code = card::get_code(d, group::get_first(d, &c1).nil())[0];
    let c2 = duel::select_matching_card(d, tp, Some(&|d, c| filter2(d, c, e, tp, mft, sft, code)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if c1 and c2 then
    // Duel.SendtoHand(c1,nil,REASON_EFFECT)
    duel::send_to_hand(d, c1.clone(), None, REASON_EFFECT, None);
    // Duel.ConfirmCards(1-tp,c1)
    duel::confirm_cards(d, 1 - tp, c1);
    // if c2:GetFirst():IsMonster() then
    if card::is_monster(d, group::get_first(d, &c2).nil()) {
        // Duel.SpecialSummon(c2,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)
        duel::special_summon(d, c2.clone(), 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
        // Duel.ConfirmCards(1-tp,c2)
        duel::confirm_cards(d, 1 - tp, c2);
    } else {
        // Duel.SSet(tp,c2)
        duel::s_set(d, tp, c2, None, true);
    }
}
