//! Dragunity Divine Lance
use super::prelude::*;

pub const ID: u32 = 60004971;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGUNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_DRAGUNITY))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_DRAGUNITY as u16]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // atk
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // immune
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    // e4:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e4, EFFECT_IMMUNE_EFFECT);
    // e4:SetValue(s.efilter)
    effect::set_value_fn(d, e4, lua_fn(efilter));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // equip
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e5, CATEGORY_EQUIP);
    // e5:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e5, EFFECT_TYPE_IGNITION);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCountLimit(1,id)
    effect::set_count_limit(d, e5, 1, ID, 0, 0);
    // e5:SetTarget(s.eqtg)
    effect::set_target(d, e5, Some(cost_target(eqtg)));
    // e5:SetOperation(s.eqop)
    effect::set_operation(d, e5, Some(operation(eqop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetLevel()*100
    card::get_level(d, c) as i64 * 100
}
/// `s.efilter(e,re)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return re:IsTrapEffect()
    let re = a.e(1);
    effect::is_trap_effect(d, re)
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DRAGUNITY) and c:IsRace(RACE_DRAGON) and c:IsType(TYPE_TUNER) and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_DRAGUNITY as u16])
        && card::is_race(d, c, RACE_DRAGON)
        && card::is_type(d, c, TYPE_TUNER)
        && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,...,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&eqfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.eqop(e,tp,...)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c);
    // if c:IsRelateToEffect(e) and ec:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, ec.nil()) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
        // local g=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&eqfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g);
        // if not tc or not Duel.Equip(tp,tc,ec,true) then return end
        let Some(tc) = tc else { return };
        if !duel::equip(d, a.tp, tc, ec.nil(), true, false) {
            return;
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(s.eqlimit2)
        effect::set_value_fn(d, e1, lua_fn(eqlimit2));
        // e1:SetLabelObject(ec)
        effect::set_label_object(d, e1, Value::Card(ec.nil()));
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.eqlimit2(e,c)`
fn eqlimit2(d: &mut Duel, a: Args) -> bool {
    // return c==e:GetLabelObject()
    let c = a.c(1);
    matches!(effect::get_label_object(d, a.e(0)), Value::Card(x) if x == c)
}
