//! Dinowrestler Martial Anga
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 11755663;

// s.listed_series={SET_DINOWRESTLER}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOWRESTLER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // battle end
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.bpcon)
    effect::set_condition(d, e1, Some(condition(bpcon)));
    // e1:SetCost(s.bpcost)
    effect::set_cost(d, e1, Some(cost_target(bpcost)));
    // e1:SetOperation(s.bpop)
    effect::set_operation(d, e1, Some(operation(bpop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_TRIGGER_O+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.sscon)
    effect::set_condition(d, e2, Some(condition(sscon)));
    // e2:SetTarget(s.sstg)
    effect::set_target(d, e2, Some(cost_target(sstg)));
    // e2:SetOperation(s.ssop)
    effect::set_operation(d, e2, Some(operation(ssop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.bpcon(e,tp,eg,ep,ev,re,r,rp)
fn bpcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // local b=a:GetBattleTarget()
    // if not b then return false end
    let Some(bt) = card::get_battle_target(d, att) else {
        return false;
    };
    // if a:IsControler(1-tp) then a,b=b,a end
    let (x, y) = if card::is_controler(d, att, 1 - tp) { (bt, att) } else { (att, bt) };
    // if a and b then
    // local dif=b:GetAttack()-a:GetAttack()
    let dif = card::get_attack(d, y) - card::get_attack(d, x);
    // return a:GetControler()~=b:GetControler() and a~=e:GetHandler()
    //     and a:IsSetCard(SET_DINOWRESTLER) and a:IsRelateToBattle()
    //     and Duel.GetAttackTarget()~=nil and dif>=0
    card::get_controler(d, x) != card::get_controler(d, y)
        && x != effect::get_handler(d, a.e).nil()
        && card::is_set_card(d, x, &[SET_DINOWRESTLER as u16])
        && card::is_relate_to_battle(d, x)
        && duel::get_attack_target(d).is_some()
        && dif >= 0
}
// function s.bpcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn bpcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAbleToGraveAsCost() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_able_to_grave_as_cost(d, h);
    }
    // Duel.SendtoGrave(e:GetHandler(),REASON_COST)
    let h = effect::get_handler(d, a.e).nil();
    duel::send_to_grave(d, h, REASON_COST, None, None);
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    true
}
// function s.bpop(e,tp,eg,ep,ev,re,r,rp,chk)
fn bpop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local a=Duel.GetAttacker()
    let att = duel::get_attacker(d).nil();
    // local b=a:GetBattleTarget()
    // if a:IsControler(1-tp) then a,b=b,a end
    let x = if card::is_controler(d, att, 1 - tp) { card::get_battle_target(d, att).nil() } else { att };
    // if a:IsRelateToBattle() and not a:IsImmuneToEffect(e) then
    if card::is_relate_to_battle(d, x) && !card::is_immune_to_effect(d, x, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
        // a:RegisterEffect(e1)
        card::register_effect(d, x, e1, false);
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e2, EVENT_DAMAGE_STEP_END);
    // e2:SetOperation(s.skop)
    effect::set_operation(d, e2, Some(operation(skop)));
    // e2:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
    // Duel.RegisterEffect(e2,Duel.GetTurnPlayer())
    let turn_p = duel::get_turn_player(d);
    duel::register_effect(d, e2, turn_p);
}
// function s.skop(e,tp,eg,ep,ev,re,r,rp)
fn skop(d: &mut Duel, _a: &EffArgs) {
    // local p=Duel.GetTurnPlayer()
    let p = duel::get_turn_player(d);
    // Duel.SkipPhase(p,PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
    duel::skip_phase(d, p, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
}
// function s.sscon(e,c)
fn sscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetLocationCount(e:GetHandler():GetControler(),LOCATION_MZONE)>0
    //     and Duel.GetFieldGroupCount(e:GetHandler():GetControler(),LOCATION_MZONE,0,nil)<Duel.GetFieldGroupCount(e:GetHandler():GetControler(),0,LOCATION_MZONE,nil)
    //     and e:GetHandler():GetFlagEffect(id)>0
    let h = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, h);
    duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::get_field_group_count(d, p, LOCATION_MZONE, 0) < duel::get_field_group_count(d, p, 0, LOCATION_MZONE)
        && card::get_flag_effect(d, h, ID) > 0
}
// function s.sstg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sstg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, h, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// function s.ssop(e,tp,eg,ep,ev,re,r,rp)
fn ssop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():IsRelateToEffect(e) then
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        // Duel.SpecialSummon(e:GetHandler(),0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, h, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
