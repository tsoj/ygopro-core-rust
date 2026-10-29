//! Batteryman AA
//! 電池メン－単三型
use super::prelude::*;

pub const ID: u32 = 63142001;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[63142001], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If all "Batteryman AA"(s) on your side of the field are in Attack Position, this card gains 1000 ATK for each "Batteryman AA" on your side of the field
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE);
    // e1a:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1a, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1a:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetCondition(s.atkdefcon(Card.IsAttackPos))
    effect::set_condition(d, e1a, Some(lua_fn(|d, a| atkdefcon(d, a.e(0), true))));
    // e1a:SetValue(function(e,c) return 1000*Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsCode,id),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil) end)
    effect::set_value_fn(d, e1a, effect_value(atkval));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // If all "Batteryman AA"(s) on your side of the field are in Defense Position, this card gains 1000 DEF for each "Batteryman AA" on your side of the field
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
    // e1b:SetCondition(s.atkdefcon(Card.IsDefensePos))
    effect::set_condition(d, e1b, Some(lua_fn(|d, a| atkdefcon(d, a.e(0), false))));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
}
/// `function(e,c) ... end` of `s.atkdefcon(pos_function)`: `pos_attack` is `Card.IsAttackPos` (else `Card.IsDefensePos`)
fn atkdefcon(d: &mut Duel, e: EffectId, pos_attack: bool) -> bool {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsCode,id),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil)
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[ID])), p, LOCATION_MZONE, 0, NO_EX);
    // return #g>0 and g:FilterCount(pos_function,nil)==#g
    if g.is_empty() {
        return false;
    }
    let n = if pos_attack {
        group::filter_count(d, &g, &|d, x| card::is_attack_pos(d, x), NO_EX)
    } else {
        group::filter_count(d, &g, &|d, x| card::is_defense_pos(d, x), NO_EX)
    };
    n == g.len()
}
/// `function(e,c) return 1000*Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsCode,id),e:GetHandlerPlayer(),LOCATION_MZONE,0,nil) end`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    let p = effect::get_handler_player(d, e);
    1000 * duel::get_matching_group_count(d, Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[ID])), p, LOCATION_MZONE, 0, NO_EX) as i64
}
