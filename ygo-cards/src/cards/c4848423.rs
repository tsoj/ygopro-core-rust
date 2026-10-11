//! Muscle Medic
//! 衛生兵マッスラー
use super::prelude::*;

pub const ID: u32 = 4848423;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // damage conversion
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_REVERSE_DAMAGE)
    effect::set_code(d, e1, EFFECT_REVERSE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetValue(s.rev)
    effect::set_value_fn(d, e1, lua_fn(rev));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rev(e,re,r,rp,rc)`
fn rev(d: &mut Duel, a: Args) -> bool {
    // return r&REASON_BATTLE~=0 and (Duel.GetAttacker()==e:GetHandler() or (Duel.GetAttackTarget() and Duel.GetAttackTarget()==e:GetHandler()))
    let r = a.int(2) as u32;
    let h = effect::get_handler(d, a.e(0)).nil();
    (r & REASON_BATTLE) != 0
        && (duel::get_attacker(d) == Some(h)
            || (duel::get_attack_target(d).is_some() && duel::get_attack_target(d) == Some(h)))
}
