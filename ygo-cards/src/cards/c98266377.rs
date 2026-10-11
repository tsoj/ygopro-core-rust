//! Elemental HERO Heat
use super::prelude::*;

pub const ID: u32 = 98266377;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ELEMENTAL_HERO],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, effect_card_value(val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_ELEMENTAL_HERO),c:GetControler(),LOCATION_MZONE,0,nil)*200
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&|d, x| card::is_faceup(d, x) && card::is_set_card(d, x, &[SET_ELEMENTAL_HERO as u16])), p, LOCATION_MZONE, 0, NO_EX) as i64 * 200
}
