//! Berserking
use super::prelude::*;

pub const ID: u32 = 56535497;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Halve and increase ATK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, None);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.condition)
    effect::set_condition(d, e2, Some(condition(cond)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return (Duel.IsTurnPlayer(tp) and Duel.IsMainPhase())
    //   or (Duel.IsTurnPlayer(1-tp) and Duel.IsBattlePhase() and aux.StatChangeDamageStepCondition())
    (duel::is_turn_player(d, a.tp) && duel::is_main_phase(d, None))
        || (duel::is_turn_player(d, 1 - a.tp) && duel::is_battle_phase(d, None) && aux::stat_change_damage_step_condition(d))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsRace,RACE_BEAST),tp,LOCATION_MZONE,LOCATION_MZONE,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_BEAST))), tp, LOCATION_MZONE, LOCATION_MZONE, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsRace,RACE_BEAST),tp,LOCATION_MZONE,LOCATION_MZONE,2,2,nil)
    let _g = duel::select_target(d, tp, Some(&aux::faceup_filter(|d, c| card::is_race(d, c, RACE_BEAST))), tp, LOCATION_MZONE, LOCATION_MZONE, 2, 2, false, NO_EX);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tg=Duel.GetTargetCards(e):Match(Card.IsFaceup,nil)
    let mut tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    group::match_(d, &mut tg, &|d, x| card::is_faceup(d, x), NO_EX);
    // if #tg~=2 then return end
    if tg.len() != 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local tc1=tg:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &tg, tp, 1, 1, false, NO_EX).nil();
    let Some(tc1) = group::get_first(d, &sel) else {
        // if not tc1 then return end
        return;
    };
    // Duel.HintSelection(tc1,true)
    duel::hint_selection(d, tc1, true);
    // local tc2=(tg-tc1):GetFirst()
    let mut rest = tg.clone();
    group::remove_card(&mut rest, &CardSet::from_card(tc1));
    let tc2 = group::get_first(d, &rest).nil();
    // if tc1:IsImmuneToEffect(e) then return end
    if card::is_immune_to_effect(d, tc1, a.e) {
        return;
    }
    // Halve ATK
    // local atk=tc1:GetAttack()/2
    let atk = card::get_attack(d, tc1) as f64 / 2.0;
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
    effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
    // e1:SetValue(atk)
    effect::set_value(d, e1, lua_int(atk));
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc1:RegisterEffect(e1)
    card::register_effect(d, tc1, e1, false);
    // Increase ATK
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // tc2:RegisterEffect(e2)
    card::register_effect(d, tc2, e2, false);
}
