//! Shining Hope Road
//! 輝望道
use super::prelude::*;

pub const ID: u32 = 62829077;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UTOPIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON])[0] == 0;
    }
    let h = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(|d, a| splimit(d, a))));
    // e1:SetLabelObject(e)
    effect::set_label_object(d, e1, Value::Effect(a.e));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, h, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args<'_>) -> bool {
    // return se~=e:GetLabelObject() and c:GetFlagEffect(id)==0
    let se = a.v(6).as_effect();
    let label = effect::get_label_object(d, a.e(0)).as_effect();
    se != label && card::get_flag_effect(d, a.c(1), ID) == 0
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_effect_target(d, c, Some(e)) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.xyzfilter(c,mg,tp)`
fn xyzfilter(d: &mut Duel, c: CardId, mg: &CardSet) -> bool {
    // return c:IsSetCard(SET_UTOPIC) and c:IsXyzSummonable(nil,mg,3,3)
    card::is_set_card(d, c, &[SET_UTOPIC as u16]) && card::is_xyz_summonable(d, c, None, Some(mg.clone()), 3, 3)
}
/// `s.rescon(exg)`: the returned `function(sg)`
fn rescon(d: &mut Duel, sg: &CardSet, exg: &CardSet) -> bool {
    // return #sg==3 and exg:IsExists(Card.IsXyzSummonable,1,nil,nil,sg,3,3)
    sg.len() == 3 && group::is_exists(d, exg, &|d, c| card::is_xyz_summonable(d, c, None, Some(sg.clone()), 3, 3), 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local mg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let mg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // local exg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil,mg)
    let exg = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, &mg)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if chk==0 then return #exg>0
    //     and Duel.IsPlayerCanSpecialSummonCount(tp,2)
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>2 end
    if a.chk == Some(0) {
        return !exg.is_empty()
            && duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 2;
    }
    // local sg=aux.SelectUnselectGroup(mg,e,tp,3,3,s.rescon(exg),1,tp,HINTMSG_SPSUMMON)
    let exg_c = exg.clone();
    let rs = move |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| (rescon(d, sg, &exg_c), false);
    let rs: aux::Rescon = &rs;
    let sg = aux::select_unselect_group(d, &mg, Some(a.e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg), 2, 0, 0);
    true
}
/// `s.filter2(c,e,tp)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<3 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 3 {
        return;
    }
    // local g=Duel.GetTargetCards(e):Match(s.filter2,nil,e,tp)
    let mut g = duel::get_target_cards(d, a.e).unwrap_or_default();
    group::match_(d, &mut g, &|d, c| filter2(d, c, a.e, tp), NO_EX);
    // if #g~=3 then return end
    if g.len() != 3 {
        return;
    }
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        let h = effect::get_handler(d, a.e).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
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
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // local xyzg=Duel.GetMatchingGroup(s.xyzfilter,tp,LOCATION_EXTRA,0,nil,g,tp)
    let g_c = g.clone();
    let xyzg = duel::get_matching_group(d, Some(&|d, c| xyzfilter(d, c, &g_c)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #xyzg>0 then
    if !xyzg.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local xyz=xyzg:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &xyzg, tp, 1, 1, false, NO_EX).nil();
        let xyz = group::get_first(d, &sel).nil();
        // xyz:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, xyz, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // Duel.XyzSummon(tp,xyz,g,nil,3,3)
        duel::xyz_summon(d, tp, xyz, Some(g), None, 3, 3);
    }
}
