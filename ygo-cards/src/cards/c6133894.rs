//! Mazera DeVille
use super::prelude::*;

pub const ID: u32 = 6133894;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[66073051, 94585852], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e1, false);
    // special summon
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    effect::set_target(d, e2, Some(lua_fn(sptg)));
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    card::register_effect(d, c, e2, false);
    // handes
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_HANDES);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_target(d, e3, Some(cost_target(hdtg)));
    effect::set_operation(d, e3, Some(operation(hdop)));
    card::register_effect(d, c, e3, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return Duel.IsEnvironment(94585852) end
    let Some(c) = a.card(1) else {
        return duel::is_environment(d, 94585852, None, None);
    };
    let tp = card::get_controler(d, c);
    // return Duel.CheckReleaseGroup(c:GetControler(),aux.FaceupFilter(Card.IsCode,66073051),1,false,1,true,c,c:GetControler(),nil,false,nil)
    duel::check_release_group(
        d,
        tp,
        Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[66073051])),
        1,
        duel::ReleaseArgs { use_hand: Some(false), max: Some(1), check_field: true, to_check: Some(c), toplayer: Some(tp), ..Default::default() },
        NO_EX,
        false,
    )
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=Duel.SelectReleaseGroup(tp,aux.FaceupFilter(Card.IsCode,66073051),1,1,false,true,true,c,nil,nil,false,nil)
    let g = duel::select_release_group(
        d,
        tp,
        Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[66073051])),
        1,
        1,
        duel::ReleaseArgs { use_hand: Some(false), cancelable: true, check_field: true, to_check: None, ..Default::default() },
        NO_EX,
        false,
    );
    if let Some(g) = g {
        // g:KeepAlive() does nothing in Rust (groups are values)
        effect::set_label_object(d, e, Value::Group(g));
        return true;
    }
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else { return };
    duel::release(d, g, REASON_COST, None);
    // g:DeleteGroup() does nothing in Rust
}
/// `s.hdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn hdtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, 1 - a.tp, 3);
    true
}
/// `s.hdop(e,tp,eg,ep,ev,re,r,rp)`
fn hdop(d: &mut Duel, a: &EffArgs) {
    if duel::is_environment(d, 94585852, None, None) {
        let hand = duel::get_field_group(d, a.tp, 0, LOCATION_HAND);
        let g = group::random_select(d, &hand, a.tp, 3);
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_DISCARD, None, None);
    }
}
