//! Ichiki Sayori-Hime
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 9627299;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:AddCannotBeSpecialSummoned()
    let _e0 = card::add_cannot_be_special_summoned(d, c);
    // Spirit.AddProcedure(c,EVENT_SUMMON_SUCCESS,EVENT_FLIP)
    let _spirit = spirit::add_procedure(d, c, &[EVENT_SUMMON_SUCCESS, EVENT_FLIP]);
    // Immediately after this effect resolves, Normal Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SUMMON)
    effect::set_category(d, e1, CATEGORY_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.nscon)
    effect::set_condition(d, e1, Some(condition(nscon)));
    // e1:SetTarget(s.nstg)
    effect::set_target(d, e1, Some(cost_target(nstg)));
    // e1:SetOperation(s.nsop)
    effect::set_operation(d, e1, Some(operation(nsop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 Level 4 LIGHT or DARK monster with 800 ATK or DEF from your Deck to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.nsconfilter(c)`
fn nsconfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsAttack(800) or c:IsDefense(800)) and c:IsFaceup()
    (card::is_attack(d, c, &[800]) || card::is_defense(d, c, &[800])) && card::is_faceup(d, c)
}
/// `s.nscon(e,tp,eg,ep,ev,re,r,rp)`
fn nscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.nsconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&nsconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.nstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn nstg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSummonable(true,nil) end
    if a.chk == Some(0) {
        return card::is_summonable(d, c, true, None, 0, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.nsop(e,tp,eg,ep,ev,re,r,rp)`
fn nsop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.Summon(tp,c,true,nil)
        duel::summon(d, a.tp, c, true, None, 0, None);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevel(4) and c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) and (c:IsAttack(800) or c:IsDefense(800))
    //     and c:IsAbleToHand() and not c:IsCode(id)
    card::is_level(d, c, &[4])
        && card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)
        && (card::is_attack(d, c, &[800]) || card::is_defense(d, c, &[800]))
        && card::is_able_to_hand(d, c, None)
        && !card::is_code(d, c, &[ID])
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
