//! Cross Counter
use super::prelude::*;

pub const ID: u32 = 37083210;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=Duel.GetAttackTarget()
    // return bc and bc:IsPosition(POS_FACEUP_DEFENSE) and bc:IsControler(tp) and Duel.GetAttacker():GetAttack()<bc:GetDefense()
    let Some(bc) = duel::get_attack_target(d) else {
        return false;
    };
    if !card::is_position(d, bc, POS_FACEUP_DEFENSE) || !card::is_controler(d, bc, a.tp) {
        return false;
    }
    let atk_card = duel::get_attacker(d).nil();
    let atk = card::get_attack(d, atk_card);
    let def = card::get_defense(d, bc);
    atk < def
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,Duel.GetAttacker(),1,tp,0)
    let atk = duel::get_attacker(d).map(CardSet::from_card);
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, atk, 1, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=Duel.GetAttackTarget()
    let bc = duel::get_attack_target(d).nil();
    // if bc:IsFaceup() and bc:IsRelateToBattle() then
    if card::is_faceup(d, bc) && card::is_relate_to_battle(d, bc) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
        effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
        // e1:SetCondition(function(e) return Duel.GetAttackTarget()==e:GetHandler() end)
        effect::set_condition(d, e1, Some(lua_fn(|d, a: Args| {
            duel::get_attack_target(d) == effect::get_handler(d, a.e(0))
        })));
        // e1:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
        effect::set_value_fn(d, e1, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        // bc:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
    }
    // local ac=Duel.GetAttacker()
    let ac = duel::get_attacker(d);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetOperation(function() if ac:IsRelateToBattle() then Duel.Destroy(ac,REASON_EFFECT) end end)
    effect::set_operation(d, e2, Some(lua_fn(move |d, _a: Args| {
        let ac_c = ac.nil();
        if card::is_relate_to_battle(d, ac_c) {
            duel::destroy(d, ac_c, REASON_EFFECT, None, None);
        }
    })));
    // e2:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
