//! Naturia Blessing
use super::prelude::*;

pub const ID: u32 = 34813545;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NATURIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.acttg)
    effect::set_target(d, e1, Some(cost_target(acttg)));
    // e1:SetOperation(s.actop)
    effect::set_operation(d, e1, Some(operation(actop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NATURIA) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_NATURIA as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)` (`check` is `chk==0`)
fn sp_tg_impl(d: &mut Duel, e: EffectId, tp: u8, check: bool) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if check {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, e: EffectId, tp: u8) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp));
    let g = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.syncheck(tp,sg,sc)`
fn syncheck(d: &mut Duel, a: Args) -> bool {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_NATURIA)
    let sg = a.g(1);
    group::is_exists(d, &sg, &|d, x| card::is_set_card(d, x, &[SET_NATURIA as u16]), 1, NO_EX)
}
/// `s.syntg(e,tp,eg,ep,ev,re,r,rp,chk)` (`check` is `chk==0`)
fn syn_tg_impl(d: &mut Duel, tp: u8, check: bool) -> bool {
    // if chk==0 then
    if check {
        // Synchro.CheckAdditional=s.syncheck
        synchro::set_check_additional(d, Some(lua_fn(syncheck)));
        // local res=Duel.IsExistingMatchingCard(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,1,nil)
        let res = duel::is_existing_matching_card(d, Some(&|d, c| card::is_synchro_summonable(d, c, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
        // Synchro.CheckAdditional=nil
        synchro::set_check_additional(d, None);
        // return res
        return res;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.synop(e,tp,eg,ep,ev,re,r,rp)`
fn synop(d: &mut Duel, tp: u8) {
    // Synchro.CheckAdditional=s.syncheck
    synchro::set_check_additional(d, Some(lua_fn(syncheck)));
    // local g=Duel.GetMatchingGroup(Card.IsSynchroSummonable,tp,LOCATION_EXTRA,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_synchro_summonable(d, c, None, None, 0, 0)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SynchroSummon(tp,sg:GetFirst())
        let sc = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, tp, sc, None, None, 0, 0);
    }
    // Synchro.CheckAdditional=nil
    synchro::set_check_additional(d, None);
}
/// `s.fusextra(e,tp,mg)`: returns `nil, s.fuscheck`
fn fusextra(_d: &mut Duel, _e: EffectId, _tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    (None, Some(lua_fn(fuscheck)))
}
/// `s.fuscheck(tp,sg,fc)`
fn fuscheck(d: &mut Duel, a: Args) -> bool {
    // return sg:IsExists(aux.FilterBoolFunction(Card.IsSetCard,SET_NATURIA,fc,SUMMON_TYPE_FUSION,tp),1,nil)
    let tp = a.p(0);
    let sg = a.g(1);
    let fc = a.card(2);
    group::is_exists(
        d,
        &sg,
        &|d, x| card::is_set_card_ctx(d, x, &[SET_NATURIA as u16], fc, SUMMON_TYPE_FUSION as u64, Some(tp)),
        1,
        NO_EX,
    )
}
/// `Fusion.SummonEffTG(nil,Fusion.OnFieldMat,s.fusextra)` / `Fusion.SummonEffOP(...)`
fn fus_params() -> fusion::SummonEff {
    fusion::SummonEff {
        matfilter: Some(fusion::on_field_mat()),
        extrafil: Some(std::sync::Arc::new(fusextra)),
        ..Default::default()
    }
}
/// `s.fustg(e,tp,eg,ep,ev,re,r,rp,0)`: the check call of the Fusion target
fn fus_tg_check(d: &mut Duel, a: &EffArgs) -> bool {
    let a0 = EffArgs { e: a.e, tp: a.tp, eg: a.eg.clone(), ep: a.ep, ev: a.ev, re: a.re, r: a.r, rp: a.rp, chk: Some(0), extra: vec![] };
    fusion::summon_eff_tg_call(d, &fus_params(), &a0)
}
/// `s.acttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn acttg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local b1=s.sptg(e,tp,eg,ep,ev,re,r,rp,0)
    let b1 = sp_tg_impl(d, e, tp, true);
    // local b2=s.syntg(e,tp,eg,ep,ev,re,r,rp,0)
    let b2 = syn_tg_impl(d, tp, true);
    // local b3=s.fustg(e,tp,eg,ep,ev,re,r,rp,0)
    let b3 = fus_tg_check(d, a);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)},
    //     {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // s.sptg(e,tp,eg,ep,ev,re,r,rp,1)
        sp_tg_impl(d, e, tp, false);
    // elseif op==2 then
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // s.syntg(e,tp,eg,ep,ev,re,r,rp,1)
        syn_tg_impl(d, tp, false);
    // elseif op==3 then
    } else if op == Some(3) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
        // s.sptg(e,tp,eg,ep,ev,re,r,rp,1)
        sp_tg_impl(d, e, tp, false);
    }
    // Duel.SetTargetParam(op)
    duel::set_target_param(d, op.unwrap_or(0) as u32);
    true
}
/// `s.actop(e,tp,eg,ep,ev,re,r,rp,chk)`
fn actop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local op=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let op = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // if op==1 then s.spop(...)
    if op == 1 {
        spop(d, e, tp);
    // elseif op==2 then s.synop(...)
    } else if op == 2 {
        synop(d, tp);
    // elseif op==3 then s.fusop(...) end
    } else if op == 3 {
        // Fusion.SummonEffOP
        fusion::summon_eff_op_call(d, &fus_params(), a);
    }
}
