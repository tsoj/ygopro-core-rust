//! The Black Goat Laughs
use super::prelude::*;

pub const ID: u32 = 49299410;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Neither player can Special Summon monsters with the declared name, except from the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.decltg)
    effect::set_target(d, e1, Some(cost_target(decltg)));
    // e1:SetOperation(s.sumlimop)
    effect::set_operation(d, e1, Some(operation(sumlimop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Neither player can activate the effects on the field of monsters with the declared name
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMINGS_CHECK_MONSTER));
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.decltg)
    effect::set_target(d, e2, Some(cost_target(decltg)));
    // e2:SetOperation(s.efflimop)
    effect::set_operation(d, e2, Some(operation(efflimop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.decltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn decltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CODE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CODE as u64);
    // local code=Duel.AnnounceCard(tp,TYPE_MONSTER,OPCODE_ISTYPE)
    let code = duel::announce_card(d, a.tp, &[TYPE_MONSTER as u64, OPCODE_ISTYPE]);
    // Duel.SetTargetParam(code)
    duel::set_target_param(d, code as u32);
    // Duel.SetOperationInfo(0,CATEGORY_ANNOUNCE,nil,0,tp,ANNOUNCE_CARD_FILTER)
    duel::set_operation_info(d, 0, CATEGORY_ANNOUNCE, None, 0, a.tp, ANNOUNCE_CARD_FILTER as i32);
    true
}
/// `s.sumlimop(e,tp,eg,ep,ev,re,r,rp)`
fn sumlimop(d: &mut Duel, a: &EffArgs) {
    // local code=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let code = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetTarget(function(_,c) return c:IsOriginalCodeRule(code) and not c:IsLocation(LOCATION_GRAVE) end)
    effect::set_target(d, e1, Some(effect_card_fn(move |d, _e, c| card::is_original_code_rule(d, c, &[code]) && !card::is_location(d, c, LOCATION_GRAVE))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.efflimop(e,tp,eg,ep,ev,re,r,rp)`
fn efflimop(d: &mut Duel, a: &EffArgs) {
    // local code=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let code = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int() as u32;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetValue(function(_e,_re,_tp) return _re:GetHandler():IsOriginalCodeRule(code) and _re:IsMonsterEffect() and _re:GetActivateLocation()==LOCATION_MZONE end)
    effect::set_value_fn(
        d,
        e1,
        lua_fn(move |d, a| {
            let re = a.e(1);
            let rh = effect::get_handler(d, re).nil();
            card::is_original_code_rule(d, rh, &[code]) && effect::is_monster_effect(d, re) && effect::get_activate_location(d, re) == LOCATION_MZONE
        }),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
