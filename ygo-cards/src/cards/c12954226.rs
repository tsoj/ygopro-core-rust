//! Circle of the Fairies
use super::prelude::*;

pub const ID: u32 = 12954226;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon Procedure
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Additional Normal Summon for Insect or Plant monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_HAND|LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_HAND | LOCATION_MZONE, 0);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_INSECT|RACE_PLANT))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_INSECT | RACE_PLANT))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Inflict damage equal to half of the target and recover the same amount
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(aux.Stringid(id,1))
    effect::set_category(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE+CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_DAMAGE + CATEGORY_RECOVER);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e2:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e2:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.damcfilter(c,tp)
fn damcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // local rc=c:GetReasonCard()
    let rc = card::get_reason_card(d, c);
    // if not (c:IsLocation(LOCATION_GRAVE) and rc) then return false end
    if !(card::is_location(d, c, LOCATION_GRAVE) && rc.is_some()) {
        return false;
    }
    let Some(rc) = rc else { return false };
    // if c:IsPreviousControler(tp) then
    if card::is_previous_controler(d, c, tp) {
        // local prev_race=c:GetPreviousRaceOnField()
        let prev_race = card::get_previous_race_on_field(d, c);
        // return prev_race&RACE_INSECT>0 or prev_race&RACE_PLANT>0
        return prev_race & RACE_INSECT > 0 || prev_race & RACE_PLANT > 0;
    }
    // if rc:IsRelateToBattle() then
    if card::is_relate_to_battle(d, rc) {
        // return rc:IsControler(tp) and rc:IsRace(RACE_INSECT|RACE_PLANT)
        return card::is_controler(d, rc, tp) && card::is_race(d, rc, RACE_INSECT | RACE_PLANT);
    }
    // local prev_race=rc:GetPreviousRaceOnField()
    let prev_race = card::get_previous_race_on_field(d, rc);
    // return rc:IsPreviousControler(tp) and (prev_race&RACE_INSECT>0 or prev_race&RACE_PLANT>0)
    card::is_previous_controler(d, rc, tp) && (prev_race & RACE_INSECT > 0 || prev_race & RACE_PLANT > 0)
}
// function s.damcon(e,tp,eg,ep,ev,re,r,rp)
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.damcfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| damcfilter(d, c, tp), 1, NO_EX)
}
// function s.damtgfilter(c,e)
fn damtgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsCanBeEffectTarget(e) and c:GetPreviousAttackOnField()>0
    card::is_location(d, c, LOCATION_GRAVE)
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::get_previous_attack_on_field(d, c) > 0
}
// function s.damtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    // if chkc then return eg:IsContains(chkc) and s.damtgfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&eg, chkc) && damtgfilter(d, chkc, e);
    }
    // if chk==0 then return eg:IsExists(s.damtgfilter,1,nil,e) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| damtgfilter(d, c, e), 1, NO_EX);
    }
    // local tg=eg
    let mut tg = eg.clone();
    // if #eg>1 then
    if eg.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // tg=eg:FilterSelect(tp,s.damtgfilter,1,1,nil,e)
        tg = group::filter_select(d, &eg, tp, &|d, c| damtgfilter(d, c, e), 1, 1, false, NO_EX).nil();
    }
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // local value=tg:GetFirst():GetPreviousAttackOnField()/2
    let first = group::get_first(d, &tg).nil();
    let value = card::get_previous_attack_on_field(d, first) as f64 / 2.0;
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,value)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, lua_int(value) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,value)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, lua_int(value) as i32);
    true
}
// function s.damop(e,tp,eg,ep,ev,re,r,rp)
fn damop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // local value=Duel.Damage(1-tp,tc:GetPreviousAttackOnField()/2,REASON_EFFECT)
    let prev = card::get_previous_attack_on_field(d, tc);
    let value = duel::damage(d, 1 - tp, lua_int(prev as f64 / 2.0), REASON_EFFECT, false, None);
    // if value>0 then
    if value > 0 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Recover(tp,value,REASON_EFFECT)
        duel::recover(d, tp, value as i64, REASON_EFFECT, false, None);
    }
}
