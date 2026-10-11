//! Heroic Retribution Sword
use super::prelude::*;

pub const ID: u32 = 49551909;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HEROIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_BATTLE_PHASE, None);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_HEROIC as u16])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(h.into()), 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_location(d, c, LOCATION_SZONE) || !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            duel::equip(d, tp, c, tc, true, false);
            // Destroy after damage calculation
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
            effect::set_category(d, e1, CATEGORY_DESTROY);
            effect::set_description(d, e1, aux::stringid(ID, 0));
            effect::set_code(d, e1, EVENT_BATTLED);
            effect::set_range(d, e1, LOCATION_SZONE);
            effect::set_condition(d, e1, Some(condition(descon)));
            effect::set_target(d, e1, Some(cost_target(destg)));
            effect::set_operation(d, e1, Some(operation(desop)));
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, c, e1, false);
            // Battle damage is shared
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
            effect::set_code(d, e2, EFFECT_ALSO_BATTLE_DAMAGE);
            card::register_effect(d, c, e2, false);
            // Equip limit
            let e3 = effect::create_effect(d, c);
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e3, EFFECT_EQUIP_LIMIT);
            effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_value_fn(d, e3, effect_card_fn(|d, e, c| eqlimit(d, e, c)));
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, c, e3, false);
            return;
        }
    }
    card::cancel_to_grave(d, c, false);
}
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let owner = effect::get_owner_player(d, e);
    card::get_controler(d, c) == owner && card::is_set_card(d, c, &[SET_HEROIC as u16])
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    let at = duel::get_attack_target(d);
    if at == ec {
        return true;
    }
    duel::get_attacker(d) == ec && at.is_some()
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let h = effect::get_handler(d, a.e).nil();
    let et = card::get_equip_target(d, h).nil();
    let bt = card::get_battle_target(d, et);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, bt.map(|x| x.into()), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let et = card::get_equip_target(d, h).nil();
    let bc = card::get_battle_target(d, et);
    if let Some(bc) = bc {
        if card::is_relate_to_battle(d, bc) {
            duel::destroy(d, bc, REASON_EFFECT, None, None);
        }
    }
}
