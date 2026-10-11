//! Lunalight Leo Dancer
use super::prelude::*;

pub const ID: u32 = 24550676;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LUNALIGHT], material_setcode: &[SET_LUNALIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,false,false,97165977,1,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_LUNALIGHT),2)
    fusion::add_proc_mix_n(
        d,
        c,
        false,
        false,
        vec![
            (fusion::Mat::Code(97165977), 1),
            (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_LUNALIGHT as u16], Some(fc), st, Some(tp)))), 2),
        ],
    );
    // spsummon condition
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e0, aux::fuslimit());
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Immune
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetValue(s.tgvalue)
    effect::set_value_fn(d, e2, lua_fn(tgvalue));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Double attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_EXTRA_ATTACK)
    effect::set_code(d, e3, EFFECT_EXTRA_ATTACK);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e4, EVENT_DAMAGE_STEP_END);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.condition)
    effect::set_condition(d, e4, Some(condition(cond)));
    // e4:SetTarget(s.target)
    effect::set_target(d, e4, Some(cost_target(target)));
    // e4:SetOperation(s.operation)
    effect::set_operation(d, e4, Some(operation(op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.tgvalue(e,re,rp)`
fn tgvalue(d: &mut Duel, a: Args) -> bool {
    // return rp~=e:GetHandlerPlayer()
    let e = a.e(0);
    let rp = a.p(2);
    rp != effect::get_handler_player(d, e)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetAttacker()==e:GetHandler() and Duel.GetAttackTarget()
    let h = effect::get_handler(d, a.e).nil();
    duel::get_attacker(d) == Some(h) && duel::get_attack_target(d).is_some()
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsSummonType,tp,0,LOCATION_MZONE,1,nil,SUMMON_TYPE_SPECIAL) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL])), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsSummonType,tp,0,LOCATION_MZONE,nil,SUMMON_TYPE_SPECIAL)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL])), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsSummonType,tp,0,LOCATION_MZONE,nil,SUMMON_TYPE_SPECIAL)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL])), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
