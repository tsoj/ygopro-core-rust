//! Forbidden Scripture
//! 禁じられた聖典
use super::prelude::*;

pub const ID: u32 = 54773234;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetAttackTarget()~=nil
    duel::get_attack_target(d).is_some()
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local a=Duel.GetAttacker()
    let attacker = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let defender = duel::get_attack_target(d);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetTargetRange(LOCATION_ONFIELD,LOCATION_ONFIELD)
    effect::set_target_range(d, e1, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // e1:SetTarget(s.distg)
    effect::set_target(d, e1, Some(effect_card_fn(distg)));
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // e2:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_DISABLE_TRAPMONSTER)
    effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e3, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, a.tp);
    // if a:IsRelateToBattle() then
    let atk = attacker.nil();
    if card::is_relate_to_battle(d, atk) {
        // local aa=a:GetTextAttack()
        let mut aa = card::get_text_attack(d, atk) as i64;
        // local ad=a:GetTextDefense()
        let mut ad = card::get_text_defense(d, atk) as i64;
        // if a:IsImmuneToEffect(e) then
        if card::is_immune_to_effect(d, atk, a.e) {
            // aa=a:GetBaseAttack()
            aa = card::get_base_attack(d, atk) as i64;
            // ad=a:GetBaseDefense()
            ad = card::get_base_defense(d, atk) as i64;
        }
        // if aa<0 then aa=0 end
        if aa < 0 {
            aa = 0;
        }
        // if ad<0 then ad=0 end
        if ad < 0 {
            ad = 0;
        }
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e4:SetRange(LOCATION_MZONE)
        effect::set_range(d, e4, LOCATION_MZONE);
        // e4:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e4, EFFECT_SET_ATTACK_FINAL);
        // e4:SetValue(aa)
        effect::set_value(d, e4, aa);
        // e4:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e4, RESET_PHASE | PHASE_DAMAGE, 1);
        // a:RegisterEffect(e4,true)
        card::register_effect(d, atk, e4, true);
        // local e5=Effect.CreateEffect(c)
        let e5 = effect::create_effect(d, c);
        // e5:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
        // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e5:SetRange(LOCATION_MZONE)
        effect::set_range(d, e5, LOCATION_MZONE);
        // e5:SetCode(EFFECT_SET_DEFENSE_FINAL)
        effect::set_code(d, e5, EFFECT_SET_DEFENSE_FINAL);
        // e5:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e5, RESET_PHASE | PHASE_DAMAGE, 1);
        // e5:SetValue(ad)
        effect::set_value(d, e5, ad);
        // a:RegisterEffect(e5,true)
        card::register_effect(d, atk, e5, true);
    }
    // if d and d:IsRelateToBattle() then
    if let Some(dfn) = defender {
        if card::is_relate_to_battle(d, dfn) {
            // local da=d:GetTextAttack()
            let mut da = card::get_text_attack(d, dfn) as i64;
            // local dd=d:GetTextDefense()
            let mut dd = card::get_text_defense(d, dfn) as i64;
            // if d:IsImmuneToEffect(e) then
            if card::is_immune_to_effect(d, dfn, a.e) {
                // da=d:GetBaseAttack()
                da = card::get_base_attack(d, dfn) as i64;
                // dd=d:GetBaseDefense()
                dd = card::get_base_defense(d, dfn) as i64;
            }
            // if da<0 then da=0 end
            if da < 0 {
                da = 0;
            }
            // if dd<0 then dd=0 end
            if dd < 0 {
                dd = 0;
            }
            // local e6=Effect.CreateEffect(c)
            let e6 = effect::create_effect(d, c);
            // e6:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
            // e6:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e6, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e6:SetRange(LOCATION_MZONE)
            effect::set_range(d, e6, LOCATION_MZONE);
            // e6:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e6, EFFECT_SET_ATTACK_FINAL);
            // e6:SetValue(da)
            effect::set_value(d, e6, da);
            // e6:SetReset(RESET_PHASE|PHASE_DAMAGE)
            effect::set_reset(d, e6, RESET_PHASE | PHASE_DAMAGE, 1);
            // d:RegisterEffect(e6,true)
            card::register_effect(d, dfn, e6, true);
            // local e7=Effect.CreateEffect(c)
            let e7 = effect::create_effect(d, c);
            // e7:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e7, EFFECT_TYPE_SINGLE);
            // e7:SetProperty(EFFECT_FLAG_SINGLE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e7, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e7:SetRange(LOCATION_MZONE)
            effect::set_range(d, e7, LOCATION_MZONE);
            // e7:SetCode(EFFECT_SET_DEFENSE_FINAL)
            effect::set_code(d, e7, EFFECT_SET_DEFENSE_FINAL);
            // e7:SetValue(dd)
            effect::set_value(d, e7, dd);
            // e7:SetReset(RESET_PHASE|PHASE_DAMAGE)
            effect::set_reset(d, e7, RESET_PHASE | PHASE_DAMAGE, 1);
            // d:RegisterEffect(e7,true)
            card::register_effect(d, dfn, e7, true);
        }
    }
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c~=e:GetHandler()
    Some(c) != effect::get_handler(d, e)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // if (loc&LOCATION_ONFIELD)~=0 then
    if (loc & (LOCATION_ONFIELD as i64)) != 0 {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
