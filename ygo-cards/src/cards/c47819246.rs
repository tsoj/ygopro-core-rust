//! Super Quantal Mech Sword - Magnaslayer
use super::prelude::*;

pub const ID: u32 = 47819246;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPER_QUANT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // atk up
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    card::register_effect(d, c, e2, false);
    // pierce
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_PIERCE);
    card::register_effect(d, c, e3, false);
    // multi attack
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_condition(d, e4, Some(condition(mtcon)));
    effect::set_cost(d, e4, Some(cost_target(mtcost)));
    effect::set_target(d, e4, Some(cost_target(mttg)));
    effect::set_operation(d, e4, Some(operation(mtop)));
    card::register_effect(d, c, e4, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SUPER_QUANT as u16]) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let _ = duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_location(d, c, LOCATION_SZONE) || !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::get_controler(d, tc) == card::get_controler(d, c) {
        duel::equip(d, tp, c, tc, true, false);
        // Equip limit
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
    } else {
        card::cancel_to_grave(d, c, false);
    }
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    card::get_controler(d, c) == effect::get_owner_player(d, e)
        && card::is_set_card(d, c, &[SET_SUPER_QUANT as u16])
        && card::is_type(d, c, TYPE_XYZ)
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_rank(d, c) as i64 * 100
}
/// `s.mtcon(e,tp,eg,ep,ev,re,r,rp)`
fn mtcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_type(d, h, TYPE_EQUIP) && duel::is_turn_player(d, a.tp) && duel::is_battle_phase(d, None)
}
/// `s.mtcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mtcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c);
    }
    if let Some(ec) = card::get_equip_target(d, c) {
        duel::set_target_card(d, ec);
    }
    duel::send_to_grave(d, CardSet::from_card(c), REASON_COST, None, None);
    true
}
/// `s.mttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mttg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    if a.chk == Some(0) {
        return match ec {
            None => false,
            Some(ec) => card::is_has_effect(d, ec, EFFECT_EXTRA_ATTACK, None).is_empty(),
        };
    }
    true
}
/// `s.mtop(e,tp,eg,ep,ev,re,r,rp)`
fn mtop(d: &mut Duel, a: &EffArgs) {
    if let Some(ec) = duel::first_target(d) {
        if card::is_location(d, ec, LOCATION_MZONE) && card::is_faceup(d, ec) && card::is_relate_to_effect(d, ec, a.e) {
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_value(d, e1, 2);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, ec, e1, false);
        }
    }
}
