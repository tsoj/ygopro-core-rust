//! Performapal Revue Dancer
use super::prelude::*;

pub const ID: u32 = 36527535;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_PERFORMAPAL as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon rule
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.sprcon)
    effect::set_condition(d, e1, Some(lua_fn(sprcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // double tribute
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DOUBLE_TRIBUTE)
    effect::set_code(d, e2, EFFECT_DOUBLE_TRIBUTE);
    // e2:SetValue(s.dtcon)
    effect::set_value_fn(d, e2, effect_card_fn(dtcon));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sprcon(e,c)`
fn sprcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
        && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
/// `s.dtcon(e,c)`
fn dtcon(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_PERFORMAPAL)
    card::is_set_card(d, c, &[SET_PERFORMAPAL as u16])
}
