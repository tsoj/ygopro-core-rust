//! Skilled Blue Magician
use super::prelude::*;

pub const ID: u32 = 88901771;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_GAIA_THE_FIERCE_KNIGHT}
    listed_series: &[SET_GAIA_THE_FIERCE_KNIGHT],
    // s.counter_place_list={COUNTER_SPELL}
    counter_place_list: &[COUNTER_SPELL as u32],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_SPELL)
    card::enable_counter_permit(d, c, COUNTER_SPELL as u16, None, None);
    // c:SetCounterLimit(COUNTER_SPELL,3)
    card::set_counter_limit(d, c, COUNTER_SPELL as u16, 3);
    // add counter
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_CHAINING)
    effect::set_code(d, e0, EVENT_CHAINING);
    // e0:SetRange(LOCATION_MZONE)
    effect::set_range(d, e0, LOCATION_MZONE);
    // e0:SetOperation(aux.chainreg)
    effect::set_operation(d, e0, Some(aux::chainreg()));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.acop)
    effect::set_operation(d, e1, Some(operation(acop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // counter
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.cttg)
    effect::set_target(d, e3, Some(cost_target(cttg)));
    // e3:SetOperation(s.ctop)
    effect::set_operation(d, e3, Some(operation(ctop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.acop(e,tp,eg,ep,ev,re,r,rp)
fn acop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let c = effect::get_handler(d, a.e).nil();
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and e:GetHandler():GetFlagEffect(1)>0 then
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
        && effect::is_spell_effect(d, re)
        && card::get_flag_effect(d, c, 1) > 0
    {
        // e:GetHandler():AddCounter(COUNTER_SPELL,1)
        card::add_counter(d, c, COUNTER_SPELL as u16, 1, false);
    }
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():GetCounter(COUNTER_SPELL)==3 and e:GetHandler():IsReleasable() end
    if a.chk == Some(0) {
        return card::get_counter(d, c, COUNTER_SPELL as u16) == 3 && card::is_releasable(d, c, None, None);
    }
    // Duel.Release(e:GetHandler(),REASON_COST)
    duel::release(d, c, REASON_COST, None);
    true
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GAIA_THE_FIERCE_KNIGHT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_GAIA_THE_FIERCE_KNIGHT as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    if card::get_sequence(d, c) < 5 {
        ft += 1;
    }
    // if chk==0 then return ft>0 and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return ft > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, x| filter(d, x, a.e, tp))),
        tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.ctfilter(c)
fn ctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCanAddCounter(COUNTER_SPELL,1)
    card::is_faceup(d, c) && card::is_can_add_counter(d, c, COUNTER_SPELL as u16, 1, false, 0)
}
// function s.cttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and s.ctfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp) && ctfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.ctfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&ctfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,94)
    duel::hint(d, HINT_SELECTMSG as u8, tp, 94);
    // Duel.SelectTarget(tp,s.ctfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    duel::select_target(d, tp, Some(&ctfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,0,COUNTER_SPELL)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, COUNTER_SPELL as i32);
    true
}
// function s.ctop(e,tp,eg,ep,ev,re,r,rp)
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // tc:AddCounter(COUNTER_SPELL,1)
            card::add_counter(d, tc, COUNTER_SPELL as u16, 1, false);
        }
    }
}
