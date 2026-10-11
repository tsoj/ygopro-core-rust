//! Ultimate Dragon of Pride and Soul
use super::prelude::*;

pub const ID: u32 = 3149401;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 3 monsters with 2500 original ATK and DEF
    // Fusion.AddProcMixN(c,true,true,function(c) return c:IsBaseAttack(2500) and c:IsBaseDefense(2500) end,3)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, _fc, _st, _tp| {
        card::is_base_attack(d, c, 2500) && card::is_base_defense(d, c, 2500)
    })), 3)]);
    // This Fusion Summoned card cannot be destroyed by your opponent's card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() end)
    effect::set_condition(d, e1, Some(condition(fusion_summoned_cond)));
    // e1:SetValue(aux.indoval)
    effect::set_value_fn(d, e1, aux::indoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your opponent cannot target this Fusion Summoned card with card effects
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // While you have 25 or more cards in your GY, this Fusion Summoned card gains 4500 ATK/DEF
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetCondition(function(e) return e:GetHandler():IsFusionSummoned() and Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_GRAVE,0)>=25 end)
    effect::set_condition(d, e3, Some(condition(gain_cond)));
    // e3:SetValue(4500)
    effect::set_value(d, e3, 4500);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Destroy all cards your opponent controls
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e5, CATEGORY_DESTROY);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCountLimit(1)
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    // e5:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_GRAVE)>=25 end)
    effect::set_condition(d, e5, Some(condition(des_cond)));
    // e5:SetTarget(s.destg)
    effect::set_target(d, e5, Some(cost_target(destg)));
    // e5:SetOperation(s.desop)
    effect::set_operation(d, e5, Some(operation(desop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `function(e) return e:GetHandler():IsFusionSummoned() end`
fn fusion_summoned_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, c)
}
/// `function(e) return e:GetHandler():IsFusionSummoned() and Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_GRAVE,0)>=25 end`
fn gain_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_fusion_summoned(d, c) {
        return false;
    }
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, LOCATION_GRAVE, 0) >= 25
}
/// `function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_GRAVE)>=25 end`
fn des_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, 0, LOCATION_GRAVE) >= 25
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) > 0;
    }
    // local dg=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let dg = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,tp,0)
    let n = dg.len();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n as u8, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local dg=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let dg = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // if #dg>0 then Duel.Destroy(dg,REASON_EFFECT) end
    if !dg.is_empty() {
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
