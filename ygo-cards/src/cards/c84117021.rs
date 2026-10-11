//! Spell Wall
//! 魔力隔壁
use super::prelude::*;

pub const ID: u32 = 84117021;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER)`
fn spellcaster_faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, RACE_SPELLCASTER)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local opt=0
    let mut opt: i64 = 0;
    // if Duel.IsExistingTarget(aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),tp,LOCATION_MZONE,0,1,nil) then
    if duel::is_existing_target(d, Some(&spellcaster_faceup), tp, LOCATION_MZONE, 0, 1, NO_EX) {
        // opt=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
        opt = duel::select_option(d, tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]) as i64;
    }
    // Duel.SetTargetParam(opt)
    duel::set_target_param(d, opt as u32);
    // if opt==0 then
    if opt == 0 {
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
    } else {
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsRace,RACE_SPELLCASTER),tp,LOCATION_MZONE,0,1,1,nil)
        duel::select_target(d, tp, Some(&spellcaster_faceup), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local opt=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let opt = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if opt==0 then
    if opt == 0 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_DISABLE_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_DISABLE_SUMMON);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_SET_AVAILABLE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_SET_AVAILABLE, 0);
        // e1:SetTarget(aux.TargetBoolFunction(Card.IsRace,RACE_SPELLCASTER))
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_SPELLCASTER))));
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CANNOT_DISABLE_SPSUMMON)
        effect::set_code(d, e2, EFFECT_CANNOT_DISABLE_SPSUMMON);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
        // local e3=e1:Clone()
        let e3 = effect::clone(d, e1);
        // e3:SetCode(EFFECT_CANNOT_DISABLE_FLIP_SUMMON)
        effect::set_code(d, e3, EFFECT_CANNOT_DISABLE_FLIP_SUMMON);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, tp);
        // local e4=Effect.CreateEffect(e:GetHandler())
        let e4 = effect::create_effect(d, h);
        // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e4:SetCode(EVENT_SUMMON_SUCCESS)
        effect::set_code(d, e4, EVENT_SUMMON_SUCCESS);
        // e4:SetCondition(s.sumcon)
        effect::set_condition(d, e4, Some(condition(sumcon)));
        // e4:SetOperation(s.sumsuc)
        effect::set_operation(d, e4, Some(operation(sumsuc)));
        // e4:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e4,tp)
        duel::register_effect(d, e4, tp);
        // local e5=e4:Clone()
        let e5 = effect::clone(d, e4);
        // e5:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
        // Duel.RegisterEffect(e5,tp)
        duel::register_effect(d, e5, tp);
        // local e6=e4:Clone()
        let e6 = effect::clone(d, e4);
        // e6:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
        effect::set_code(d, e6, EVENT_FLIP_SUMMON_SUCCESS);
        // Duel.RegisterEffect(e6,tp)
        duel::register_effect(d, e6, tp);
        // local e7=Effect.CreateEffect(e:GetHandler())
        let e7 = effect::create_effect(d, h);
        // e7:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e7:SetCode(EVENT_CHAIN_END)
        effect::set_code(d, e7, EVENT_CHAIN_END);
        // e7:SetOperation(s.cedop)
        effect::set_operation(d, e7, Some(operation(cedop)));
        // e7:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e7, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e7,tp)
        duel::register_effect(d, e7, tp);
    } else {
        // local tc=Duel.GetFirstTarget()
        let tc = duel::first_target(d).nil();
        // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, e) {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
            effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
            // e1:SetOperation(s.atkop)
            effect::set_operation(d, e1, Some(operation(atkop)));
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then
    if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CHANGE_DAMAGE)
        effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetTargetRange(0,1)
        effect::set_target_range(d, e1, 0, 1);
        // e1:SetValue(0)
        effect::set_value(d, e1, 0);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_NO_EFFECT_DAMAGE)
        effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
    }
}
/// `s.sumcon(e,tp,eg,ep,ev,re,r,rp)`
fn sumcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.filter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &spellcaster_faceup, 1, NO_EX)
}
/// `s.sumsuc(e,tp,eg,ep,ev,re,r,rp)`
fn sumsuc(d: &mut Duel, _a: &EffArgs) {
    // Duel.SetChainLimitTillChainEnd(s.chlimit)
    duel::set_chain_limit_till_chain_end(d, lua_fn(chlimit));
}
/// `s.cedop(e,tp,eg,ep,ev,re,r,rp)`
fn cedop(d: &mut Duel, _a: &EffArgs) {
    // local _,g=Duel.CheckEvent(EVENT_SPSUMMON_SUCCESS,true)
    let g = duel::check_event(d, EVENT_SPSUMMON_SUCCESS).and_then(|evt| evt.event_cards);
    // if g and g:IsExists(s.filter,1,nil) and Duel.CheckEvent(EVENT_SPSUMMON_SUCCESS) then
    if let Some(g) = g {
        if group::is_exists(d, &g, &spellcaster_faceup, 1, NO_EX) && duel::check_event(d, EVENT_SPSUMMON_SUCCESS).is_some() {
            // Duel.SetChainLimitTillChainEnd(s.chlimit)
            duel::set_chain_limit_till_chain_end(d, lua_fn(chlimit));
        }
    }
}
/// `s.chlimit(re,rp,tp)`: return rp==tp
fn chlimit(_d: &mut Duel, a: Args) -> bool {
    a.p(1) == a.p(2)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
