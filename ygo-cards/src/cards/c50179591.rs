//! Abyss Script - Fire Dragon's Lair
use super::prelude::*;

pub const ID: u32 = 50179591;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_ABYSS_ACTOR}
    listed_series: &[SET_ABYSS_ACTOR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroyed
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.rmcon2)
    effect::set_condition(d, e2, Some(condition(rmcon2)));
    // e2:SetTarget(s.rmtg2)
    effect::set_target(d, e2, Some(cost_target(rmtg2)));
    // e2:SetOperation(s.rmop2)
    effect::set_operation(d, e2, Some(operation(rmop2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ABYSS_ACTOR)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_LEAVE|RESET_TODECK|RESET_TEMP_REMOVE|RESET_REMOVE|RESET_TOGRAVE)|RESET_PHASE|PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,0))
        card::register_flag_effect(
            d,
            tc,
            ID,
            RESET_EVENT | (RESETS_STANDARD & !(RESET_LEAVE | RESET_TODECK | RESET_TEMP_REMOVE | RESET_REMOVE | RESET_TOGRAVE)) | RESET_PHASE | PHASE_END,
            EFFECT_FLAG_CLIENT_HINT,
            1,
            0,
            aux::stringid(ID, 0),
        );
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_BATTLE_DESTROYING)
        effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // e1:SetCondition(s.rmcon1)
        effect::set_condition(d, e1, Some(condition(rmcon1)));
        // e1:SetOperation(s.rmop1)
        effect::set_operation(d, e1, Some(operation(rmop1)));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
    }
}
/// `s.rmcon1(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // return eg:IsContains(tc) and tc:GetFlagEffect(id)~=0
    group::is_contains(&a.eg.clone().nil(), tc) && card::get_flag_effect(d, tc, ID) != 0
}
/// `s.rmop1(e,tp,eg,ep,ev,re,r,rp)`
fn rmop1(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_EXTRA,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_EXTRA, NO_EX);
    // if #g<3 then return end
    if g.len() < 3 {
        return;
    }
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_REMOVE as u64);
    // local mg=g:Select(1-tp,3,3,nil)
    let mg = group::select(d, &g, 1 - a.tp, 3, 3, false, NO_EX).nil();
    // if #mg>0 then
    if !mg.is_empty() {
        // Duel.Remove(mg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, mg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.rmcon2(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsReason(REASON_EFFECT) and c:IsPreviousControler(tp)
    //     and c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEDOWN)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_EXTRA,0,1,nil)
    a.rp == 1 - a.tp
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEDOWN)
        && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.rmtg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_EXTRA,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_EXTRA, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 0, 0);
    true
}
/// `s.rmop2(e,tp,eg,ep,ev,re,r,rp)`
fn rmop2(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_EXTRA)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_EXTRA);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, a.tp, g.clone());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local mg=g:FilterSelect(tp,Card.IsAbleToRemove,1,1,nil)
    let mg = group::filter_select(d, &g, a.tp, &|d, c| card::is_able_to_remove(d, c, None, None, None), 1, 1, false, NO_EX).nil();
    // if #mg>0 then
    if !mg.is_empty() {
        // Duel.Remove(mg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, mg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
    // Duel.ShuffleExtra(1-tp)
    duel::shuffle_extra(d, 1 - a.tp);
}
