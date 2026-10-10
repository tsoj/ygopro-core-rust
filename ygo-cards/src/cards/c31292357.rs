//! Performage Hat Tricker
use super::prelude::*;

pub const ID: u32 = 31292357;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x36], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, 0x36, Some(LOCATION_MZONE), None);
    card::set_counter_limit(d, c, 0x36, 3);
    // special summon rule
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    card::register_effect(d, c, e1, false);
    // counter
    let e2 = effect::create_effect(d, c);
    effect::set_category(d, e2, CATEGORY_COUNTER);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(aux::damcon1()));
    effect::set_target(d, e2, Some(cost_target(cttg)));
    effect::set_operation(d, e2, Some(operation(ctop)));
    card::register_effect(d, c, e2, false);
    // atk
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_ADD_COUNTER + 0x36);
    effect::set_condition(d, e3, Some(condition(atkcon)));
    effect::set_operation(d, e3, Some(operation(atkop)));
    card::register_effect(d, c, e3, false);
}
/// `function(e,c)`: `c == nil` only when the core checks the effect
fn spcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return true };
    let tp = card::get_controler(d, c);
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && duel::get_field_group_count(d, tp, LOCATION_MZONE, LOCATION_MZONE) >= 2
}
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return card::is_can_add_counter(d, c, 0x36, 1, false, 0);
    }
    true
}
fn ctop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::add_counter(d, c, 0x36, 1, false) {
        let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID])[0].int();
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_target_range(d, e1, 1, 0);
        effect::set_label(d, e1, &[cid]);
        effect::set_value_fn(d, e1, lua_fn(damval));
        effect::set_reset(d, e1, RESET_CHAIN, 1);
        duel::register_effect(d, e1, a.tp);
    }
}
/// `function(e,re,val,r,rp,rc)`
fn damval(d: &mut Duel, a: Args) -> i64 {
    let (e, val, r) = (a.e(0), a.int(2), a.int(3));
    let cc = duel::get_current_chain(d, false);
    if cc == 0 || (r & REASON_EFFECT as i64) == 0 {
        return val;
    }
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID])[0].int();
    if cid != effect::get_label(d, e) {
        return val;
    }
    0
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::get_counter(d, c, 0x36) == 3
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    effect::set_value(d, e1, 3300);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
    card::register_effect(d, c, e2, false);
}
