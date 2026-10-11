//! Phantom Hand
//! ハンドレス・フェイク
use super::prelude::*;

pub const ID: u32 = 40555959;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_INFERNITY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.remcon)
    effect::set_condition(d, e2, Some(condition(remcon)));
    // e2:SetTarget(s.remtg)
    effect::set_target(d, e2, Some(cost_target(remtg)));
    // e2:SetOperation(s.remop)
    effect::set_operation(d, e2, Some(operation(remop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_INFERNITY)`
fn infernity_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_INFERNITY as u16])
}
/// `s.remcon(e,tp,eg,ep,ev,re,r,rp)`
fn remcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_INFERNITY),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&infernity_faceup), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.remtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn remtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_HAND,0,1,nil,tp,POS_FACEDOWN) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)),
            tp,
            LOCATION_HAND,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.remop(e,tp,eg,ep,ev,re,r,rp)`
fn remop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not (e:GetHandler():IsRelateToEffect(e) and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_INFERNITY),tp,LOCATION_MZONE,0,1,nil)) then return end
    let h = effect::get_handler(d, e).nil();
    let related = card::is_relate_to_effect(d, h, e);
    if !(related && duel::is_existing_matching_card(d, Some(&infernity_faceup), tp, LOCATION_MZONE, 0, 1, NO_EX)) {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_HAND,0,nil,tp,POS_FACEDOWN)
    let g = duel::get_matching_group(
        d,
        Some(&|d, c| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)),
        tp,
        LOCATION_HAND,
        0,
        NO_EX,
    );
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)
        duel::remove(d, g.clone(), POS_FACEDOWN as u8, REASON_EFFECT, None, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
        effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // if Duel.IsPhase(PHASE_STANDBY) then e1:SetLabel(Duel.GetTurnCount()) end
        if duel::is_phase(d, PHASE_STANDBY) {
            let tc = duel::get_turn_count(d, None);
            effect::set_label(d, e1, &[tc as i64]);
        }
        // e1:SetLabelObject(g)
        effect::set_label_object(d, e1, Value::Group(g.clone()));
        // e1:SetCondition(s.retcon)
        effect::set_condition(d, e1, Some(condition(retcon)));
        // e1:SetOperation(s.retop)
        effect::set_operation(d, e1, Some(operation(retop)));
        // if Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_STANDBY) then e1:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,2)
        if duel::is_turn_player(d, tp) && duel::is_phase(d, PHASE_STANDBY) {
            effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 2);
        } else {
            // else e1:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN) end
            effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 1);
        }
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // g:KeepAlive()
        // local tc=g:GetFirst()
        let _ = group::get_first(d, &g);
        // for tc in aux.Next(g) do
        for tc in group::iter(d, &g) {
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        }
    }
}
/// `s.retfilter(c)`: `c:GetFlagEffect(id)>0`
fn retfilter(d: &mut Duel, c: CardId) -> bool {
    card::get_flag_effect(d, c, ID) > 0
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.GetTurnCount()~=e:GetLabel()
    let label = effect::get_label(d, a.e);
    duel::is_turn_player(d, a.tp) && duel::get_turn_count(d, None) as i64 != label
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // local sg=g:Filter(s.retfilter,nil)
    let sg = group::filter(d, &g, &retfilter, NO_EX);
    // g:DeleteGroup()
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
    }
}
