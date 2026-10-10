//! フレムベル・マジカル
//! Flamvell Magician
use super::prelude::*;

pub const ID: u32 = 95621257;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALLY_OF_JUSTICE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gain 400 ATK if you control an "Ally of Justice" monster
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
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetValue(400)
    effect::set_value(d, e1, 400);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_ALLY_OF_JUSTICE),e:GetHandler():GetControler(),LOCATION_MZONE,0,1,nil)
    let c = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, c);
    duel::is_existing_matching_card(d, Some(&|d, cc| card::is_faceup(d, cc) && card::is_set_card(d, cc, &[SET_ALLY_OF_JUSTICE as u16])), p, LOCATION_MZONE, 0, 1, NO_EX)
}
