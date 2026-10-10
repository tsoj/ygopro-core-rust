//! Knightmare Goblin
use super::prelude::*;

pub const ID: u32 = 39064822;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Link Summon procedure: 2 monsters with different names
    // Link.AddProcedure(c,nil,2,2,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(2), Some(group_check(lcheck)), None, None);
    // --If this card is Link Summoned during your turn: You can discard 1 card; ...
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.sumcon)
    effect::set_condition(d, e1, Some(condition(sumcon)));
    // e1:SetCost(s.sumcost)
    effect::set_cost(d, e1, Some(cost_target(sumcost)));
    // e1:SetTarget(s.sumtg)
    effect::set_target(d, e1, Some(cost_target(sumtg)));
    // e1:SetOperation(s.sumop)
    effect::set_operation(d, e1, Some(operation(sumop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Neither player can target co-linked monsters you control with card effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(effect_card_fn(tgtg)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.lcheck(g,lc,sumtype,tp)
fn lcheck(d: &mut Duel, g: &CardSet, _lc: CardId, _sumtype: u64, _tp: u8) -> bool {
    // return g:CheckDifferentProperty(Card.GetCode,lc,sumtype,tp)
    group::check_different_property(d, g, &|d, c| card::get_code(d, c).into_iter().map(i64::from).collect())
}
// function s.sumcon(e,tp,eg,ep,ev,re,r,rp)
fn sumcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.IsTurnPlayer(tp) and e:GetHandler():IsLinkSummoned() and Duel.IsPlayerCanAdditionalSummon(tp)
    let h = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, tp) && card::is_link_summoned(d, h) && duel::is_player_can_additional_summon(d, tp)
}
// function s.sumcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn sumcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 and Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, a.tp, ID) == 0
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
// function s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sumtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanSummon(tp) end
    if a.chk == Some(0) {
        return duel::is_player_can_summon(d, a.tp, None);
    }
    let h = effect::get_handler(d, a.e).nil();
    // if e:GetHandler():GetMutualLinkedGroupCount()>0 then
    if card::get_mutual_linked_group_count(d, h) > 0 {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    }
    // else e:SetLabel(0) end
    else {
        effect::set_label(d, a.e, &[0]);
    }
    // local cat=0
    let mut cat: u64 = 0;
    // if e:GetLabel()==1 then cat=cat+CATEGORY_DRAW end
    if effect::get_label(d, a.e) == 1 {
        cat = cat + CATEGORY_DRAW;
    }
    // e:SetCategory(cat)
    effect::set_category(d, a.e, cat);
    true
}
// function s.sumop(e,tp,eg,ep,ev,re,r,rp)
fn sumop(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()==1 and Duel.IsPlayerCanDraw(tp,1)
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if effect::get_label(d, a.e) == 1 && duel::is_player_can_draw(d, a.tp, 1) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, a.tp, 1, REASON_EFFECT);
    }
    // if Duel.GetFlagEffect(tp,id)~=0 then return end
    if duel::get_flag_effect(d, a.tp, ID) != 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetTargetRange(LOCATION_HAND,0)
    effect::set_target_range(d, e1, LOCATION_HAND, 0);
    // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    // e1:SetLabelObject(c)
    effect::set_label_object(d, e1, Value::Card(c));
    // e1:SetCondition(s.sumcon2)
    effect::set_condition(d, e1, Some(condition(sumcon2)));
    // e1:SetValue(s.sumval)
    effect::set_value_fn(d, e1, lua_fn(sumval));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
}
// function s.sumcon2(e,tp,eg,ep,ev,re,r,rp)
fn sumcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabelObject():IsLocation(LOCATION_MZONE)
    let c = effect::get_label_object(d, a.e).as_card().nil();
    card::is_location(d, c, LOCATION_MZONE)
}
// function s.sumval(e,c)
fn sumval(d: &mut Duel, a: Args) -> Value {
    // local c=e:GetLabelObject()
    let c = effect::get_label_object(d, a.e(0)).as_card().nil();
    // local sumzone=c:GetLinkedZone()
    let sumzone = card::get_linked_zone(d, c, None);
    // local relzone=0x7f007f&~(1<<c:GetSequence())
    let relzone = 0x7f007f & !(1u32 << card::get_sequence(d, c));
    // return 0,sumzone,relzone
    Value::Multi(vec![Value::Int(0), Value::Int(sumzone as i64), Value::Int(relzone as i64)])
}
// function s.tgtg(e,c)
fn tgtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:GetMutualLinkedGroupCount()>0
    card::get_mutual_linked_group_count(d, c) > 0
}
