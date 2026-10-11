//! Medusa, Watcher of the Evil Eye
use super::prelude::*;

pub const ID: u32 = 18551923;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_EVIL_EYE_SELENE], listed_series: &[SET_EVIL_EYE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Evil Eye" card from the GY to the hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_LEAVE_GRAVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 monster from the opponent's GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e2:SetCondition(s.rmcon)
    effect::set_condition(d, e2, Some(condition(rmcon)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish 1 card from your GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_LEAVE_GRAVE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.rmscon)
    effect::set_condition(d, e3, Some(condition(rmscon)));
    // e3:SetTarget(s.rmstg)
    effect::set_target(d, e3, Some(cost_target(rmstg)));
    // e3:SetOperation(s.rmsop)
    effect::set_operation(d, e3, Some(operation(rmsop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EVIL_EYE) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_EVIL_EYE as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,tp,LOCATION_GRAVE)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - a.tp, tc);
    }
}
/// `s.rmcon(e)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local eg=c:GetEquipGroup()
    let eg = card::get_equip_group(d, c);
    // return #eg>0 and eg:IsExists(Card.IsCode,1,nil,CARD_EVIL_EYE_SELENE)
    !eg.is_empty() && group::is_exists(d, &eg, &|d, x| card::is_code(d, x, &[CARD_EVIL_EYE_SELENE]), 1, NO_EX)
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove()
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(1-tp) and s.rmfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - a.tp) && rmfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,0,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,0,LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&rmfilter), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    let h = effect::get_handler(d, a.e).nil();
    // if Duel.IsPhase(PHASE_STANDBY) then
    if duel::is_phase(d, PHASE_STANDBY) {
        // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY,EFFECT_FLAG_OATH,2,Duel.GetTurnCount())
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY, EFFECT_FLAG_OATH, 2, duel::get_turn_count(d, None) as i64, 0);
    } else {
        // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY,EFFECT_FLAG_OATH,1)
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY, EFFECT_FLAG_OATH, 1, 0, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.rmscon(e,tp,eg,ep,ev,re,r,rp)`
fn rmscon(d: &mut Duel, a: &EffArgs) -> bool {
    // local label=e:GetHandler():GetFlagEffectLabel(id)
    let h = effect::get_handler(d, a.e).nil();
    let label = card::get_flag_effect_label(d, h, ID).first().copied();
    // return label and label~=Duel.GetTurnCount()
    match label {
        Some(l) => l != duel::get_turn_count(d, None) as i64,
        None => false,
    }
}
/// `s.rmstg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmstg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local sg=Duel.GetMatchingGroup(nil,tp,LOCATION_GRAVE,0,nil)
    let sg = duel::get_matching_group(d, None, a.tp, LOCATION_GRAVE, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,sg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(sg), 1, 0, 0);
    true
}
/// `s.rmsop(e,tp,eg,ep,ev,re,r,rp)`
fn rmsop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g then   (#g is a number, so this is always true in Lua)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
