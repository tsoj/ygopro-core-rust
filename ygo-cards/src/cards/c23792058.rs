//! Speedroid Hexasaucer
use super::prelude::*;

pub const ID: u32 = 23792058;

// s.listed_series={SET_SPEEDROID}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPEEDROID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Return a monster to the Extra Deck
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e0, CATEGORY_TODECK);
    // e0:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e0, EFFECT_TYPE_IGNITION);
    // e0:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e0, EFFECT_FLAG_CARD_TARGET, 0);
    // e0:SetRange(LOCATION_PZONE)
    effect::set_range(d, e0, LOCATION_PZONE);
    // e0:SetCountLimit(1,id)
    effect::set_count_limit(d, e0, 1, ID, 0, 0);
    // e0:SetTarget(s.tdtg)
    effect::set_target(d, e0, Some(cost_target(tdtg)));
    // e0:SetOperation(s.tdop)
    effect::set_operation(d, e0, Some(operation(tdop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Both players take battle damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_BOTH_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_BOTH_BATTLE_DAMAGE);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Halve battle damage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetValue(HALF_DAMAGE)
    effect::set_value(d, e2, HALF_DAMAGE as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Speedroid" from the Extra Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WIND) and c:IsType(TYPE_SYNCHRO) and (c:IsFaceup() or not c:IsLocation(LOCATION_REMOVED)) and c:IsAbleToDeck()
    card::is_attribute(d, c, ATTRIBUTE_WIND)
        && card::is_type(d, c, TYPE_SYNCHRO)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_REMOVED))
        && card::is_able_to_deck(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED) and chkc:IsControler(tp) and s.tdfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE | LOCATION_REMOVED) && card::is_controler(d, chkc, tp) && tdfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_target(d, tp, Some(&tdfilter), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsPreviousLocation(LOCATION_PZONE)
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, h, LOCATION_PZONE)
}
/// `s.spfilter(c,e,tp)`: the Lua `rp` in this filter is a global, hence nil
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_PENDULUM) and c:IsSetCard(SET_SPEEDROID)
    //   and Duel.GetLocationCountFromEx(tp,rp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_set_card(d, c, &[SET_SPEEDROID as u16])
        && duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,rp) end
    if a.chk == Some(0) {
        let e = a.e;
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,rp)
    let e = a.e;
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
