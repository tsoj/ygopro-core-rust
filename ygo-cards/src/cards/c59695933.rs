//! Intercept
//! インターセプト
use super::prelude::*;

pub const ID: u32 = 59695933;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD);
        // ge1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE)
        effect::set_property(d, ge1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE, 0);
        // ge1:SetCode(EFFECT_MATERIAL_CHECK)
        effect::set_code(d, ge1, EFFECT_MATERIAL_CHECK);
        // ge1:SetValue(s.valcheck)
        effect::set_value_fn(d, ge1, effect_card_value(valcheck));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
// function s.valcheck(e,c)
fn valcheck(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // if c:GetMaterialCount()==1 and c:GetMaterial():GetFirst():IsMonster() then
    if card::get_material_count(d, c) == 1 {
        let mat = card::get_material(d, c);
        let first = group::get_first(d, &mat).nil();
        if card::is_monster(d, first) {
            // c:RegisterFlagEffect(id,0,0,0)
            card::register_flag_effect(d, c, ID, 0, 0, 0, 0, 0);
        }
    }
    // (falls off the end: nil, read as 0)
    0
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // local res=tc:IsTributeSummoned() and tc:GetFlagEffect(id)~=0
    let res = card::is_tribute_summoned(d, tc) && card::get_flag_effect(d, tc, ID) != 0;
    // tc:ResetFlagEffect(id)
    card::reset_flag_effect(d, tc, ID);
    // return res
    res
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return eg:GetFirst():IsCanBeEffectTarget(e) and eg:GetFirst():IsControlerCanBeChanged() end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        let tc = group::get_first(d, &eg).nil();
        return card::is_can_be_effect_target(d, tc, Some(a.e)) && card::is_controler_can_be_changed(d, tc, false, None);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, a.eg());
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, a.eg.clone(), 1, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.GetControl(tc,tp)
            duel::get_control(d, tc, a.tp, 0, 0, None, None);
        }
    }
}
