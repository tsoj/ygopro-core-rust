//! Banishing Trap Hole
use super::prelude::*;

pub const ID: u32 = 89569453;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.remfilter(c,tp)`
fn remfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsFaceup() and c:IsAttackBelow(1500) and c:IsSummonPlayer(1-tp)
    //     and c:IsAbleToRemove(tp,POS_FACEDOWN)
    card::is_location(d, c, LOCATION_MZONE)
        && card::is_faceup(d, c)
        && card::is_attack_below(d, c, 1500)
        && card::is_summon_player(d, c, 1 - tp)
        && card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=eg:Filter(s.remfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| remfilter(d, c, tp), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, &g);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=eg:Filter(s.remfilter,nil,tp):Match(Card.IsRelateToEffect,nil,e)
    let eg = a.eg.clone().nil();
    let mut g = group::filter(d, &eg, &|d, c| remfilter(d, c, tp), NO_EX);
    group::match_(d, &mut g, &|d, c| card::is_relate_to_effect(d, c, a.e), NO_EX);
    // if #g>0 then Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT) end
    if !g.is_empty() {
        duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
    }
}
