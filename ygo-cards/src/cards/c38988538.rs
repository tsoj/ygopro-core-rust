//! Dinomist Plesios
use super::prelude::*;

pub const ID: u32 = 38988538;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOMIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // disable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // decrease atk/def
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
fn tfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DINOMIST) and c:IsControler(tp) and c:IsLocation(LOCATION_ONFIELD)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DINOMIST as u16]) && card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_ONFIELD)
}
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    let g = info[0].as_group().cloned();
    // return e:GetHandler():GetFlagEffect(id)==0 and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET)
    //   and g and g:IsExists(s.tfilter,1,e:GetHandler(),tp) and Duel.IsChainDisablable(ev)
    let h = effect::get_handler(d, a.e).nil();
    if card::get_flag_effect(d, h, ID) != 0 {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    let tp = a.tp;
    match g {
        Some(g) => group::is_exists(d, &g, &|d, c| tfilter(d, c, tp), 1, h.into()) && duel::is_chain_disablable(d, a.ev as u8),
        None => false,
    }
}
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.SelectEffectYesNo(tp,e:GetHandler()) then
    let h = effect::get_handler(d, a.e).nil();
    if duel::select_effect_yes_no(d, a.tp, h, None) {
        // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
        card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
        // if Duel.NegateEffect(ev) then
        if duel::negate_effect(d, a.ev as u8) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
            duel::destroy(d, CardSet::from_card(h), REASON_EFFECT, None, None);
        }
    }
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_DINOMIST)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_DINOMIST as u16])
}
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // return Duel.GetMatchingGroupCount(s.cfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,0,nil)*-100
    let p = effect::get_handler_player(d, e);
    duel::get_matching_group_count(d, Some(&cfilter), p, LOCATION_ONFIELD, 0, NO_EX) as i64 * -100
}
