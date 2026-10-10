//! Evenly Matched
use super::prelude::*;

pub const ID: u32 = 15693423;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make the opponent banish cards they control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_BATTLE_END)
    effect::set_hint_timing(d, e1, TIMING_BATTLE_END, None);
    // e1:SetCondition(function() return Duel.GetCurrentPhase()==PHASE_BATTLE end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::get_current_phase(d) == PHASE_BATTLE)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be activated from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_TRAP_ACT_IN_HAND)
    effect::set_code(d, e2, EFFECT_TRAP_ACT_IN_HAND);
    // e2:SetCondition(s.handcon)
    effect::set_condition(d, e2, Some(lua_fn(handcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    // local ct=#g-Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0)
    let mut ct = g.len() as i32 - duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD, 0);
    // if e:GetHandler():IsLocation(LOCATION_HAND) then ct=ct-1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        ct -= 1;
    }
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(1-tp,30459350)
    //     and ct>0 and g:IsExists(Card.IsAbleToRemove,1,nil,1-tp,POS_FACEDOWN,REASON_RULE) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, 1 - a.tp, 30459350).is_none()
            && ct > 0
            && group::is_exists(d, &g, &|d, c| card::is_able_to_remove(d, c, Some(1 - a.tp), Some(POS_FACEDOWN as u8), Some(REASON_RULE)), 1, NO_EX);
    }
    // --Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,ct,0,0)
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(1-tp,30459350) then return end
    if duel::is_player_affected_by_effect(d, 1 - a.tp, 30459350).is_some() {
        return;
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    // local ct=#g-Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0)
    let ct = g.len() as i32 - duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD, 0);
    // if ct>0 then
    if ct > 0 {
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_REMOVE as u64);
        // local sg=g:FilterSelect(1-tp,Card.IsAbleToRemove,ct,ct,nil,1-tp,POS_FACEDOWN,REASON_RULE)
        let sg = group::filter_select(
            d,
            &g,
            1 - a.tp,
            &|d, c| card::is_able_to_remove(d, c, Some(1 - a.tp), Some(POS_FACEDOWN as u8), Some(REASON_RULE)),
            ct as u16,
            ct as u16,
            false,
            NO_EX,
        )
        .nil();
        // Duel.Remove(sg,POS_FACEDOWN,REASON_RULE,PLAYER_NONE,1-tp)
        duel::remove(d, sg, POS_FACEDOWN as u8, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - a.tp));
    }
}
/// `s.handcon(e)`
fn handcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_ONFIELD,0)==0
    //     and Duel.GetFieldGroupCount(1-e:GetHandlerPlayer(),LOCATION_ONFIELD,0)>1
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_field_group_count(d, p, LOCATION_ONFIELD, 0) == 0 && duel::get_field_group_count(d, 1 - p, LOCATION_ONFIELD, 0) > 1
}
