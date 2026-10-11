//! Magikey Locking
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 40493210;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGIKEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsType(TYPE_TOKEN) and (c:IsType(TYPE_NORMAL) or c:IsSetCard(SET_MAGIKEY))
    !card::is_type(d, c, TYPE_TOKEN) && (card::is_type(d, c, TYPE_NORMAL) || card::is_set_card(d, c, &[SET_MAGIKEY as u16]))
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.costfilter,1,false,aux.ReleaseCheckMMZ,nil) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, false, Some(&aux::release_check_mmz), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.costfilter,1,1,false,aux.ReleaseCheckMMZ,nil)
    let g = duel::select_release_group_cost(d, a.tp, Some(&costfilter), 1, 1, false, Some(&aux::release_check_mmz), NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and c:HasLevel() and (c:IsType(TYPE_NORMAL) or c:IsSetCard(SET_MAGIKEY))
    //     and c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_monster(d, c)
        && card::has_level(d, c)
        && (card::is_type(d, c, TYPE_NORMAL) || card::is_set_card(d, c, &[SET_MAGIKEY as u16]))
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetSum(Card.GetLevel)<=8
    (group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64) <= 8, false)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local tg=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let tg = duel::get_matching_group(d, Some(&|d, c| filter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(tg,e,tp,1,1,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &tg, Some(a.e), a.tp, Some(1), Some(1), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_MZONE),2)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0.min(2);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or not aux.SelectUnselectGroup(tg,e,tp,2,2,s.rescon,0) then
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
        || !matches!(aux::select_unselect_group(d, &tg, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false), Err(true))
    {
        // ft=1
        ft = 1;
    }
    // local g=aux.SelectUnselectGroup(tg,e,tp,1,ft,s.rescon,1,tp,HINTMSG_SPSUMMON)
    let g = aux::select_unselect_group(d, &tg, Some(a.e), a.tp, Some(1), Some(ft as usize), Some(&rescon), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), n, 0, 0);
    true
}
// function s.exfilter(c,sfunc)
fn exfilter(d: &mut Duel, c: CardId, sfunc: &dyn Fn(&mut Duel, CardId) -> bool) -> bool {
    // return c:IsSetCard(SET_MAGIKEY) and sfunc(c)
    card::is_set_card(d, c, &[SET_MAGIKEY as u16]) && sfunc(d, c)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g<=0 or Duel.GetLocationCount(tp,LOCATION_MZONE)<#g or (#g>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then return end
    if g.is_empty()
        || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < g.len() as i32
        || (g.len() > 1 && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some())
    {
        return;
    }
    let n = g.len() as i32;
    // if Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE)==#g then
    if duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) == n {
        // local sg=Duel.GetMatchingGroup(s.exfilter,tp,LOCATION_EXTRA,0,nil,Card.IsSynchroSummonable)
        let sg = duel::get_matching_group(d, Some(&|d, c| exfilter(d, c, &|d, c| card::is_synchro_summonable(d, c, None, None, 0, 0))), a.tp, LOCATION_EXTRA, 0, NO_EX);
        // local xg=Duel.GetMatchingGroup(s.exfilter,tp,LOCATION_EXTRA,0,nil,Card.IsXyzSummonable)
        let xg = duel::get_matching_group(d, Some(&|d, c| exfilter(d, c, &|d, c| card::is_xyz_summonable(d, c, None, None, 0, 0))), a.tp, LOCATION_EXTRA, 0, NO_EX);
        // local opt1,opt2=#sg>0,#xg>0
        let opt1 = !sg.is_empty();
        let opt2 = !xg.is_empty();
        // if (opt1 or opt2) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if (opt1 || opt2) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local opt
            let opt: i32;
            // if opt1 and opt2 then
            if opt1 && opt2 {
                // opt=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))
                opt = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)]);
            // elseif opt1 and not opt2 then
            } else if opt1 && !opt2 {
                // opt=Duel.SelectOption(tp,aux.Stringid(id,2))
                opt = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)]);
            // elseif opt2 and not opt1 then
            } else {
                // opt=Duel.SelectOption(tp,aux.Stringid(id,3))+1
                opt = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 3)]) + 1;
            }
            // if opt==0 then
            if opt == 0 {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
                // local sc=sg:Select(tp,1,1,nil):GetFirst()
                let sel = group::select(d, &sg, a.tp, 1, 1, false, NO_EX).nil();
                let sc = group::get_first(d, &sel).nil();
                // Duel.SynchroSummon(tp,sc)
                duel::synchro_summon(d, a.tp, sc, None, None, 0, 0);
            // elseif opt==1 then
            } else if opt == 1 {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
                // local xc=xg:Select(tp,1,1,nil):GetFirst()
                let sel = group::select(d, &xg, a.tp, 1, 1, false, NO_EX).nil();
                let xc = group::get_first(d, &sel).nil();
                // Duel.XyzSummon(tp,xc)
                duel::xyz_summon(d, a.tp, xc, None, None, 0, 0);
            }
        }
    }
}
