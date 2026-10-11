//! Morphtronic Slingen
use super::prelude::*;

pub const ID: u32 = 75775867;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MORPHTRONIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetCost(s.descost)
    effect::set_cost(d, e1, Some(cost_target(descost)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy replace
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
    // e2:SetOperation(s.repop)
    effect::set_operation(d, e2, Some(operation(repop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsDisabled() and e:GetHandler():IsAttackPos()
    let h = effect::get_handler(d, a.e).nil();
    !card::is_disabled(d, h) && card::is_attack_pos(d, h)
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetCode()~=id and c:IsSetCard(SET_MORPHTRONIC)
    card::get_code(d, c)[0] != ID && card::is_set_card(d, c, &[SET_MORPHTRONIC as u16])
}
// function s.descost(e,tp,eg,ep,ev,re,r,rp,chk)
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local dg=Duel.GetMatchingGroup(Card.IsCanBeEffectTarget,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil,e)
    let e = a.e;
    let dg = duel::get_matching_group(d, Some(&|d, c| card::is_can_be_effect_target(d, c, Some(e))), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // aux.ReleaseCheckTarget
    let check = |d: &mut Duel, sg: &CardSet, tp: u8, exg: &CardSet| aux::release_check_target(d, sg, tp, exg, &dg);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,aux.ReleaseCheckTarget,nil,dg) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&cfilter), 1, 1, false, Some(&check), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,aux.ReleaseCheckTarget,nil,dg)
    let g = duel::select_release_group_cost(d, tp, Some(&cfilter), 1, 1, false, Some(&check), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
// function s.repfilter(c,e)
fn repfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MORPHTRONIC)
    //     and c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_MORPHTRONIC as u16])
        && card::is_destructable(d, c, Some(e))
        && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let e = a.e;
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and c:IsOnField() and c:IsFaceup() and c:IsDefensePos()
    //     and Duel.IsExistingMatchingCard(s.repfilter,tp,LOCATION_MZONE,0,1,c,e) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE)
            && card::is_on_field(d, c)
            && card::is_faceup(d, c)
            && card::is_defense_pos(d, c)
            && duel::is_existing_matching_card(d, Some(&|d, x| repfilter(d, x, e)), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.repfilter,tp,LOCATION_MZONE,0,1,1,c,e)
        let g = duel::select_matching_card(d, tp, Some(&|d, x| repfilter(d, x, e)), tp, LOCATION_MZONE, 0, 1, 1, false, c.into()).nil();
        // e:SetLabelObject(g:GetFirst())
        let first = group::get_first(d, &g).nil();
        effect::set_label_object(d, e, Value::Card(first));
        // g:GetFirst():SetStatus(STATUS_DESTROY_CONFIRMED,true)
        card::set_status(d, first, STATUS_DESTROY_CONFIRMED, true);
        true
    } else {
        false
    }
}
// function s.repop(e,tp,eg,ep,ev,re,r,rp)
fn repop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // tc:SetStatus(STATUS_DESTROY_CONFIRMED,false)
    card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(tc,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, tc, REASON_EFFECT | REASON_REPLACE, None, None);
}
