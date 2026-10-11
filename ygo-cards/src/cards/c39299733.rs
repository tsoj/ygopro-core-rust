//! Fortune Chariot
use super::prelude::*;

pub const ID: u32 = 39299733;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddUnionProcedure(c,aux.FilterBoolFunction(Card.IsRace,RACE_FAIRY),false)
    aux::add_union_procedure(d, c, Some(std::sync::Arc::new(|d: &mut Duel, cc: CardId| card::is_race(d, cc, RACE_FAIRY))), false, None);
    // Direct attack
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Damage reduce
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.rdcon)
    effect::set_condition(d, e2, Some(condition(rdcon)));
    // e2:SetValue(aux.ChangeBattleDamage(1,HALF_DAMAGE))
    effect::set_value_fn(d, e2, aux::change_battle_damage(1, HALF_DAMAGE as i64));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rdcon(e)`
fn rdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c,tp=e:GetHandler():GetEquipTarget(),e:GetHandlerPlayer()
    let h = effect::get_handler(d, a.e).nil();
    let c = card::get_equip_target(d, h);
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.GetAttackTarget()==nil and c:GetEffectCount(EFFECT_DIRECT_ATTACK)<2 and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    duel::get_attack_target(d).is_none()
        && card::get_effect_count(d, c.nil(), EFFECT_DIRECT_ATTACK) < 2
        && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
}
