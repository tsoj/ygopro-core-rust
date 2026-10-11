//! Necroid Synchro
use super::prelude::*;

pub const ID: u32 = 26194151;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_STARDUST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local lv=c:GetLevel()
    let _lv = card::get_level(d, c);
    // return c:IsSetCard(SET_STARDUST) and c:IsType(TYPE_SYNCHRO) and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,tp,c)
    card::is_set_card(d, c, &[SET_STARDUST as u16])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, tp, c)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.rescon(tuner,scard)`: the function it returns, `function(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, tp: u8, tuner: CardId, scard: CardId) -> bool {
    // sg:AddCard(tuner)
    let mut sg = sg.clone();
    group::add_card(&mut sg, &CardSet::from_card(tuner));
    // local res=Duel.GetLocationCountFromEx(tp,tp,sg,scard)>0
    //     and sg:CheckWithSumEqual(Card.GetLevel,scard:GetLevel(),#sg,#sg)
    let n = sg.len() as i32;
    let res = duel::get_location_count_from_ex(d, tp, Some(tp), (&sg).into(), duel::ExtraCard::Card(scard), None).0 > 0 && {
        let lv = card::get_level(d, scard);
        group::check_with_sum_equal(d, &sg, &|d, x| card::get_level(d, x) as i64, lv, n, n).0
    };
    // sg:RemoveCard(tuner)
    // return res
    res
}
/// `s.filter2(c,tp,sc)`
fn filter2(d: &mut Duel, c: CardId, tp: u8, sc: CardId) -> bool {
    // local rg=Duel.GetMatchingGroup(s.filter3,tp,LOCATION_MZONE|LOCATION_GRAVE,0,c)
    let rg = duel::get_matching_group(d, Some(&filter3), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, c.into());
    // return c:IsType(TYPE_TUNER) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    //     and aux.SelectUnselectGroup(rg,e,tp,nil,2,s.rescon(c,sc),0)
    // (the Lua `e` here is not a parameter of filter2: it is the global nil)
    if !(card::is_type(d, c, TYPE_TUNER)
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false))
    {
        return false;
    }
    let r = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>| (rescon(d, sg, tp, c, sc), false);
    matches!(aux::select_unselect_group(d, &rg, None, tp, None, Some(2), Some(&r), 0, tp, None, None, None, false), Err(true))
}
/// `s.filter3(c)`
fn filter3(d: &mut Duel, c: CardId) -> bool {
    // return c:HasLevel() and not c:IsType(TYPE_TUNER) and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    card::has_level(d, c) && !card::is_type(d, c, TYPE_TUNER) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,nil,nil,REASON_SYNCHRO)
        let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::new()), a.tp, None, None, REASON_SYNCHRO);
        // return #pg<=0 and Duel.IsExistingMatchingCard(s.filter1,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
        return pg.is_empty() && duel::is_existing_matching_card(d, Some(&|d, c| filter1(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,nil,nil,REASON_SYNCHRO)
    let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::new()), a.tp, None, None, REASON_SYNCHRO);
    // if #pg>0 then return end
    if !pg.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g1=Duel.SelectMatchingCard(tp,s.filter1,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g1 = duel::select_matching_card(d, a.tp, Some(&|d, c| filter1(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=g1:GetFirst()
    let sc = group::get_first(d, &g1);
    // if sc then
    if let Some(sc) = sc {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
        // local g2=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tp,sc)
        let g2 = duel::select_matching_card(d, a.tp, Some(&|d, c| filter2(d, c, a.tp, sc)), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        // local tuner=g2:GetFirst()
        let tuner = group::get_first(d, &g2).nil();
        // local rg=Duel.GetMatchingGroup(s.filter3,tp,LOCATION_MZONE|LOCATION_GRAVE,0,tuner)
        let rg = duel::get_matching_group(d, Some(&filter3), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, tuner.into());
        // local sg=aux.SelectUnselectGroup(rg,e,tp,1,2,s.rescon(tuner,sc),1,tp,HINTMSG_REMOVE,s.rescon(tuner,sc))
        let r1 = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>| (rescon(d, sg, tp, tuner, sc), false);
        let r2 = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>| (rescon(d, sg, tp, tuner, sc), false);
        let mut sg = aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(1), Some(2), Some(&r1), 1, a.tp, Some(HINTMSG_REMOVE as u64), Some(&r2), None, false)
            .unwrap_or_default();
        // sg:AddCard(tuner)
        group::add_card(&mut sg, &CardSet::from_card(tuner));
        // Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        // Duel.SpecialSummonStep(sc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, sc, SUMMON_TYPE_SYNCHRO, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let handler = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, handler);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e1,true)
        card::register_effect(d, sc, e1, true);
        // local e2=Effect.CreateEffect(e:GetHandler())
        let e2 = effect::create_effect(d, handler);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // sc:RegisterEffect(e2,true)
        card::register_effect(d, sc, e2, true);
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
