//! Master Peace, the True Dracoverlord
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 12800564;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Requires 3 Tributes to Normal Summon (cannot be Normal Set)
    // local e0a=aux.AddNormalSummonProcedure(c,true,false,3,3,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0))
    let _e0a = aux::add_normal_summon_procedure(d, c, true, false, Some(3), Some(3), Some(SUMMON_TYPE_TRIBUTE as u32), Some(aux::stringid(ID, 0)), None, None);
    // local e0b=aux.AddNormalSetProcedure(c)
    let _e0b = aux::add_normal_set_procedure(d, c, false, false, None, None, None, None, None, None);
    // To Tribute Summon this card face-up, you can Tribute Continuous Spells/Traps you control, as well as monsters
    // local e0c=Effect.CreateEffect(c)
    let e0c = effect::create_effect(d, c);
    // e0c:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0c, EFFECT_TYPE_SINGLE);
    // e0c:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0c, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0c:SetCode(EFFECT_ADD_EXTRA_TRIBUTE)
    effect::set_code(d, e0c, EFFECT_ADD_EXTRA_TRIBUTE);
    // e0c:SetTargetRange(LOCATION_SZONE,0)
    effect::set_target_range(d, e0c, LOCATION_SZONE, 0);
    // e0c:SetTarget(aux.TargetBoolFunction(Card.IsContinuousSpellTrap))
    effect::set_target(d, e0c, Some(effect_card_fn(|d, _e, c| card::is_continuous_spell_trap(d, c))));
    // e0c:SetValue(POS_FACEUP)
    effect::set_value(d, e0c, POS_FACEUP as i64);
    // c:RegisterEffect(e0c)
    card::register_effect(d, c, e0c, false);
    // Negate the activation of an opponent's monster effect activated in the hand or field, and if you do, destroy that card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Skip your opponent's next Main Phase
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.skipcon)
    effect::set_condition(d, e2, Some(condition(skipcon)));
    // e2:SetOperation(s.skipop)
    effect::set_operation(d, e2, Some(operation(skipop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.negcon(e,tp,eg,ep,ev,re,r,rp,chk)
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local trig_loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let trig_loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION]).first().map_or(0, |v| v.int()) as u32;
    // return ep==1-tp and re:IsMonsterEffect() and trig_loc&(LOCATION_HAND|LOCATION_MZONE)>0 and c:IsTributeSummoned()
    //     and not c:IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)
    a.ep == 1 - a.tp
        && effect::is_monster_effect(d, a.re.nil())
        && (trig_loc & (LOCATION_HAND | LOCATION_MZONE)) > 0
        && card::is_tribute_summoned(d, c)
        && !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
        && duel::is_chain_negatable(d, a.ev as u8)
}
// function s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
    }
    true
}
// function s.negop(e,tp,eg,ep,ev,re,r,rp)
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rc, a.re.nil()) {
            // Duel.Destroy(eg,REASON_EFFECT)
            let eg = a.eg.clone().nil();
            duel::destroy(d, eg, REASON_EFFECT, None, None);
        }
    }
}
// function s.skipcon(e,tp,eg,ep,ev,re,r,rp)
fn skipcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and rp==1-tp)) and c:IsTributeSummoned()
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    (card::is_reason(d, c, REASON_BATTLE) || (card::is_reason(d, c, REASON_EFFECT) && a.rp == 1 - a.tp))
        && card::is_tribute_summoned(d, c)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, a.tp)
}
// function s.skipop(e,tp,eg,ep,ev,re,r,rp)
fn skipop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local oppo_main1_flag=nil
    let mut oppo_main1_flag = false;
    // local effect_code=0
    let effect_code: u32;
    // local turn_count=Duel.GetTurnCount()
    let turn_count = duel::get_turn_count(d, None);
    // local current_phase=Duel.GetCurrentPhase()
    let current_phase = duel::get_current_phase(d);
    // local reset_count=(Duel.IsTurnPlayer(1-tp) and (current_phase==PHASE_MAIN1 or current_phase>=PHASE_MAIN2)) and 2 or 1
    let reset_count: u8 = if duel::is_turn_player(d, 1 - tp) && (current_phase == PHASE_MAIN1 || current_phase >= PHASE_MAIN2) { 2 } else { 1 };
    // if Duel.IsTurnPlayer(tp) or current_phase<=PHASE_STANDBY or current_phase>=PHASE_MAIN2 then
    if duel::is_turn_player(d, tp) || current_phase <= PHASE_STANDBY || current_phase >= PHASE_MAIN2 {
        // effect_code=EFFECT_SKIP_M1
        effect_code = EFFECT_SKIP_M1;
    } else if duel::is_turn_player(d, 1 - tp) && duel::is_battle_phase(d, None) {
        // effect_code=EFFECT_SKIP_M2
        effect_code = EFFECT_SKIP_M2;
    } else {
        // oppo_main1_flag=true
        oppo_main1_flag = true;
        // effect_code=EFFECT_SKIP_M1
        effect_code = EFFECT_SKIP_M1;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(effect_code)
    effect::set_code(d, e1, effect_code);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetCondition(function() return not oppo_main1_flag or turn_count~=Duel.GetTurnCount() end)
    effect::set_condition(
        d,
        e1,
        Some(lua_fn(move |d: &mut Duel, _a: Args| -> bool { !oppo_main1_flag || turn_count != duel::get_turn_count(d, None) })),
    );
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,reset_count)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, reset_count as u16);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // if oppo_main1_flag then
    if oppo_main1_flag {
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_SKIP_M2)
        effect::set_code(d, e2, EFFECT_SKIP_M2);
        // e2:SetCondition(aux.TRUE)
        effect::set_condition(d, e2, Some(lua_fn(|_d: &mut Duel, _a: Args| -> bool { true })));
        // e2:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN,1)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, tp);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e3:SetCode(EVENT_PHASE+PHASE_BATTLE)
        effect::set_code(d, e3, EVENT_PHASE + PHASE_BATTLE);
        // e3:SetCountLimit(1)
        effect::set_count_limit(d, e3, 1, 0, 0, 0);
        // e3:SetCondition(function() return Duel.IsTurnPlayer(1-tp) end)
        effect::set_condition(d, e3, Some(lua_fn(move |d: &mut Duel, _a: Args| -> bool { duel::is_turn_player(d, 1 - tp) })));
        // e3:SetOperation(function() e1:Reset() end)
        effect::set_operation(
            d,
            e3,
            Some(lua_fn(move |d: &mut Duel, _a: Args| {
                effect::reset(d, e1);
            })),
        );
        // e3:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, tp);
    }
}
