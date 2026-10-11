//! D/D/D/D Super-Dimensional Sovereign Emperor Zero Paradox
use super::prelude::*;

pub const ID: u32 = 97417863;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DDD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum Summon
    pendulum::add_procedure(d, c, None, None);
    // Must be Special Summoned
    card::enable_revive_limit(d, c);

    // Special Summon condition
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    card::register_effect(d, c, e0, false);

    // Special Summon this card and place an opponent's Pendulum Scale in your Pendulum Zone
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_CONTROL);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(pentg)));
    effect::set_operation(d, e1, Some(operation(penop)));
    card::register_effect(d, c, e1, false);

    // Special Summon this card and destroy all other cards on the field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);

    // This card's ATK becomes 6000
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(atkcon)));
    effect::set_operation(d, e3, Some(operation(atkop)));
    card::register_effect(d, c, e3, false);
}
fn changeable_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_change_controler(d, c)
}
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_PZONE) && card::is_controler(d, chkc, 1 - tp);
    }
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, true, true, None, None, None)
            && duel::is_existing_target(d, Some(&changeable_filter), tp, 0, LOCATION_PZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, tp, Some(&changeable_filter), tp, 0, LOCATION_PZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, tp, 0);
    true
}
fn penop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || duel::special_summon(d, c, 0, tp, tp, true, true, POS_FACEUP as u8, None) == 0 {
        return;
    }
    card::complete_procedure(d, c);
    let tc = duel::first_target(d).nil();
    if !(card::is_relate_to_effect(d, tc, a.e) && duel::check_pendulum_zones(d, tp)) {
        return;
    }
    if duel::move_to_field(d, tc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None) {
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        // Destroy it during the End Phase of the next turn
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        effect::set_label(d, e1, &[duel::get_turn_count(d, None) as i64]);
        effect::set_label_object(d, e1, Value::Card(tc));
        effect::set_condition(d, e1, Some(condition(pen_destroy_cond)));
        effect::set_operation(d, e1, Some(operation(pen_destroy_op)));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
        duel::register_effect(d, e1, tp);
    }
}
fn pen_destroy_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let label = effect::get_label(d, a.e);
    if duel::get_turn_count(d, None) as i64 != label + 1 {
        return false;
    }
    let obj = effect::get_label_object(d, a.e).as_card().nil();
    card::has_flag_effect(d, obj, ID, None)
}
fn pen_destroy_op(d: &mut Duel, a: &EffArgs) {
    let obj = effect::get_label_object(d, a.e).as_card().nil();
    duel::destroy(d, obj, REASON_EFFECT, None, None);
}
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_pendulum_summoned(d, c) && card::has_level(d, c) && card::is_summon_player(d, c, tp) && card::is_faceup(d, c)
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    let sg = group::filter(d, &eg, &|d, c| spcfilter(d, c, tp), NO_EX);
    if sg.is_empty() {
        return false;
    }
    let pg = duel::get_field_group(d, tp, LOCATION_PZONE, 0);
    if pg.is_empty() {
        return false;
    }
    let total_scales = group::get_sum(d, &pg, &|d, c| card::get_scale(d, c) as i64);
    let total_levels = group::get_sum(d, &sg, &|d, c| card::get_level(d, c) as i64);
    total_scales > total_levels
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_field_group(d, 0, LOCATION_ONFIELD, LOCATION_ONFIELD);
    if a.chk == Some(0) {
        return !g.is_empty()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, true, true, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || duel::special_summon(d, c, 0, tp, tp, true, true, POS_FACEUP as u8, None) == 0 {
        return;
    }
    card::complete_procedure(d, c);
    let g = duel::get_matching_group(d, None, 0, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    if !g.is_empty()
        && duel::destroy(d, g, REASON_EFFECT, None, None) > 0
        && duel::check_pendulum_zones(d, tp)
        && card::is_type(d, c, TYPE_PENDULUM)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
    {
        duel::break_effect(d);
        duel::move_to_field(d, c, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
fn atkcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_set_card(d, c, &[SET_DDD as u16])
        && card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_EFFECT)
        && {
            let re = card::get_reason_effect(d, c).nil();
            effect::is_spell_effect(d, re)
        }
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| atkcfilter(d, c, tp), 1, NO_EX)
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Its ATK becomes 6000
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e1, 6000);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
    }
}
