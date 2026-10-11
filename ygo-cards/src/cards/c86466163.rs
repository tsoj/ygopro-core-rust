//! Satellarknight Capella
use super::prelude::*;

pub const ID: u32 = 86466163;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TELLARKNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Apply a "You can treat Level 4 or lower "tellarknight" monsters you control as Level 5 when Xyz Summoning using 3 or more monsters as Xyz Materials" effect
    let e1a = effect::create_effect(d, c);
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    effect::set_operation(d, e1a, Some(operation(effop)));
    card::register_effect(d, c, e1a, false);
    let e1ab = effect::clone(d, e1a);
    effect::set_code(d, e1ab, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e1ab, false);
    let e1ac = effect::clone(d, e1a);
    effect::set_code(d, e1ac, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e1ac, false);
}
fn effop(d: &mut Duel, a: &EffArgs) {
    // For the rest of this turn, you can treat Level 4 or lower "tellarknight" monsters you control as Level 5 when Xyz Summoning using 3 or more monsters as Xyz Materials
    let h = effect::get_handler(d, a.e).nil();
    let e1a = effect::create_effect(d, h);
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1a, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e1a, EFFECT_XYZ_LEVEL);
    effect::set_target_range(d, e1a, LOCATION_MZONE, 0);
    effect::set_target(d, e1a, Some(effect_card_fn(|d, _e, c| card::is_level_below(d, c, 4) && card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16]))));
    effect::set_value_fn(d, e1a, lua_fn(|d, a| {
        let c = a.c(1);
        0x50000 + card::get_level(d, c) as i64
    }));
    effect::set_reset(d, e1a, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1a, a.tp);
    let e1b = effect::clone(d, e1a);
    effect::set_code(d, e1b, EFFECT_SATELLARKNIGHT_CAPELLA);
    effect::set_label_object(d, e1b, Value::Effect(e1a));
    effect::set_value(d, e1b, 0x30003); // 0x1 >, 0x2 =, 0x4 <, value == last digit(s)
    duel::register_effect(d, e1b, a.tp);
}
