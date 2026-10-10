//! Amazoness Paladin
use super::prelude::*;

pub const ID: u32 = 47480070;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZONESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(val));
    card::register_effect(d, c, e1, false);
}
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.filter,c:GetControler(),LOCATION_MZONE,0,nil)*100
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&filter), p, LOCATION_MZONE, 0, NO_EX) as i64 * 100
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_AMAZONESS)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_AMAZONESS as u16])
}
