//! コンタクト・ゲート (Contact Gate)
use super::prelude::*;

pub const ID: u32 = 41933425;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NEO_SPACIAN],
    listed_names: &[CARD_NEOS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 banished "Neo-Spacian"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_TO_DECK)
    effect::set_code(d, e2, EVENT_TO_DECK);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NEO_SPACIAN) and c:IsMonster() and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16]) && card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NEO_SPACIAN) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_NEO_SPACIAN as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.rescon1(sg,e,tp,mg)`
fn rescon1(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, mg: &CardSet, c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,sg,e,tp)
    let sg_ex: Exclude = sg.into();
    let g = duel::get_matching_group(d, Some(&|d, x| filter(d, x, e, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, sg_ex);
    // return aux.ChkfMMZ(2)(sg,e,tp,mg) and sg:GetClassCount(Card.GetCode)==2
    //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon2,0)
    let chk = aux::chkf_mmz(2)(d, sg, Some(e), tp, mg, c).0;
    let res = chk
        && group::get_class_count(d, sg, &|d, x| card::get_code(d, x)[0] as i64) == 2
        && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon2), 0, tp, None, None, None, false), Err(true));
    (res, false)
}
/// `s.rescon2(sg,e,tp,mg)`
fn rescon2(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetCode)==2
    (group::get_class_count(d, sg, &|d, x| card::get_code(d, x)[0] as i64) == 2, false)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local rg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE|LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_GRAVE | LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon1,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon1), 0, tp, None, None, None, false), Err(true));
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,s.rescon1,1,tp,HINTMSG_REMOVE)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&rescon1), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none();
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
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
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // lizard check
    // aux.addTempLizardCheck(e:GetHandler(),tp,s.lizfilter)
    aux::add_temp_lizard_check(d, h, tp, Some(effect_card_fn(lizfilter)), None, None, None, None);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp)
    let nv = aux::necro_valley_filter(move |d: &mut Duel, x: CardId| filter(d, x, e, tp));
    let g = duel::get_matching_group(d, Some(&nv), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon2,1,tp,HINTMSG_SPSUMMON)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&rescon2), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #sg==2 then
    if sg.len() == 2 {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsType(TYPE_FUSION) and c:IsLocation(LOCATION_EXTRA)
    !card::is_type(d, c, TYPE_FUSION) && card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsOriginalType(TYPE_FUSION)
    !card::is_original_type(d, c, TYPE_FUSION)
}
/// `s.spcfilter(c,tp)`
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsType(TYPE_FUSION) and c:ListsCodeAsMaterial(CARD_NEOS)
    //     and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_ONFIELD)
    card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::is_type(d, c, TYPE_FUSION)
        && card::lists_code_as_material(d, c, &[CARD_NEOS])
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.spcfilter,1,nil,tp)
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| spcfilter(d, c, tp), 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return s.filter(c,e,tp) and c:IsFaceup()
    filter(d, c, e, tp) && card::is_faceup(d, c)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_REMOVED as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
