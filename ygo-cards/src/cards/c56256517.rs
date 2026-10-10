//! Ledger of Legerdemain
use super::prelude::*;

pub const ID: u32 = 56256517;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetDecktopGroup(tp,3)
    let g = duel::get_decktop_group(d, a.tp, 3);
    // if chk==0 then return g:FilterCount(Card.IsAbleToRemove,nil,tp,POS_FACEDOWN)==3 end
    if a.chk == Some(0) {
        return group::filter_count(d, &g, &|d, c| card::is_able_to_remove(d, c, Some(a.tp), Some(POS_FACEDOWN as u8), None), NO_EX) == 3;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,3,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 3, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,3,tp,LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 3, a.tp, LOCATION_REMOVED as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)<3 then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) < 3 {
        return;
    }
    // local g=Duel.GetDecktopGroup(tp,3)
    let g = duel::get_decktop_group(d, a.tp, 3);
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // if Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)~=3 or not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if duel::remove(d, g.clone(), POS_FACEDOWN as u8, REASON_EFFECT, None, None) != 3 || !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // g:KeepAlive()
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // c:SetTurnCounter(0)
    card::set_turn_counter(d, c.nil(), 0);
    // local fid=Duel.GetTurnCount()
    let fid = duel::get_turn_count(d, None) as i64;
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid, 0);
    }
    // Add those 3 cards to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c.nil());
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetLabel(fid)
    effect::set_label(d, e1, &[fid]);
    // e1:SetLabelObject(g)
    effect::set_label_object(d, e1, Value::Group(g));
    // e1:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // e1:SetReset(RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,3)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 3);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `function(e,tp) return Duel.IsTurnPlayer(tp) end` of the delayed effect
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
/// `s.thfilter(c,fid)`
fn thfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, a.e);
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, a.e);
    // if ct+6==Duel.GetTurnCount() then
    if ct + 6 == duel::get_turn_count(d, None) as i64 {
        // local g=e:GetLabelObject():Filter(s.thfilter,nil,e:GetLabel())
        let lo = effect::get_label_object(d, a.e).as_group().cloned().nil();
        let g = group::filter(d, &lo, &|d, c| thfilter(d, c, ct), NO_EX);
        // if g and #g==3 then
        if g.len() == 3 {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g, None, REASON_EFFECT, None);
        }
    }
}
