//! 金神の戦鬼 アカスナ
use super::prelude::*;

pub const ID: u32 = 38331244;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ASHTRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Special Summon this card (from your hand) by revealing 1 face-down card you control and returning it to the hand/Extra Deck. You can only Special Summon "Akasuna the Golden Deity Asutra" once per turn this way
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0},EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During the Main Phase (Quick Effect): You can send 1 Trap from your hand or face-down field to the GY; change all face-up monsters your opponent controls to face-down Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(main_phase_cond)));
    // e2:SetCost(s.poscost)
    effect::set_cost(d, e2, Some(cost_target(poscost)));
    // e2:SetTarget(s.postg)
    effect::set_target(d, e2, Some(cost_target(postg)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // During your End Phase: You can Set 1 "Asutra" Trap from your Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e3, Some(condition(turn_player_cond)));
    // e3:SetTarget(s.settg)
    effect::set_target(d, e3, Some(cost_target(settg)));
    // e3:SetOperation(s.setop)
    effect::set_operation(d, e3, Some(operation(setop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsFacedown() and (c:IsAbleToHandAsCost() or c:IsAbleToExtraAsCost())
    card::is_facedown(d, c) && (card::is_able_to_hand_as_cost(d, c) || card::is_able_to_extra_as_cost(d, c))
}
/// `s.spcon(e,c)`: the Special Summon procedure condition; `c` is nil when the core only checks the effect.
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetMatchingGroup(s.spconfilter,tp,LOCATION_ONFIELD,0,nil)
    let rg = duel::get_matching_group(d, Some(&spconfilter), tp, LOCATION_ONFIELD, 0, NO_EX);
    // return #rg>0 and aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),0)
    !rg.is_empty()
        && matches!(
            aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(1), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false),
            Err(true)
        )
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local rg=Duel.GetMatchingGroup(s.spconfilter,tp,LOCATION_ONFIELD,0,nil)
    let rg = duel::get_matching_group(d, Some(&spconfilter), tp, LOCATION_ONFIELD, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_CONFIRM,nil,nil,true)
    let g = aux::select_unselect_group(
        d,
        &rg,
        Some(a.e),
        tp,
        Some(1),
        Some(1),
        Some(&aux::chkf_mmz(1)),
        1,
        tp,
        Some(HINTMSG_CONFIRM as u64),
        None,
        None,
        true,
    );
    // if g and #g>0 then
    match g {
        Ok(g) if !g.is_empty() => {
            // e:SetLabelObject(g)
            effect::set_label_object(d, a.e, Value::Group(g));
            // return true
            true
        }
        // return false
        _ => false,
    }
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned();
    // if g and #g>0 then
    if let Some(g) = g {
        if !g.is_empty() {
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, &g);
            // Duel.SendtoHand(g,nil,REASON_COST)
            duel::send_to_hand(d, &g, None, REASON_COST, None);
        }
    }
}
fn poscostfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsTrap() and (c:IsFacedown() or c:IsLocation(LOCATION_HAND)) and c:IsAbleToGraveAsCost()
    card::is_trap(d, c) && (card::is_facedown(d, c) || card::is_location(d, c, LOCATION_HAND)) && card::is_able_to_grave_as_cost(d, c)
}
fn poscost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.poscostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&poscostfilter), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.poscostfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&poscostfilter), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsCanTurnSet,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_can_turn_set(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsCanTurnSet,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_can_turn_set(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,tp,POS_FACEDOWN_DEFENSE)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsCanTurnSet,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_can_turn_set(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.ChangePosition(g,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, &g, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsSetCard(SET_ASHTRA) and c:IsTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_ASHTRA as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SET,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SET, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g)
        duel::s_set(d, tp, &g, None, true);
    }
}
fn main_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
fn turn_player_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
