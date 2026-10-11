//! Void Cauldron
//! 煉獄の釜
use super::prelude::*;

pub const ID: u32 = 45313724;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_INFERNITY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_SELF_TOGRAVE)
    duel::enable_global_flag(d, GLOBALFLAG_SELF_TOGRAVE);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Banish to prevent destruction
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTarget(s.reptg)
    effect::set_target(d, e1, Some(cost_target(reptg)));
    // e1:SetValue(s.repval)
    effect::set_value_fn(d, e1, effect_card_fn(repval));
    // e1:SetOperation(s.repop)
    effect::set_operation(d, e1, Some(operation(repop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send to GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_SELF_TOGRAVE)
    effect::set_code(d, e2, EFFECT_SELF_TOGRAVE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.sdcon)
    effect::set_condition(d, e2, Some(condition(sdcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.dsdlv8filter(c)`
fn dsdlv8filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsAttribute(ATTRIBUTE_DARK) and c:IsType(TYPE_SYNCHRO)
    card::is_race(d, c, RACE_DRAGON) && card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_type(d, c, TYPE_SYNCHRO)
}
/// `(s.dsdlv8filter(c) or (c:IsSetCard(SET_INFERNITY) and c:IsMonster()))`, shared by `dfilter` and `repval`
fn dsd_or_infernity_monster(d: &mut Duel, c: CardId) -> bool {
    dsdlv8filter(d, c) || (card::is_set_card(d, c, &[SET_INFERNITY as u16]) && card::is_monster(d, c))
}
/// `s.dfilter(c,tp)`
fn dfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (s.dsdlv8filter(c) or (c:IsSetCard(SET_INFERNITY) and c:IsMonster())) and c:IsControler(tp) and c:IsReason(REASON_EFFECT)
    dsd_or_infernity_monster(d, c) && card::is_controler(d, c, tp) && card::is_reason(d, c, REASON_EFFECT)
}
/// `s.repfilter(c)`
fn repfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_INFERNITY) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::is_set_card(d, c, &[SET_INFERNITY as u16])
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return eg:IsExists(s.dfilter,1,nil,tp) and Duel.IsExistingMatchingCard(s.repfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return group::is_exists(d, &a.eg(), &|d, c| dfilter(d, c, tp), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&repfilter), tp, LOCATION_GRAVE | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,e:GetHandler(),96)
    let h = effect::get_handler(d, e).nil();
    duel::select_effect_yes_no(d, tp, h, Some(96))
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsControler(e:GetHandlerPlayer()) and (s.dsdlv8filter(c) or (c:IsSetCard(SET_INFERNITY) and c:IsMonster())) and c:IsReason(REASON_EFFECT)
    let p = effect::get_handler_player(d, e);
    card::is_controler(d, c, p) && dsd_or_infernity_monster(d, c) && card::is_reason(d, c, REASON_EFFECT)
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.repfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&repfilter), tp, LOCATION_GRAVE | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
/// `s.sdcon(e)`
fn sdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_HAND,0)>0
    let p = effect::get_handler_player(d, a.e);
    duel::get_field_group_count(d, p, LOCATION_HAND, 0) > 0
}
