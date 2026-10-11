//! Dragunity Angusticlavii
use super::prelude::*;

pub const ID: u32 = 88361177;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGUNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pierce
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e1, EFFECT_PIERCE);
    // e1:SetCondition(s.pcon)
    effect::set_condition(d, e1, Some(condition(pcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.pcon(e)`
fn pcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipGroup():IsExists(Card.IsSetCard,1,nil,SET_DRAGUNITY)
    let h = effect::get_handler(d, a.e).nil();
    let eg = card::get_equip_group(d, h);
    group::is_exists(d, &eg, &|d, c| card::is_set_card(d, c, &[SET_DRAGUNITY as u16]), 1, NO_EX)
}
