//! Altergeist Camouflage
use super::prelude::*;

pub const ID: u32 = 80143954;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALTERGEIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // destroy replace
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_target(d, e2, Some(cost_target(reptg)));
    effect::set_value_fn(d, e2, effect_card_fn(repval));
    effect::set_operation(d, e2, Some(operation(repop)));
    card::register_effect(d, c, e2, false);
}

fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ALTERGEIST as u16])
}

fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}

fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    let tc = duel::get_first_target(d).first().copied();
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            duel::equip(d, a.tp, c, tc, true, false);
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, c, e1, false);
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
            effect::set_code(d, e2, EFFECT_IGNORE_BATTLE_TARGET);
            effect::set_value(d, e2, 1);
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, c, e2, false);
            // negate
            let e4 = effect::create_effect(d, c);
            effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e4, EVENT_CHAIN_SOLVING);
            effect::set_range(d, e4, LOCATION_SZONE);
            effect::set_label_object(d, e4, Value::Card(tc));
            effect::set_condition(d, e4, Some(condition(negcon)));
            effect::set_operation(d, e4, Some(operation(negop)));
            effect::set_reset(d, e4, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, c, e4, false);
            return;
        }
    }
    card::cancel_to_grave(d, c, false);
}

fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let p = effect::get_handler_player(d, e);
    if card::get_controler(d, c) == p {
        return true;
    }
    let h = effect::get_handler(d, e).nil();
    card::get_equip_target(d, h) == Some(c)
}

fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    let lo = effect::get_label_object(d, a.e).as_card();
    if a.rp == a.tp || !effect::is_monster_effect(d, re) {
        return false;
    }
    match (g, lo) {
        (Some(g), Some(lo)) => group::is_contains(&g, lo),
        _ => false,
    }
}

fn negop(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}

fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_ALTERGEIST as u16])
        && card::is_location(d, c, LOCATION_ONFIELD)
        && card::is_controler(d, c, tp)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_EFFECT | REASON_BATTLE)
}

fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_able_to_remove(d, h, None, None, None) && group::is_exists(d, &eg, &|d, c| repfilter(d, c, a.tp), 1, NO_EX);
    }
    duel::select_effect_yes_no(d, a.tp, h, Some(96))
}

fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let p = effect::get_handler_player(d, e);
    repfilter(d, c, p)
}

fn repop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, h, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
