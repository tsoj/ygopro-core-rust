//! Revendread Evolution
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 7986397;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VENDREAD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Ritual.CreateProc({handler=c,lvtype=RITPROC_EQUAL,filter=aux.FilterBoolFunction(Card.IsSetCard,SET_VENDREAD),extrafil=s.extragroup,
    //   extraop=s.extraop,stage2=s.stage2,location=LOCATION_HAND|LOCATION_GRAVE,forcedselection=s.ritcheck,extratg=s.extratg})
    let e1 = ritual::create_proc(
        d,
        c,
        ritual::RitualProc {
            lvtype: RITPROC_EQUAL,
            filter: Some(Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_VENDREAD as u16]))),
            extrafil: Some(Arc::new(|d: &mut Duel, a: &EffArgs| Some(extragroup(d, a)))),
            extraop: Some(Arc::new(extraop)),
            stage2: Some(Arc::new(stage2)),
            location: Some(LOCATION_HAND | LOCATION_GRAVE),
            forcedselection: Some(Arc::new(ritcheck)),
            extratg: Some(cost_target(extratg)),
            ..Default::default()
        },
    );
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.extragroup(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extragroup(d: &mut Duel, a: &EffArgs) -> CardSet {
    duel::get_matching_group(d, Some(&matfilter1), a.tp, LOCATION_DECK, 0, NO_EX)
}
/// `s.matfilter1(c)`
fn matfilter1(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_VENDREAD as u16]) && card::is_able_to_grave(d, c, None) && card::is_level_above(d, c, 1)
}
/// `s.extraop(mat,e,tp,eg,ep,ev,re,r,rp,tc)`
fn extraop(d: &mut Duel, mat: &CardSet, _a: &EffArgs, _tc: CardId) {
    // local mat2=mat:Filter(Card.IsLocation,nil,LOCATION_DECK)
    let mat2 = group::filter(d, mat, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // mat:Sub(mat2)
    let mut mat = mat.clone();
    group::remove_card(&mut mat, &mat2);
    // Duel.ReleaseRitualMaterial(mat)
    duel::release_ritual_material(d, &mat, false);
    // Duel.SendtoGrave(mat2,REASON_EFFECT+REASON_MATERIAL+REASON_RITUAL)
    duel::send_to_grave(d, mat2, REASON_EFFECT | REASON_MATERIAL | REASON_RITUAL, None, None);
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.stage2(mat,e,tp,eg,ep,ev,re,r,rp,tc)`
fn stage2(d: &mut Duel, _mat: &CardSet, a: &EffArgs, tc: CardId) {
    let (e, tp) = (a.e, a.tp);
    // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,0))
    card::register_flag_effect(d, tc, ID, RESET_EVENT + RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 0));
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetLabel(Duel.GetTurnCount())
    let turn = duel::get_turn_count(d, None) as i64;
    effect::set_label(d, e1, &[turn]);
    // e1:SetLabelObject(tc)
    effect::set_label_object(d, e1, Value::Card(tc));
    effect::set_reset(d, e1, RESET_PHASE + PHASE_END, 2);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_operation(d, e1, Some(operation(desop)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.ritcheck(e,tp,g,sc)`
fn ritcheck(d: &mut Duel, _e: EffectId, _tp: u8, g: &CardSet, _sc: CardId) -> (bool, bool) {
    let deck_ct = group::filter_count(d, g, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    (deck_ct <= 1, false)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let turn = duel::get_turn_count(d, None) as i64;
    if turn <= effect::get_label(d, a.e) {
        return false;
    }
    let obj = effect::get_label_object(d, a.e).as_card().nil();
    card::get_flag_effect(d, obj, ID) != 0
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let obj = effect::get_label_object(d, a.e).as_card().nil();
    duel::destroy(d, obj, REASON_EFFECT, None, None);
}
