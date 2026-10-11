//! Fake Explosion
use super::prelude::*;

pub const ID: u32 = 41234315;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[89493368], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:GetFirst():IsControler(1-tp)
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    card::is_controler(d, tc, 1 - a.tp)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local a=Duel.GetAttacker()
    let atk = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // if a then
    if let Some(atk) = atk {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
        // a:RegisterEffect(e1)
        card::register_effect(d, atk, e1, false);
    }
    // if d then
    if let Some(at) = at {
        // local e2=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e2 = effect::create_effect(d, h);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
        // e2:SetValue(1)
        effect::set_value(d, e2, 1);
        // e2:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
        // d:RegisterEffect(e2)
        card::register_effect(d, at, e2, false);
    }
    // local e3=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e3 = effect::create_effect(d, h);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_BATTLED)
    effect::set_code(d, e3, EVENT_BATTLED);
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // e3:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(89493368) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[89493368]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let (e, tp) = (a.e, a.tp);
    let g = duel::select_matching_card(
        d,
        a.tp,
        Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp))),
        a.tp,
        LOCATION_HAND | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,true,true,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, true, true, POS_FACEUP as u8, None);
    }
}
