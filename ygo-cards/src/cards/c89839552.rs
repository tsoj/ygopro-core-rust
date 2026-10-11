//! Puppet Pawn
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 89839552;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[88617904, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Can attack directly
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Search "Promotion"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e2, EVENT_BATTLE_DAMAGE);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg1)
    effect::set_target(d, e2, Some(cost_target(thtg1)));
    // e2:SetOperation(s.thop1)
    effect::set_operation(d, e2, Some(operation(thop1)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Search 1 EARTH Warrior monster
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.thtg2)
    effect::set_target(d, e3, Some(cost_target(thtg2)));
    // e3:SetOperation(s.thop2)
    effect::set_operation(d, e3, Some(operation(thop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp
    a.ep as u8 == 1 - a.tp
}
// function s.thfilter1(c)
fn thfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(88617904) and c:IsAbleToHand()
    card::is_code(d, c, &[88617904]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg1(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg1(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter1,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter1), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop1(e,tp,eg,ep,ev,re,r,rp)
fn thop1(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter1,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter1), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
// function s.thfilter2(c)
fn thfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR) and c:IsMonster()
    //   and c:IsAbleToHand() and not c:IsCode(id)
    card::is_attribute(d, c, ATTRIBUTE_EARTH)
        && card::is_race(d, c, RACE_WARRIOR)
        && card::is_monster(d, c)
        && card::is_able_to_hand(d, c, None)
        && !card::is_code(d, c, &[ID])
}
// function s.thtg2(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter2,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter2), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,1-tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 1 - a.tp, LOCATION_DECK as i32);
    true
}
// function s.thfilter3(c)
fn thfilter3(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thop2(e,tp,eg,ep,ev,re,r,rp)
fn thop2(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter2,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter2), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, &g, None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
        // local sg=Duel.GetMatchingGroup(s.thfilter3,tp,0,LOCATION_DECK,nil)
        let sg = duel::get_matching_group(d, Some(&thfilter3), tp, 0, LOCATION_DECK, NO_EX);
        // if #sg>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,2)) then
        if !sg.is_empty() && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_ATOHAND as u64);
            // local sc=sg:Select(1-tp,1,1,nil)
            let sc = group::select(d, &sg, 1 - tp, 1, 1, false, NO_EX).nil();
            // if #sc==0 then return end
            if sc.is_empty() {
                return;
            }
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(sc,nil,REASON_EFFECT)
            duel::send_to_hand(d, &sc, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(tp,sc)
            duel::confirm_cards(d, tp, &sc);
        }
    }
}
