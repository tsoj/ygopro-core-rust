//! Fusion Armament
use super::prelude::*;

pub const ID: u32 = 61070601;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon listed Fusion material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.exfilter(c,e,tp,filter_func)`
fn exfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, filter_func: &dyn Fn(&mut Duel, CardId, EffectId, u8, CardId) -> bool) -> bool {
    // return c.material and c:IsType(TYPE_FUSION) and not c:IsPublic()
    //     and Duel.IsExistingMatchingCard(filter_func,tp,LOCATION_EXTRA|LOCATION_GRAVE,0,1,nil,e,tp,c)
    !crate::class::material(d, c).is_empty()
        && card::is_type(d, c, TYPE_FUSION)
        && !card::is_public(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter_func(d, x, e, tp, c)), tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,fc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, fc: CardId) -> bool {
    // if not (c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsCode(table.unpack(fc.material))) then return false end
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
        return false;
    }
    let mat = crate::class::material(d, fc);
    if !card::is_code(d, c, &mat) {
        return false;
    }
    // if c:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, c, LOCATION_GRAVE) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // elseif c:IsLocation(LOCATION_EXTRA) then
    if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
        return duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0;
    }
    false
}
/// `aux.NecroValleyFilter(s.spfilter)`
fn necro_spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, fc: CardId) -> bool {
    aux::necro_valley_filter(|d, x| spfilter(d, x, e, tp, fc))(d, c)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local filter_func=s.spfilter
    // if chk==0 then return Duel.IsExistingMatchingCard(s.exfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,filter_func) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| exfilter(d, c, a.e, a.tp, &spfilter)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_EXTRA | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local filter_func=aux.NecroValleyFilter(s.spfilter)
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local rc=Duel.SelectMatchingCard(tp,s.exfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,filter_func):GetFirst()
    let rc_group = duel::select_matching_card(d, a.tp, Some(&|d, c| exfilter(d, c, a.e, a.tp, &necro_spfilter)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &rc_group);
    // if not rc then return end
    let Some(rc) = rc else { return };
    // Duel.ConfirmCards(1-tp,rc)
    duel::confirm_cards(d, 1 - a.tp, rc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA|LOCATION_GRAVE,0,1,1,rc,e,tp,rc):GetFirst()
    let sc_group = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, rc)), a.tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, 1, 1, false, rc.into()).nil();
    let sc = group::get_first(d, &sc_group);
    // if sc and Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(sc) = sc {
        if duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, a.e).nil();
            // local ct=Duel.IsTurnPlayer(tp) and 2 or 1
            let ct: u16 = if duel::is_turn_player(d, a.tp) { 2 } else { 1 };
            // Cannot attack
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3206)
            effect::set_description(d, e1, 3206);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END,ct)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, ct);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
            // Its effects are negated
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e2, EFFECT_DISABLE);
            // e2:SetReset(RESETS_STANDARD_PHASE_END,ct)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, ct);
            // sc:RegisterEffect(e2)
            card::register_effect(d, sc, e2, false);
            // local e3=e2:Clone()
            let e3 = effect::clone(d, e2);
            // e3:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
            // e3:SetValue(RESET_TURN_SET)
            effect::set_value(d, e3, RESET_TURN_SET as i64);
            // sc:RegisterEffect(e3)
            card::register_effect(d, sc, e3, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
