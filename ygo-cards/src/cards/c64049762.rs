//! Blitzclique - Breakaway
use super::prelude::*;

pub const ID: u32 = 64049762;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BLITZCLIQUE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects;
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a card(s) is destroyed by your "Blitzclique" card's effect, while this card is in your GY: You can add this card to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.selfthcon)
    effect::set_condition(d, e2, Some(condition(selfthcon)));
    // e2:SetTarget(s.selfthtg)
    effect::set_target(d, e2, Some(cost_target(selfthtg)));
    // e2:SetOperation(s.selfthop)
    effect::set_operation(d, e2, Some(operation(selfthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.plfilter(c,tp)
fn plfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_BLITZCLIQUE) and c:IsContinuousTrap() and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_set_card(d, c, &[SET_BLITZCLIQUE as u16]) && card::is_continuous_trap(d, c) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
// function s.descostfilter(c,hc)
fn descostfilter(d: &mut Duel, c: CardId, hc: CardId) -> bool {
    // return c:IsRace(RACE_THUNDER) and c:IsFaceup() and c:IsAbleToHandAsCost()
    //     and Duel.IsExistingTarget(nil,0,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c:GetEquipGroup()+c+hc)
    card::is_race(d, c, RACE_THUNDER)
        && card::is_faceup(d, c)
        && card::is_able_to_hand_as_cost(d, c)
        && {
            let mut ex = card::get_equip_group(d, c);
            group::add_card(&mut ex, &CardSet::from_card(c));
            group::add_card(&mut ex, &CardSet::from_card(hc));
            duel::is_existing_target(d, None, 0, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, (&ex).into())
        }
}
// function s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if c:IsLocation(LOCATION_HAND) then ft=ft-1 end
    if card::is_location(d, c, LOCATION_HAND) {
        ft -= 1;
    }
    // --● Place 1 "Blitzclique" Continuous Trap from your Deck face-up on your field
    // local b1=ft>0 and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil,tp)
    let b1 = ft > 0 && duel::is_existing_matching_card(d, Some(&|d, x| plfilter(d, x, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    // --● Return 1 Thunder monster you control to the hand, then target 1 card on the field; destroy it
    // local b2=Duel.IsExistingMatchingCard(s.descostfilter,tp,LOCATION_MZONE,0,1,nil,c)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, x| descostfilter(d, x, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local cd=e:GetChainData()
    // cd.choice=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)})
    let choice = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, choice.map_or(Value::Nil, |x| Value::Int(x as i64)));
    // if cd.choice==2 then
    if matches!(chain::effect_data(d, a.e, None, chain::field::CARD), Value::Int(2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.descostfilter,tp,LOCATION_MZONE,0,1,1,nil,c)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| descostfilter(d, x, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_COST)
        duel::send_to_hand(d, g, None, REASON_COST, None);
    }
    true
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)==2 and chkc:IsOnField() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        let param = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM]);
        return matches!(param.first(), Some(Value::Int(2))) && card::is_on_field(d, chkc) && chkc != c;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if c:IsLocation(LOCATION_HAND) then ft=ft-1 end
    if card::is_location(d, c, LOCATION_HAND) {
        ft -= 1;
    }
    // local b1=ft>0 and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK,0,1,nil,tp)
    let b1 = ft > 0 && duel::is_existing_matching_card(d, Some(&|d, x| plfilter(d, x, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local b2=Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c)
    let b2 = duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // cd.choice=cd.choice or Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,2)},
    //     {b2,aux.Stringid(id,3)})
    let cur = chain::effect_data(d, a.e, None, chain::field::CARD);
    let choice = if cur.truthy() {
        cur
    } else {
        duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).map_or(Value::Nil, |x| Value::Int(x as i64))
    };
    chain::set_effect_data(d, a.e, None, chain::field::CARD, choice.clone());
    // if cd.choice==1 then
    if matches!(choice, Value::Int(1)) {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
    // elseif cd.choice==2 then
    } else if matches!(choice, Value::Int(2)) {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c)
        let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into());
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local cd=e:GetChainData()
    // if cd.choice==1 then
    let choice = chain::effect_data(d, a.e, None, chain::field::CARD);
    if matches!(choice, Value::Int(1)) {
        // --● Place 1 "Blitzclique" Continuous Trap from your Deck face-up on your field
        // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.plfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, x| plfilter(d, x, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc then
        if let Some(sc) = sc {
            // Duel.MoveToField(sc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
            duel::move_to_field(d, sc, tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        }
    // elseif cd.choice==2 then
    } else if matches!(choice, Value::Int(2)) {
        // --● Return 1 Thunder monster you control to the hand, then target 1 card on the field; destroy it
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
// function s.selfthcon(e,tp,eg,ep,ev,re,r,rp)
fn selfthcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==tp and re and not eg:IsContains(e:GetHandler()) and eg:IsExists(Card.IsReason,1,nil,REASON_EFFECT)
    //     and re:GetHandler() and re:GetHandler():IsSetCard(SET_BLITZCLIQUE)
    if a.rp != a.tp {
        return false;
    }
    let Some(re) = a.re else { return false };
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    if eg.contains(h) {
        return false;
    }
    if !group::is_exists(d, &eg, &|d, x| card::is_reason(d, x, REASON_EFFECT), 1, NO_EX) {
        return false;
    }
    match effect::get_handler(d, re) {
        Some(rc) => card::is_set_card(d, rc, &[SET_BLITZCLIQUE as u16]),
        None => false,
    }
}
// function s.selfthtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn selfthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.selfthop(e,tp,eg,ep,ev,re,r,rp)
fn selfthop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
