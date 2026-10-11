//! Cosmic Quasar Dragon
use super::prelude::*;

pub const ID: u32 = 37442336;
pub const INFO: ScriptInfo = ScriptInfo { synchro_tuner_required: Some(1), synchro_nt_required: Some(2), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_SYNCHRO),1,1,Synchro.NonTunerEx(Card.IsType,TYPE_SYNCHRO),2,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_SYNCHRO, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_SYNCHRO, sc, st, Some(tp)))),
        2,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Must first be Synchro Summoned with the above materials
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(s.splimit)
    effect::set_value_fn(d, e1, lua_fn(splimit));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the effects of face-up cards on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_CANNOT_INACTIVATE+EFFECT_FLAG_CANNOT_NEGATE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(
        d,
        e2,
        EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_CANNOT_INACTIVATE + EFFECT_FLAG_CANNOT_NEGATE + EFFECT_FLAG_CANNOT_DISABLE,
        0,
    );
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 Dragon Synchro Monster from your Extra Deck that lists 2+ non-Tuner Synchros as materials
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e3:SetCondition(function(e) return e:GetHandler():IsSynchroSummoned() end)
    effect::set_condition(d, e3, Some(condition(e3cond)));
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.splimit(e,se,sp,st)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not e:GetHandler():IsLocation(LOCATION_EXTRA) or ((st&SUMMON_TYPE_SYNCHRO)==SUMMON_TYPE_SYNCHRO and not se)
    let h = effect::get_handler(d, a.e(0)).nil();
    let st = a.int(3) as u64;
    !card::is_location(d, h, LOCATION_EXTRA)
        || ((st & SUMMON_TYPE_SYNCHRO as u64) == SUMMON_TYPE_SYNCHRO as u64 && a.v(1).as_effect().is_none())
}
// function s.negtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsNegatable() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_negatable(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:IsSynchroSummoned() and c:GetMaterialCount() or 0
    let ct = if card::is_synchro_summoned(d, c) { card::get_material_count(d, c) as u16 } else { 0 };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,ct+1,nil)
    let g = duel::select_target(
        d,
        a.tp,
        Some(&|d, x| card::is_negatable(d, x)),
        a.tp,
        LOCATION_ONFIELD,
        LOCATION_ONFIELD,
        1,
        ct + 1,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
// function s.negop(e,tp,eg,ep,ev,re,r,rp)
fn negop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e):Match(Card.IsCanBeDisabledByEffect,nil,e)
    let mut g = duel::get_target_cards(d, a.e).nil();
    group::match_(d, &mut g, &|d, c| card::is_can_be_disabled_by_effect(d, c, a.e, true), NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // Negate its effects
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
// function s.spfilter(c,e,tp,mc)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    // local mt=c:GetMetatable()
    // return mt.synchro_nt_required and mt.synchro_nt_required>=2 and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //   and c:IsRace(RACE_DRAGON) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    let nt = crate::card_info(d, c).synchro_nt_required;
    if !nt.is_some_and(|n| n >= 2) {
        return false;
    }
    let ex = match mc {
        Some(m) => m.into(),
        None => NO_EX,
    };
    duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_race(d, c, RACE_DRAGON)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| spfilter(d, c, a.e, a.tp, Some(h))),
            a.tp,
            LOCATION_EXTRA,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, SUMMON_TYPE_SYNCHRO, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function(e) return e:GetHandler():IsSynchroSummoned() end
fn e3cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h)
}
