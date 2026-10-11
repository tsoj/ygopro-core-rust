//! Switch Point
use super::prelude::*;

pub const ID: u32 = 47961342;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCondition(s.tgcon)
    effect::set_condition(d, e1, Some(condition(tgcon)));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tgcon(e,tp,eg,ep,ev,re,r,rp)
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>=3
    duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) >= 3
}
// function s.tgfilter(c,p)
fn tgfilter(d: &mut Duel, c: CardId, p: u8) -> bool {
    // return Duel.IsPlayerCanSendtoGrave(p,c)
    duel::is_player_can_send_to_grave(d, p, Some((c, None)))
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,0,LOCATION_MZONE,1,nil,1-tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| tgfilter(d, c, 1 - tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - tp, LOCATION_MZONE as i32);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
    let mut g = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
    // if #g<3 then return end
    if g.len() < 3 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELECT as u64);
    // local tc=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if not tc then return end
    let Some(tc) = tc else { return };
    // Duel.HintSelection(tc,true)
    duel::hint_selection(d, CardSet::from_card(tc), true);
    // g:RemoveCard(tc)
    group::remove_card(&mut g, &CardSet::from_card(tc));
    // local b1=Duel.IsPlayerCanSendtoGrave(1-tp,tc)
    let b1 = duel::is_player_can_send_to_grave(d, 1 - tp, Some((tc, None)));
    // local b2=g:IsExists(s.tgfilter,1,nil,1-tp)
    let b2 = group::is_exists(d, &g, &|d, c| tgfilter(d, c, 1 - tp), 1, NO_EX);
    // local op=Duel.SelectEffect(1-tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, 1 - tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // if op then
    if let Some(op) = op {
        // Duel.SendtoGrave(op==1 and tc or g,REASON_RULE,PLAYER_NONE,1-tp)
        let targets = if op == 1 { CardSet::from_card(tc) } else { g };
        duel::send_to_grave(d, targets, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - tp));
    }
}
