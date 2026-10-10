//! Doom Virus Dragon
use super::prelude::*;

pub const ID: u32 = 22804644;

pub const INFO: ScriptInfo = ScriptInfo { material_trap: Some(57728570), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // spsummon condition
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_UNCOPYABLE, 0);
    card::register_effect(d, c, e1, false);
    //
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::get_attack(d, c) >= 1500
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, Some(&tgfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_attack_above(d, c, 1500)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let conf = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE | LOCATION_HAND);
    if !conf.is_empty() {
        duel::confirm_cards(d, a.tp, conf.clone());
        let dg = group::filter(d, &conf, &filter, NO_EX);
        duel::destroy(d, dg, REASON_EFFECT, None, None);
        duel::shuffle_hand(d, 1 - a.tp);
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_DRAW);
    effect::set_operation(d, e1, Some(operation(desop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 3);
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(turncon)));
    effect::set_operation(d, e2, Some(operation(turnop)));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 3);
    duel::register_effect(d, e2, a.tp);
    effect::set_label_object(d, e2, Value::Effect(e1));
    // local descnum=tp==c:GetOwner() and 0 or 1
    let descnum = if a.tp == card::get_owner(d, c) { 0 } else { 1 };
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_description(d, e3, aux::stringid(ID, descnum));
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_UNCOPYABLE | EFFECT_FLAG_IGNORE_IMMUNE | EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e3, 1082946);
    effect::set_label_object(d, e3, Value::Effect(e2));
    effect::set_owner_player(d, e3, a.tp);
    effect::set_operation(d, e3, Some(operation(reset)));
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 3);
    card::register_effect(d, c, e3, false);
}
/// `s.reset(e,tp,eg,ep,ev,re,r,rp)`
fn reset(d: &mut Duel, a: &EffArgs) {
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    turnop_e(d, lo, Some(a.e));
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    if a.ep == effect::get_owner_player(d, a.e) {
        return;
    }
    let eg = a.eg.clone().nil();
    let hg = group::filter(d, &eg, &|d, c| card::is_location(d, c, LOCATION_HAND), NO_EX);
    if hg.is_empty() {
        return;
    }
    duel::confirm_cards(d, 1 - a.ep, hg.clone());
    let dg = group::filter(d, &hg, &filter, NO_EX);
    duel::destroy(d, dg, REASON_EFFECT, None, None);
    duel::shuffle_hand(d, a.ep);
}
/// `s.turncon(e,tp,eg,ep,ev,re,r,rp)`
fn turncon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.turnop(e,tp,eg,ep,ev,re,r,rp)`
fn turnop(d: &mut Duel, a: &EffArgs) {
    turnop_e(d, a.e, a.re);
}
/// The body of `s.turnop` with `e` and `re` as the Lua function gets them.
fn turnop_e(d: &mut Duel, e: EffectId, re: Option<EffectId>) {
    let ct = effect::get_label(d, e) + 1;
    effect::set_label(d, e, &[ct]);
    let h = effect::get_handler(d, e).nil();
    card::set_turn_counter(d, h, ct as u16);
    if ct == 3 {
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::reset(d, lo);
        if let Some(re) = re {
            effect::reset(d, re);
        }
    }
}
