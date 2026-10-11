//! Angelica's Angelic Ring
use super::prelude::*;

pub const ID: u32 = 40678060;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip only to a monster you control that is equipped with an Equip Card
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, e, tp| eqfilter(d, c, e, tp))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // Can only control 1 "Angel Ring"
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Negate an opponent's activated Spell effect
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(condition(|d, a: &EffArgs| {
        a.rp == 1 - a.tp && effect::is_spell_effect(d, a.re.nil()) && duel::is_chain_disablable(d, a.ev as u8)
    })));
    effect::set_operation(d, e1, Some(operation(negate)));
    card::register_effect(d, c, e1, false);
    // Gain 500 LP, then destroy this card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_RECOVER + CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(|d, a: &EffArgs| {
        let c = effect::get_handler(d, a.e).nil();
        card::get_equip_target(d, c).is_some()
    })));
    effect::set_target(d, e2, Some(cost_target(lptg)));
    effect::set_operation(d, e2, Some(operation(lpop)));
    card::register_effect(d, c, e2, false);
}
fn negate(d: &mut Duel, a: &EffArgs) {
    duel::negate_effect(d, a.ev as u8);
}
fn eqfilter(d: &mut Duel, c: CardId, _e: EffectId, tp: u8) -> bool {
    card::is_controler(d, c, tp) && card::get_equip_count(d, c) > 0
}
fn lptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, 500);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, 500);
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(c.into()), 1, a.tp, 0);
    true
}
fn lpop(d: &mut Duel, a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let amount = info[1].int() as u32;
    if duel::recover(d, p, amount as i64, REASON_EFFECT, false, None) == 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) || duel::destroy(d, c, REASON_EFFECT, None, None) == 0 {
        return;
    }
    let tc = card::get_previous_equip_target(d, c).nil();
    if card::is_facedown(d, tc) {
        return;
    }
    // Opponent cannot target the monster this card was equipped to with card effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, 3061);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, lua_fn(|d, a: Args| {
        let e = a.e(0);
        let rp = a.p(2);
        rp == 1 - effect::get_owner_player(d, e)
    }));
    effect::set_owner_player(d, e1, a.tp);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, tc, e1, false);
}
