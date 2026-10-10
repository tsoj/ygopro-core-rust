//! Gift Exchange
use super::prelude::*;

pub const ID: u32 = 82257940;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_DECK,0,1,nil,tp,POS_FACEDOWN)
    //     and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_DECK,1,nil,1-tp,POS_FACEDOWN) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(a.tp), Some(POS_FACEDOWN as u8), None)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(1 - a.tp), Some(POS_FACEDOWN as u8), None)), 1 - a.tp, 0, LOCATION_DECK, 1, NO_EX);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g1=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,LOCATION_DECK,0,nil,tp,POS_FACEDOWN)
    let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(a.tp), Some(POS_FACEDOWN as u8), None)), a.tp, LOCATION_DECK, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(Card.IsAbleToRemove,1-tp,LOCATION_DECK,0,nil,1-tp,POS_FACEDOWN)
    let g2 = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, Some(1 - a.tp), Some(POS_FACEDOWN as u8), None)), 1 - a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g1==0 or #g2==0 then return end
    if g1.is_empty() || g2.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local rg1=g1:Select(tp,1,1,nil)
    let mut rg1 = group::select(d, &g1, a.tp, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_REMOVE as u64);
    // local rg2=g2:Select(1-tp,1,1,nil)
    let rg2 = group::select(d, &g2, 1 - a.tp, 1, 1, false, NO_EX).nil();
    // rg1:Merge(rg2)
    group::add_card(&mut rg1, &rg2);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local fid=c:GetFieldID()
    let fid = card::get_field_id(d, c);
    // Duel.Remove(rg1,POS_FACEDOWN,REASON_EFFECT)
    duel::remove(d, rg1.clone(), POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    // rg1:GetFirst():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0,fid)
    let first = group::get_first(d, &rg1).nil();
    card::register_flag_effect(d, first, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, fid as i64, 0);
    // rg1:GetNext():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0,fid)
    let second = group::iter(d, &rg1).get(1).copied().nil();
    card::register_flag_effect(d, second, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, fid as i64, 0);
    // rg1:KeepAlive()  (groups are values in Rust: nothing to do)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetLabel(fid)
    effect::set_label(d, e1, &[fid as i64]);
    // e1:SetLabelObject(rg1)
    effect::set_label_object(d, e1, Value::Group(rg1));
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.thfilter(c,fid)`
fn thfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // if g:IsDeleted() then return false end
    // (a Rust group is never deleted: DeleteGroup below does nothing)
    // if g:Filter(s.thfilter,nil,e:GetLabel()):GetCount()<2 then
    let fid = effect::get_label(d, a.e);
    if group::filter(d, &g, &|d, c| thfilter(d, c, fid), NO_EX).len() < 2 {
        // g:DeleteGroup()
        // return false
        return false;
    }
    // else return true end
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // local tc1=g:GetFirst()
    let tc1 = group::get_first(d, &g).nil();
    // local tc2=g:GetNext()
    let tc2 = group::iter(d, &g).get(1).copied().nil();
    // g:DeleteGroup()
    // Duel.SendtoHand(tc1,1-tc1:GetControler(),REASON_EFFECT)
    let p1 = 1 - card::get_controler(d, tc1);
    duel::send_to_hand(d, tc1, Some(p1), REASON_EFFECT, None);
    // Duel.SendtoHand(tc2,1-tc2:GetControler(),REASON_EFFECT)
    let p2 = 1 - card::get_controler(d, tc2);
    duel::send_to_hand(d, tc2, Some(p2), REASON_EFFECT, None);
}
