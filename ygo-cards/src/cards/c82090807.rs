//! Fallen of Argyros
use super::prelude::*;

pub const ID: u32 = 82090807;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon procedure (your field)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SPSUM_PARAM)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SPSUM_PARAM, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetTargetRange(POS_FACEUP,0)
    effect::set_target_range(d, e1, POS_FACEUP, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon1)
    effect::set_condition(d, e1, Some(lua_fn(spcon1)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon procedure (your opponent's field)
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetTargetRange(POS_FACEUP,1)
    effect::set_target_range(d, e2, POS_FACEUP, 1);
    // e2:SetCondition(s.spcon2)
    effect::set_condition(d, e2, Some(lua_fn(spcon2)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Neither player can activate the effects of Link Monsters that point to this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(1,1)
    effect::set_target_range(d, e3, 1, 1);
    // e3:SetValue(s.aclimit)
    effect::set_value_fn(d, e3, lua_fn(aclimit));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Detach materials from an Xyz monster
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e4, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e4:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e4, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e4:SetTarget(s.detchtg)
    effect::set_target(d, e4, Some(cost_target(detchtg)));
    // e4:SetOperation(s.detchop)
    effect::set_operation(d, e4, Some(operation(detchop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:IsLevel(2) or c:IsRank(2) or c:IsLink(2))
    card::is_faceup(d, c) && (card::is_level(d, c, &[2]) || card::is_rank(d, c, &[2]) || card::is_link(d, c, &[2]))
}
/// `s.spcon1(e,c)`
fn spcon1(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // return Duel.GetLocationCount(e:GetHandlerPlayer(),LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spconfilter,0,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    let tp = effect::get_handler_player(d, a.e(0));
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spconfilter), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.spcon2(e,c)`
fn spcon2(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
    //     and Duel.IsExistingMatchingCard(s.spconfilter,0,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spconfilter), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:GetHandler():GetLinkedGroup():IsContains(e:GetHandler())
    let re = a.e(1);
    let h = effect::get_handler(d, re).nil();
    let lg = card::get_linked_group(d, h);
    let eh = effect::get_handler(d, a.e(0)).nil();
    group::is_contains(&lg, eh)
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::check_remove_overlay_card(d, c, tp, 1, REASON_EFFECT)
}
/// `s.detchtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn detchtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.cfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && cfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| cfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.cfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.detchop(e,tp,eg,ep,ev,re,r,rp)`
fn detchop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // tc:RemoveOverlayCard(tp,1,2,REASON_EFFECT)
        card::remove_overlay_card(d, tc, a.tp, 1, 2, REASON_EFFECT);
    }
}
