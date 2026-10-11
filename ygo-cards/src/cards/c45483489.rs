//! Deep Sea Sentry
use super::prelude::*;

pub const ID: u32 = 45483489;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make the opponent banish 1 monster from the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.rmcond)
    effect::set_condition(d, e1, Some(condition(rmcond)));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 Level 4 or Lower WATER monster from the from GY to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.tdcost)
    effect::set_cost(d, e2, Some(cost_target(tdcost)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rmcond(e,tp,eg,ep,ev,re,r,rp)`
fn rmcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsReason(REASON_COST) and re:IsActivated() and re:IsMonsterEffect()
    //     and re:GetHandler():IsAttribute(ATTRIBUTE_WATER)
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_reason(d, h, REASON_COST) {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_activated(d, re) {
        return false;
    }
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    card::is_attribute(d, rh, ATTRIBUTE_WATER)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let opp = 1 - tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_HAND,1,nil,1-tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(opp), None, None)), tp, 0, LOCATION_HAND, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, opp, LOCATION_HAND as i32);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local hg=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let hg = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // if Duel.IsPlayerAffectedByEffect(1-tp,30459350) or #hg==0 then return end
    if duel::is_player_affected_by_effect(d, 1 - tp, 30459350).is_some() || hg.len() == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=hg:Select(1-tp,1,1,nil)
    let g = group::select(d, &hg, 1 - tp, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT,PLAYER_NONE,1-tp)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, Some(PLAYER_NONE as u8), Some(1 - tp));
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local fid=c:GetFieldID()
    let fid = card::get_field_id(d, c) as i64;
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetLabel(fid)
    effect::set_label(d, e1, &[fid]);
    // e1:SetLabelObject(tc)
    effect::set_label_object(d, e1, Value::Card(tc.nil()));
    // e1:SetCondition(s.retcon)
    effect::set_condition(d, e1, Some(condition(retcon)));
    // e1:SetOperation(s.retop)
    effect::set_operation(d, e1, Some(operation(retop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // tc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1,fid)
    card::register_flag_effect(d, tc.nil(), ID, RESETS_STANDARD_PHASE_END, 0, 1, fid, 0);
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffectLabel(id)==e:GetLabel() then
    let flag = card::get_flag_effect_label(d, tc, ID).first().copied();
    if flag == Some(effect::get_label(d, a.e)) {
        // return true
        return true;
    }
    // e:Reset()
    effect::reset(d, a.e);
    // return false
    false
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.SendtoHand(tc,nil,REASON_EFFECT)
    duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
}
/// `s.tdcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeckAsCost(tp,2) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck_as_cost(d, a.tp, 2);
    }
    // Duel.DiscardDeck(tp,2,REASON_COST)
    duel::discard_deck(d, a.tp, 2, REASON_COST);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelBelow(4) and c:IsAttribute(ATTRIBUTE_WATER) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_level_below(d, c, 4) && card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local sg=tg:Filter(Card.IsRelateToEffect,nil,e)
    let sg = group::filter(d, &tg, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #sg>0 then
    if sg.len() > 0 {
        // Duel.SendtoHand(sg,tp,REASON_EFFECT)
        duel::send_to_hand(d, sg, Some(tp), REASON_EFFECT, None);
    }
}
