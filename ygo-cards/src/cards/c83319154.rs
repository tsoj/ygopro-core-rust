//! Annihilate Retroglight
//! 時空穿つ遡光
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 83319154;

// local STATUS_SUMMONED_THIS_TURN=STATUS_SUMMON_TURN|STATUS_FLIP_SUMMON_TURN|STATUS_SPSUMMON_TURN
const STATUS_SUMMONED_THIS_TURN: u32 = STATUS_SUMMON_TURN | STATUS_FLIP_SUMMON_TURN | STATUS_SPSUMMON_TURN;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // When your opponent activates the effect of a monster on the field that was not Summoned this turn: Negate the activation, then you can banish (face-down) all face-up monsters your opponent controls that were not Summoned this turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If this Set card in its owner's control is destroyed by an opponent's card effect: You can Set 1 Trap from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(s.setcon)
    effect::set_condition(d, e2, Some(condition(setcon)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.negcon(e,tp,eg,ep,ev,re,r,rp)
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local trig_location,trig_status=Duel.GetChainInfo(0,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_STATUS)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_STATUS]);
    let trig_location = info[0].int();
    let trig_status = info[1].int();
    // return rp==1-tp and re:IsMonsterEffect() and (trig_location&LOCATION_MZONE)>0 and (trig_status&STATUS_SUMMONED_THIS_TURN)==0
    //     and Duel.IsChainNegatable(ev)
    a.rp == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && (trig_location & LOCATION_MZONE as i64) > 0
        && (trig_status & STATUS_SUMMONED_THIS_TURN as i64) == 0
        && duel::is_chain_negatable(d, a.ev as u8)
}
// function s.notsummonturnfilter(c,tp)
fn notsummonturnfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and not c:IsStatus(STATUS_SUMMONED_THIS_TURN) and c:IsAbleToRemove(c,tp,POS_FACEDOWN)
    if !(card::is_faceup(d, c) && !card::is_status(d, c, STATUS_SUMMONED_THIS_TURN)) {
        return false;
    }
    // c:IsAbleToRemove(c,tp,POS_FACEDOWN): the C++ IsAbleToRemove reads (playerid, pos, reason) from
    // arguments 2, 3, 4. Argument 2 is the card, not a number, so playerid takes its default
    // (the reason player); argument 3 is tp (the position), argument 4 is POS_FACEDOWN (the reason).
    card::is_able_to_remove(d, c, None, Some(tp), Some(POS_FACEDOWN as u32))
}
// function s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, tp, 0);
    // local g=Duel.GetMatchingGroup(s.notsummonturnfilter,tp,0,LOCATION_MZONE,nil,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| notsummonturnfilter(d, c, tp)), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, tp, 0);
    true
}
// function s.negop(e,tp,eg,ep,ev,re,r,rp)
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.NegateActivation(ev) then
    if duel::negate_activation(d, a.ev as u8) {
        // local g=Duel.GetMatchingGroup(s.notsummonturnfilter,tp,0,LOCATION_MZONE,nil,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| notsummonturnfilter(d, c, tp)), tp, 0, LOCATION_MZONE, NO_EX);
        // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)
            duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
        }
    }
}
// function s.setcon(e,tp,eg,ep,ev,re,r,rp)
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousPosition(POS_FACEDOWN) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_ONFIELD)
    //     and rp==1-tp and c:IsReason(REASON_EFFECT)
    card::is_previous_position(d, c, POS_FACEDOWN as u32)
        && card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && a.rp == 1 - a.tp
        && card::is_reason(d, c, REASON_EFFECT)
}
// function s.setfilter: aux.AND(Card.IsTrap,Card.IsSSetable)
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
// function s.settg(e,tp,eg,ep,ev,re,r,rp,chk)
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.AND(Card.IsTrap,Card.IsSSetable),tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SET,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SET, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.setop(e,tp,eg,ep,ev,re,r,rp)
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.AND(Card.IsTrap,Card.IsSSetable),tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&setfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
    // if #g>0 then
    if let Some(g) = g {
        if !g.is_empty() {
            // Duel.SSet(tp,g)
            duel::s_set(d, tp, g, None, true);
        }
    }
}
