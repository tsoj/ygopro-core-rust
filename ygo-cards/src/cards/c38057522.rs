//! Grand Spiritual Art - Ichirin
use super::prelude::*;

pub const ID: u32 = 38057522;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate 1 of opponent's activated monster effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 monster with 1500 ATK and 200 DEF from deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.negf(c,tp)`
fn negf(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetDefense()==1500 and c:IsRace(RACE_SPELLCASTER)
    card::is_faceup(d, c) && card::get_defense(d, c) == 1500 && card::is_race(d, c, RACE_SPELLCASTER as u64)
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.negf,tp,LOCATION_MZONE,0,1,nil)
    //     and rp==1-tp and re:IsMonsterEffect() and Duel.IsChainDisablable(ev)
    duel::is_existing_matching_card(d, Some(&negf), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && a.rp == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let _rc = effect::get_handler(d, a.re.nil());
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
/// `s.thfilter(c,e,tp)`
fn thfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and not c:IsPublic()
    //     and Duel.IsExistingMatchingCard(s.thfilter2,tp,LOCATION_DECK,0,1,nil,c:GetAttribute(),e,tp,c)
    let _ = e;
    if !(card::is_race(d, c, RACE_SPELLCASTER as u64) && !card::is_public(d, c)) {
        return false;
    }
    let att = card::get_attribute(d, c) as i64;
    duel::is_existing_matching_card(d, Some(&|d, x| thfilter2(d, x, att)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thfilter2(c,att)`
fn thfilter2(d: &mut Duel, c: CardId, att: i64) -> bool {
    // return c:GetAttack()==1500 and c:GetDefense()==200 and c:IsAttribute(att) and c:IsAbleToHand()
    card::get_attack(d, c) == 1500 && card::get_defense(d, c) == 200 && card::is_attribute(d, c, att as u32) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let e = a.e;
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g1 = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g1>0 then
    if !g1.is_empty() {
        // Duel.ConfirmCards(1-tp,g1)
        duel::confirm_cards(d, 1 - tp, &g1);
        // e:SetLabel(g1:GetFirst():GetAttribute())
        let first = group::get_first(d, &g1).nil();
        let attr = card::get_attribute(d, first) as i64;
        effect::set_label(d, e, &[attr]);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g2=Duel.SelectMatchingCard(tp,s.thfilter2,tp,LOCATION_DECK,0,1,1,nil,e:GetLabel())
        let label = effect::get_label(d, e);
        let g2 = duel::select_matching_card(d, tp, Some(&|d, c| thfilter2(d, c, label)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g1>0 and #g2>0 then
        if !g1.is_empty() && !g2.is_empty() {
            // Duel.SendtoHand(g2,nil,REASON_EFFECT)
            duel::send_to_hand(d, &g2, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g2)
            duel::confirm_cards(d, 1 - tp, &g2);
            // Duel.SendtoDeck(g1,tp,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, &g1, Some(tp), SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
