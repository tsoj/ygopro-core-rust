//! Fiend Skull Dragon
use super::prelude::*;

pub const ID: u32 = 66235877;

pub const INFO: ScriptInfo = ScriptInfo { material_setcode: &[SET_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    card::enable_revive_limit(d, c);
    fusion::add_proc_mix(d, c, false, false, vec![fusion::Mat::Code(93220472), fusion::Mat::Code(16475472)]);
    // Negate FLIP monster
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_DISABLE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // aux.TargetBoolFunction(Card.IsType,TYPE_FLIP)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_FLIP))));
    card::register_effect(d, c, e1, false);
    // Negate flip effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(disop)));
    card::register_effect(d, c, e2, false);
    // Summoner of Illusions interaction
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, ID);
    effect::set_range(d, e3, LOCATION_MZONE);
    card::register_effect(d, c, e3, false);
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    if effect::is_active_type(d, re, TYPE_FLIP) {
        duel::negate_effect(d, a.ev as u8);
    }
    let re = a.re.nil();
    if effect::is_trap_effect(d, re) && effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned();
        if let Some(g) = g {
            let c = effect::get_handler(d, a.e).nil();
            if group::is_contains(&g, c) {
                duel::negate_effect(d, a.ev as u8);
            }
        }
    }
}
