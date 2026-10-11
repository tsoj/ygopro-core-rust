//! Beelze Frog
use super::prelude::*;

pub const ID: u32 = 49522489;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[10456559], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, effect_card_value(val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsCode,c:GetControler(),LOCATION_GRAVE,0,nil,10456559)*300
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&|d, x| card::is_code(d, x, &[10456559])), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 300
}
