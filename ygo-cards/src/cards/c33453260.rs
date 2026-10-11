//! Comic Hand
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 33453260;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_TOON_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,1,aux.CheckStealEquip,s.eqlimit,nil,s.target,nil,s.condition)
    aux::add_equip_procedure(
        d,
        c,
        Some(1),
        Some(aux::equip_filter_fn(|d, c, e, tp| aux::check_steal_equip(d, c, e, tp))),
        Some(lua_fn(eqlimit)),
        None,
        Some(Arc::new(target_tg)),
        None,
        Some(condition(cond)),
        None,
    );
    // Take control
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_SET_CONTROL)
    effect::set_code(d, e3, EFFECT_SET_CONTROL);
    // e3:SetValue(s.cval)
    effect::set_value_fn(d, e3, effect_card_value(cval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Equipped monster is treated as a Toon
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    // e4:SetCode(EFFECT_ADD_TYPE)
    effect::set_code(d, e4, EFFECT_ADD_TYPE);
    // e4:SetValue(TYPE_TOON)
    effect::set_value(d, e4, TYPE_TOON as i64);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Can attack directly if the opponent controls no Toons
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e5, EFFECT_TYPE_EQUIP);
    // e5:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e5, EFFECT_DIRECT_ATTACK);
    // e5:SetCondition(s.dircon)
    effect::set_condition(d, e5, Some(condition(dircon)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Self destruction
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
    // e6:SetCode(EFFECT_SELF_DESTROY)
    effect::set_code(d, e6, EFFECT_SELF_DESTROY);
    // e6:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e6, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetCondition(s.descon)
    effect::set_condition(d, e6, Some(condition(descon)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(15259703)
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_TOON_WORLD])
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,tc)`
fn target_tg(d: &mut Duel, _a: &EffArgs, tc: Option<CardId>) {
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, tc.map(CardSet::from_card), 1, 0, 0);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let c = a.c(1);
    // return e:GetHandlerPlayer()~=c:GetControler() or e:GetHandler():GetEquipTarget()==c
    let h = effect::get_handler(d, e).nil();
    effect::get_handler_player(d, e) != card::get_controler(d, c) || card::get_equip_target(d, h) == Some(c)
}
/// `s.cval(e,c)`
fn cval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return e:GetHandlerPlayer()
    effect::get_handler_player(d, e) as i64
}
/// `s.dirfilter(c)`
fn dirfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_TOON)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_TOON)
}
/// `s.dircon(e)`
fn dircon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.dirfilter,e:GetHandlerPlayer(),0,LOCATION_MZONE,1,nil)
    let tp = effect::get_handler_player(d, a.e);
    !duel::is_existing_matching_card(d, Some(&dirfilter), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.descon(e)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.cfilter,e:GetHandlerPlayer(),LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let tp = effect::get_handler_player(d, a.e);
    !duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
