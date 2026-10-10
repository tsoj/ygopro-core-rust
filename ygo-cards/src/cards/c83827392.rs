//! Expurrely Noir
use super::prelude::*;

pub const ID: u32 = 83827392;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PURRELY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 7 monsters, or 1 Rank 2 monster with 5+ Xyz materials
    // Xyz.AddProcedure(c,nil,7,2,s.ovfilter,aux.Stringid(id,0))
    xyz::add_procedure(d, c, None, Some(7), 2, Some(xyz::alter_filter(ovfilter)), Some(aux::stringid(ID, 0)), None, None, false, None);
    // Unaffected by opponent's activated effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return e:GetHandler():GetOverlayCount()>=5 end)
    effect::set_condition(d, e1, Some(condition(immune_cond)));
    // e1:SetValue(function(e,te) return te:IsActivated() and e:GetOwnerPlayer()~=te:GetOwnerPlayer() end)
    effect::set_value_fn(d, e1, lua_fn(immune_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 1 card to the bottom of the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.tdcon)
    effect::set_condition(d, e2, Some(condition(tdcon)));
    // e2:SetCost(Cost.DetachFromSelf(2,2,nil))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(2), Some(cost::Count::N(2)), None)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, _tp: u8, _lc: CardId) -> bool {
    // return c:IsFaceup() and c:IsRank(2) and c:GetOverlayCount()>=5
    card::is_faceup(d, c) && card::is_rank(d, c, &[2]) && card::get_overlay_count(d, c) >= 5
}
/// `function(e) return e:GetHandler():GetOverlayCount()>=5 end` (e1's condition)
fn immune_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, h) >= 5
}
/// `function(e,te) return te:IsActivated() and e:GetOwnerPlayer()~=te:GetOwnerPlayer() end` (e1's value)
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    effect::is_activated(d, te) && effect::get_owner_player(d, a.e(0)) != effect::get_owner_player(d, te)
}
/// `s.tdconfilter(c)`
fn tdconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PURRELY) and c:IsLevel(1)
    card::is_set_card(d, c, &[SET_PURRELY as u16]) && card::is_level(d, c, &[1])
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:IsHasType(EFFECT_TYPE_QUICK_O)==e:GetHandler():GetOverlayGroup():IsExists(s.tdconfilter,1,nil)
    let is_quick = effect::is_has_type(d, a.e, EFFECT_TYPE_QUICK_O);
    let h = effect::get_handler(d, a.e).nil();
    let overlay = card::get_overlay_group(d, h);
    is_quick == group::is_exists(d, &overlay, &tdconfilter, 1, NO_EX)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local loc=LOCATION_ONFIELD|LOCATION_GRAVE
    let loc = LOCATION_ONFIELD | LOCATION_GRAVE;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(loc) and chkc:IsAbleToDeck() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, loc) && card::is_able_to_deck(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToDeck,tp,0,loc,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&card::is_able_to_deck), a.tp, 0, loc, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToDeck,tp,0,loc,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&card::is_able_to_deck), a.tp, 0, loc, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), 1, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoDeck(tc,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, tc, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
