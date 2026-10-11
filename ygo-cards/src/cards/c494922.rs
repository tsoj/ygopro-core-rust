//! Superheavy Samurai Warlord Susanowo
use super::prelude::*;

pub const ID: u32 = 494922;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPERHEAVY_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro summon procedure
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_MACHINE),1,1,Synchro.NonTuner(Card.IsSetCard,SET_SUPERHEAVY_SAMURAI),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| card::is_race_ctx(d, x, RACE_MACHINE, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(Some(std::sync::Arc::new(|d: &mut Duel, x: CardId| {
            card::is_set_card(d, x, &[SET_SUPERHEAVY_SAMURAI as u16])
        })))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Attack while in defense position
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DEFENSE_ATTACK)
    effect::set_code(d, e1, EFFECT_DEFENSE_ATTACK);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 1 spell/trap from opponent's GY
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    // e2:SetCondition(s.setcon)
    effect::set_condition(d, e2, Some(condition(setcon)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_GRAVE,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_spell_trap(d, c)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.setfilter(c,tp)`
fn setfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSpellTrap() and c:IsSSetable(true) and (c:IsType(TYPE_FIELD) or Duel.GetLocationCount(tp,LOCATION_SZONE)>0)
    card::is_spell_trap(d, c)
        && card::is_s_setable(d, c, true, None)
        && (card::is_type(d, c, TYPE_FIELD) || duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_GRAVE) and s.setfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && setfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.setfilter,tp,0,LOCATION_GRAVE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| setfilter(d, c, a.tp)), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectTarget(tp,s.setfilter,tp,0,LOCATION_GRAVE,1,1,nil,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| setfilter(d, c, a.tp)), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and (tc:IsType(TYPE_FIELD) or Duel.GetLocationCount(tp,LOCATION_SZONE)>0) then
    if card::is_relate_to_effect(d, tc, a.e)
        && (card::is_type(d, tc, TYPE_FIELD) || duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0)
    {
        // Duel.SSet(tp,tc)
        duel::s_set(d, tp, tc, None, true);
        // Banish it if it leaves the field
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetDescription(3300)
        effect::set_description(d, e1, 3300);
        // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
    }
}
