//! Tantrum Toddler
use super::prelude::*;

pub const ID: u32 = 7020743;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure
    // Xyz.AddProcedure(c,nil,1,2)
    xyz::add_procedure(d, c, None, Some(1), 2, None, None, None, None, false, None);
    // Attach up to 2 cards to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e1, CATEGORY_LEAVE_GRAVE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_CHAIN_NEGATED)
    effect::set_code(d, e1, EVENT_CHAIN_NEGATED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e1:SetTarget(s.atchtg)
    effect::set_target(d, e1, Some(cost_target(atchtg)));
    // e1:SetOperation(s.atchop)
    effect::set_operation(d, e1, Some(operation(atchop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains 700 ATK/DEF for each material attached to it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(function(e,c) return c:GetOverlayCount()*700 end)
    effect::set_value_fn(d, e2, effect_card_value(|d, _e, c| card::get_overlay_count(d, c) as i64 * 700));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Cannot be destroyed by battle
    // local e4=e2:Clone()
    let e4 = effect::clone(d, e2);
    // e4:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_BATTLE);
    // e4:SetCondition(function(e) return e:GetHandler():GetOverlayCount()>=4 end)
    effect::set_condition(
        d,
        e4,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::get_overlay_count(d, h) >= 4
        })),
    );
    // e4:SetValue(1)
    effect::set_value(d, e4, 1);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Cannot be destroyed by card effects
    // local e5=e4:Clone()
    let e5 = effect::clone(d, e4);
    // e5:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e5, EFFECT_INDESTRUCTABLE_EFFECT);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // Destroy all cards on the field
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e6, aux::stringid(ID, 0));
    // e6:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e6, CATEGORY_DESTROY);
    // e6:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e6, EFFECT_TYPE_QUICK_O);
    // e6:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e6, EVENT_FREE_CHAIN);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e6, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e6:SetCondition(function(e) return e:GetHandler():GetOverlayCount()>=8 end)
    effect::set_condition(
        d,
        e6,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::get_overlay_count(d, h) >= 8
        })),
    );
    // e6:SetCost(Cost.DetachFromSelf(4,4,nil))
    effect::set_cost(d, e6, Some(cost::detach_from_self(cost::Count::N(4), Some(cost::Count::N(4)), None)));
    // e6:SetTarget(s.destg)
    effect::set_target(d, e6, Some(cost_target(destg)));
    // e6:SetOperation(s.desop)
    effect::set_operation(d, e6, Some(operation(desop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
// s.atchtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atchtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsType(TYPE_XYZ) and Duel.GetFieldGroupCount(0,LOCATION_GRAVE,LOCATION_GRAVE)>0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_type(d, h, TYPE_XYZ) && duel::get_field_group_count(d, 0, LOCATION_GRAVE, LOCATION_GRAVE) > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,PLAYER_ALL,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    true
}
// s.atchop(e,tp,eg,ep,ev,re,r,rp)
fn atchop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetFieldGroup(0,LOCATION_GRAVE,LOCATION_GRAVE)
    let g = duel::get_field_group(d, 0, LOCATION_GRAVE, LOCATION_GRAVE);
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_ATTACH)
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(1),
        Some(2),
        Some(&aux::dpcheck(controler_value)),
        1,
        a.tp,
        Some(HINTMSG_ATTACH as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.HintSelection(sg,true)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.Overlay(c,sg)
        duel::overlay(d, c, sg, false);
    }
}
// Card.GetControler as a number, for aux.dpcheck
fn controler_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_controler(d, c) as i64
}
// s.destg(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
// s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
