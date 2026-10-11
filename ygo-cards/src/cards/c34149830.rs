//! Desperate Tag
use super::prelude::*;

pub const ID: u32 = 34149830;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // no damage & spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e2, EVENT_PRE_DAMAGE_CALCULATE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tc=Duel.GetAttacker()
    // local bc=tc:GetBattleTarget()
    let mut tc = Some(duel::get_attacker(d).nil());
    let mut bc = card::get_battle_target(d, tc.nil());
    // if tc:IsControler(1-tp) then
    if card::is_controler(d, tc.nil(), 1 - tp) {
        // tc=Duel.GetAttackTarget()
        // bc=Duel.GetAttacker()
        tc = duel::get_attack_target(d);
        bc = duel::get_attacker(d);
    }
    // if tc and tc:IsControler(1-tp) then return false end
    if let Some(t) = tc {
        if card::is_controler(d, t, 1 - tp) {
            return false;
        }
    }
    // if not tc or not bc or not tc:IsPosition(POS_FACEUP_ATTACK) then return false end
    let (Some(tc), Some(bc)) = (tc, bc) else { return false };
    if !card::is_position(d, tc, POS_FACEUP_ATTACK as u32) {
        return false;
    }
    // if tc:IsHasEffect(EFFECT_INDESTRUCTABLE_BATTLE) then
    if !card::is_has_effect(d, tc, EFFECT_INDESTRUCTABLE_BATTLE, None).is_empty() {
        // local tcind={tc:GetCardEffect(EFFECT_INDESTRUCTABLE_BATTLE)}
        let tcind = card::get_card_effect(d, tc, EFFECT_INDESTRUCTABLE_BATTLE);
        // for i=1,#tcind do
        for te in tcind {
            // local f=te:GetValue()
            // if type(f)=='function' then
            match effect::get_value(d, te) {
                Value::Fn(f) => {
                    // if f(te,bc) then return false end
                    if f.call(d, &[Value::Effect(te), Value::Card(bc)]).truthy() {
                        return false;
                    }
                }
                // else return false end
                _ => return false,
            }
        }
    }
    // if Duel.GetAttacker()==tc and bc:IsDefensePos() then return false end
    if duel::get_attacker(d) == Some(tc) && card::is_defense_pos(d, bc) {
        return false;
    }
    // if bc:IsHasEffect(EFFECT_DEFENSE_ATTACK) then
    if !card::is_has_effect(d, bc, EFFECT_DEFENSE_ATTACK, None).is_empty() {
        // if bc:IsHasEffect(75372290) then
        if !card::is_has_effect(d, bc, 75372290, None).is_empty() {
            // return tc:GetAttack()<=bc:GetAttack()
            return card::get_attack(d, tc) <= card::get_attack(d, bc);
        }
        // return tc:GetAttack()<=bc:GetDefense()
        return card::get_attack(d, tc) <= card::get_defense(d, bc);
    }
    // return tc:GetAttack()<=bc:GetAttack()
    card::get_attack(d, tc) <= card::get_attack(d, bc)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_WARRIOR) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4) && card::is_race(d, c, RACE_WARRIOR) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return not e:GetHandler():IsStatus(STATUS_CHAINING)
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return !card::is_status(d, h, STATUS_CHAINING)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE_CAL)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE_CAL, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let e = a.e;
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g~=0 then
    if g.len() != 0 {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
