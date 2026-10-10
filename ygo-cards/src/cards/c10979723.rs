//! Amazoness Tiger
use super::prelude::*;

pub const ID: u32 = 10979723;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AMAZONESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // atkup
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_value_fn(d, e1, effect_card_value(val));
    card::register_effect(d, c, e1, false);
    // at limit
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    effect::set_value_fn(d, e2, effect_card_fn(atlimit));
    card::register_effect(d, c, e2, false);
}
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_AMAZONESS),c:GetControler(),LOCATION_MZONE,0,nil)*400
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, cc| card::is_set_card(d, cc, &[SET_AMAZONESS as u16]))), p, LOCATION_MZONE, 0, NO_EX) as i64 * 400
}
fn atlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_AMAZONESS) and c~=e:GetHandler()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_AMAZONESS as u16]) && effect::get_handler(d, e) != Some(c)
}
