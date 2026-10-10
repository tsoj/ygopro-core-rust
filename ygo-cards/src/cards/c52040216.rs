//! Harpie's Pet Dragon
use super::prelude::*;

pub const ID: u32 = 52040216;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_HARPIE_LADY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // atkup
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.val)
    effect::set_value_fn(d, e1, effect_card_value(val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.val(e,c)`
fn val(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.filter,c:GetControler(),LOCATION_ONFIELD,LOCATION_ONFIELD,nil)*300
    let p = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&filter), p, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX) as i64 * 300
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_HARPIE_LADY)
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_HARPIE_LADY])
}
