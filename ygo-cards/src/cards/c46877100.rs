//! Scareclaw Acro
use super::prelude::*;

pub const ID: u32 = 46877100;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SCARECLAW], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon procedure
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_value_fn(d, e1, lua_fn(hspval));
    card::register_effect(d, c, e1, false);
    // Boost ATK
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_EMZONE, 0);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| sclawfilter(d, c))));
    effect::set_value_fn(d, e2, lua_fn(atkval));
    card::register_effect(d, c, e2, false);
}
/// `aux.FaceupFilter(Card.IsSetCard,SET_SCARECLAW)`
fn sclawfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_SCARECLAW as u16])
}
/// `s.hspval(e,c)`: `return 0,zone&ZONES_MMZ`
fn hspval(d: &mut Duel, a: Args) -> Value {
    let c = a.c(1);
    let mut zone: u32 = 0;
    let tp = card::get_controler(d, c);
    let lg = duel::get_matching_group(d, Some(&sclawfilter), tp, LOCATION_MZONE, 0, NO_EX);
    for tc in group::iter(d, &lg) {
        let left_right: u8 = if card::is_in_main_m_zone(d, tc, None) { 1 } else { 0 };
        zone |= card::get_column_zone(d, tc, LOCATION_MZONE, left_right, left_right, Some(tp));
    }
    Value::Multi(vec![Value::Int(0), Value::Int((zone & ZONES_MMZ) as i64)])
}
/// `s.atkval(e)`: `return Duel.GetMatchingGroupCount(Card.IsDefensePos,e:GetHandlerPlayer(),LOCATION_MZONE,0,nil)*300`
fn atkval(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&|d, c| card::is_defense_pos(d, c)), p, LOCATION_MZONE, 0, NO_EX) as i64 * 300
}
