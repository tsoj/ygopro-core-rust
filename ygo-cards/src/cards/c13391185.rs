//! Noble Knight Gwalchavad
use super::prelude::*;

pub const ID: u32 = 13391185;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NOBLE_KNIGHT, SET_NOBLE_ARMS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Normal monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_ADD_TYPE)
    effect::set_code(d, e1, EFFECT_ADD_TYPE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.eqcon1)
    effect::set_condition(d, e1, Some(lua_fn(|d, a| eqcon1(d, a.e(0)))));
    // e1:SetValue(TYPE_NORMAL)
    effect::set_value(d, e1, TYPE_NORMAL as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_REMOVE_TYPE)
    effect::set_code(d, e2, EFFECT_REMOVE_TYPE);
    // e2:SetValue(TYPE_EFFECT)
    effect::set_value(d, e2, TYPE_EFFECT as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // tohand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e3, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// s.eqcon1(e)
fn eqcon1(d: &mut Duel, e: EffectId) -> bool {
    // local eg=e:GetHandler():GetEquipGroup()
    let h = effect::get_handler(d, e).nil();
    let eg = card::get_equip_group(d, h);
    // return not eg or not eg:IsExists(Card.IsSetCard,1,nil,SET_NOBLE_ARMS)
    !group::is_exists(d, &eg, &|d, x| card::is_set_card(d, x, &[SET_NOBLE_ARMS as u16]), 1, NO_EX)
}
// s.eqcon2(e)
fn eqcon2(d: &mut Duel, e: EffectId) -> bool {
    // local eg=e:GetHandler():GetEquipGroup()
    let h = effect::get_handler(d, e).nil();
    let eg = card::get_equip_group(d, h);
    // return eg and eg:IsExists(Card.IsSetCard,1,nil,SET_NOBLE_ARMS)
    group::is_exists(d, &eg, &|d, x| card::is_set_card(d, x, &[SET_NOBLE_ARMS as u16]), 1, NO_EX)
}
// s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return s.eqcon2(e)
    eqcon2(d, a.e)
}
// s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NOBLE_KNIGHT) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
// s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, 0, 0);
    true
}
// s.desfilter(c)
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_NOBLE_ARMS) and c:IsType(TYPE_EQUIP)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16]) && card::is_type(d, c, TYPE_EQUIP)
}
// s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.SendtoHand(tc,nil,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_HAND) then
    if card::is_relate_to_effect(d, tc, a.e) && duel::send_to_hand(d, tc, None, REASON_EFFECT, None) > 0 && card::is_location(d, tc, LOCATION_HAND) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local dg=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_SZONE,0,1,1,nil)
        let dg = duel::select_matching_card(d, a.tp, Some(&desfilter), a.tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX).nil();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(dg,REASON_EFFECT)
        duel::destroy(d, dg, REASON_EFFECT, None, None);
    }
}
