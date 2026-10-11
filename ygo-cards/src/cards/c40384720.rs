//! Sonic Shooter
//! ソニック・シューター
use super::prelude::*;

pub const ID: u32 = 40384720;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Direct attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetCondition(s.dircon)
    effect::set_condition(d, e1, Some(lua_fn(dircon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Reduce damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetCondition(s.rdcon)
    effect::set_condition(d, e2, Some(condition(rdcon)));
    // e2:SetValue(s.rdval)
    effect::set_value_fn(d, e2, lua_fn(rdval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.dfilter(c)`
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()<5
    card::get_sequence(d, c) < 5
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: Args) -> bool {
    // return not Duel.IsExistingMatchingCard(s.dfilter,e:GetHandlerPlayer(),0,LOCATION_SZONE,1,nil)
    let p = effect::get_handler_player(d, a.e(0));
    !duel::is_existing_matching_card(d, Some(&dfilter), p, 0, LOCATION_SZONE, 1, NO_EX)
}
/// `s.rdcon(e,tp,eg,ep,ev,re,r,rp)`
fn rdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c,tp=e:GetHandler(),e:GetHandlerPlayer()
    let c = effect::get_handler(d, a.e).nil();
    let tp = effect::get_handler_player(d, a.e);
    // return Duel.GetAttackTarget()==nil and c:GetEffectCount(EFFECT_DIRECT_ATTACK)<2
    //     and Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)>0
    duel::get_attack_target(d).is_none()
        && card::get_effect_count(d, c, EFFECT_DIRECT_ATTACK) < 2
        && duel::get_field_group_count(d, tp, 0, LOCATION_MZONE) > 0
}
/// `s.rdval(e,damp)`
fn rdval(d: &mut Duel, a: Args) -> i64 {
    // if damp==1-e:GetHandlerPlayer() then
    let damp = a.int(1);
    if damp == 1 - effect::get_handler_player(d, a.e(0)) as i64 {
        // return e:GetHandler():GetBaseAttack()
        let h = effect::get_handler(d, a.e(0)).nil();
        card::get_base_attack(d, h) as i64
    } else {
        // return -1
        -1
    }
}
