//! 陽炎獣 バジリコック
//! Hazy Flame Basiltrice
use super::prelude::*;

pub const ID: u32 = 23776077;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_FIRE),6,2,nil,nil,5)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_FIRE, sc, st, Some(tp)))),
        Some(6),
        2,
        None,
        None,
        Some(xyz::MaxCt::N(5)),
        None,
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // remove
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // atk/def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetCondition(s.adcon)
    effect::set_condition(d, e2, Some(condition(adcon)));
    // e2:SetValue(s.adval)
    effect::set_value_fn(d, e2, effect_value(|d, e| adval(d, e)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // cannot be target
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e4, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e4:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.tgcon)
    effect::set_condition(d, e4, Some(condition(tgcon)));
    // e4:SetValue(aux.tgoval)
    effect::set_value_fn(d, e4, aux::tgoval());
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // cannot be destroyed
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e5, EFFECT_INDESTRUCTABLE_EFFECT);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCondition(s.indcon)
    effect::set_condition(d, e5, Some(condition(indcon)));
    // e5:SetValue(1)
    effect::set_value(d, e5, 1);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.rmfilter(c)
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove() and aux.SpElimFilter(c,false,true)
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, true)
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and s.rmfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && rmfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&rmfilter), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
// function s.adcon(e)
fn adcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetOverlayCount()>=3
    let h = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, h) >= 3
}
// function s.adval(e,c)
fn adval(d: &mut Duel, e: EffectId) -> i64 {
    // return e:GetHandler():GetOverlayCount()*200
    let h = effect::get_handler(d, e).nil();
    card::get_overlay_count(d, h) as i64 * 200
}
// function s.tgcon(e)
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetOverlayCount()>=4
    let h = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, h) >= 4
}
// function s.indcon(e)
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetOverlayCount()==5
    let h = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, h) == 5
}
