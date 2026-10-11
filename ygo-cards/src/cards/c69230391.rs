//! Thestalos the Mega Monarch
use super::prelude::*;

pub const ID: u32 = 69230391;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 1 tribute
    // local e1=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e1 = aux::add_normal_summon_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE),
        Some(aux::stringid(ID, 0)),
        Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _tp: u8| otfilter(d, c))),
        None,
    );
    // local e2=aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _e2 = aux::add_normal_set_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE),
        Some(aux::stringid(ID, 0)),
        Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _tp: u8| otfilter(d, c))),
        None,
    );
    // handes
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_HANDES+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_HANDES + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    // e3:SetCondition(s.condition)
    effect::set_condition(d, e3, Some(condition(cond)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    // e4:SetValue(s.valcheck)
    effect::set_value_fn(d, e4, effect_card_value(valcheck));
    // e4:SetLabelObject(e3)
    effect::set_label_object(d, e4, Value::Effect(e3));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.otfilter(c)`
fn otfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTributeSummoned()
    card::is_tribute_summoned(d, c)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    // if e:GetLabel()==1 then
    if effect::get_label(d, a.e) == 1 {
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1000)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1000);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_HAND)
    let g = duel::get_field_group(d, tp, 0, LOCATION_HAND);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.ConfirmCards(tp,g)
    duel::confirm_cards(d, tp, g.clone());
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DISCARD as u64);
    // local hg=g:Select(tp,1,1,nil)
    let hg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(hg,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, hg.clone(), REASON_EFFECT | REASON_DISCARD, None, None);
    // Duel.ShuffleHand(1-tp)
    duel::shuffle_hand(d, 1 - tp);
    // local tc=hg:GetFirst()
    let tc = group::get_first(d, &hg).nil();
    // if tc:IsMonster() then
    if card::is_monster(d, tc) {
        // Duel.Damage(1-tp,tc:GetLevel()*200,REASON_EFFECT)
        let lv = card::get_level(d, tc);
        duel::damage(d, 1 - tp, (lv * 200) as i64, REASON_EFFECT, false, None);
    }
    // if Duel.GetOperationInfo(0,CATEGORY_DAMAGE) then
    if duel::get_operation_info(d, 0, CATEGORY_DAMAGE).is_some() {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,1000,REASON_EFFECT)
        duel::damage(d, 1 - tp, 1000, REASON_EFFECT, false, None);
    }
}
/// `s.valcheck(e,c)`: the label of e3 says whether the material includes a FIRE monster
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_FIRE) then
    if group::is_exists(d, &g, &|d, x| card::is_attribute(d, x, ATTRIBUTE_FIRE), 1, NO_EX) {
        // e:GetLabelObject():SetLabel(1)
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, lo, &[1]);
    } else {
        // e:GetLabelObject():SetLabel(0)
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, lo, &[0]);
    }
    // (a value function that returns nothing gives 0)
    0
}
