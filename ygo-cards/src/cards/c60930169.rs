//! Broken Blocker
use super::prelude::*;

pub const ID: u32 = 60930169;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    //     local ge1=Effect.CreateEffect(c)
    //     ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    //     ge1:SetCode(EVENT_BATTLED)
    //     ge1:SetOperation(s.checkop)
    //     Duel.RegisterEffect(ge1,0)
    // end)
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_BATTLED);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkop(e,tp,...)`
fn checkop(d: &mut Duel, _a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let a = duel::get_attacker(d);
    // local t=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // if a and a:IsDefensePos() and a:GetDefense()>a:GetAttack() and a:IsStatus(STATUS_BATTLE_DESTROYED) then
    //     a:RegisterFlagEffect(id,RESET_PHASE|PHASE_DAMAGE,0,1)
    // end
    if let Some(a) = a {
        if card::is_defense_pos(d, a) && card::get_defense(d, a) > card::get_attack(d, a) && card::is_status(d, a, STATUS_BATTLE_DESTROYED) {
            card::register_flag_effect(d, a, ID, RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
        }
    }
    // if t and t:IsDefensePos() and t:GetDefense()>t:GetAttack() and t:IsStatus(STATUS_BATTLE_DESTROYED) then
    //     t:RegisterFlagEffect(id,RESET_PHASE|PHASE_DAMAGE,0,1)
    // end
    if let Some(t) = t {
        if card::is_defense_pos(d, t) && card::get_defense(d, t) > card::get_attack(d, t) && card::is_status(d, t, STATUS_BATTLE_DESTROYED) {
            card::register_flag_effect(d, t, ID, RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
        }
    }
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:GetFlagEffect(id)~=0 and c:IsPreviousControler(tp) then
    if card::get_flag_effect(d, c, ID) != 0 && card::is_previous_controler(d, c, tp) {
        // e:SetLabel(c:GetCode())
        let code = card::get_code(d, c)[0];
        effect::set_label(d, e, &[code as i64]);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.filter,1,nil,e,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| filter(d, c, a.e, a.tp), 1, NO_EX)
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,e:GetLabel()) end
    if a.chk == Some(0) {
        let code = effect::get_label(d, a.e) as u32;
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp, code)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if ft>2 then ft=2 end
    if ft > 2 {
        ft = 2;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,ft,nil,e,tp,e:GetLabel())
    let code = effect::get_label(d, a.e) as u32;
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, code)), tp, LOCATION_DECK, 0, 1, ft as u16, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
