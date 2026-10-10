//! Weighbridge
use super::prelude::*;

pub const ID: u32 = 39103226;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make opponent send monsters to the GY
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
/// `s.tgcon(e,tp,eg,ep,ev,re,r,rp)`
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)-Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)>=2
    duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE) - duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0) >= 2
}
/// `s.tgfilter(c,p)`
fn tgfilter(d: &mut Duel, c: CardId, p: u8) -> bool {
    // return Duel.IsPlayerCanSendtoGrave(p,c)
    duel::is_player_can_send_to_grave(d, p, Some((c, None)))
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,0,LOCATION_MZONE,nil,1-tp)
    let g = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, 1 - tp)), tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)
    let ct = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,math.min(ct-1,#g),0,0)
    let n = (ct - 1).min(g.len() as i32) as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, 0, 0);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.tgfilter,tp,0,LOCATION_MZONE,nil,1-tp)
    let g = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, 1 - tp)), tp, 0, LOCATION_MZONE, NO_EX);
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)
    let ct = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE);
    // if ct<=1 or #g==0 then return end
    if ct <= 1 || g.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TOGRAVE as u64);
    // local tg=g:Select(1-tp,ct-1,ct-1,nil)
    let tg = group::select(d, &g, 1 - tp, (ct - 1) as u16, (ct - 1) as u16, false, NO_EX).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.HintSelection(tg,true)
        duel::hint_selection(d, tg.clone(), true);
        // Duel.SendtoGrave(tg,REASON_RULE,PLAYER_NONE,1-tp)
        duel::send_to_grave(d, tg, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - tp));
    }
}
