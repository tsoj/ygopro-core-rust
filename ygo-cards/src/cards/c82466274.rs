//! Serziel, Watcher of the Evil Eye
use super::prelude::*;

pub const ID: u32 = 82466274;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_EVIL_EYE_SELENE], listed_series: &[SET_EVIL_EYE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // tohand
    // e1:SetDescription(aux.Stringid(id,0))
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // e2:SetDescription(aux.Stringid(id,1))
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
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
    // e2:SetCondition(s.descond)
    effect::set_condition(d, e2, Some(condition(descond)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // negate
    // e3:SetDescription(aux.Stringid(id,2))
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.discond)
    effect::set_condition(d, e3, Some(condition(discond)));
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(cost_target(distg)));
    // e3:SetOperation(s.disop)
    effect::set_operation(d, e3, Some(operation(disop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EVIL_EYE) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_EVIL_EYE as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
/// `s.descond(e)`
fn descond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local eg=c:GetEquipGroup()
    let eg = card::get_equip_group(d, c);
    // return #eg>0 and eg:IsExists(Card.IsCode,1,nil,CARD_EVIL_EYE_SELENE)
    !eg.is_empty() && group::is_exists(d, &eg, &|d, x| card::is_code(d, x, &[CARD_EVIL_EYE_SELENE]), 1, NO_EX)
}
/// `s.destfilt(c)`
fn destfilt(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpecialSummoned()
    card::is_special_summoned(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.destfilt(chkc) and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && destfilt(d, chkc) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.destfilt,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&destfilt), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.destfilt,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&destfilt), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    let h = effect::get_handler(d, a.e).nil();
    // if Duel.IsPhase(PHASE_STANDBY) then
    if duel::is_phase(d, PHASE_STANDBY) {
        // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY,EFFECT_FLAG_OATH,2,Duel.GetTurnCount())
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY, EFFECT_FLAG_OATH, 2, duel::get_turn_count(d, None) as i64, 0);
    } else {
        // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY,EFFECT_FLAG_OATH,1)
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY, EFFECT_FLAG_OATH, 1, 0, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
/// `s.discond(e,tp,eg,ep,ev,re,r,rp)`
fn discond(d: &mut Duel, a: &EffArgs) -> bool {
    // local label=e:GetHandler():GetFlagEffectLabel(id)
    let h = effect::get_handler(d, a.e).nil();
    let label = card::get_flag_effect_label(d, h, ID).first().copied();
    // return label and label~=Duel.GetTurnCount()
    match label {
        Some(l) => l != duel::get_turn_count(d, None) as i64,
        None => false,
    }
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local sg=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,0,nil)
    let sg = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), 1, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.TRUE,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, &g, true);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, &g, REASON_EFFECT, None, None);
    }
}
