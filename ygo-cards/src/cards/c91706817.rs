//! Angel of Blue Tears
use super::prelude::*;

pub const ID: u32 = 91706817;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate 1 face-up monster on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DAMAGE + CATEGORY_DISABLE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 1 Normal Trap from hand or Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DAMAGE)
    effect::set_code(d, e2, EVENT_DAMAGE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetCondition(function(_,_,_,_,_,_,r)return r&REASON_EFFECT~=0 end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c,ac)`
fn tgfilter(d: &mut Duel, c: CardId, ac: CardId) -> bool {
    // return c:IsNegatableMonster() and Duel.GetMatchingGroupCount(nil,c:GetControler(),0,LOCATION_HAND,ac)>0
    let p = card::get_controler(d, c);
    card::is_negatable_monster(d, c) && duel::get_matching_group_count(d, None, p, 0, LOCATION_HAND, ac.into()) > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,c) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, c);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,c)
    let g = duel::select_target(d, a.tp, Some(&|d, x| tgfilter(d, x, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // local p=g:GetFirst():GetControler()
    let first = group::get_first(d, &g).nil();
    let p = card::get_controler(d, first);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-p,Duel.GetFieldGroupCount(1-p,LOCATION_HAND,0)*200)
    let amount = duel::get_field_group_count(d, 1 - p, LOCATION_HAND, 0) * 200;
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - p, amount);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local p=tc:GetControler()
        let p = card::get_controler(d, tc);
        // local dam=Duel.GetFieldGroupCount(1-p,LOCATION_HAND,0)*200
        let dam = duel::get_field_group_count(d, 1 - p, LOCATION_HAND, 0) * 200;
        // if Duel.Damage(1-p,dam,REASON_EFFECT)>0 and tc:IsFaceup() and not tc:IsDisabled() then
        if duel::damage(d, 1 - p, dam as i64, REASON_EFFECT, false, None) > 0 && card::is_faceup(d, tc) && !card::is_disabled(d, tc) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.NegateRelatedChain(tc,RESET_TURN_SET)
            duel::negate_related_chain(d, tc, RESET_TURN_SET);
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNormalTrap() and c:IsSSetable()
    card::is_normal_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<1 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local hand_chk=g:GetFirst():IsLocation(LOCATION_HAND)
    let first = group::get_first(d, &g).nil();
    let hand_chk = card::is_location(d, first, LOCATION_HAND);
    // if Duel.SSet(tp,g)>0 and hand_chk then
    if duel::s_set(d, a.tp, g, None, true) > 0 && hand_chk {
        // If it was Set from the hand, it can be activated this turn
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(aux.Stringid(id,3))
        effect::set_description(d, e1, aux::stringid(ID, 3));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
        effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
        // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
        effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // g:GetFirst():RegisterEffect(e1)
        card::register_effect(d, first, e1, false);
    }
}
/// `function(...,r) return r&REASON_EFFECT~=0 end` (e2's condition)
fn cond2(_d: &mut Duel, a: &EffArgs) -> bool {
    a.r & REASON_EFFECT != 0
}
