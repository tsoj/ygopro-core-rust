//! Ground Spider
//! グランド・スパイダー
use super::prelude::*;

pub const ID: u32 = 17243896;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pos
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: Option<EffectId>, tp: u8) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and c:IsSummonPlayer(1-tp) and c:IsCanChangePosition()
    //     and (not e or c:IsRelateToEffect(e))
    card::is_position(d, c, POS_FACEUP_ATTACK)
        && card::is_summon_player(d, c, 1 - tp)
        && card::is_can_change_position(d, c)
        && e.map_or(true, |e| card::is_relate_to_effect(d, c, e))
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPosition(POS_FACEUP_DEFENSE)
    let c = effect::get_handler(d, a.e).nil();
    card::is_position(d, c, POS_FACEUP_DEFENSE)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(s.filter,1,nil,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return group::is_exists(d, &eg, &|d, c| filter(d, c, None, tp), 1, NO_EX);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(eg.clone()), eg.len() as u8, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    // local g=eg:Filter(s.filter,nil,e,tp)
    let g = group::filter(d, &eg, &|d, c| filter(d, c, Some(a.e), a.tp), NO_EX);
    // Duel.ChangePosition(g,POS_FACEUP_DEFENSE)
    duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, None, None, None, false);
}
