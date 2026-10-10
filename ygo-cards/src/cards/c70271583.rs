//! Karakuri Watchdog mdl 313 "Saizan"
use super::prelude::*;

pub const ID: u32 = 70271583;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_KARAKURI as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // must attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_MUST_ATTACK);
    card::register_effect(d, c, e1, false);
    // pos change
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_POSITION);
    effect::set_type(d, e3, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EVENT_BE_BATTLE_TARGET);
    effect::set_operation(d, e3, Some(operation(posop)));
    card::register_effect(d, c, e3, false);
    // atkup
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_type(d, e4, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_FIELD);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_category(d, e4, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    effect::set_code(d, e4, EVENT_BATTLE_DAMAGE);
    effect::set_condition(d, e4, Some(condition(atkcon)));
    effect::set_operation(d, e4, Some(operation(atkop)));
    card::register_effect(d, c, e4, false);
    // indes
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e5, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_condition(d, e5, Some(lua_fn(indcon)));
    effect::set_value(d, e5, 1);
    card::register_effect(d, c, e5, false);
}
fn posop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE,0,POS_FACEUP_ATTACK,0)
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(0), false);
    }
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==tp and (e:GetHandler()==Duel.GetAttacker() or e:GetHandler()==Duel.GetAttackTarget())
    let c = effect::get_handler(d, a.e).nil();
    a.ep == a.tp && (Some(c) == duel::get_attacker(d) || Some(c) == duel::get_attack_target(d))
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_KARAKURI)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_KARAKURI as u16])
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local tc=g:GetFirst()
    let _ = group::get_first(d, &g);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        effect::set_value(d, e1, 800);
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        card::register_effect(d, tc, e2, false);
    }
}
/// `function(e) return e:GetHandler():IsPosition(POS_FACEUP_ATTACK) end`
fn indcon(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    card::is_position(d, c, POS_FACEUP_ATTACK)
}
