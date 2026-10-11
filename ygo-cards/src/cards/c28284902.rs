//! Morphtronic Monitron
use super::prelude::*;

pub const ID: u32 = 28284902;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_MORPHTRONIC}
    listed_series: &[SET_MORPHTRONIC],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // pos
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e2, CATEGORY_POSITION);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);

    // local e3=e2:Clone()
    // e3:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    // c:RegisterEffect(e3)
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);

    // local e4=e2:Clone()
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    // c:RegisterEffect(e4)
    let e4 = effect::clone(d, e2);
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e4, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MORPHTRONIC) and c:IsAttackPos() and c:IsCanChangePosition()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && card::is_attack_pos(d, c) && card::is_can_change_position(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(s.cfilter,1,nil) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &cfilter, 1, NO_EX);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(eg.clone()), eg.len() as u8, 0, 0);
    true
}
// function s.filter(c,e)
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MORPHTRONIC) and c:IsAttackPos() and c:IsRelateToEffect(e)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MORPHTRONIC as u16]) && card::is_attack_pos(d, c) && card::is_relate_to_effect(d, c, e)
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=eg:Filter(s.filter,nil,e)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, a.e), NO_EX);
    // Duel.ChangePosition(g,POS_FACEUP_DEFENSE)
    duel::change_position(d, g, POS_FACEUP_DEFENSE as u8, None, None, None, false);
}
