//! Magikey Unlocking
//! 魔鍵錠－解－
//! scripted by XyleN5967
use super::prelude::*;

pub const ID: u32 = 98904974;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGIKEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MAGIKEY) and (c:IsType(TYPE_RITUAL) or c:IsSummonLocation(LOCATION_EXTRA))
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_MAGIKEY as u16])
        && (card::is_type(d, c, TYPE_RITUAL) || card::is_summon_location(d, c, LOCATION_EXTRA))
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil) then return false end
    if !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX) {
        return false;
    }
    // return ep~=tp and re:IsSpellTrapEffect() and re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev)
    let re = a.re.nil();
    a.ep != a.tp && effect::is_spell_trap_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && duel::is_chain_negatable(d, a.ev as u8)
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
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re)
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) && duel::destroy(d, a.eg(), REASON_EFFECT, None, None) != 0 && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTRIBUTE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTRIBUTE as u64);
            // local attr=Duel.AnnounceAttribute(tp,1,ATTRIBUTE_ALL)
            let attr = duel::announce_attribute(d, a.tp, 1, ATTRIBUTE_ALL);
            // Change attributes
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetCode(EFFECT_CHANGE_ATTRIBUTE)
            effect::set_code(d, e1, EFFECT_CHANGE_ATTRIBUTE);
            // e1:SetTargetRange(0,LOCATION_MZONE)
            effect::set_target_range(d, e1, 0, LOCATION_MZONE);
            // e1:SetValue(attr)
            effect::set_value(d, e1, attr as i64);
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, a.tp);
        }
    }
}
