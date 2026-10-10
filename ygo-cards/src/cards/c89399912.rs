//! Tempest, Dragon Ruler of Storms
use super::prelude::*;

pub const ID: u32 = 89399912;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.selfspcost)
    effect::set_cost(d, e1, Some(cost_target(selfspcost)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return this card to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE|PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_END);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) and e:GetHandler():IsSpecialSummoned() end)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // e2:SetTarget(s.rthtg)
    effect::set_target(d, e2, Some(cost_target(rthtg)));
    // e2:SetOperation(s.rthop)
    effect::set_operation(d, e2, Some(operation(rthop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add 1 Dragon monster from your Deck to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(DragonRuler.SelfDiscardToGraveCost(ATTRIBUTE_WIND))
    effect::set_cost(d, e3, Some(aux::dragon_ruler::self_discard_to_grave_cost(ATTRIBUTE_WIND)));
    // e3:SetTarget(s.dthtg)
    effect::set_target(d, e3, Some(cost_target(dthtg)));
    // e3:SetOperation(s.dthop)
    effect::set_operation(d, e3, Some(operation(dthop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Add 1 WIND Dragon monster from your Deck to your hand
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e4, aux::stringid(ID, 3));
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_REMOVE)
    effect::set_code(d, e4, EVENT_REMOVE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetTarget(s.wdthtg)
    effect::set_target(d, e4, Some(cost_target(wdthtg)));
    // e4:SetOperation(s.wdthop)
    effect::set_operation(d, e4, Some(operation(wdthop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spcostfilter(c)`
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsRace(RACE_DRAGON) or c:IsAttribute(ATTRIBUTE_WIND)) and c:IsAbleToRemoveAsCost()
    //   and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true))
    (card::is_race(d, c, RACE_DRAGON) || card::is_attribute(d, c, ATTRIBUTE_WIND))
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, false))
}
/// `s.selfspcost(e,tp,...,chk)`
fn selfspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local rg=Duel.GetMatchingGroup(s.spcostfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let rg = duel::get_matching_group(d, Some(&spcostfilter), a.tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, h.into());
    // if chk==0 then return #rg>1 and aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),0) end
    if a.chk == Some(0) {
        return rg.len() > 1
            && matches!(aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 0, a.tp, None, None, None, false), Err(true));
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE)
    let g = aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.selfsptg(e,tp,...,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.selfspop(e,tp,...)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) and e:GetHandler():IsSpecialSummoned() end`
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp) && {
        let h = effect::get_handler(d, a.e).nil();
        card::is_special_summoned(d, h)
    }
}
/// `s.rthtg(e,tp,...,chk)`
fn rthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.rthop(e,tp,...)`
fn rthop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(c), None, REASON_EFFECT, None);
    }
}
/// `s.dthfilter(c)`
fn dthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsAbleToHand()
    card::is_race(d, c, RACE_DRAGON) && card::is_able_to_hand(d, c, None)
}
/// `s.dthtg(e,tp,...,chk)`
fn dthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // Excluding itself for a correct interaction with "Chasma, Dragon Ruler of Auroras" [04965193]
    // if chk==0 then return Duel.IsExistingMatchingCard(s.dthfilter,tp,LOCATION_DECK,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&dthfilter), a.tp, LOCATION_DECK, 0, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.dthop(e,tp,...)`
fn dthop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.dthfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&dthfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
/// `s.wdthfilter(c)`
fn wdthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsAttribute(ATTRIBUTE_WIND) and c:IsAbleToHand()
    card::is_race(d, c, RACE_DRAGON) && card::is_attribute(d, c, ATTRIBUTE_WIND) && card::is_able_to_hand(d, c, None)
}
/// `s.wdthtg(e,tp,...,chk)`
fn wdthtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.wdthfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&wdthfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.wdthop(e,tp,...)`
fn wdthop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.wdthfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&wdthfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
