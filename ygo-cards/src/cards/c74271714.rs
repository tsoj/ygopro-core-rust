//! Fish and Bids
use super::prelude::*;

pub const ID: u32 = 74271714;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(function() return Duel.IsPhase(PHASE_MAIN1) and not Duel.CheckPhaseActivity() end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.rmtgcost)
    effect::set_cost(d, e1, Some(cost_target(rmtgcost)));
    // e1:SetTarget(s.rmtgtg)
    effect::set_target(d, e1, Some(cost_target(rmtgtg)));
    // e1:SetOperation(s.rmtgop)
    effect::set_operation(d, e1, Some(operation(rmtgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsPhase(PHASE_MAIN1) and not Duel.CheckPhaseActivity() end
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_phase(d, PHASE_MAIN1) && !duel::check_phase_activity(d)
}
// function s.rmtgcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmtgcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, cc| card::is_able_to_remove_as_cost(d, cc, None)), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, cc| card::is_able_to_remove_as_cost(d, cc, None)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.rmtgfilter(c)
fn rmtgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FISH) and (c:IsAbleToRemove() or c:IsAbleToGrave())
    card::is_race(d, c, RACE_FISH) && (card::is_able_to_remove(d, c, None, None, None) || card::is_able_to_grave(d, c, None))
}
// function s.rescon(sg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsAbleToRemove,nil)==2 or sg:FilterCount(Card.IsAbleToGrave,nil)==2
    let r = group::filter_count(d, sg, &|d, c| card::is_able_to_remove(d, c, None, None, None), NO_EX) == 2
        || group::filter_count(d, sg, &|d, c| card::is_able_to_grave(d, c, None), NO_EX) == 2;
    (r, false)
}
// function s.rmtgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmtgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.rmtgfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&rmtgfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,2,1-tp,LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 2, 1 - a.tp, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,2,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 2, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,2,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
// function s.rmtgop(e,tp,eg,ep,ev,re,r,rp)
fn rmtgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local oppo=1-tp
    let oppo = 1 - tp;
    // local hg=Duel.GetMatchingGroup(Card.IsAbleToRemove,oppo,LOCATION_HAND,0,nil,oppo)
    let hg = duel::get_matching_group(d, Some(&|d, cc| card::is_able_to_remove(d, cc, Some(oppo), None, None)), oppo, LOCATION_HAND, 0, NO_EX);
    // if #hg>=2 and Duel.SelectYesNo(oppo,aux.Stringid(id,1)) then
    if hg.len() >= 2 && duel::select_yes_no(d, oppo, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,oppo,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, oppo, HINTMSG_REMOVE as u64);
        // local hsg=hg:Select(oppo,2,2,nil)
        let hsg = group::select(d, &hg, oppo, 2, 2, false, NO_EX).nil();
        // return #hsg==2 and Duel.Remove(hsg,POS_FACEUP,REASON_EFFECT,PLAYER_NONE,oppo)==2
        if hsg.len() == 2 {
            duel::remove(d, hsg, POS_FACEUP as u8, REASON_EFFECT, Some(PLAYER_NONE as u8), Some(oppo));
        }
        return;
    }
    // local g=Duel.GetMatchingGroup(s.rmtgfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&rmtgfilter), tp, LOCATION_DECK, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,aux.Stringid(id,2))
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(aux::stringid(ID, 2)), None, None, false)
        .unwrap_or_default();
    // if #sg==2 then
    if sg.len() == 2 {
        // local b1=sg:FilterCount(Card.IsAbleToRemove,nil)==2
        let b1 = group::filter_count(d, &sg, &|d, c| card::is_able_to_remove(d, c, None, None, None), NO_EX) == 2;
        // local b2=sg:FilterCount(Card.IsAbleToGrave,nil)==2
        let b2 = group::filter_count(d, &sg, &|d, c| card::is_able_to_grave(d, c, None), NO_EX) == 2;
        // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,3)},{b2,aux.Stringid(id,4)})
        let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
        if op == Some(1) {
            // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        } else if op == Some(2) {
            // Duel.SendtoGrave(sg,REASON_EFFECT)
            duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
        }
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,5))
    effect::set_description(d, e1, aux::stringid(ID, 5));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsRace(RACE_FISH) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, cc| !card::is_race(d, cc, RACE_FISH))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
