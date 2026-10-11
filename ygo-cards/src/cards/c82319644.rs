//! Tlakalel, His Malevolent Majesty
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 82319644;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon success
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
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
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttackBelow,e:GetLabel()),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        let lbl = effect::get_label(d, a.e) as i32;
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_attack_below(d, c, lbl)),
            tp,
            0,
            LOCATION_MZONE,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - tp, LOCATION_MZONE as i32);
    true
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local atk=0
    let mut atk: i64 = 0;
    // g:ForEach(function(c) atk=atk+math.max(c:GetBaseAttack(),0) end)
    for tc in group::iter(d, &g) {
        atk += card::get_base_attack(d, tc).max(0) as i64;
    }
    // e:GetLabelObject():SetLabel(atk)
    let e1 = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, e1, &[atk]);
    0
}
/// `s.check(atk)`: the `sg:GetSum(Card.GetAttack)<=atk` restriction
fn check(d: &mut Duel, sg: &CardSet, atk: i64) -> (bool, bool) {
    (group::get_sum(d, sg, &|d, c| card::get_attack(d, c) as i64) <= atk, false)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sg=Duel.GetMatchingGroup(Card.IsFaceup,tp,0,LOCATION_MZONE,nil)
    let sg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // local atk=e:GetLabel()
    let atk = effect::get_label(d, a.e);
    // local g=aux.SelectUnselectGroup(sg,e,tp,1,#sg,s.check(atk),1,tp,HINTMSG_DESTROY)
    let rs: aux::Rescon = &|d, sg, _e, _tp, _mg, _c| check(d, sg, atk);
    let g = aux::select_unselect_group(d, &sg, Some(a.e), tp, Some(1), Some(sg.len()), Some(rs), 1, tp, Some(HINTMSG_DESTROY as u64), None, None, false).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
