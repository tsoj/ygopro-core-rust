//! Ryzeal Detonator
use super::prelude::*;

pub const ID: u32 = 34909328;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure: 2+ Level 4 "Ryzeal" monsters
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_RYZEAL),4,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, _sc, _st, _tp| card::is_set_card(d, c, &[SET_RYZEAL as u16]))),
        Some(4),
        2,
        None,
        None,
        Some(xyz::INFINITE_MATS),
        None,
        false,
        None,
    );
    // Destroy 1 card on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end)
    effect::set_condition(d, e1, Some(condition(|_d, a| a.rp == 1 - a.tp)));
    // e1:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach 1 monster from your GY to this card as material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e2, CATEGORY_LEAVE_GRAVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.attachtg)
    effect::set_target(d, e2, Some(cost_target(attachtg)));
    // e2:SetOperation(s.attachop)
    effect::set_operation(d, e2, Some(operation(attachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If an Xyz Monster(s) you control would be destroyed by battle or card effect, you can detach 1 material from this card instead
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.reptg)
    effect::set_target(d, e3, Some(cost_target(reptg)));
    // e3:SetOperation(function(e,tp) e:GetHandler():RemoveOverlayCard(tp,1,1,REASON_EFFECT) end)
    effect::set_operation(d, e3, Some(operation(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::remove_overlay_card(d, h, a.tp, 1, 1, REASON_EFFECT);
    })));
    // e3:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e3, lua_fn(|d, a: Args| {
        let c = a.c(1);
        let p = effect::get_handler_player(d, a.e(0));
        repfilter(d, c, p)
    }));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
/// `s.attachfilter(c,xyzc,tp)`
fn attachfilter(d: &mut Duel, c: CardId, xyzc: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsCanBeXyzMaterial(xyzc,tp,REASON_EFFECT)
    card::is_monster(d, c) && card::is_can_be_xyz_material(d, c, Some(xyzc), Some(tp), Some(REASON_EFFECT))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.attachfilter,tp,LOCATION_GRAVE,0,1,nil,e:GetHandler(),tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, c| attachfilter(d, c, h, tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, 0);
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.attachfilter,tp,LOCATION_GRAVE,0,1,1,nil,c,tp):GetFirst()
    let tp = a.tp;
    let g = duel::select_matching_card(d, tp, Some(&|d, x| attachfilter(d, x, c, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.HintSelection(tc)
        duel::hint_selection(d, CardSet::from_card(tc), true);
        // Duel.Overlay(c,tc)
        duel::overlay(d, c, tc, false);
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsControler(tp) and c:IsType(TYPE_XYZ) and c:IsFaceup()
    //     and c:IsReason(REASON_BATTLE|REASON_EFFECT) and not c:IsReason(REASON_REPLACE)
    card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_type(d, c, TYPE_XYZ)
        && card::is_faceup(d, c)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return eg:IsExists(s.repfilter,1,nil,tp) and c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        let tp = a.tp;
        return group::is_exists(d, &eg, &|d, x| repfilter(d, x, tp), 1, NO_EX) && card::check_remove_overlay_card(d, c, a.tp, 1, REASON_EFFECT);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
