//! Full Force Virus
use super::prelude::*;

pub const ID: u32 = 4931121;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_TOHAND|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_TOHAND | TIMINGS_CHECK_MONSTER));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.costfilter(c)`
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsDefenseAbove(2000)
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_defense_above(d, c, 2000)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil)
    let g = duel::select_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsDefenseBelow(1500)
    card::is_faceup(d, c) && card::is_defense_below(d, c, 1500)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = group::get_count(&g) as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsDefenseBelow(1500)
    card::is_monster(d, c) && card::is_defense_below(d, c, 1500)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local conf=Duel.GetFieldGroup(tp,0,LOCATION_MZONE|LOCATION_HAND)
    let conf = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE | LOCATION_HAND);
    // if #conf>0 then
    if group::get_count(&conf) > 0 {
        // Duel.ConfirmCards(tp,conf)
        duel::confirm_cards(d, a.tp, conf.clone());
        // local dg=conf:Filter(s.filter,nil)
        let dg = group::filter(d, &conf, &filter, NO_EX);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
        // Duel.ShuffleHand(1-tp)
        duel::shuffle_hand(d, 1 - a.tp);
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_DRAW)
    effect::set_code(d, e1, EVENT_DRAW);
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,3)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 3);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.turncon)
    effect::set_condition(d, e2, Some(condition(turncon)));
    // e2:SetOperation(s.turnop)
    effect::set_operation(d, e2, Some(operation(turnop)));
    // e2:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,3)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 3);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // local descnum=tp==c:GetOwner() and 0 or 1
    let descnum = if a.tp == card::get_owner(d, c) { 0 } else { 1 };
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetDescription(aux.Stringid(id,descnum))
    effect::set_description(d, e3, aux::stringid(ID, descnum));
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_SET_AVAILABLE, 0);
    // e3:SetCode(1082946)
    effect::set_code(d, e3, 1082946);
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // e3:SetOwnerPlayer(tp)
    effect::set_owner_player(d, e3, a.tp);
    // e3:SetOperation(s.reset)
    effect::set_operation(d, e3, Some(operation(reset)));
    // e3:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,3)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 3);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e4 = effect::create_effect(d, h);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e4:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e4, aux::stringid(ID, 3));
    // e4:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,3)
    effect::set_reset(d, e4, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 3);
    // e4:SetTargetRange(0,1)
    effect::set_target_range(d, e4, 0, 1);
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, a.tp);
}
/// `s.reset(e,tp,eg,ep,ev,re,r,rp)`: `s.turnop(e:GetLabelObject(),tp,eg,ep,ev,e,r,rp)`
fn reset(d: &mut Duel, a: &EffArgs) {
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    turnop_body(d, lo, Some(a.e));
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // if ep==e:GetOwnerPlayer() then return end
    if a.ep == effect::get_owner_player(d, a.e) {
        return;
    }
    // local hg=eg:Filter(Card.IsLocation,nil,LOCATION_HAND)
    let hg = group::filter(d, &a.eg.clone().nil(), &|d, x| card::is_location(d, x, LOCATION_HAND), NO_EX);
    // if #hg==0 then return end
    if group::get_count(&hg) == 0 {
        return;
    }
    // Duel.ConfirmCards(1-ep,hg)
    duel::confirm_cards(d, 1 - a.ep, hg.clone());
    // local dg=hg:Filter(s.filter,nil)
    let dg = group::filter(d, &hg, &filter, NO_EX);
    // Duel.Destroy(dg,REASON_EFFECT)
    duel::destroy(d, dg, REASON_EFFECT, None, None);
    // Duel.ShuffleHand(ep)
    duel::shuffle_hand(d, a.ep);
}
/// `s.turncon(e,tp,eg,ep,ev,re,r,rp)`
fn turncon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.turnop(e,tp,eg,ep,ev,re,r,rp)`
fn turnop(d: &mut Duel, a: &EffArgs) {
    turnop_body(d, a.e, a.re);
}
/// The body of `s.turnop` for the effect `e`, with the `re` argument of the call.
fn turnop_body(d: &mut Duel, e: EffectId, re: Option<EffectId>) {
    // local ct=e:GetLabel()
    let ct = effect::get_label(d, e);
    // ct=ct+1
    let ct = ct + 1;
    // e:SetLabel(ct)
    effect::set_label(d, e, &[ct]);
    // e:GetHandler():SetTurnCounter(ct)
    let h = effect::get_handler(d, e).nil();
    card::set_turn_counter(d, h, ct as u16);
    // if ct==3 then
    if ct == 3 {
        // e:GetLabelObject():Reset()
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::reset(d, lo);
        // if re then re:Reset() end
        if let Some(re) = re {
            effect::reset(d, re);
        }
    }
}
