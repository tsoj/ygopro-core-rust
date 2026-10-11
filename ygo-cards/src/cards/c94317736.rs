//! Hidden Springs of the Far East
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 94317736;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Recover/Effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_RECOVER);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetProperty(EFFECT_FLAG_BOTH_SIDE+EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_BOTH_SIDE + EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e2:SetCondition(s.reccon)
    effect::set_condition(d, e2, Some(condition(reccon)));
    // e2:SetOperation(s.recop)
    effect::set_operation(d, e2, Some(operation(recop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.reccon(e,tp,eg,ep,ev,re,r,rp)`
fn reccon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_MAIN2)
    duel::is_phase(d, PHASE_MAIN2)
}
/// `s.recop(e,tp,eg,ep,ev,re,r,rp)`
fn recop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local p=Duel.GetTurnPlayer()
    let p = duel::get_turn_player(d);
    // if Duel.Recover(p,500,REASON_EFFECT)~=0 then
    if duel::recover(d, p, 500, REASON_EFFECT, false, None) != 0 {
        // Prevent negation of Normal/Special Summons
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_DISABLE_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_DISABLE_SUMMON);
        // e1:SetRange(LOCATION_FZONE)
        effect::set_range(d, e1, LOCATION_FZONE);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_SET_AVAILABLE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_SET_AVAILABLE, 0);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e1,p)
        duel::register_effect(d, e1, p);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_CANNOT_DISABLE_SPSUMMON)
        effect::set_code(d, e2, EFFECT_CANNOT_DISABLE_SPSUMMON);
        // Duel.RegisterEffect(e2,p)
        duel::register_effect(d, e2, p);
        // Cannot negate activation of Spell/Trap/Monster effects to SS a monster
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD);
        // e3:SetCode(EFFECT_CANNOT_INACTIVATE)
        effect::set_code(d, e3, EFFECT_CANNOT_INACTIVATE);
        // e3:SetRange(LOCATION_FZONE)
        effect::set_range(d, e3, LOCATION_FZONE);
        // e3:SetValue(s.efilter)
        effect::set_value_fn(d, e3, lua_fn(efilter));
        // e3:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e3,p)
        duel::register_effect(d, e3, p);
        // Set Spell/Traps cannot be destroyed or targeted
        // local e4=Effect.CreateEffect(c)
        let e4 = effect::create_effect(d, c);
        // e4:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e4, EFFECT_TYPE_FIELD);
        // e4:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
        effect::set_property(d, e4, EFFECT_FLAG_SET_AVAILABLE, 0);
        // e4:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
        effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_EFFECT);
        // e4:SetRange(LOCATION_FZONE)
        effect::set_range(d, e4, LOCATION_FZONE);
        // e4:SetTargetRange(LOCATION_SZONE,0)
        effect::set_target_range(d, e4, LOCATION_SZONE, 0);
        // e4:SetTarget(aux.TargetBoolFunction(Card.IsPosition,POS_FACEDOWN))
        effect::set_target(d, e4, Some(effect_card_fn(|d, _e, c| card::is_position(d, c, POS_FACEDOWN))));
        // e4:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
        // e4:SetValue(s.tgvalue)
        effect::set_value_fn(d, e4, lua_fn(tgvalue));
        // Duel.RegisterEffect(e4,p)
        duel::register_effect(d, e4, p);
        // local e5=e4:Clone()
        let e5 = effect::clone(d, e4);
        // e5:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
        effect::set_code(d, e5, EFFECT_CANNOT_BE_EFFECT_TARGET);
        // e5:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e5, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // Duel.RegisterEffect(e5,p)
        duel::register_effect(d, e5, p);
    }
}
/// `s.efilter(e,ct)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let ct = a.int(1) as u8;
    // local p=Duel.GetTurnPlayer()
    let p = duel::get_turn_player(d);
    // local te,tp=Duel.GetChainInfo(ct,CHAININFO_TRIGGERING_EFFECT,CHAININFO_TRIGGERING_PLAYER)
    let info = duel::get_chain_info(d, ct, &[CHAININFO_TRIGGERING_EFFECT, CHAININFO_TRIGGERING_PLAYER]);
    let te = info[0].as_effect();
    let tp = info[1].int() as u8;
    // return p==tp and te:IsHasCategory(CATEGORY_SPECIAL_SUMMON) and te:IsActiveType(TYPE_SPELL+TYPE_TRAP+TYPE_MONSTER)
    if p != tp {
        return false;
    }
    let te = te.nil();
    effect::is_has_category(d, te, CATEGORY_SPECIAL_SUMMON) && effect::is_active_type(d, te, TYPE_SPELL + TYPE_TRAP + TYPE_MONSTER)
}
/// `s.tgvalue(e,re,rp)`
fn tgvalue(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let rp = a.p(2);
    // return rp==1-e:GetHandlerPlayer()
    rp == 1 - effect::get_handler_player(d, e)
}
