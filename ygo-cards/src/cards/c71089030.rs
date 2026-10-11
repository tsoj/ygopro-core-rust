//! Earthbound Prison
use super::prelude::*;

pub const ID: u32 = 71089030;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EARTHBOUND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Additional Normal Summon of 1 "Earthbound" monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e2, EFFECT_EXTRA_SUMMON_COUNT);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_HAND | LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_EARTHBOUND))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_EARTHBOUND as u16]))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Halve your opponent's LP
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e3, CATEGORY_DISABLE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.discon)
    effect::set_condition(d, e3, Some(condition(discon)));
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(cost_target(distg)));
    // e3:SetOperation(s.disop)
    effect::set_operation(d, e3, Some(operation(disop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() and chkc:IsType(TYPE_EFFECT) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp)
            && card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_faceup(d, chkc)
            && card::is_type(d, chkc, TYPE_EFFECT);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if Duel.IsExistingTarget(aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,0,LOCATION_MZONE,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if duel::is_existing_target(d, Some(&faceup_effect_monster), a.tp, 0, LOCATION_MZONE, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0))
    {
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // e:SetOperation(s.activate)
        effect::set_operation(d, a.e, Some(operation(activate)));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
        // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,0,LOCATION_MZONE,1,1,nil)
        duel::select_target(d, a.tp, Some(&faceup_effect_monster), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    } else {
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // e:SetOperation(nil)
        effect::set_operation(d, a.e, None);
    }
    true
}
// aux.FaceupFilter(Card.IsType,TYPE_EFFECT)
fn faceup_effect_monster(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetRange(LOCATION_FZONE)
        effect::set_range(d, e1, LOCATION_FZONE);
        // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
        effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
        // e1:SetTarget(function(_e,_c) return _c==tc and c:IsHasCardTarget(tc) end)
        effect::set_target(d, e1, Some(effect_card_fn(move |d, _e, x| x == tc && card::is_has_card_target(d, c, tc))));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
// function s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsPreviousControler(tp) and c:IsReason(REASON_EFFECT)
    a.rp == 1 - a.tp && card::is_previous_controler(d, c, a.tp) && card::is_reason(d, c, REASON_EFFECT)
}
// function s.disconfilter(c)
fn disconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_EARTHBOUND) and c:IsFaceup() and c:IsMonster()
    card::is_set_card(d, c, &[SET_EARTHBOUND as u16]) && card::is_faceup(d, c) && card::is_monster(d, c)
}
// function s.distg(e,tp,eg,ep,ev,re,r,rp,chk)
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.disconfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&disconfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    true
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.SetLP(1-tp,Duel.GetLP(1-tp)/2,REASON_EFFECT)
    let lp = duel::get_lp(d, 1 - a.tp);
    duel::set_lp(d, 1 - a.tp, lua_int(lp as f64 / 2.0) as i32);
    // local g=Duel.GetMatchingGroup(Card.IsNegatable,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&card::is_negatable), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // tc:NegateEffects(c,RESET_PHASE|PHASE_END,true)
        card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), true, None);
    }
}
