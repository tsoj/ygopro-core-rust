//! Drake Shark
use super::prelude::*;

pub const ID: u32 = 81096431;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SHARK_DRAKE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card if it is added to the hand, except by drawing it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e1, EVENT_TO_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return not e:GetHandler():IsReason(REASON_DRAW) end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            !card::is_reason(d, h, REASON_DRAW)
        })),
    );
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be treated as 2 Materials for the Xyz Summon of a WATER Xyz monter that requires 3 or more materials
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DOUBLE_XYZ_MATERIAL)
    effect::set_code(d, e2, EFFECT_DOUBLE_XYZ_MATERIAL);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetOperation(function(e,c) return c.minxyzct and c.minxyzct>=3 and c:IsAttribute(ATTRIBUTE_WATER) end)
    effect::set_operation(d, e2, Some(lua_fn(|d, a| {
        // `c` is the second argument of the operation call (the Lua parameter named c)
        let c = a.c(1);
        let m = crate::class::get(d, c, crate::class::MIN_XYZ_CT);
        m.truthy() && m.int() >= 3 && card::is_attribute(d, c, ATTRIBUTE_WATER)
    })));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Provide an effect to a "Shark Drake" Xyz Monster that this card as Xyz material
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_XMATERIAL+EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_XMATERIAL + EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e) return e:GetHandler():IsSetCard(SET_SHARK_DRAKE) end)
    effect::set_condition(
        d,
        e3,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_set_card(d, h, &[SET_SHARK_DRAKE as u16])
        })),
    );
    // e3:SetCost(Cost.DetachFromSelf(2,2,nil))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(2), Some(cost::Count::N(2)), None)));
    // e3:SetTarget(s.attachtg)
    effect::set_target(d, e3, Some(cost_target(attachtg)));
    // e3:SetOperation(s.attachop)
    effect::set_operation(d, e3, Some(operation(attachop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.attachfilter(c,tp)`
fn attachfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSpellTrap() and (c:IsControler(tp) or c:IsAbleToChangeControler())
    card::is_spell_trap(d, c) && (card::is_controler(d, c, tp) || card::is_able_to_change_controler(d, c))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
///
/// The Lua function has no `chkc` parameter, so its `if chkc then` branch reads a nil global and
/// is never taken: it is left out.
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsType(TYPE_XYZ) and Duel.IsExistingTarget(s.attachfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_type(d, h, TYPE_XYZ)
            && duel::is_existing_target(d, Some(&|d, c| attachfilter(d, c, a.tp)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACH as u64);
    // Duel.SelectTarget(tp,s.attachfilter,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, c| attachfilter(d, c, a.tp)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and c:IsType(TYPE_XYZ) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc, a.e) && card::is_type(d, c, TYPE_XYZ) {
        // Duel.Overlay(c,tc)
        duel::overlay(d, c, CardSet::from_card(tc), false);
    }
}
