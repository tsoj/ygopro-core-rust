//! Performapal Gatlinghoul
use super::prelude::*;

pub const ID: u32 = 49820233;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PERFORMAPAL], material_setcode: &[SET_PERFORMAPAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcFun2(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_PERFORMAPAL),s.ffilter,true)
    fusion::add_proc_fun2(
        d,
        c,
        fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_PERFORMAPAL as u16], Some(fc), st, Some(tp))),
        fusion::fus_mat(ffilter),
        true,
    );
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.damcon)
    effect::set_condition(d, e1, Some(condition(damcon)));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    // e2:SetValue(s.valcheck)
    effect::set_value_fn(d, e2, lua_fn(valcheck));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.ffilter(c,fc,sumtype,tp)
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK,fc,sumtype,tp) and c:IsLevelAbove(5)
    card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, Some(fc), sumtype, Some(tp)) && card::is_level_above(d, c, 5)
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)>0 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD) > 0;
    }
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)
    let ct = duel::get_field_group_count(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,ct*200)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, ct * 200);
    // if e:GetLabel()==1 then
    if effect::get_label(d, e) == 1 {
        // local g=Duel.GetFieldGroup(tp,0,LOCATION_MZONE)
        let g = duel::get_field_group(d, tp, 0, LOCATION_MZONE);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    }
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local p=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER)
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)
    let ct = duel::get_field_group_count(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // if ct>0 and Duel.Damage(p,ct*200,REASON_EFFECT)~=0 and e:GetLabel()==1 then
    if ct > 0 && duel::damage(d, p, (ct * 200) as i64, REASON_EFFECT, false, None) != 0 && effect::get_label(d, e) == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, None, tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.HintSelection(g)
            duel::hint_selection(d, g.clone(), true);
            // if Duel.Destroy(g,REASON_EFFECT)~=0 then
            if duel::destroy(d, g.clone(), REASON_EFFECT, None, None) != 0 {
                // local dam=g:GetFirst():GetBaseAttack()
                let first = group::get_first(d, &g).nil();
                let dam = card::get_base_attack(d, first);
                // Duel.Damage(p,dam,REASON_EFFECT)
                duel::damage(d, p, dam as i64, REASON_EFFECT, false, None);
            }
        }
    }
}
// function s.valcheck(e,c)
fn valcheck(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(1);
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsType,1,nil,TYPE_PENDULUM,c,SUMMON_TYPE_FUSION) then
    let found = group::is_exists(d, &g, &|d, x| card::is_type_ctx(d, x, TYPE_PENDULUM, Some(c), SUMMON_TYPE_FUSION as u64, None), 1, NO_EX);
    // e:GetLabelObject():SetLabel(1)
    // else e:GetLabelObject():SetLabel(0) end
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, lo, &[if found { 1 } else { 0 }]);
}
