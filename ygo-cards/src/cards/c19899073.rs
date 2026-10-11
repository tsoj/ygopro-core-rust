//! Ame no Murakumo no Mitsurugi
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 19899073;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MITSURUGI], listed_names: &[ID, 81560239], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // If this card is Special Summoned: You can destroy all monsters your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When your opponent activates a card or effect (Quick Effect): You can activate this effect; your opponent can discard 1 card, or that effect is negated
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_HANDES)
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_HANDES);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and (...) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card is Tributed: You can add 1 "Mitsurugi" card from your Deck to your hand, except "Ame no Murakumo no Mitsurugi", then you can Special Summon this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_RELEASE)
    effect::set_code(d, e3, EVENT_RELEASE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// The condition of the second effect: `rp==1-tp and (...)`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return rp==1-tp and (Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,0,LOCATION_HAND,1,nil,REASON_EFFECT,1-tp) or Duel.IsChainDisablable(ev))
    a.rp == 1 - tp
        && (duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), Some(1 - tp))), tp, 0, LOCATION_HAND, 1, NO_EX)
            || duel::is_chain_disablable(d, a.ev as u8))
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_HANDES,nil,1,1-tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_HANDES, None, 1, 1 - tp, 1);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=nil
    // if Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,0,LOCATION_HAND,1,nil,REASON_EFFECT) then
    let op: Option<usize> = if duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, Some(REASON_EFFECT), None)), tp, 0, LOCATION_HAND, 1, NO_EX) {
        // op=Duel.SelectEffect(1-tp,{true,aux.Stringid(id,3)},{true,aux.Stringid(id,4)})
        duel::select_effect(d, 1 - tp, &[(true, aux::stringid(ID, 3)), (true, aux::stringid(ID, 4))])
    } else {
        // op=2
        Some(2)
    };
    // if op==1 then
    if op == Some(1) {
        // Duel.DiscardHand(1-tp,nil,1,1,REASON_EFFECT|REASON_DISCARD,nil)
        duel::discard_hand(d, 1 - tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    } else if op == Some(2) {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MITSURUGI) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_MITSURUGI as u16]) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 or Duel.SendtoHand(g,nil,REASON_EFFECT)==0 then return end
    if g.is_empty() || duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) == 0 {
        return;
    }
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsRelateToEffect(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and aux.nvfilter(c) and Duel.SelectYesNo(tp,aux.Stringid(id,5)) then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_relate_to_effect(d, c, e)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
        && aux::nvfilter(d, c)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 5))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
