//! Stellarknight Constellar Diamond
use super::prelude::*;

pub const ID: u32 = 9272381;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3+ Level 5 LIGHT monsters OR During your Main Phase 2, you can also Xyz Summon this card by using a "tellarknight" Xyz Monster you control as material, except "Stellarknight Constellar Diamond"
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_LIGHT),5,3,s.ovfilter,aux.Stringid(id,0),Xyz.InfiniteMats)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, sc, st, Some(tp)))),
        Some(5),
        3,
        Some(xyz::alter_filter(|d, c, tp, xyzc| ovfilter(d, c, tp, xyzc))),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::Infinite),
        None,
        false,
        None,
    );
    // While this card has material, neither player can send cards from the Deck to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_TO_GRAVE)
    effect::set_code(d, e1, EFFECT_CANNOT_TO_GRAVE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_DECK,LOCATION_DECK)
    effect::set_target_range(d, e1, LOCATION_DECK, LOCATION_DECK);
    // e1:SetCondition(s.effcon)
    effect::set_condition(d, e1, Some(lua_fn(effcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_DISCARD_DECK)
    effect::set_code(d, e2, EFFECT_CANNOT_DISCARD_DECK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(lua_fn(effcon)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Any card that returns from the GY to the hand is banished instead
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_TO_HAND_REDIRECT)
    effect::set_code(d, e3, EFFECT_TO_HAND_REDIRECT);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_GRAVE,LOCATION_GRAVE)
    effect::set_target_range(d, e3, LOCATION_GRAVE, LOCATION_GRAVE);
    // e3:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e3, LOCATION_REMOVED as i64);
    // e3:SetCondition(s.effcon)
    effect::set_condition(d, e3, Some(lua_fn(effcon)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // When an opponent's DARK monster's effect is activated (Quick Effect): You can detach 1 material from this card; negate that activation, and if you do, destroy it
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e4, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e4:SetCondition(s.condition)
    effect::set_condition(d, e4, Some(condition(condition_fn)));
    // e4:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e4, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e4:SetTarget(s.target)
    effect::set_target(d, e4, Some(cost_target(target)));
    // e4:SetOperation(s.operation)
    effect::set_operation(d, e4, Some(operation(operation_fn)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.ovfilter(c,tp,xyzc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_TELLARKNIGHT,xyzc,SUMMON_TYPE_XYZ,tp) and c:IsType(TYPE_XYZ,xyzc,SUMMON_TYPE_XYZ,tp)
    //     and not c:IsSummonCode(xyzc,SUMMON_TYPE_XYZ,tp,id) and Duel.IsPhase(PHASE_MAIN2)
    card::is_faceup(d, c)
        && card::is_set_card_ctx(d, c, &[SET_TELLARKNIGHT as u16], Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && !card::is_summon_code(d, c, Some(xyzc), SUMMON_TYPE_XYZ as u64, tp, &[ID])
        && duel::is_phase(d, PHASE_MAIN2)
}
/// `s.effcon(e)`: `e:GetHandler():GetOverlayCount()>0`
fn effcon(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_overlay_count(d, h) > 0
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and re:IsMonsterEffect() and re:GetHandler():IsAttribute(ATTRIBUTE_DARK) and Duel.IsChainNegatable(ev)
    if a.ep != 1 - a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    let rh = effect::get_handler(d, re).nil();
    card::is_attribute(d, rh, ATTRIBUTE_DARK) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
