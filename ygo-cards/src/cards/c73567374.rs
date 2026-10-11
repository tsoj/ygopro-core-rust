//! Unleash Your Power!
use super::prelude::*;

pub const ID: u32 = 73567374;

pub const INFO: ScriptInfo = ScriptInfo { listed_card_types: &[TYPE_GEMINI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_POSITION + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_GEMINI) and not c:IsGeminiStatus()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_GEMINI) && !card::is_gemini_status(d, c)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetPossibleOperationInfo(0,CATEGORY_POSITION,g,#g,tp,POS_FACEDOWN_DEFENSE)
    let n = g.len() as u8;
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
fn filter2(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return s.filter(c) and not c:IsImmuneToEffect(e)
    filter(d, c) && !card::is_immune_to_effect(d, c, e)
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e):Match(s.filter2,nil,e)
    let mut g = duel::get_target_cards(d, a.e).nil();
    group::match_(d, &mut g, &|d, c| filter2(d, c, a.e), NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // g:ForEach(Card.EnableGeminiStatus)
    for tc in group::iter(d, &g) {
        card::enable_gemini_status(d, tc);
    }
    // aux.DelayedOperation(g,PHASE_END,id,e,tp,function(ag) Duel.ChangePosition(ag,POS_FACEDOWN_DEFENSE) end)
    aux::delayed_operation(d, g, PHASE_END, ID, a.e, a.tp, Some(std::sync::Arc::new(change_pos)), None, None, None, None, None);
}
fn change_pos(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    // Duel.ChangePosition(ag,POS_FACEDOWN_DEFENSE)
    duel::change_position(d, ag.clone(), POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    true
}
