//! Dogurad, the Stonetrooper
use super::prelude::*;

pub const ID: u32 = 42021064;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, including a Rock monster
    // Link.AddProcedure(c,nil,2,3,s.matcheck)
    link::add_procedure(d, c, None, 2, Some(3), Some(group_check(matcheck)), None, None);
    // Gains the ATK of 1 Rock monster in your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 other monster on the field with ATK less than or equal to this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsMainPhase() and Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matcheck(g,sc,sumtype,tp)`
fn matcheck(d: &mut Duel, g: &CardSet, sc: CardId, st: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsRace,1,nil,RACE_ROCK,sc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_race_ctx(d, c, RACE_ROCK, Some(sc), st, Some(tp)), 1, NO_EX)
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_ROCK) and c:GetAttack()>0
    card::is_race(d, c, RACE_ROCK) && card::get_attack(d, c) > 0
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.atkfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && atkfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&atkfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&atkfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc_opt = duel::first_target(d);
    // if c:IsFaceup() and c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        let tc = tc_opt.nil();
        if card::is_relate_to_effect(d, tc, a.e) {
            // This card gains ATK equal to that monster's
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(tc:GetAttack())
            let tc_atk = card::get_attack(d, tc) as i64;
            effect::set_value(d, e1, tc_atk);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
/// `s.desfilter(c,atk)`
fn desfilter(d: &mut Duel, c: CardId, atk: i64) -> bool {
    // return c:HasNonZeroAttack() and c:IsAttackBelow(atk)
    card::has_non_zero_attack(d, c) && card::is_attack_below(d, c, atk as i32)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local atk=c:GetAttack()
    let atk = card::get_attack(d, c) as i64;
    // if chkc then return chkc:IsOnField() and s.desfilter(chkc,atk) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && desfilter(d, chkc, atk) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,atk) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| desfilter(d, cc, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c,atk)
    let g = duel::select_target(d, a.tp, Some(&|d, cc| desfilter(d, cc, atk)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,c,1,tp,-g:GetFirst():GetAttack())
    let gf = group::get_first(d, &g).nil();
    let neg = -(card::get_attack(d, gf) as i64);
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(c)), 1, a.tp, neg as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc_opt = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc:IsRelateToEffect(e) and tc:HasNonZeroAttack()
    //     and c:IsAttackAbove(tc:GetAttack()) and c:UpdateAttack(-tc:GetAttack())==-tc:GetAttack() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        let tc = tc_opt.nil();
        if card::is_relate_to_effect(d, tc, a.e) && card::has_non_zero_attack(d, tc) {
            let atk = card::get_attack(d, tc) as i64;
            if card::is_attack_above(d, c, atk as i32) && card::update_attack(d, c, -atk, None, None, None) as i64 == -atk {
                // Duel.Destroy(tc,REASON_EFFECT)
                duel::destroy(d, tc, REASON_EFFECT, None, None);
            }
        }
    }
}
/// `function(e,tp) return Duel.IsMainPhase() and Duel.IsTurnPlayer(1-tp) end`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_main_phase(d, None) && duel::is_turn_player(d, 1 - a.tp)
}
