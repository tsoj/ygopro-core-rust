//! Toon Gemini Elf
use super::prelude::*;

pub const ID: u32 = 42386471;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[15259703], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_operation(d, e1, Some(operation(atklimit)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
    // destroy
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    effect::set_condition(d, e4, Some(condition(sdescon)));
    effect::set_operation(d, e4, Some(operation(sdesop)));
    card::register_effect(d, c, e4, false);
    // direct attack
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e5, EFFECT_DIRECT_ATTACK);
    effect::set_condition(d, e5, Some(condition(dircon)));
    card::register_effect(d, c, e5, false);
    // handes
    let e6 = effect::create_effect(d, c);
    effect::set_description(d, e6, aux::stringid(ID, 0));
    effect::set_category(d, e6, CATEGORY_HANDES);
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e6, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e6, Some(condition(cond)));
    effect::set_target(d, e6, Some(cost_target(target)));
    effect::set_operation(d, e6, Some(operation(op)));
    card::register_effect(d, c, e6, false);
}
/// `s.atklimit(e,...)`
fn atklimit(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // e:GetHandler():RegisterEffect(e1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_effect(d, h, e1, false);
}
/// `s.sfilter(c)`
fn sfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsReason(REASON_DESTROY) and c:IsPreviousPosition(POS_FACEUP) and c:GetPreviousCodeOnField()==15259703 and c:IsPreviousLocation(LOCATION_ONFIELD)
    card::is_reason(d, c, REASON_DESTROY)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::get_previous_code_on_field(d, c).first() == Some(&15259703)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.sdescon(e,...)`
fn sdescon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.sfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &sfilter, 1, NO_EX)
}
/// `s.sdesop(e,...)`
fn sdesop(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
/// `s.dirfilter1(c)`
fn dirfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(15259703)
    card::is_faceup(d, c) && card::is_code(d, c, &[15259703])
}
/// `s.dirfilter2(c)`
fn dirfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_TOON)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_TOON)
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.dirfilter1,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil) and not Duel.IsExistingMatchingCard(s.dirfilter2,e:GetHandlerPlayer(),0,LOCATION_MZONE,1,nil)
    let hp = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&dirfilter1), hp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && !duel::is_existing_matching_card(d, Some(&dirfilter2), hp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(_d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp
    a.ep != a.tp
}
/// `s.target(e,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return Duel.GetFieldGroupCount(ep,LOCATION_HAND,0)>0
        return duel::get_field_group_count(d, a.ep, LOCATION_HAND, 0) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,1-tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 1);
    true
}
/// `s.operation(e,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(ep,LOCATION_HAND,0)
    let g = duel::get_field_group(d, a.ep, LOCATION_HAND, 0);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=g:RandomSelect(1-tp,1)
    let sg = group::random_select(d, &g, 1 - a.tp, 1);
    // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
}
