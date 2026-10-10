//! Mental Tuner
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 30291086;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Change this card's level
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.lvtg)
    effect::set_target(d, e1, Some(cost_target(lvtg)));
    // e1:SetOperation(s.lvop)
    effect::set_operation(d, e1, Some(operation(lvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) and c:IsAbleToRemoveAsCost()
    //     and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true))
    card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, false))
}
/// `s.tgfilter(c,e)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK) and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.attrescon(sg)`
fn attrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetAttribute)==#sg
    (group::get_class_count(d, sg, &|d, c| card::get_attribute(d, c) as i64) == sg.len(), false)
}
/// `s.lvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rmg=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE|LOCATION_HAND|LOCATION_GRAVE,0,c)
    let rmg = duel::get_matching_group(d, Some(&rmfilter), tp, LOCATION_MZONE | LOCATION_HAND | LOCATION_GRAVE, 0, c.into());
    // local tgg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_REMOVED,0,nil,e)
    let tgg = duel::get_matching_group(d, Some(&|d, tc| tgfilter(d, tc, a.e)), tp, LOCATION_REMOVED, 0, NO_EX);
    // if chk==0 then return c:HasLevel() and (#rmg>0 or #tgg>0) end
    if a.chk == Some(0) {
        return card::has_level(d, c) && (!rmg.is_empty() || !tgg.is_empty());
    }
    // e:SetCategory(0)
    effect::set_category(d, a.e, 0);
    // e:SetProperty(0)
    effect::set_property(d, a.e, 0, 0);
    // local cd=e:GetChainData()
    // cd.choice=Duel.SelectEffect(tp,{#rmg>0,aux.Stringid(id,1)},{#tgg>0,aux.Stringid(id,2)})
    let choice = duel::select_effect(d, tp, &[(!rmg.is_empty(), aux::stringid(ID, 1)), (!tgg.is_empty(), aux::stringid(ID, 2))]);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(choice.unwrap_or(0) as i64));
    // if cd.choice==1 then
    if choice == Some(1) {
        // local rg=aux.SelectUnselectGroup(rmg,e,tp,1,2,s.attrescon,1,tp,HINTMSG_REMOVE)
        let rg = aux::select_unselect_group(d, &rmg, Some(a.e), tp, Some(1), Some(2), Some(&attrescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
        // Duel.Remove(rg,POS_FACEUP,REASON_COST)
        duel::remove(d, rg, POS_FACEUP as u8, REASON_COST, None, None);
        // cd.banished_amount=Duel.GetOperatedGroup():FilterCount(Card.IsLocation,nil,LOCATION_REMOVED)
        let operated = duel::get_operated_group(d);
        let amount = group::filter_count(d, &operated, &|d, c| card::is_location(d, c, LOCATION_REMOVED), NO_EX);
        chain::set_effect_data(d, a.e, None, chain::field::CARD + 1, Value::Int(amount as i64));
    } else if choice == Some(2) {
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE);
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // local tg=aux.SelectUnselectGroup(tgg,e,tp,1,2,s.attrescon,1,tp,HINTMSG_TOGRAVE)
        let tg = aux::select_unselect_group(d, &tgg, Some(a.e), tp, Some(1), Some(2), Some(&attrescon), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
        // Duel.SetTargetCard(tg)
        duel::set_target_card(d, tg.clone());
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,tg,#tg,0,0)
        let n = tg.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(tg), n, 0, 0);
    }
    true
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local cd=e:GetChainData()
    // local ct=cd.banished_amount
    // (a nil value reads as 0 here: every use below returns on nil and on 0)
    let choice = chain::effect_data(d, a.e, None, chain::field::CARD).int();
    let mut ct = chain::effect_data(d, a.e, None, chain::field::CARD + 1).int();
    // if cd.choice==2 then
    if choice == 2 {
        // local tg=Duel.GetTargetCards(e)
        let tg = duel::get_target_cards(d, a.e).nil();
        // if #tg>0 then
        if !tg.is_empty() {
            // ct=Duel.SendtoGrave(tg,REASON_EFFECT|REASON_RETURN)
            ct = duel::send_to_grave(d, tg, REASON_EFFECT | REASON_RETURN, None, None) as i64;
        }
    // elseif cd.choice~=1 then return end
    } else if choice != 1 {
        return;
    }
    // if not ct or ct==0 then return end
    if ct == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup() and c:HasLevel()) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::has_level(d, c)) {
        return;
    }
    // local b1=c:IsLevelAbove(ct+1)
    let b1 = card::is_level_above(d, c, (ct + 1) as u32);
    // local b2=true
    let b2 = true;
    // local lvop=Duel.SelectEffect(tp,{b1,aux.Stringid(id,3)},{b2,aux.Stringid(id,4)})
    let lvop = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
    // if lvop then
    if let Some(lvop) = lvop {
        // c:UpdateLevel(lvop==1 and ct or -ct,RESETS_STANDARD_DISABLE_PHASE_END)
        let amt = if lvop == 1 { ct } else { -ct };
        card::update_level(d, c, amt, Some(RESETS_STANDARD_DISABLE_PHASE_END), None);
    }
}
