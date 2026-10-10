//! Scarlight Red Dragon Archfiend
use super::prelude::*;

pub const ID: u32 = 80666118;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RED_DRAGON_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon Procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Name becomes "Red Dragon Archfiend"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetValue(CARD_RED_DRAGON_ARCHFIEND)
    effect::set_value(d, e1, CARD_RED_DRAGON_ARCHFIEND as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy monsters and inflict damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,atk)`
fn filter(d: &mut Duel, c: CardId, atk: i32) -> bool {
    // return c:IsFaceup() and c:IsAttackBelow(atk) and (c:GetSummonType()&SUMMON_TYPE_SPECIAL)==SUMMON_TYPE_SPECIAL
    //     and c:IsType(TYPE_EFFECT)
    card::is_faceup(d, c)
        && card::is_attack_below(d, c, atk)
        && (card::get_summon_type(d, c) & SUMMON_TYPE_SPECIAL as u32) == SUMMON_TYPE_SPECIAL as u32
        && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,c:GetAttack()) end
    if a.chk == Some(0) {
        let atk = card::get_attack(d, c);
        return duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,c,c:GetAttack())
    let atk = card::get_attack(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| filter(d, x, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#g*500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, n as i32 * 500);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,c,c:GetAttack())
    let atk = card::get_attack(d, c);
    let g = duel::get_matching_group(d, Some(&|d, x| filter(d, x, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, c.into());
    // local ct=Duel.Destroy(g,REASON_EFFECT)
    let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    // if ct>0 then
    if ct > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Damage(1-tp,ct*500,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, ct as i64 * 500, REASON_EFFECT, false, None);
    }
}
