//! Armed Dragon LV7
use super::prelude::*;

pub const ID: u32 = 73879377;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[46384672], lv: Some((7, SET_ARMED_DRAGON)), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.FALSE)
    effect::set_value_fn(d, e1, aux_false());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(s.descost)
    effect::set_cost(d, e2, Some(cost_target(descost)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local atk=c:GetAttack()
    let mut atk = card::get_attack(d, c);
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // return c:IsMonster() and c:IsAbleToGraveAsCost()
    //   and Duel.IsExistingMatchingCard(s.dfilter,tp,0,LOCATION_MZONE,1,nil,atk)
    card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c) && duel::is_existing_matching_card(d, Some(&|d, sc| dfilter(d, sc, atk)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.dfilter(c,atk)`
fn dfilter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:GetAttack()<=atk
    card::is_faceup(d, c) && card::get_attack(d, c) <= atk
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local atk=g:GetFirst():GetAttack()
    let first = group::get_first(d, &g).nil();
    let mut atk = card::get_attack(d, first);
    // if atk<0 then atk=0 end
    if atk < 0 {
        atk = 0;
    }
    // e:SetLabel(atk)
    effect::set_label(d, a.e, &[atk as i64]);
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.dfilter,tp,0,LOCATION_MZONE,nil,e:GetLabel())
    let label = effect::get_label(d, a.e);
    let g = duel::get_matching_group(d, Some(&|d, c| dfilter(d, c, label as i32)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.dfilter,tp,0,LOCATION_MZONE,nil,e:GetLabel())
    let label = effect::get_label(d, a.e);
    let g = duel::get_matching_group(d, Some(&|d, c| dfilter(d, c, label as i32)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
