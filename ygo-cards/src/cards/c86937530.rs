//! Fairy Tail - Luna
use super::prelude::*;

pub const ID: u32 = 86937530;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add to hand 1 Spellcaster with 1850 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return monsters to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetAttack()==1850 and c:IsRace(RACE_SPELLCASTER) and c:IsAbleToHand()
    card::get_attack(d, c) == 1850 && card::is_race(d, c, RACE_SPELLCASTER) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&filter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsAbleToHand() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_able_to_hand(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,0,LOCATION_MZONE,1,1,nil)
    let mut g = duel::select_target(d, tp, Some(&thfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // g:AddCard(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    group::add_card(&mut g, &CardSet::from_card(h));
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,g,2,0,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 2, 0, LOCATION_MZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.cfilter(c,code)`
fn cfilter(d: &mut Duel, c: CardId, code: u32) -> bool {
    // return c:IsCode(code) and c:IsAbleToGraveAsCost()
    card::is_code(d, c, &[code]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if Duel.IsChainDisablable(0) then
    if duel::is_chain_disablable(d, 0) {
        // local g=Duel.GetMatchingGroup(s.cfilter,tp,0,LOCATION_DECK|LOCATION_EXTRA,nil,tc:GetCode())
        let code = card::get_code(d, tc.nil()).first().copied().nil();
        let g = duel::get_matching_group(d, Some(&|d, x| cfilter(d, x, code)), tp, 0, LOCATION_DECK | LOCATION_EXTRA, NO_EX);
        // if #g>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TOGRAVE as u64);
            // local sg=g:Select(1-tp,1,1,nil)
            let sg = group::select(d, &g, 1 - tp, 1, 1, false, NO_EX).nil();
            // Duel.SendtoGrave(sg,REASON_EFFECT)
            duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
            // Duel.NegateEffect(0)
            duel::negate_effect(d, 0);
            // return
            return;
        }
    }
    // if tc:IsRelateToEffect(e) then
    let tc = tc.nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        // local rg=Group.FromCards(c,tc)
        let rg = CardSet::from_iter([c, tc]);
        // Duel.SendtoHand(rg,nil,REASON_EFFECT)
        duel::send_to_hand(d, rg, None, REASON_EFFECT, None);
    }
}
