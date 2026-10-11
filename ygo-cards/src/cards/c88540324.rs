//! Salamangreat Claw
use super::prelude::*;

pub const ID: u32 = 88540324;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SALAMANGREAT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_SALAMANGREAT))
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, x, _e, _tp| card::is_set_card(d, x, &[SET_SALAMANGREAT as u16]))), None, None, None, None, None, None);
    // aux.EnableCheckReincarnation(c)
    aux::enable_check_reincarnation(d, c);
    // indes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // pierce
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_PIERCE)
    effect::set_code(d, e3, EFFECT_PIERCE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // multiattack
    // local e4=e1:Clone()
    let e4 = effect::clone(d, e1);
    // e4:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
    effect::set_code(d, e4, EFFECT_EXTRA_ATTACK_MONSTER);
    // e4:SetCondition(s.macon)
    effect::set_condition(d, e4, Some(condition(macon)));
    // e4:SetValue(function(e,c) return c:GetLink()-1 end)
    effect::set_value_fn(d, e4, effect_card_value(|d, _e, c| card::get_link(d, c) as i64 - 1));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.macon(e)`
fn macon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let c = card::get_equip_target(d, h).nil();
    // return c:IsSetCard(SET_SALAMANGREAT) and c:IsLinkMonster() and c:IsReincarnationSummoned()
    card::is_set_card(d, c, &[SET_SALAMANGREAT as u16]) && card::is_link_monster(d, c) && card::is_reincarnation_summoned(d, c)
}
