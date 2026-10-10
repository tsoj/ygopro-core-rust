//! Kashtira Akstra
use super::prelude::*;

pub const ID: u32 = 82286798;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_VISAS_STARFROST, 65815684], listed_series: &[SET_KASHTIRA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 1 "Visas Starfrost"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e1, EVENT_DESTROYED);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.rmcon)
    effect::set_condition(d, e1, Some(condition(rmcon)));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 banished "Visas Starfrost" to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.rmconfilter(c,tp)
fn rmconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:GetPreviousTypeOnField()&TYPE_XYZ>0
    //   and c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousSetCard(SET_KASHTIRA)
    //   and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && (card::get_previous_type_on_field(d, c) & TYPE_XYZ) != 0
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_set_card(d, c, &[SET_KASHTIRA as u16])
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
}
// function s.rmcon(e,tp,eg,ep,ev,re,r,rp)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.rmconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| rmconfilter(d, c, a.tp), 1, NO_EX)
}
// function s.rmfilter(c,e,tp)
fn rmfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAbleToRemove() and c:IsCode(CARD_VISAS_STARFROST)
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    card::is_able_to_remove(d, c, None, None, None)
        && card::is_code(d, c, &[CARD_VISAS_STARFROST])
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.spfilter(c,e,tp,mc)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // return c:IsCode(65815684) and c:IsCanBeSpecialSummoned(e,0,tp,true,false)
    //   and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    card::is_code(d, c, &[65815684])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.map_or(NO_EX, |m| m.into()), duel::ExtraCard::Card(c), None).0 > 0
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| rmfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_MZONE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local rg=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_MZONE,0,1,1,nil,e,tp)
    let rg = duel::select_matching_card(d, a.tp, Some(&|d, x| rmfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #rg==0 or Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)==0 or not rg:GetFirst():IsLocation(LOCATION_REMOVED) then return end
    if rg.is_empty() {
        return;
    }
    if duel::remove(d, rg.clone(), POS_FACEUP as u8, REASON_EFFECT, None, None) == 0 {
        return;
    }
    let first = group::get_first(d, &rg).nil();
    if !card::is_location(d, first, LOCATION_REMOVED) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    // (no mc argument: the Lua passes nil for it)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, None)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_VISAS_STARFROST) and c:IsAbleToHand()
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_VISAS_STARFROST]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, a.tp, 0);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(tc), None, REASON_EFFECT, None);
    }
}
