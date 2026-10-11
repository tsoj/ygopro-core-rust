//! Litmus Doom Swordsman
//! リトマスの死の剣士
use super::prelude::*;

pub const ID: u32 = 72566043;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[8955148], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Unaffected by Trap effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetValue(function(e,te) return te:IsTrapEffect() end)
    effect::set_value_fn(d, e1, lua_fn(immune_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by battle
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Gains 3000 ATK/DEF when there is a face-up Trap on the field
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsTrap),e:GetHandlerPlayer(),LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end)
    effect::set_condition(d, e3, Some(condition(trap_cond)));
    // e3:SetValue(3000)
    effect::set_value(d, e3, 3000);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Set 1 Trap Card from either GY
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_SET)
    effect::set_category(d, e5, CATEGORY_SET);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e5:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e5, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e5:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e5, EVENT_DESTROYED);
    // e5:SetCondition(s.setcon)
    effect::set_condition(d, e5, Some(condition(setcon)));
    // e5:SetTarget(s.settg)
    effect::set_target(d, e5, Some(cost_target(settg)));
    // e5:SetOperation(s.setop)
    effect::set_operation(d, e5, Some(operation(setop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `function(e,te) return te:IsTrapEffect() end`
fn immune_value(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    effect::is_trap_effect(d, te)
}
/// `function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsTrap),e:GetHandlerPlayer(),LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end`
fn trap_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_trap(d, c)), p, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return rp==1-tp and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsRitualSummoned()
    a.rp == 1 - a.tp && card::is_previous_controler(d, c, a.tp) && card::is_previous_location(d, c, LOCATION_MZONE) && card::is_ritual_summoned(d, c)
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsSSetable()
    card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and s.setfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && setfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.setfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&setfilter), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectTarget(tp,s.setfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&setfilter), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsSSetable() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_s_setable(d, tc, false, None) {
        // Duel.SSet(tp,tc)
        duel::s_set(d, a.tp, tc, None, true);
    }
}
