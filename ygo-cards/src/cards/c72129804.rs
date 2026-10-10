//! Predaplast
use super::prelude::*;

pub const ID: u32 = 72129804;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PREDAP, SET_PREDAPLANT],
    counter_place_list: &[COUNTER_PREDATOR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy replace
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetValue(s.repval)
    effect::set_value_fn(d, e2, effect_card_fn(repval));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PREDAP) and not c:IsPublic()
    card::is_set_card(d, c, &[SET_PREDAP as u16]) && !card::is_public(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local hg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_HAND,0,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let hg = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c)), a.tp, LOCATION_HAND, 0, h.into());
    // local ct=Duel.GetTargetCount(Card.IsCanAddCounter,tp,0,LOCATION_MZONE,nil,COUNTER_PREDATOR,1)
    let ct = duel::get_target_count(d, Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_PREDATOR as u16, 1, false, 0)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsCanAddCounter(COUNTER_PREDATOR,1) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp)
            && card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_can_add_counter(d, chkc, COUNTER_PREDATOR as u16, 1, false, 0);
    }
    // if chk==0 then return #hg>0 and ct>0 end
    if a.chk == Some(0) {
        return !hg.is_empty() && ct > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local g=hg:Select(tp,1,ct,nil)
    let g = group::select(d, &hg, a.tp, 1, ct as u16, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, g.clone());
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsCanAddCounter,tp,0,LOCATION_MZONE,#g,#g,nil,COUNTER_PREDATOR,1)
    let n = g.len() as u16;
    duel::select_target(d, a.tp, Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_PREDATOR as u16, 1, false, 0)), a.tp, 0, LOCATION_MZONE, n, n, false, NO_EX);
    false
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // local tc=tg:GetFirst()
    let _ = group::get_first(d, &tg);
    // for tc in aux.Next(tg) do
    for tc in group::iter(d, &tg) {
        // if tc:AddCounter(COUNTER_PREDATOR,1) and tc:GetLevel()>1 then
        if card::add_counter(d, tc, COUNTER_PREDATOR as u16, 1, false) && card::get_level(d, tc) > 1 {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LEVEL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetCondition(s.lvcon)
            effect::set_condition(d, e1, Some(condition(lvcon)));
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.lvcon(e)`
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetCounter(COUNTER_PREDATOR)>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, COUNTER_PREDATOR as u16) > 0
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_PREDAPLANT) and c:IsLocation(LOCATION_MZONE)
    //   and c:IsControler(tp) and c:IsReason(REASON_BATTLE)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_PREDAPLANT as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_BATTLE)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, h, None, None, None)
            && group::is_exists(d, &eg, &|d, c| repfilter(d, c, a.tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    let h = effect::get_handler(d, a.e).nil();
    duel::select_effect_yes_no(d, a.tp, h, Some(96))
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return s.repfilter(c,e:GetHandlerPlayer())
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
