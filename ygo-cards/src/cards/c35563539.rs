//! Mystical Refpanel
use super::prelude::*;

pub const ID: u32 = 35563539;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && effect::is_spell_effect(d, re) && effect::is_has_property(d, re, EFFECT_FLAG_PLAYER_TARGET, 0)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let te = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
        let ftg = effect::get_target(d, te);
        return match ftg {
            None => true,
            Some(ftg) => ftg
                .call(
                    d,
                    &[
                        Value::Effect(a.e),
                        Value::Int(a.tp as i64),
                        a.eg.clone().map_or(Value::Nil, Value::Group),
                        Value::Int(a.ep as i64),
                        Value::Int(a.ev as i64),
                        Value::effect(a.re),
                        Value::Int(a.r as i64),
                        Value::Int(a.rp as i64),
                        Value::Int(0),
                    ],
                )
                .truthy(),
        };
    }
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let p = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_PLAYER])[0].int();
    duel::change_target_player(d, a.ev as u8, (1 - p) as u8);
}
