//! Yuki-Onna, the Absolute Zero Mayakashi
//! scripted by CyberCatMan
use super::prelude::*;

pub const ID: u32 = 2645637;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // You can only control 1 "Yuki-Onna, the Absolute Zero Mayakashi"
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Link Summon procedure
    link::add_procedure(d, c, Some(mat_filter(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_ZOMBIE, sc, st, Some(tp)))), 2, None, None, None, None);
    // Negated any activated effects of your opponent's banished monsters
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_operation(d, e1, Some(operation(|d, a| {
        duel::negate_effect(d, a.ev as u8);
    })));
    card::register_effect(d, c, e1, false);
    // Change a monster's ATK to 0 and negate its effects
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DISABLE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 2, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    effect::set_target(d, e2, Some(cost_target(atktg)));
    effect::set_operation(d, e2, Some(operation(atkop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_condition(d, e3, Some(condition(atkchaincon)));
    card::register_effect(d, c, e3, false);
}
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_status(d, h, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && loc == LOCATION_REMOVED as i64
}
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &a.eg.clone().nil(), &|d, c| card::is_summon_location(d, c, LOCATION_GRAVE), 1, h.into())
}
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::has_non_zero_attack(d, chkc) && chkc != c;
    }
    if a.chk == Some(0) {
        return !card::has_flag_effect(d, c, ID, None)
            && duel::is_existing_target(d, Some(&|d, x| card::has_non_zero_attack(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    card::register_flag_effect(d, c, ID, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, x| card::has_non_zero_attack(d, x)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into());
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::get_attack(d, tc) > 0 {
        let c = effect::get_handler(d, a.e).nil();
        // Change its ATK to 0
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e1, 0);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
        if !card::is_attack(d, tc, &[0]) {
            return;
        }
        // Negate its effects
        card::negate_effects(d, tc, c, None, false, None);
    }
}
fn atkchaincon(d: &mut Duel, a: &EffArgs) -> bool {
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    loc == LOCATION_GRAVE as i64 && effect::is_monster_effect(d, a.re.nil())
}
