//! A.I. Shadow
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 77421977;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_IGNISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddPersistentProcedure(c,0,aux.FaceupFilter(Card.IsSetCard,SET_IGNISTER),CATEGORY_ATKCHANGE,
    //     EFFECT_FLAG_DAMAGE_STEP,TIMING_DAMAGE_STEP,TIMING_DAMAGE_STEP,aux.StatChangeDamageStepCondition)
    aux::add_persistent_procedure(
        d,
        c,
        Some(0),
        Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, _tp: u8| {
            card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_IGNISTER as u16])
        })),
        Some(CATEGORY_ATKCHANGE),
        Some(EFFECT_FLAG_DAMAGE_STEP),
        Some(TIMING_DAMAGE_STEP as u32),
        Some(TIMING_DAMAGE_STEP as u32),
        Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))),
        None,
        None,
        None,
        false,
    );
    // atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(aux.PersistentTargetFilter)
    effect::set_target(d, e1, Some(aux::persistent_target_filter()));
    // e1:SetValue(800)
    effect::set_value(d, e1, 800);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // must attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e2, EFFECT_MUST_ATTACK);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    effect::set_code(d, e3, EFFECT_MUST_ATTACK_MONSTER);
    // e3:SetValue(aux.PersistentTargetFilter)
    effect::set_value_fn(d, e3, aux::persistent_target_filter());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // draw
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e4, CATEGORY_DRAW);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(s.drcon)
    effect::set_condition(d, e4, Some(condition(drcon)));
    // e4:SetTarget(s.drtg)
    effect::set_target(d, e4, Some(cost_target(drtg)));
    // e4:SetOperation(s.drop)
    effect::set_operation(d, e4, Some(operation(drop_op)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EVENT_REMOVE)
    effect::set_code(d, e5, EVENT_REMOVE);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetTurnPlayer()~=e:GetHandlerPlayer() and e:GetHandler():GetCardTargetCount()>0
    let turn = duel::get_turn_player(d);
    let hp = effect::get_handler_player(d, a.e);
    if turn == hp {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::get_card_target_count(d, h) > 0
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_EFFECT) and rp~=tp and c:GetPreviousControler()==tp
    //     and c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEUP)
    card::is_reason(d, c, REASON_EFFECT)
        && a.rp != a.tp
        && card::get_previous_controler(d, c) == a.tp
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEUP)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
