//! T.G. Trident Launcher
use super::prelude::*;

pub const ID: u32 = 50750868;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_TG],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2,nil,s.lcheck)
    link::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))),
        2,
        None,
        Some(group_check(|d, g, lc, st, tp| lcheck(d, g, lc, st, tp))),
        None,
        None,
    );
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // cannot be target/battle indestructable
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(effect_card_fn(|d, e, c| tgtg(d, e, c))));
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.lcheck(g,lc,sumtype,tp)
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(s.mzfilter,1,nil,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| mzfilter(d, c, lc, sumtype, tp), 1, NO_EX)
}
// function s.mzfilter(c,lc,sumtype,tp)
fn mzfilter(d: &mut Duel, c: CardId, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_TG,lc,sumtype,tp) and c:IsType(TYPE_TUNER,lc,sumtype,tp)
    card::is_set_card_ctx(d, c, &[SET_TG as u16], Some(lc), sumtype, Some(tp))
        && card::is_type_ctx(d, c, TYPE_TUNER, Some(lc), sumtype, Some(tp))
}
// function s.tgtg(e,c)
fn tgtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return e:GetHandler():GetLinkedGroup():IsContains(c) and c:IsSetCard(SET_TG) and c:IsType(TYPE_SYNCHRO)
    let h = effect::get_handler(d, e).nil();
    let linked = card::get_linked_group(d, h);
    group::is_contains(&linked, c) && card::is_set_card(d, c, &[SET_TG as u16]) && card::is_type(d, c, TYPE_SYNCHRO)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
// function s.spfilter(c,e,tp,zone)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsSetCard(SET_TG) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE,tp,zone)
    card::is_set_card(d, c, &[SET_TG as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(tp), Some(zone))
}
// function s.spcheck(sg,e,tp,mg)
fn spcheck(d: &mut Duel, sg: &CardSet) -> bool {
    // return sg:GetClassCount(Card.GetLocation)==#sg
    group::get_class_count(d, sg, &|d, c| card::get_location(d, c) as i64) == group::get_count(sg)
}
// Rescon wrapper for s.spcheck (one return value: (res, false))
fn spcheck_rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (spcheck(d, sg), false)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local zone=e:GetHandler():GetLinkedZone(tp)&ZONES_MMZ
        let h = effect::get_handler(d, a.e).nil();
        let zone = card::get_linked_zone(d, h, Some(a.tp)) & ZONES_MMZ;
        // local ct=Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)
        let ct = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0;
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp,zone)
        let e = a.e;
        let tp = a.tp;
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp, zone)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
        // return ct>2 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and aux.SelectUnselectGroup(g,e,tp,3,3,s.spcheck,0)
        return ct > 2
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&spcheck_rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,3,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 3, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    let tp = a.tp;
    let e = a.e;
    // local zone=e:GetHandler():GetLinkedZone(tp)&ZONES_MMZ
    let zone = card::get_linked_zone(d, c, Some(tp)) & ZONES_MMZ;
    // local ct=Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_TOFIELD,zone)
    let _ct = duel::get_location_count(d, tp, LOCATION_MZONE as u8, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0;
    // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp,zone)
    let sg = duel::get_matching_group(
        d,
        Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp, zone))),
        tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        NO_EX,
    );
    // if #sg==0 then return end
    if group::get_count(&sg) == 0 {
        return;
    }
    // local rg=aux.SelectUnselectGroup(sg,e,tp,3,3,s.spcheck,1,tp,HINTMSG_SPSUMMON)
    let rg = aux::select_unselect_group(d, &sg, Some(e), tp, Some(3), Some(3), Some(&spcheck_rescon), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SpecialSummon(rg,0,tp,tp,true,false,POS_FACEUP_DEFENSE,zone)
    duel::special_summon(d, rg, 0, tp, tp, true, false, POS_FACEUP_DEFENSE as u8, Some(zone));
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| splimit(d, c))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
// function s.splimit(e,c)
fn splimit(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsSetCard(SET_TG)
    !card::is_set_card(d, c, &[SET_TG as u16])
}
