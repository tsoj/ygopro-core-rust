//! Mark of the Rose
use super::prelude::*;

pub const ID: u32 = 45247637;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate this card by banishing 1 Plant monster from your GY, then target 1 face-up monster your opponent controls; equip this card to it
    // aux.AddEquipProcedure(c,1,aux.CheckStealEquip,s.eqlimit,s.cost,s.target)
    aux::add_equip_procedure(
        d,
        c,
        Some(1),
        Some(aux::equip_filter_fn(|d, c, e, tp| aux::check_steal_equip(d, c, e, tp))),
        Some(effect_card_fn(eqlimit)),
        Some(cost_target(cost_fn)),
        Some(std::sync::Arc::new(|d: &mut Duel, a: &EffArgs, tc: Option<CardId>| target(d, a, tc))),
        None,
        None,
        None,
    );
    // Take control of the equipped monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_SET_CONTROL)
    effect::set_code(d, e1, EFFECT_SET_CONTROL);
    // e1:SetValue(function(e,c) return e:GetHandler():GetFlagEffectLabel(id) end)
    effect::set_value_fn(d, e1, effect_value(control_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate this card's effect until your next Standby Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e2, Some(operation(e2_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn control_value(d: &mut Duel, e: EffectId) -> i64 {
    // return e:GetHandler():GetFlagEffectLabel(id)
    let h = effect::get_handler(d, e).nil();
    card::get_flag_effect_label(d, h, ID).first().copied().unwrap_or(0)
}
fn e2_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    // c:SetFlagEffectLabel(id,1-tp)
    card::set_flag_effect_label(d, c, ID, (1 - tp) as u32);
    // c:RegisterFlagEffect(id+1,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY,EFFECT_FLAG_CLIENT_HINT,2,0,aux.Stringid(id,1))
    card::register_flag_effect(d, c, ID + 1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY, EFFECT_FLAG_CLIENT_HINT, 2, 0, aux::stringid(ID, 1));
    // Choose when to reset the negation during your next Standby Phase
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_STANDBY);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function() return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e1, Some(condition(move |d, _a| duel::is_turn_player(d, tp))));
    // e1:SetOperation(function() Duel.Hint(HINT_CARD,0,id) c:SetFlagEffectLabel(id,tp) e1:Reset() end)
    effect::set_operation(d, e1, Some(operation(move |d, _a| {
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        card::set_flag_effect_label(d, c, ID, tp as u32);
        effect::reset(d, e1);
    })));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY,2)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY, 2);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsControler(1-e:GetHandlerPlayer()) or e:GetHandler():GetEquipTarget()==c
    let hp = effect::get_handler_player(d, e);
    let h = effect::get_handler(d, e).nil();
    card::is_controler(d, c, 1 - hp) || card::get_equip_target(d, h) == Some(c)
}
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PLANT) and c:IsAbleToRemove()
    card::is_race(d, c, RACE_PLANT) && card::is_able_to_remove(d, c, None, None, None)
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&costfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn target(d: &mut Duel, a: &EffArgs, tc: Option<CardId>) {
    // e:SetCategory(CATEGORY_CONTROL+CATEGORY_EQUIP)
    effect::set_category(d, a.e, CATEGORY_CONTROL + CATEGORY_EQUIP);
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,tp)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, a.tp as i64, 0);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, tc.map(CardSet::from_card), 1, a.tp, 0);
}
