//! The Flute of Summoning Kuriboh
use super::prelude::*;

pub const ID: u32 = 20065322;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_KURIBOH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,ft,e,tp)
fn filter(d: &mut Duel, c: CardId, ft: i32, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_KURIBOH,57116033) and (c:IsAbleToHand() or (ft>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_code(d, c, &[CARD_KURIBOH, 57116033])
        && (card::is_able_to_hand(d, c, None) || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then
    if a.chk == Some(0) {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,ft,e,tp)
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, ft, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,0)
    duel::hint(d, HINT_SELECTMSG as u8, tp, 0);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil,ft,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, ft, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // local th=g:GetFirst():IsAbleToHand()
        let first = group::get_first(d, &g).nil();
        let th = card::is_able_to_hand(d, first, None);
        // local sp=ft>0 and g:GetFirst():IsCanBeSpecialSummoned(e,0,tp,false,false)
        let sp = ft > 0 && card::is_can_be_special_summoned(d, first, e, 0, tp, false, false, None, None, None);
        // local op=0 ... if th and sp then op=Duel.SelectOption(...) elseif th then op=0 else op=1 end
        let op = if th && sp {
            duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)])
        } else if th {
            0
        } else {
            1
        };
        // if op==0 then
        if op == 0 {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        } else {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
