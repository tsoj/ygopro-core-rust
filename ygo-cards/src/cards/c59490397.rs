//! Gergonne's End
use super::prelude::*;

pub const ID: u32 = 59490397;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TINDANGLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    card::register_effect(d, c, e1, false);
    // Destroy
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let c = effect::get_handler(d, a.e).nil();
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID])[0].int();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_REMAIN_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_OATH, 0);
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_DISABLED);
    effect::set_operation(d, e2, Some(operation(tgop)));
    effect::set_label(d, e2, &[cid]);
    effect::set_reset(d, e2, RESET_CHAIN, 1);
    duel::register_effect(d, e2, a.tp);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID])[0].int();
    if cid != effect::get_label(d, a.e) {
        return;
    }
    let owner = effect::get_owner(d, a.e).nil();
    card::cancel_to_grave(d, owner, false);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_link_monster(d, c) && card::is_set_card(d, c, &[SET_TINDANGLE as u16])
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    card::get_controler(d, c) == effect::get_handler_player(d, e) && card::is_link_monster(d, c) && card::is_set_card(d, c, &[SET_TINDANGLE as u16])
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_location(d, c, LOCATION_SZONE) {
        return;
    }
    if !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        duel::equip(d, tp, c, tc, true, false);
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
        effect::set_value(d, e2, 1);
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e2, false);
        let e3 = effect::clone(d, e2);
        effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
        card::register_effect(d, c, e3, false);
        let e4 = effect::clone(d, e2);
        effect::set_code(d, e4, EFFECT_CANNOT_BE_EFFECT_TARGET);
        effect::set_property(d, e4, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        effect::set_value_fn(d, e4, aux::tgoval());
        card::register_effect(d, c, e4, false);
    } else {
        card::cancel_to_grave(d, c, false);
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    match card::get_equip_target(d, h) {
        None => false,
        Some(tc) => card::get_linked_group_count(d, tc) == card::get_link(d, tc) as usize,
    }
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, h).nil();
    if a.chk == Some(0) {
        return card::get_linked_group_count(d, tc) > 0;
    }
    let mut lg = card::get_linked_group(d, tc);
    group::add_card(&mut lg, &CardSet::from_card(h));
    let n = lg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(lg), n, 0, 0);
    let atk = card::get_attack(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, c);
    if !card::is_relate_to_effect(d, c, a.e) || tc.is_none() {
        return;
    }
    let tc = tc.nil();
    let atk = card::get_attack(d, tc);
    let mut lg = card::get_linked_group(d, tc);
    group::add_card(&mut lg, &CardSet::from_card(c));
    let ct = lg.len();
    if ct > 0 {
        let dc = duel::destroy(d, lg, REASON_EFFECT, None, None);
        if dc == ct as i32 {
            duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
