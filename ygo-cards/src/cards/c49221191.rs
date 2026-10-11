//! Number C32: Shark Drake Veiss
use super::prelude::*;

pub const ID: u32 = 49221191;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[65676461], xyz_number: Some(32), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WATER),4,4,s.ovfilter,aux.Stringid(id,0))
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, xyz, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, xyz, st, Some(tp)))),
        Some(4),
        4,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        None,
        None,
        false,
        None,
    );
    // Make the ATK/DEF of 1 face-up monster on the field become 0 until the end of your turn
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP|TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, Some(TIMING_DAMAGE_STEP | TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(function(e,tp) return Duel.GetLP(tp)<=1000 and aux.StatChangeDamageStepCondition() end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.AND(s.atkdefcost,Cost.DetachFromSelf(1)))
    effect::set_cost(d, e1, Some(cost::and(vec![cost_target(atkdefcost), cost::detach_from_self(cost::Count::N(1), None, None)])));
    // e1:SetTarget(s.atkdeftg)
    effect::set_target(d, e1, Some(cost_target(atkdeftg)));
    // e1:SetOperation(s.atkdefop)
    effect::set_operation(d, e1, Some(operation(atkdefop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsSummonCode(lc,SUMMON_TYPE_XYZ,tp,65676461) and c:IsFaceup()
    card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_XYZ as u64, tp, &[65676461]) && card::is_faceup(d, c)
}
/// `s.atkdefcostfilter(c)`
fn atkdefcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c)
    //     and Duel.IsExistingTarget(aux.OR(Card.HasNonZeroAttack,Card.HasNonZeroDefense),0,LOCATION_MZONE,LOCATION_MZONE,1,c)
    card::is_monster(d, c)
        && card::is_able_to_remove_as_cost(d, c, None)
        && aux::sp_elim_filter(d, c, false, false)
        && duel::is_existing_target(
            d,
            Some(&|d, c| card::has_non_zero_attack(d, c) || card::has_non_zero_defense(d, c)),
            0,
            LOCATION_MZONE,
            LOCATION_MZONE,
            1,
            c.into(),
        )
}
/// `s.atkdefcost(e,tp,...,chk)`
fn atkdefcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkdefcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&atkdefcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.atkdefcostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&atkdefcostfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.atkdeftg(e,tp,...,chk,chkc)`
fn atkdeftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and (chkc:HasNonZeroAttack() or chkc:HasNonZeroDefense()) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && (card::has_non_zero_attack(d, chkc) || card::has_non_zero_defense(d, chkc));
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.OR(Card.HasNonZeroAttack,Card.HasNonZeroDefense),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&|d, c| card::has_non_zero_attack(d, c) || card::has_non_zero_defense(d, c)),
            a.tp,
            LOCATION_MZONE,
            LOCATION_MZONE,
            1,
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,aux.OR(Card.HasNonZeroAttack,Card.HasNonZeroDefense),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(
        d,
        a.tp,
        Some(&|d, c| card::has_non_zero_attack(d, c) || card::has_non_zero_defense(d, c)),
        a.tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        1,
        false,
        NO_EX,
    );
    true
}
/// `s.atkdefop(e,tp,...)`
fn atkdefop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local ct=Duel.IsTurnPlayer(1-tp) and 2 or 1
        let ct = if duel::is_turn_player(d, 1 - a.tp) { 2 } else { 1 };
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESETS_STANDARD_PHASE_END,ct)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, ct);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e2, EFFECT_SET_DEFENSE_FINAL);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
/// `s.cond(e,tp)` (the activation condition of e1)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetLP(tp)<=1000 and aux.StatChangeDamageStepCondition()
    duel::get_lp(d, a.tp) <= 1000 && aux::stat_change_damage_step_condition(d)
}
