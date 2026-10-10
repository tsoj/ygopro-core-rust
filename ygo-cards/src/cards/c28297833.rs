//! Necroface
use super::prelude::*;

pub const ID: u32 = 28297833;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.tdtg)
    effect::set_target(d, e1, Some(cost_target(tdtg)));
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // remove
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_ALL,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_ALL as u8, LOCATION_REMOVED as i32);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetFieldGroup(tp,LOCATION_REMOVED,LOCATION_REMOVED)
    let g = duel::get_field_group(d, a.tp, LOCATION_REMOVED, LOCATION_REMOVED);
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    // local ct=g:FilterCount(Card.IsLocation,nil,LOCATION_DECK)
    let ct = group::filter_count(d, &g, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // e1:SetValue(ct*100)
        effect::set_value(d, e1, ct as i64 * 100);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,5,PLAYER_ALL,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 5, PLAYER_ALL as u8, LOCATION_DECK as i32);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g1=Duel.GetDecktopGroup(tp,5)
    let mut g1 = duel::get_decktop_group(d, a.tp, 5);
    // local g2=Duel.GetDecktopGroup(1-tp,5)
    let g2 = duel::get_decktop_group(d, 1 - a.tp, 5);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.Remove(g1,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g1, POS_FACEUP as u8, REASON_EFFECT, None, None);
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(500)
        effect::set_value(d, e1, 500);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
