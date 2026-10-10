//! Muzurhythm the String Djinn
use super::prelude::*;

pub const ID: u32 = 26563200;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DJINN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // attack up
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_operation(d, e1, Some(operation(atkop)));
    card::register_effect(d, c, e1, false);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local ph=Duel.GetCurrentPhase()
    // if ph~=PHASE_DAMAGE or Duel.IsDamageCalculated() then return false end
    let ph = duel::get_current_phase(d);
    if ph != PHASE_DAMAGE || duel::is_damage_calculated(d) {
        return false;
    }
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d).nil();
    // return tc:IsControler(tp) and tc:IsRelateToBattle() and tc:IsSetCard(SET_DJINN) and tc:IsType(TYPE_XYZ) and Duel.GetAttackTarget()~=nil
    card::is_controler(d, tc, a.tp)
        && card::is_relate_to_battle(d, tc)
        && card::is_set_card(d, tc, &[SET_DJINN as u16])
        && card::is_type(d, tc, TYPE_XYZ)
        && duel::get_attack_target(d).is_some()
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d).nil();
    // if tc:IsRelateToBattle() and tc:IsFaceup() then
    if card::is_relate_to_battle(d, tc) && card::is_faceup(d, tc) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // e1:SetValue(tc:GetAttack()*2)
        let v = (card::get_attack(d, tc) * 2) as i64;
        effect::set_value(d, e1, v);
        card::register_effect(d, tc, e1, false);
    }
}
