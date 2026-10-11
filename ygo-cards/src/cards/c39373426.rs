//! SPYRAL MISSION - Recapture
use super::prelude::*;

pub const ID: u32 = 39373426;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPYRAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_description(d, e0, aux::stringid(ID, 0));
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    effect::set_target(d, e0, Some(cost_target(target)));
    card::register_effect(d, c, e0, false);
    // Take control 1 of opponent's monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_category(d, e1, CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    effect::set_condition(d, e1, Some(condition(cncon)));
    effect::set_cost(d, e1, Some(cost_target(cncost)));
    effect::set_target(d, e1, Some(cost_target(cntg1)));
    effect::set_operation(d, e1, Some(operation(cnop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_description(d, e2, aux::stringid(ID, 3));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target(d, e2, Some(cost_target(cntg2)));
    card::register_effect(d, c, e2, false);
    // Substitute destruction for 1 "SPYRAL" monster
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_target(d, e3, Some(cost_target(reptg)));
    effect::set_value_fn(d, e3, effect_card_fn(repval));
    effect::set_operation(d, e3, Some(operation(repop)));
    card::register_effect(d, c, e3, false);
}
/// The self-destruction effect that `s.target` and `s.cntg1` create (chk == 1 part).
fn set_self_destruction(d: &mut Duel, c: CardId) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_operation(d, e1, Some(operation(desop)));
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END | RESET_SELF_TURN, 3);
    card::set_turn_counter(d, c, 0);
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    // Destroy itself
    set_self_destruction(d, c);
    true
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ct = card::get_turn_counter(d, c) + 1;
    card::set_turn_counter(d, c, ct as u16);
    if ct == 3 {
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
/// `s.cncfilter(c,tp)`
fn cncfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SPYRAL as u16]) && card::is_controler(d, c, tp)
}
fn cncon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cncfilter(d, c, tp), 1, NO_EX)
}
fn cncost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::get_flag_effect(d, c, ID) == 0;
    }
    card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    true
}
fn cntg1(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_controler_can_be_changed(d, chkc, false, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    let c = effect::get_handler(d, e).nil();
    // Destroy itself
    set_self_destruction(d, c);
    true
}
fn cntg2(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_controler_can_be_changed(d, chkc, false, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, 0, 0);
    true
}
fn cnop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && duel::get_control(d, tc, tp, PHASE_END as u16, 1, None, None) != 0 {
            // Cannot attack directly
            let e1 = effect::create_effect(d, h);
            effect::set_description(d, e1, 3207);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SPYRAL as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_EFFECT | REASON_BATTLE)
}
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, h, None, None, None) && group::is_exists(d, &eg, &|d, c| repfilter(d, c, tp), 1, NO_EX);
    }
    if duel::select_effect_yes_no(d, tp, h, Some(96)) {
        let eg = a.eg.clone().nil();
        let g = group::filter(d, &eg, &|d, c| repfilter(d, c, tp), NO_EX);
        if g.len() == 1 {
            let first = group::get_first(d, &g);
            effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
        } else {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESREPLACE as u64);
            let cg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            let first = group::get_first(d, &cg);
            effect::set_label_object(d, e, first.map_or(Value::Nil, Value::Card));
        }
        true
    } else {
        false
    }
}
/// `function(e,c) return c==e:GetLabelObject() end`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    effect::get_label_object(d, e).as_card() == Some(c)
}
fn repop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
