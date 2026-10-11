//! Gouki Thunder Ogre
use super::prelude::*;

pub const ID: u32 = 30010480;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GOUKI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GOUKI),2)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_GOUKI as u16], sc, st, Some(tp)))),
        2,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // extra summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCode(EFFECT_EXTRA_SUMMON_COUNT)
    effect::set_code(d, e1, EFFECT_EXTRA_SUMMON_COUNT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_HAND,LOCATION_HAND)
    effect::set_target_range(d, e1, LOCATION_HAND, LOCATION_HAND);
    // e1:SetValue(s.sumval)
    effect::set_value_fn(d, e1, lua_fn(sumval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `bit.extract(v,pos)`
fn bit_extract(v: u32, pos: i64) -> u32 {
    if !(0..64).contains(&pos) {
        return 0;
    }
    (((v as u64) >> pos) & 1) as u32
}
/// `s.sumval(e,c)`
fn sumval(d: &mut Duel, a: Args) -> Value {
    let e = a.e(0);
    let c = a.c(1);
    // local hp=e:GetHandlerPlayer()
    let hp = effect::get_handler_player(d, e);
    let h = effect::get_handler(d, e).nil();
    // if c:IsControler(e:GetHandlerPlayer()) then
    if card::is_controler(d, c, hp) {
        // local sumzone=e:GetHandler():GetLinkedZone()
        let sumzone = card::get_linked_zone(d, h, None);
        // local relzone=-(1<<e:GetHandler():GetSequence())
        let relzone = -(1i64 << card::get_sequence(d, h));
        // return 0,sumzone,relzone
        Value::Multi(vec![Value::Int(0), Value::Int(sumzone as i64), Value::Int(relzone)])
    } else {
        // local sumzone=e:GetHandler():GetLinkedZone(1-e:GetHandlerPlayer())
        let sumzone = card::get_linked_zone(d, h, Some(1 - hp));
        // local relzone=-(1<<e:GetHandler():GetSequence()+16)
        let relzone = -(1i64 << (card::get_sequence(d, h) as i64 + 16));
        // return 0,sumzone,relzone
        Value::Multi(vec![Value::Int(0), Value::Int(sumzone as i64), Value::Int(relzone)])
    }
}
/// `s.cfilter(c,tp,zone)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8, zone: u32) -> bool {
    // local seq=c:GetPreviousSequence()
    let mut seq = card::get_previous_sequence(d, c) as i64;
    // if not c:IsPreviousControler(tp) then seq=seq+16 end
    if !card::is_previous_controler(d, c, tp) {
        seq += 16;
    }
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:IsPreviousLocation(LOCATION_MZONE) and bit.extract(zone,seq)~=0
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && card::is_previous_location(d, c, LOCATION_MZONE) && bit_extract(zone, seq) != 0
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local zone=e:GetHandler():GetLinkedZone()
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_linked_zone(d, h, None);
    // return eg:IsExists(s.cfilter,1,nil,tp,e:GetHandler():GetLinkedZone())
    let eg = a.eg.clone().nil();
    let tp = a.tp;
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp, zone), 1, NO_EX)
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
        effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
        // e1:SetValue(400)
        effect::set_value(d, e1, 400);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
