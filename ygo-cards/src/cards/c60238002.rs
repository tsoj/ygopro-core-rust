//! Interdimensional Matter Forwarder
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 60238002;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // --Banish a monster and return it to the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.rmvtg(s.rmvfilter1))
    effect::set_target(d, e1, Some(rmvtg(rmvfilter1)));
    // e1:SetOperation(s.rmvop)
    effect::set_operation(d, e1, Some(operation(rmvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Banish a monster that has its effects negated and return it to the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp,eg,ep) return ep==1-tp end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.rmvtg(s.rmvfilter2))
    effect::set_target(d, e2, Some(rmvtg(rmvfilter2)));
    // e2:SetOperation(s.rmvop)
    effect::set_operation(d, e2, Some(operation(rmvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Banish this card until the End Phase of the next turn before resolving an opponent's card effect that targets it
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e3, EVENT_CHAIN_SOLVING);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.selfrmvcon)
    effect::set_condition(d, e3, Some(condition(selfrmvcon)));
    // e3:SetOperation(s.selfrmvop)
    effect::set_operation(d, e3, Some(operation(selfrmvop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rmvfilter1(c)`
fn rmvfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and Duel.GetMZoneCount(c:GetControler(),c)>0
    if !card::is_able_to_remove(d, c, None, None, None) {
        return false;
    }
    let p = card::get_controler(d, c);
    duel::get_m_zone_count(d, p, c.into(), None, None, None).0 > 0
}
/// `s.rmvfilter2(c)`
fn rmvfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsDisabled() and c:IsType(TYPE_EFFECT) and c:IsAbleToRemove() and Duel.GetMZoneCount(c:GetControler(),c)>0
    if !(card::is_disabled(d, c) && card::is_type(d, c, TYPE_EFFECT) && card::is_able_to_remove(d, c, None, None, None)) {
        return false;
    }
    let p = card::get_controler(d, c);
    duel::get_m_zone_count(d, p, c.into(), None, None, None).0 > 0
}
/// `s.rmvtg(filter)`: the target function of the banish effects
fn rmvtg(filter: fn(&mut Duel, CardId) -> bool) -> LuaFn {
    cost_target(move |d, a| {
        let tp = a.tp;
        // if chkc then return chkc:IsLocation(LOCATION_MZONE) and filter(chkc) end
        if let Some(chkc) = a.chkc() {
            return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
        }
        // if chk==0 then return Duel.IsExistingTarget(filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
        if a.chk == Some(0) {
            return duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectTarget(tp,filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
        true
    })
}
/// `s.rmvop(e,tp,eg,ep,ev,re,r,rp)`
fn rmvop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Remove(tc,nil,REASON_EFFECT|REASON_TEMPORARY)>0 and tc:IsLocation(LOCATION_REMOVED)
    //     and not tc:IsReason(REASON_REDIRECT) then
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::remove(d, tc, 0, REASON_EFFECT | REASON_TEMPORARY, None, None) > 0
        && card::is_location(d, tc, LOCATION_REMOVED)
        && !card::is_reason(d, tc, REASON_REDIRECT)
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.ReturnToField(tc)
        duel::return_to_field(d, tc, None, None);
    }
}
/// `function(e,tp,eg,ep) return ep==1-tp end`
fn e2_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == 1 - a.tp
}
/// `s.selfrmvcon(e,tp,eg,ep,ev,re,r,rp)`
fn selfrmvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (ep==1-tp and c:IsAbleToRemove() and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET)) then return false end
    if !(a.ep == 1 - a.tp
        && card::is_able_to_remove(d, c, None, None, None)
        && effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0))
    {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    // return g and g:IsContains(c)
    match chain::get_target_cards(d, a.ev as u8).as_group() {
        Some(g) => g.contains(c),
        None => false,
    }
}
/// `s.selfrmvop(e,tp,eg,ep,ev,re,r,rp)`
fn selfrmvop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_CARD,1-tp,id)
    duel::hint(d, HINT_CARD as u8, 1 - tp, ID as u64);
    // local ct=Duel.GetTurnCount()
    let ct = duel::get_turn_count(d, None);
    // aux.RemoveUntil(e:GetHandler(),nil,REASON_EFFECT,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp,
    //     function() return Duel.GetTurnCount()==ct+1 end,nil,2)
    let h = effect::get_handler(d, a.e).nil();
    let oper: aux::DelayedFn = aux::default_field_return();
    let cond: aux::DelayedFn = std::sync::Arc::new(move |d: &mut Duel, _g: &CardSet, _a: &EffArgs| duel::get_turn_count(d, None) == ct + 1);
    aux::remove_until(
        d,
        h,
        None,
        REASON_EFFECT,
        PHASE_END,
        ID,
        a.e,
        tp,
        Some(oper),
        Some(cond),
        None,
        Some(2),
        None,
        None,
    );
}
