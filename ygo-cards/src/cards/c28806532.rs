//! The Weather Painter Cloud
use super::prelude::*;

pub const ID: u32 = 28806532;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_THE_WEATHER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Place 2 "The Weather" Spells/Traps face-up in your Spell & Trap Zone
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(tfcon)));
    effect::set_target(d, e1, Some(cost_target(tftg)));
    effect::set_operation(d, e1, Some(operation(tfop)));
    card::register_effect(d, c, e1, false);
    // Special Summon this banished card
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2, EVENT_REMOVE);
    effect::set_operation(d, e2, Some(operation(spreg)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e3, LOCATION_REMOVED);
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
}

fn tfcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_set_card(d, c, &[SET_THE_WEATHER as u16])
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
}

fn tfcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| tfcfilter(d, c, a.tp), 1, h.into())
}

fn tffilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_spell_trap(d, c)
        && card::is_set_card(d, c, &[SET_THE_WEATHER as u16])
        && !card::is_forbidden(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
        && !card::is_type(d, c, TYPE_FIELD)
}

fn tftg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && tffilter(d, chkc, a.tp);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| tffilter(d, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    let ct = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0.min(2);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| tffilter(d, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, ct as u16, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g.clone()), g.len() as u8, a.tp, 0);
    true
}

fn tfop(d: &mut Duel, a: &EffArgs) {
    let mut g = duel::get_target_cards(d, a.e).nil();
    if g.is_empty() {
        return;
    }
    let zc = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0.min(2);
    if zc < 1 {
        return;
    }
    if g.len() > zc as usize {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
        g = group::select(d, &g, a.tp, 1, zc as u16, false, NO_EX).nil();
    }
    for tc in group::iter(d, &g) {
        duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}

fn spreg(d: &mut Duel, a: &EffArgs) {
    let Some(re) = a.re else { return };
    let c = effect::get_handler(d, a.e).nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_reason(d, c, REASON_COST) && card::is_set_card(d, rc, &[SET_THE_WEATHER as u16]) {
        let turn = duel::get_turn_count(d, None) as i64 + 1;
        effect::set_label(d, a.e, &[turn]);
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 2, 0, 0);
    }
}

fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    let label = effect::get_label(d, e2);
    let c = effect::get_handler(d, a.e).nil();
    label == duel::get_turn_count(d, None) as i64 && card::get_flag_effect(d, c, ID) > 0
}

fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    card::reset_flag_effect(d, c, ID);
    true
}

fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
