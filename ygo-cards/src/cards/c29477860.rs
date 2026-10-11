//! Spiral Discharge
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 29477860;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_GAIA_CHAMPION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_ATTACK)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_ATTACK));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 card on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Prevent attacks on monsters while you control "Gaia the Dragon Champion"
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCondition(s.actcon)
    effect::set_condition(d, e3, Some(condition(actcon)));
    // e3:SetValue(s.atktg)
    effect::set_value_fn(d, e3, effect_card_fn(|d, _e, c| atktg(d, c)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.actcon(e)
fn actcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_GAIA_CHAMPION),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_GAIA_CHAMPION])), p, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.atktg(e,c)
fn atktg(d: &mut Duel, c: CardId) -> bool {
    // return c:GetCode()~=CARD_GAIA_CHAMPION
    card::get_code(d, c).first().copied().unwrap_or(0) != CARD_GAIA_CHAMPION
}
// function s.descon(e,tp,eg,ep,ev,re,r,rp)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local tc=Duel.GetAttacker()
    let tc = duel::get_attacker(d).nil();
    // local dc=Duel.GetAttackTarget()
    let dc = duel::get_attack_target(d);
    // return (tc:IsCode(CARD_GAIA_CHAMPION) and tc:IsControler(tp))
    //     or (dc and dc:IsCode(CARD_GAIA_CHAMPION) and dc:IsControler(tp))
    if card::is_code(d, tc, &[CARD_GAIA_CHAMPION]) && card::is_controler(d, tc, tp) {
        return true;
    }
    match dc {
        Some(dc) => card::is_code(d, dc, &[CARD_GAIA_CHAMPION]) && card::is_controler(d, dc, tp),
        None => false,
    }
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
// (no chkc parameter: the Lua `if chkc then` reads a nil global, so that branch never runs)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
