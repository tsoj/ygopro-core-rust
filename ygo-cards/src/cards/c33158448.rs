//! F.A. Dawn Dragster
use super::prelude::*;

pub const ID: u32 = 33158448;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // ATK increase
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    card::register_effect(d, c, e1, false);
    // Increase level
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_LVCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_code(d, e3, EVENT_CHAINING);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_condition(d, e3, Some(condition(lvcon)));
    effect::set_operation(d, e3, Some(operation(lvop)));
    card::register_effect(d, c, e3, false);
    // Piercing damage
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_PIERCE);
    card::register_effect(d, c, e4, false);
    // Negate Spell/Trap Card or effect
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_NEGATE | CATEGORY_DESTROY);
    effect::set_type(d, e5, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e5, EVENT_CHAINING);
    effect::set_property(d, e5, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_range(d, e5, LOCATION_MZONE);
    effect::set_condition(d, e5, Some(condition(negcon)));
    effect::set_target(d, e5, Some(cost_target(negtg)));
    effect::set_operation(d, e5, Some(operation(negop)));
    card::register_effect(d, c, e5, false);
}
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    card::get_level(d, c) as i64 * 300
}
fn lvcon(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    effect::is_spell_trap_effect(d, re) && card::is_set_card(d, effect::get_handler(d, re).nil(), &[SET_FA as u16])
}
fn lvop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
        effect::set_value(d, e1, 1);
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
    }
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
        && (a.ep as i64) == 1 - (a.tp as i64)
        && effect::is_spell_trap_effect(d, re)
        && duel::is_chain_negatable(d, a.ev as u8)
}
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return card::get_level(d, effect::get_handler(d, a.e).nil()) > 2;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re_h = effect::get_handler(d, a.re.nil()).nil();
    if card::is_destructable(d, re_h, None) && card::is_relate_to_effect(d, re_h, a.re.nil()) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
fn negop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_facedown(d, c)
        && card::is_relate_to_effect(d, c, a.e)
        && !card::is_immune_to_effect(d, c, a.e)
        && card::update_level(d, c, -2, None, None) != 0
        && duel::negate_activation(d, a.ev as u8)
        && card::is_relate_to_effect(d, effect::get_handler(d, a.re.nil()).nil(), a.re.nil())
    {
        duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
    }
}
