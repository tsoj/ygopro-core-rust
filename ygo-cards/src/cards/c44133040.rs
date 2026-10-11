//! Evil Eye of Selene
//! セレンの呪眼
use super::prelude::*;

pub const ID: u32 = 44133040;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_EVIL_EYE],
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,aux.FilterBoolFunction(Card.IsSetCard,SET_EVIL_EYE))
    aux::add_equip_procedure(
        d,
        c,
        None,
        Some(aux::equip_filter_fn(|d, c, _e, _tp| card::is_set_card(d, c, &[SET_EVIL_EYE as u16]))),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    // Prevent destruction by battle
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent effect target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Prevent destruction by effects
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetValue(aux.indoval)
    effect::set_value_fn(d, e3, aux::indoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Increase ATK
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e5, CATEGORY_ATKCHANGE);
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e5:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e5, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e5:SetCode(EVENT_CHAINING)
    effect::set_code(d, e5, EVENT_CHAINING);
    // e5:SetRange(LOCATION_SZONE)
    effect::set_range(d, e5, LOCATION_SZONE);
    // e5:SetCondition(s.atkupcond)
    effect::set_condition(d, e5, Some(condition(atkupcond)));
    // e5:SetOperation(s.atkupop)
    effect::set_operation(d, e5, Some(operation(atkupop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Set itself from the GY
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e6, aux::stringid(ID, 1));
    // e6:SetCategory(CATEGORY_SET)
    effect::set_category(d, e6, CATEGORY_SET);
    // e6:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e6, EFFECT_TYPE_IGNITION);
    // e6:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e6, LOCATION_GRAVE);
    // e6:SetCountLimit(1,id)
    effect::set_count_limit(d, e6, 1, ID, 0, 0);
    // e6:SetCost(s.setcost)
    effect::set_cost(d, e6, Some(cost_target(setcost)));
    // e6:SetTarget(s.settg)
    effect::set_target(d, e6, Some(cost_target(settg)));
    // e6:SetOperation(s.setop)
    effect::set_operation(d, e6, Some(operation(setop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.atkupcond(e,tp,eg,ep,ev,re,r,rp)`
fn atkupcond(d: &mut Duel, a: &EffArgs) -> bool {
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, h);
    // return ((re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellTrapEffect() and re:GetHandler():IsSetCard(SET_EVIL_EYE)
    //   and re:GetHandler()~=e:GetHandler()) or re:GetHandler()==ec) and tp==rp
    let re = a.re.nil();
    let rh = effect::get_handler(d, re);
    let part1 = effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && effect::is_spell_trap_effect(d, re)
        && card::is_set_card(d, rh.nil(), &[SET_EVIL_EYE as u16])
        && rh != Some(h);
    (part1 || rh == ec) && a.tp == a.rp
}
/// `s.atkupop(e,tp,eg,ep,ev,re,r,rp)`
fn atkupop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local ec=c:GetEquipTarget()
    let ec = card::get_equip_target(d, c).nil();
    // if ec:UpdateAttack(500,nil,c)==500 and c:IsRelateToEffect(e) then
    if card::update_attack(d, ec, 500, None, Some(c), None) == 500 && card::is_relate_to_effect(d, c, a.e) {
        // Duel.SetLP(tp,Duel.GetLP(tp)-500)
        let lp = duel::get_lp(d, a.tp);
        duel::set_lp(d, a.tp, lp - 500);
    }
}
/// `s.rfilter(c)`
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsSetCard(SET_EVIL_EYE) and c:IsAbleToRemoveAsCost() and not c:IsCode(id)
    card::is_spell_trap(d, c) && card::is_set_card(d, c, &[SET_EVIL_EYE as u16]) && card::is_able_to_remove_as_cost(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.setcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn setcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckLPCost(tp,1000) and Duel.IsExistingMatchingCard(s.rfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::check_lp_cost(d, a.tp, 1000) && duel::is_existing_matching_card(d, Some(&|d, c| rfilter(d, c)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rfilter,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| rfilter(d, c)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.PayLPCost(tp,1000)
    duel::pay_lp_cost(d, a.tp, 1000);
    true
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsSSetable() end
    if a.chk == Some(0) {
        return card::is_s_setable(d, c, false, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsSSetable() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_s_setable(d, c, false, None) {
        // Duel.SSet(tp,c)
        duel::s_set(d, a.tp, c, None, true);
    }
}
