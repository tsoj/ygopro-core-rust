//! Mekk-Knight Purple Nightfall
use super::prelude::*;

pub const ID: u32 = 28692962;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_MEKK_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetValue(s.hspval)
    effect::set_value_fn(d, e1, lua_fn(hspval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 "Mekk-Knight" monster, add 1 "Mekk-Knight" monster from Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetColumnGroupCount()>0
    card::get_column_group_count(d, c, 0, 0) > 0
}
/// `s.hspval(e,c)`
fn hspval(d: &mut Duel, a: Args) -> Value {
    // local tp=c:GetControler()
    let c = a.c(1);
    let tp = card::get_controler(d, c);
    // local zone=0
    let mut zone: u32 = 0;
    // local lg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let lg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // for tc in aux.Next(lg) do
    for tc in group::iter(d, &lg) {
        // zone=(zone|tc:GetColumnZone(LOCATION_MZONE,0,0,tp))
        zone |= card::get_column_zone(d, tc, LOCATION_MZONE, 0, 0, Some(tp));
    }
    // return 0,zone
    Value::Multi(vec![Value::Int(0), Value::Int(zone as i64)])
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MEKK_KNIGHT) and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MEKK_KNIGHT as u16]) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MEKK_KNIGHT) and c:IsMonster() and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MEKK_KNIGHT as u16]) && card::is_monster(d, c) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.rmfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && rmfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&rmfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT|REASON_TEMPORARY)~=0
    //     and tc:IsLocation(LOCATION_REMOVED) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e)
            && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT | REASON_TEMPORARY, None, None) != 0
            && card::is_location(d, tc, LOCATION_REMOVED)
        {
            // local ct=(Duel.IsTurnPlayer(tp) and Duel.GetCurrentPhase()<=PHASE_STANDBY) and 2 or 1
            let early = duel::is_turn_player(d, tp) && duel::get_current_phase(d) <= PHASE_STANDBY;
            let ct: u16 = if early { 2 } else { 1 };
            // local val=(Duel.IsTurnPlayer(tp) and Duel.GetCurrentPhase()<=PHASE_STANDBY) and Duel.GetTurnCount() or (Duel.GetTurnCount()-1)
            let val = if early { duel::get_turn_count(d, None) } else { duel::get_turn_count(d, None) - 1 };
            // local e1=Effect.CreateEffect(e:GetHandler())
            let handler = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, handler);
            // e1:SetDescription(aux.Stringid(id,1))
            effect::set_description(d, e1, aux::stringid(ID, 1));
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
            effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
            // e1:SetLabelObject(tc)
            effect::set_label_object(d, e1, Value::Card(tc));
            // e1:SetCountLimit(1)
            effect::set_count_limit(d, e1, 1, 0, 0, 0);
            // e1:SetCondition(s.retcon)
            effect::set_condition(d, e1, Some(condition(retcon)));
            // e1:SetOperation(s.retop)
            effect::set_operation(d, e1, Some(operation(retop)));
            // e1:SetLabel(val)
            effect::set_label(d, e1, &[val as i64]);
            // e1:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,ct)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, ct);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
            // tc:CreateEffectRelation(e1)
            card::create_effect_relation(d, tc, e1);
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
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if Duel.IsTurnPlayer(1-tp) or Duel.GetTurnCount()==e:GetLabel() then return false end
    if duel::is_turn_player(d, 1 - tp) || duel::get_turn_count(d, None) as i64 == effect::get_label(d, e) {
        return false;
    }
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, e).as_card().nil();
    // return tc:IsRelateToEffect(e)
    card::is_relate_to_effect(d, tc, e)
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.ReturnToField(tc)
    duel::return_to_field(d, tc, None, None);
}
