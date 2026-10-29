//! Reaper Scythe - Dreadscythe
use super::prelude::*;

pub const ID: u32 = 81954378;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[18175965], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsCode,18175965))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, x, _e, _tp| card::is_code(d, x, &[18175965]))), None, None, None, None, None, None);
    // --atk
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e3, effect_value(value));
    card::register_effect(d, c, e3, false);
}
// function s.value(e,c)
fn value(d: &mut Duel, _e: EffectId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsMonster,0,LOCATION_GRAVE,LOCATION_GRAVE,nil)*500
    duel::get_matching_group_count(d, Some(&card::is_monster), 0, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX) as i64 * 500
}
