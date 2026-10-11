//! Multi-Universe
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 885016;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetCondition(function(e,tp) return not Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_FZONE,0,1,nil) end)
    effect::set_condition(d, e0, Some(lua_fn(|d, a: Args| {
        let tp = a.p(1);
        !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_FZONE, 0, 1, NO_EX)
    })));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Destroy this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Replace Field Spell destruction
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
    // e2:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e2, lua_fn(|d, a: Args| {
        // e:GetHandlerPlayer() is passed to s.repfilter, which ignores it
        let _hp = effect::get_handler_player(d, a.e(0));
        let c = a.c(1);
        repfilter(d, c)
    }));
    // e2:SetOperation(function(e) Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT|REASON_REPLACE) end)
    effect::set_operation(d, e2, Some(operation(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFieldSpell() and c:IsFaceup() and not c:IsForbidden()
    card::is_field_spell(d, c) && card::is_faceup(d, c) && !card::is_forbidden(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED|LOCATION_GRAVE) and s.plfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED | LOCATION_GRAVE) && plfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.plfilter,tp,LOCATION_REMOVED|LOCATION_GRAVE,LOCATION_REMOVED|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&plfilter), a.tp, LOCATION_REMOVED | LOCATION_GRAVE, LOCATION_REMOVED | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.plfilter,tp,LOCATION_REMOVED|LOCATION_GRAVE,LOCATION_REMOVED|LOCATION_GRAVE,1,1,nil):GetFirst()
    let g = duel::select_target(d, a.tp, Some(&plfilter), a.tp, LOCATION_REMOVED | LOCATION_GRAVE, LOCATION_REMOVED | LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // if tc:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, tc, LOCATION_GRAVE) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    }
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.Destroy(c,REASON_EFFECT)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::destroy(d, c, REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.MoveToField(tc,tp,tp,LOCATION_FZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_FZONE as u16, POS_FACEUP as u8, true, None);
    }
}
/// `s.repfilter(c)`
fn repfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_FZONE) and not c:IsReason(REASON_REPLACE)
    card::is_location(d, c, LOCATION_FZONE) && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, c, None, None, None) && group::is_exists(d, &eg, &|d, cc| repfilter(d, cc), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
