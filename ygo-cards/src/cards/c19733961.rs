//! Batteryman C
use super::prelude::*;

pub const ID: u32 = 19733961;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If all "Batteryman C"(s) you control are in Attack Position, all Machine-Type monsters you control gain 500 ATK for each "Batteryman C" you control
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    // e1a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1a, LOCATION_MZONE as u32, 0);
    // e1a:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_MACHINE))
    effect::set_target(d, e1a, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_MACHINE as u64))));
    // e1a:SetCondition(s.atkdefcon(Card.IsAttackPos))
    effect::set_condition(d, e1a, Some(condition(atkcon)));
    // e1a:SetValue(function(e,c) return 500*Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsCode,id),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil) end)
    effect::set_value_fn(d, e1a, effect_value(valfn));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // If all "Batteryman C"(s) you control are in Defense Position, all Machine-Type monsters you control gain 500 DEF for each "Batteryman C" you control
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
    // e1b:SetCondition(s.atkdefcon(Card.IsDefensePos))
    effect::set_condition(d, e1b, Some(condition(defcon)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
}
/// `s.atkdefcon(Card.IsAttackPos)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    atkdefcon(d, a, &|d, c| card::is_attack_pos(d, c))
}
/// `s.atkdefcon(Card.IsDefensePos)`
fn defcon(d: &mut Duel, a: &EffArgs) -> bool {
    atkdefcon(d, a, &|d, c| card::is_defense_pos(d, c))
}
/// `function(e,c) local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCode,id),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil)
///  return #g>0 and g:FilterCount(pos_function,nil)==#g end`
fn atkdefcon(d: &mut Duel, a: &EffArgs, pos_function: &dyn Fn(&mut Duel, CardId) -> bool) -> bool {
    let hp = effect::get_handler_player(d, a.e);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[ID])), hp, LOCATION_MZONE, 0, NO_EX);
    !g.is_empty() && group::filter_count(d, &g, &|d, c| pos_function(d, c), NO_EX) == g.len()
}
/// `function(e,c) return 500*Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsCode,id),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil) end`
fn valfn(d: &mut Duel, e: EffectId) -> i64 {
    let hp = effect::get_handler_player(d, e);
    500 * duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[ID])), hp, LOCATION_MZONE, 0, NO_EX) as i64
}
