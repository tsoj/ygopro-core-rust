//! Primite Drillbeam
use super::prelude::*;

pub const ID: u32 = 29095457;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_PRIMITE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set this card from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_PRIMITE),tp,LOCATION_MZONE,0,1,nil) end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_PRIMITE) or (c:IsMonster() and c:IsType(TYPE_NORMAL))) and not (c:IsPublic() or c:IsCode(id))
    (card::is_set_card(d, c, &[SET_PRIMITE as u16]) || (card::is_monster(d, c) && card::is_type(d, c, TYPE_NORMAL)))
        && !(card::is_public(d, c) || card::is_code(d, c, &[ID]))
}
/// `s.nocostfilter(c)`
fn nocostfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsType(TYPE_NORMAL) or (c:IsSetCard(SET_PRIMITE) and c:IsLevelAbove(5))) and c:IsFaceup() and not c:IsType(TYPE_TOKEN)
    (card::is_type(d, c, TYPE_NORMAL) || (card::is_set_card(d, c, &[SET_PRIMITE as u16]) && card::is_level_above(d, c, 5)))
        && card::is_faceup(d, c)
        && !card::is_type(d, c, TYPE_TOKEN)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_HAND,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let b1 = duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_HAND, 0, 1, h.into());
    // local b2=Duel.IsExistingMatchingCard(s.nocostfilter,tp,LOCATION_MZONE,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&nocostfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=nil
    // if b1 and b2 then
    let op: Option<usize> = if b1 && b2 {
        // op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
        duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))])
    } else {
        // op=(b1 and 1) or (b2 and 2)
        if b1 {
            Some(1)
        } else if b2 {
            Some(2)
        } else {
            None
        }
    };
    if op == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
        // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_HAND,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
    } else if op == Some(2) {
        // return (Lua: nil)
        return true;
    }
    true
}
/// `s.negatablefilter`: `aux.AND(Card.IsNegatable,Card.IsAbleToRemove)`
fn negatable_removable(d: &mut Duel, c: CardId) -> bool {
    card::is_negatable(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsOnField() and chkc:IsNegatable() and chkc:IsAbleToRemove() and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_negatable(d, chkc) && card::is_able_to_remove(d, chkc, None, None, None) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.AND(Card.IsNegatable,Card.IsAbleToRemove),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&negatable_removable), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,aux.AND(Card.IsNegatable,Card.IsAbleToRemove),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c)
    let g = duel::select_target(d, a.tp, Some(&negatable_removable), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g.clone()), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsNegatable() and tc:IsCanBeDisabledByEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_negatable(d, tc) && card::is_can_be_disabled_by_effect(d, tc, a.e, true) {
        // Negate its effects
        // tc:NegateEffects(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, None, false, None);
        // Duel.AdjustInstantly(tc)
        duel::adjust_instantly(d, Some(tc));
        // if tc:IsDisabled() then
        if card::is_disabled(d, tc) {
            // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
/// `function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_PRIMITE),tp,LOCATION_MZONE,0,1,nil) end`
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_PRIMITE as u16]))), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsSSetable() end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsSSetable() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None) {
        // Duel.SSet(tp,c)
        duel::s_set(d, a.tp, c, None, true);
    }
}
