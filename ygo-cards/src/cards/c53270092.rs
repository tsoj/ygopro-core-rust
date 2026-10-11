//! Han-Shi Kyudo Spirit
//! キュウドウ魂 ＨＡＮ－ＳＨＩ
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 53270092;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Spirit.AddProcedure(c,EVENT_SUMMON_SUCCESS,EVENT_FLIP)
    spirit::add_procedure(d, c, &[EVENT_SUMMON_SUCCESS, EVENT_FLIP]);
    // Return this card from the Pendulum Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCondition(s.selfthcon)
    effect::set_condition(d, e1, Some(condition(selfthcon)));
    // e1:SetTarget(s.selfthtg)
    effect::set_target(d, e1, Some(cost_target(selfthtg)));
    // e1:SetOperation(s.selfthop)
    effect::set_operation(d, e1, Some(operation(selfthop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return cards in the pendulum zones and add 1 monster to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.selfthcon(e,tp,eg,ep,ev,re,r,rp)`
fn selfthcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSummonType,1,nil,SUMMON_TYPE_PENDULUM)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_PENDULUM]), 1, NO_EX)
}
/// `s.selfthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.selfthop(e,tp,eg,ep,ev,re,r,rp)`
fn selfthop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
/// `s.pendfilter(c,tp)`
fn pendfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_PZONE)
    card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_PZONE)
}
/// `s.pzfilter(c,tp)`
fn pzfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToHand() and (c:IsLocation(LOCATION_PZONE) or c:GetColumnGroup():IsExists(s.pendfilter,1,nil,tp))
    if !card::is_able_to_hand(d, c, None) {
        return false;
    }
    if card::is_location(d, c, LOCATION_PZONE) {
        return true;
    }
    let cg = card::get_column_group(d, c, 0, 0);
    group::is_exists(d, &cg, &|d, x| pendfilter(d, x, tp), 1, NO_EX)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.pzfilter,tp,LOCATION_ONFIELD,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| pzfilter(d, c, tp)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.pzfilter,tp,LOCATION_ONFIELD,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| pzfilter(d, c, tp)), tp, LOCATION_ONFIELD, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttack(2400) and c:IsDefense(1000) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_attack(d, c, &[2400]) && card::is_defense(d, c, &[1000]) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.pzfilter,tp,LOCATION_ONFIELD,0,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| pzfilter(d, c, tp)), tp, LOCATION_ONFIELD, 0, NO_EX);
    // if #g==0 or Duel.SendtoHand(g,nil,REASON_EFFECT)==0
    //     or not Duel.GetOperatedGroup():IsExists(Card.IsLocation,1,nil,LOCATION_HAND) then return end
    if g.is_empty() {
        return;
    }
    if duel::send_to_hand(d, g, None, REASON_EFFECT, None) == 0 {
        return;
    }
    let operated = duel::get_operated_group(d);
    if !group::is_exists(d, &operated, &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX) {
        return;
    }
    // local hg=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let hg = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if #hg==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
    if hg.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sg=hg:Select(tp,1,1,nil)
    let sg = group::select(d, &hg, tp, 1, 1, false, NO_EX).nil();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - tp, sg);
    }
}
