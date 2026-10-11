//! Break the Seal
use super::prelude::*;

pub const ID: u32 = 69873498;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_FORBIDDEN_ONE],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return cards your opponent controls to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(d, e2, Some(condition(rth_cond)));
    // e2:SetCost(s.rthcost)
    effect::set_cost(d, e2, Some(cost_target(rthcost)));
    // e2:SetTarget(s.rthtg)
    effect::set_target(d, e2, Some(cost_target(rthtg)));
    // e2:SetOperation(s.rthop)
    effect::set_operation(d, e2, Some(operation(rthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.plfilter(c,tp)
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(id) and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_code(d, c, &[ID]) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
// function s.thcostfilter(c)
fn thcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(id) and c:IsAbleToGraveAsCost() and c:IsFaceup()
    card::is_code(d, c, &[ID]) && card::is_able_to_grave_as_cost(d, c) && card::is_faceup(d, c)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FORBIDDEN_ONE) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_FORBIDDEN_ONE as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local b1=Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,tp)
    let b1 = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, x: CardId| plfilter(d, x, tp)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    // local b2=c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED)
    //   and Duel.IsExistingMatchingCard(s.thcostfilter,tp,LOCATION_ONFIELD,0,1,c)
    //   and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let b2 = card::is_able_to_grave_as_cost(d, c)
        && card::is_status(d, c, STATUS_EFFECT_ENABLED)
        && duel::is_existing_matching_card(d, Some(&thcostfilter), tp, LOCATION_ONFIELD, 0, 1, c.into())
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thcostfilter,tp,LOCATION_ONFIELD,0,1,1,c)
        let g = duel::select_matching_card(d, tp, Some(&thcostfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, c.into()).nil();
        // Duel.SendtoGrave(g+c,REASON_COST)
        let mut gc = g.clone();
        group::add_card(&mut gc, &CardSet::from_card(c));
        duel::send_to_grave(d, gc, REASON_COST, None, None);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // Place 1 "Break the Seal" from your Deck or GY face-up on your field
        // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.plfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(move |d: &mut Duel, x: CardId| plfilter(d, x, tp))),
            tp,
            LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        let sc = group::get_first(d, &g);
        // if sc then
        if let Some(sc) = sc {
            // Duel.MoveToField(sc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
            duel::move_to_field(d, sc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        }
    } else if op == 2 {
        // Add 1 "Forbidden One" monster from your Deck to your hand
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
// function s.rthcostfilter(c)
fn rthcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FORBIDDEN_ONE) and c:IsMonster() and not c:IsPublic()
    card::is_set_card(d, c, &[SET_FORBIDDEN_ONE as u16]) && card::is_monster(d, c) && !card::is_public(d, c)
}
// function s.rthcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn rthcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=Duel.GetMatchingGroupCount(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,nil)
    let mut ct = duel::get_matching_group_count(d, Some(&|d: &mut Duel, c: CardId| card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return ct>0 and Duel.IsExistingMatchingCard(s.rthcostfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_matching_card(d, Some(&rthcostfilter), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // ct=math.min(ct,5)
    ct = ct.min(5);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rthcostfilter,tp,LOCATION_HAND,0,1,ct,nil)
    let g = duel::select_matching_card(d, tp, Some(&rthcostfilter), tp, LOCATION_HAND, 0, 1, ct as u16, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // e:SetLabel(#g)
    effect::set_label(d, a.e, &[g.len() as i64]);
    true
}
// function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end
fn rth_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_ONFIELD)
}
// function s.rthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,e:GetLabel(),1-tp,LOCATION_ONFIELD)
    let label = effect::get_label(d, a.e) as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, label, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
// function s.rthop(e,tp,eg,ep,ev,re,r,rp)
fn rthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e) as u32;
    // if not Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,ct,nil) then return end
    if !duel::is_existing_matching_card(d, Some(&|d: &mut Duel, c: CardId| card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD, ct, NO_EX) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,0,LOCATION_ONFIELD,ct,ct,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d: &mut Duel, c: CardId| card::is_able_to_hand(d, c, None)), tp, 0, LOCATION_ONFIELD, ct as u16, ct as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
