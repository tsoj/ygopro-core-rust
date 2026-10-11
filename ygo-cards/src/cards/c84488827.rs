//! Steelswarm Caucastag
//! インヴェルズ・ガザス
use super::prelude::*;

pub const ID: u32 = 84488827;
pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_STEELSWARM],
    listed_names: &[62729173],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon success
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // tribute check
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    // e2:SetValue(s.valcheck)
    effect::set_value_fn(d, e2, effect_card_value(valcheck));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsSetCard,2,nil,SET_STEELSWARM) or g:IsExists(Card.IsCode,1,nil,62729173) then
    let a = group::is_exists(d, &g, &|d, c| card::is_set_card(d, c, &[SET_STEELSWARM as u16]), 2, NO_EX);
    let b = a || group::is_exists(d, &g, &|d, c| card::is_code(d, c, &[62729173]), 1, NO_EX);
    // e:GetLabelObject():SetLabel(1)
    // else e:GetLabelObject():SetLabel(0) end
    let le = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, le, &[if b { 1 } else { 0 }]);
    0
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned() and e:GetLabel()==1
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h) && effect::get_label(d, a.e) == 1
}
/// `s.sfilter(c)`
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local op=0
    let mut op: i64 = 0;
    let h = effect::get_handler(d, a.e).nil();
    // local g1=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
    let g1 = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, h.into());
    // local g2=Duel.GetMatchingGroup(s.sfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g2 = duel::get_matching_group(d, Some(&sfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // if #g1>0 and #g2>0 then
    if g1.len() > 0 && g2.len() > 0 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1),aux.Stringid(id,2))+1
        op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1), aux::stringid(ID, 2)]) as i64 + 1;
    } else if g1.len() > 0 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,1))+1
        op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]) as i64 + 1;
    } else if g2.len() > 0 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,2))+2
        op = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]) as i64 + 2;
    }
    // if op==1 then
    if op == 1 {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g1,#g1,0,0)
        let n = g1.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g1), n, 0, 0);
    } else if op == 2 {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g2,#g2,0,0)
        let n = g2.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g2), n, 0, 0);
    }
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_MZONE,LOCATION_MZONE,e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, h.into());
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    } else if effect::get_label(d, a.e) == 2 {
        // local g=Duel.GetMatchingGroup(s.sfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
        let g = duel::get_matching_group(d, Some(&sfilter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
