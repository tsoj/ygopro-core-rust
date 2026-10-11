//! Fabled Urustos
use super::prelude::*;

pub const ID: u32 = 73040500;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FABLED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCondition(s.con)
    effect::set_condition(d, e1, Some(condition(con)));
    // e1:SetTarget(s.tg)
    effect::set_target(d, e1, Some(effect_card_fn(tg)));
    // e1:SetValue(400)
    effect::set_value(d, e1, 400);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.con(e)`
fn con(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(e:GetHandler():GetControler(),LOCATION_HAND,0)<=2
    let h = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, h);
    duel::get_field_group_count(d, p, LOCATION_HAND, 0) <= 2
}
/// `s.tg(e,c)`
fn tg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_FABLED)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_FABLED as u16])
}
