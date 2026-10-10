//! Aegaion the Sea Castrum
//! 魔海城アイガイオン
use super::prelude::*;

pub const ID: u32 = 10678778;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon procedure: 2 Level 8 monsters
    // Xyz.AddProcedure(c,nil,8,2)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // remove
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DESTROY);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsAbleToRemove()
    card::is_facedown(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,0,LOCATION_EXTRA,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmfilter), tp, 0, LOCATION_EXTRA, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_EXTRA as i32);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_EXTRA,nil)
    let g = duel::get_matching_group(d, Some(&rmfilter), tp, 0, LOCATION_EXTRA, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local tc=g:RandomSelect(tp,1):GetFirst()
    let rs = group::random_select(d, &g, tp, 1);
    let tc = group::get_first(d, &rs).nil();
    // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    // local atk=tc:GetAttack()
    let mut atk = card::get_attack(d, tc);
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetValue(atk)
    effect::set_value(d, e1, atk as i64);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local ctype=(c:GetType()&TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ)
    let ctype = card::get_type(d, c) & (TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ);
    // return c:IsFaceup() and ctype~=0 and c:IsAbleToExtra()
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsType,ctype),tp,0,LOCATION_MZONE,1,nil)
    card::is_faceup(d, c)
        && ctype != 0
        && card::is_able_to_extra(d, c)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_type(d, x, ctype))), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(1-tp) and s.filter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_REMOVED,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, tp)), tp, 0, LOCATION_REMOVED, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_REMOVED,1,1,nil,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, tp)), tp, 0, LOCATION_REMOVED, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, g.clone(), 1, 0, 0);
    // local ctype=(g:GetFirst():GetType()&TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ)
    let first = group::get_first(d, &g.clone().nil()).nil();
    let ctype = card::get_type(d, first) & (TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ);
    // local dg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsType,ctype),tp,0,LOCATION_MZONE,nil)
    let dg = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, x| card::is_type(d, x, ctype))), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoDeck(tc,nil,SEQ_DECKTOP,REASON_EFFECT)~=0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_deck(d, tc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None) != 0 {
        // local ctype=(tc:GetType()&TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ)
        let ctype = card::get_type(d, tc) & (TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsType,ctype),tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&aux::faceup_filter(|d, x| card::is_type(d, x, ctype))), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
