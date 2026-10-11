//! Scanner
use super::prelude::*;

pub const ID: u32 = 75198893;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Copy the target's name, Attribute, Level, and ATK/DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.cptg)
    effect::set_target(d, e1, Some(cost_target(cptg)));
    // e1:SetOperation(s.cpop)
    effect::set_operation(d, e1, Some(operation(cpop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cpfilter(c)`
fn cpfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsMonster() and c:HasLevel()
    card::is_faceup(d, c) && card::is_monster(d, c) && card::has_level(d, c)
}
/// `s.cptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn cptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(1-tp) and s.cpfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, 1 - a.tp) && cpfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.cpfilter,tp,0,LOCATION_REMOVED,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&cpfilter), a.tp, 0, LOCATION_REMOVED, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.cpfilter,tp,0,LOCATION_REMOVED,1,1,nil)
    duel::select_target(d, a.tp, Some(&cpfilter), a.tp, 0, LOCATION_REMOVED, 1, 1, false, NO_EX);
    true
}
/// `s.cpop(e,tp,eg,ep,ev,re,r,rp)`
fn cpop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // --Change name
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetValue(tc:GetOriginalCode())
    let code = card::get_original_code(d, tc);
    effect::set_value(d, e1, code as i64);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Change ATK/DEF
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e2, EFFECT_SET_BASE_ATTACK);
    // e2:SetValue(tc:GetBaseAttack())
    let atk = card::get_base_attack(d, tc);
    effect::set_value(d, e2, atk as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_SET_BASE_DEFENSE)
    effect::set_code(d, e3, EFFECT_SET_BASE_DEFENSE);
    // e3:SetValue(tc:GetBaseDefense())
    let def = card::get_base_defense(d, tc);
    effect::set_value(d, e3, def as i64);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --Change Attribute
    // local e4=e1:Clone()
    let e4 = effect::clone(d, e1);
    // e4:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e4, EFFECT_CHANGE_ATTRIBUTE);
    // e4:SetValue(tc:GetAttribute())
    let att = card::get_attribute(d, tc);
    effect::set_value(d, e4, att as i64);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // --Change Level
    // local e5=e1:Clone()
    let e5 = effect::clone(d, e1);
    // e5:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e5, EFFECT_CHANGE_LEVEL);
    // e5:SetValue(tc:GetLevel())
    let lv = card::get_level(d, tc);
    effect::set_value(d, e5, lv as i64);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // --Banish it if it leaves the field
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(3300)
    effect::set_description(d, e6, 3300);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
    effect::set_code(d, e6, EFFECT_LEAVE_FIELD_REDIRECT);
    // e6:SetProperty(EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e6, EFFECT_FLAG_CLIENT_HINT, 0);
    // e6:SetReset(RESET_EVENT|RESETS_REDIRECT|RESET_PHASE|PHASE_END)
    effect::set_reset(d, e6, RESET_EVENT | RESETS_REDIRECT | RESET_PHASE | PHASE_END, 1);
    // e6:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e6, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
