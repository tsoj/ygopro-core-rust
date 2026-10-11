//! Cynet Backdoor
use super::prelude::*;

pub const ID: u32 = 43839002;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 of your cyberse monsters, add 1 cyberse monster whose ATK is lower than banished monster's ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmfilter(c,tp)`
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_CYBERSE) and c:IsAbleToRemove() and c:GetBaseAttack()>0
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c)
    card::is_faceup(d, c)
        && card::is_race(d, c, RACE_CYBERSE)
        && card::is_able_to_remove(d, c, None, None, None)
        && card::get_base_attack(d, c) > 0
        && duel::is_existing_matching_card(d, Some(&|d, cc| thfilter(d, cc, c)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.thfilter(c,rc)`
fn thfilter(d: &mut Duel, c: CardId, rc: CardId) -> bool {
    // return c:IsRace(RACE_CYBERSE) and c:GetAttack()<rc:GetBaseAttack() and c:IsAbleToHand()
    card::is_race(d, c, RACE_CYBERSE) && card::get_attack(d, c) < card::get_base_attack(d, rc) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.rmfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && rmfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| rmfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| rmfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Remove(tc,tc:GetPosition(),REASON_EFFECT|REASON_TEMPORARY)>0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            let pos = card::get_position(d, tc) as u8;
            if duel::remove(d, CardSet::from_card(tc), pos, REASON_EFFECT | REASON_TEMPORARY, None, None) > 0 {
                let h = effect::get_handler(d, a.e).nil();
                // local e1=Effect.CreateEffect(e:GetHandler())
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
                // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
                effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
                // if Duel.IsPhase(PHASE_STANDBY) then e1:SetLabel(Duel.GetTurnCount()) end
                if duel::is_phase(d, PHASE_STANDBY) {
                    let turn = duel::get_turn_count(d, None) as i64;
                    effect::set_label(d, e1, &[turn]);
                }
                // e1:SetLabelObject(tc)
                effect::set_label_object(d, e1, Value::Card(tc));
                // e1:SetCountLimit(1)
                effect::set_count_limit(d, e1, 1, 0, 0, 0);
                // e1:SetCondition(s.retcon)
                effect::set_condition(d, e1, Some(condition(retcon)));
                // e1:SetTarget(s.rettg)
                effect::set_target(d, e1, Some(cost_target(rettg)));
                // e1:SetOperation(s.retop)
                effect::set_operation(d, e1, Some(operation(retop)));
                // Duel.RegisterEffect(e1,tp)
                duel::register_effect(d, e1, a.tp);
                // if tc:IsPreviousPosition(POS_FACEDOWN) then return end
                if card::is_previous_position(d, tc, POS_FACEDOWN) {
                    return;
                }
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
                // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,tc)
                let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, tc)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
                // if #g>0 then
                if !g.is_empty() {
                    // Duel.SendtoHand(g,nil,REASON_EFFECT)
                    duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
                    // Duel.ConfirmCards(1-tp,g)
                    duel::confirm_cards(d, 1 - a.tp, g);
                }
            }
        }
    }
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and Duel.GetTurnCount()~=e:GetLabel()
    duel::is_turn_player(d, a.tp) && duel::get_turn_count(d, None) as i64 != effect::get_label(d, a.e)
}
/// `s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:Reset()
    effect::reset(d, a.e);
    true
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if tc and Duel.ReturnToField(tc) then
    if let Some(tc) = tc {
        if duel::return_to_field(d, tc, None, None) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetDescription(3205)
            effect::set_description(d, e1, 3205);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_DIRECT_ATTACK)
            effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
