//! Ghostrick Festival
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 35871958;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GHOSTRICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| matfilter(d, c, sc, st, tp))), 1, Some(1), None, None, None);
    // extra mat
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetCode(EFFECT_EXTRA_MATERIAL)
    effect::set_code(d, e0, EFFECT_EXTRA_MATERIAL);
    // e0:SetProperty(EFFECT_FLAG_PLAYER_TARGET|EFFECT_FLAG_CANNOT_DISABLE|EFFECT_FLAG_SET_AVAILABLE)
    effect::set_property(d, e0, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_SET_AVAILABLE, 0);
    // e0:SetTargetRange(1,0)
    effect::set_target_range(d, e0, 1, 0);
    // e0:SetValue(s.extraval)
    effect::set_value_fn(d, e0, lua_fn(extraval));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // direct attack
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCondition(s.dacon)
    effect::set_condition(d, e1, Some(condition(dacon)));
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_GHOSTRICK))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_GHOSTRICK as u16]))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.matfilter(c,lc,stype,tp)
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, stype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_GHOSTRICK,lc,stype,tp) and not c:IsType(TYPE_LINK,lc,stype,tp)
    card::is_set_card_ctx(d, c, &[SET_GHOSTRICK as u16], lc, stype, Some(tp)) && !card::is_type_ctx(d, c, TYPE_LINK, lc, stype, Some(tp))
}
// function s.extraval(chk,summon_type,e,...)
fn extraval(d: &mut Duel, a: Args) -> Value {
    // if chk==0 then
    if a.int(0) == 0 {
        let summon_type = a.int(1);
        let e = a.e(2);
        // local tp,sc=...
        let tp = a.p(3);
        let sc = a.card(4);
        // local c=e:GetHandler()
        let h = effect::get_handler(d, e);
        // if summon_type~=SUMMON_TYPE_LINK or sc~=e:GetHandler() then
        if summon_type != SUMMON_TYPE_LINK as i64 || sc != h {
            // return Group.CreateGroup()
            return Value::Group(group::create_group(&[]));
        }
        // return Duel.GetMatchingGroup(Card.IsFacedown,tp,LOCATION_MZONE,0,nil)
        return Value::Group(duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c)), tp, LOCATION_MZONE, 0, NO_EX));
    }
    Value::Nil
}
// function s.dacon(e,tp,eg,ep,ev,re,r,rp)
fn dacon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_GHOSTRICK),0,LOCATION_FZONE,LOCATION_FZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GHOSTRICK as u16])), 0, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_GHOSTRICK) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
    card::is_set_card(d, c, &[SET_GHOSTRICK as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetMZoneCount(tp,e:GetHandler())>0
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        // and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
        return duel::get_m_zone_count(d, a.tp, h.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)
        duel::special_summon(d, g.clone(), 0, a.tp, a.tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
