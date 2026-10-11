//! Ultimate Tyranno
use super::prelude::*;

pub const ID: u32 = 15894048;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(catg)));
    effect::set_condition(d, e1, Some(condition(cacon)));
    card::register_effect(d, c, e1, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_ATTACK_ALL);
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
}
/// `s.catg(e,c)`: `return not c:IsCode(id)`
fn catg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_code(d, c, &[ID])
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    if !card::is_faceup(d, c) || !card::is_code(d, c, &[ID]) || !card::can_attack(d, c) {
        return false;
    }
    let (ag, direct) = card::get_attackable_target(d, c);
    group::get_count(&ag) > 0 || direct
}
/// `s.cacon(e)`
fn cacon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_battle_phase(d, None) && duel::is_existing_matching_card(d, Some(&cfilter), p, LOCATION_MZONE, 0, 1, NO_EX)
}
