//! Grinning Grave Virus
use super::prelude::*;

pub const ID: u32 = 85555787;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
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
// s.costfilter(c,matk)
fn costfilter(d: &mut Duel, c: CardId, matk: i32) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsAttackBelow(matk)
    card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_attack_below(d, c, matk)
}
// s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // local dc=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND|LOCATION_DECK)
    let dc = duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND | LOCATION_DECK);
    // local matk=math.min(3000,dc*500)
    let matk = std::cmp::min(3000, dc * 500);
    // if chk==0 then return matk>0 and Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,nil,nil,matk) end
    if a.chk == Some(0) {
        return matk > 0 && duel::check_release_group_cost(d, a.tp, Some(&|d, c| costfilter(d, c, matk)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,nil,nil,matk)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| costfilter(d, c, matk)), 1, 1, false, None, NO_EX);
    // e:SetLabel(g:GetFirst():GetAttack())
    let tc = group::get_first(d, &g).nil();
    let atk = card::get_attack(d, tc);
    effect::set_label(d, a.e, &[atk as i64]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local ct=math.floor(e:GetLabel()/500)
    let ct = effect::get_label(d, a.e).div_euclid(500);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,ct,1-tp,LOCATION_DECK|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, ct as u8, 1 - a.tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
// s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=e:GetLabel()
    let atk = effect::get_label(d, a.e);
    // local ct=math.floor(atk/500)
    let ct = atk.div_euclid(500);
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(1-tp,nil,1-tp,LOCATION_DECK|LOCATION_HAND,0,ct,ct,nil)
    let g = duel::select_matching_card(d, 1 - a.tp, None, 1 - a.tp, LOCATION_DECK | LOCATION_HAND, 0, ct as u16, ct as u16, false, NO_EX).nil();
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>=0 then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) >= 0 {
        // local og=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_GRAVE)
        let operated = duel::get_operated_group(d);
        let og = group::filter(d, &operated, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
        // for tc in og:Iter() do
        for tc in group::iter(d, &og) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_TRIGGER)
            effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
            // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
            effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
            // e1:SetRange(LOCATION_GRAVE)
            effect::set_range(d, e1, LOCATION_GRAVE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_EXC_GRAVE|RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE | RESET_PHASE | PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
            effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
    // if atk>=2000 then
    if atk >= 2000 {
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
        // c:RegisterFlagEffect(1082946,RESET_PHASE|PHASE_END|RESET_OPPO_TURN,0,3)
        card::register_flag_effect(d, c, 1082946, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 0, 3, 0, 0);
        // s[c]=e2
        // (not read anywhere in this script, so nothing to keep)
        // local e4=Effect.CreateEffect(e:GetHandler())
        let e4 = effect::create_effect(d, c);
        // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e4:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e4, aux::stringid(ID, 2));
        // e4:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,3)
        effect::set_reset(d, e4, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 3);
        // e4:SetTargetRange(0,1)
        effect::set_target_range(d, e4, 0, 1);
        // Duel.RegisterEffect(e4,tp)
        duel::register_effect(d, e4, a.tp);
    }
}
// Registers the "cannot trigger / cannot activate" effects on a card that was destroyed into the GY.
fn seal_gy_card(d: &mut Duel, c: CardId, tc: CardId, reset: u32) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_TRIGGER)
    effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetReset(...)
    effect::set_reset(d, e1, reset, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
// s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // if ep==e:GetOwnerPlayer() then return end
    if a.ep == effect::get_owner_player(d, a.e) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local hg=eg:Filter(Card.IsLocation,nil,LOCATION_HAND)
    let eg = a.eg.clone().nil();
    let hg = group::filter(d, &eg, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
    // if #hg==0 then return end
    if hg.is_empty() {
        return;
    }
    // Duel.ConfirmCards(1-ep,hg)
    duel::confirm_cards(d, 1 - a.ep, hg.clone());
    // local dg=hg:Filter(Card.IsMonster,nil)
    let dg = group::filter(d, &hg, &|d, c| card::is_monster(d, c), NO_EX);
    // if Duel.Destroy(dg,REASON_EFFECT)>0 then
    if duel::destroy(d, dg, REASON_EFFECT, None, None) > 0 {
        // local og=Duel.GetOperatedGroup():Filter(Card.IsLocation,nil,LOCATION_GRAVE)
        let operated = duel::get_operated_group(d);
        let og = group::filter(d, &operated, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
        // for tc in og:Iter() do
        for tc in group::iter(d, &og) {
            seal_gy_card(d, c, tc, RESET_EVENT | RESETS_STANDARD_EXC_GRAVE | RESET_PHASE | PHASE_END);
        }
    }
    // Duel.ShuffleHand(ep)
    duel::shuffle_hand(d, a.ep);
}
// s.turncon(e,tp,eg,ep,ev,re,r,rp)
fn turncon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
// s.turnop(e,tp,eg,ep,ev,re,r,rp)
fn turnop(d: &mut Duel, a: &EffArgs) {
    // local ct=e:GetLabel()
    // ct=ct+1
    let ct = effect::get_label(d, a.e) + 1;
    // e:SetLabel(ct)
    effect::set_label(d, a.e, &[ct]);
    // e:GetHandler():SetTurnCounter(ct)
    card::set_turn_counter(d, effect::get_handler(d, a.e).nil(), ct as u16);
    // if ct==3 then
    if ct == 3 {
        // e:GetLabelObject():Reset()
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        effect::reset(d, lo);
        // e:GetOwner():ResetFlagEffect(1082946)
        let owner = effect::get_owner(d, a.e).nil();
        card::reset_flag_effect(d, owner, 1082946);
    }
}
