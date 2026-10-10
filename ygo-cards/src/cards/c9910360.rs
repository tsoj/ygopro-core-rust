//! D.3.S. Frog
//! ガエル・サンデス
use super::prelude::*;

pub const ID: u32 = 9910360;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[12538374], material_setcode: &[SET_FROG], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,false,false,84451804,3)
    fusion::add_proc_mix_n(d, c, false, false, vec![(fusion::Mat::Code(84451804), 3)]);
    // atk
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return Duel.GetMatchingGroupCount(Card.IsCode,c:GetControler(),LOCATION_GRAVE,0,nil,12538374)*500
    let tp = card::get_controler(d, c);
    duel::get_matching_group_count(d, Some(&|d, x| card::is_code(d, x, &[12538374])), tp, LOCATION_GRAVE, 0, NO_EX) as i64 * 500
}
