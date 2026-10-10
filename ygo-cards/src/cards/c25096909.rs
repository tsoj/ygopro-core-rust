//! Simultaneous Equation Cannons
use super::prelude::*;

pub const ID: u32 = 25096909;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TOEXTRA)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOEXTRA);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmfilter(c,tp)`
fn rmfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsType(TYPE_FUSION) or (c:IsType(TYPE_XYZ) and Duel.IsExistingMatchingCard(function(_c) return _c:IsType(TYPE_XYZ) and _c:IsRank(c:GetRank()) end,tp,LOCATION_EXTRA,0,1,c)))
    //     and c:IsAbleToRemove()
    let rank = card::get_rank(d, c);
    (card::is_type(d, c, TYPE_FUSION)
        || (card::is_type(d, c, TYPE_XYZ)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, _c| card::is_type(d, _c, TYPE_XYZ) && card::is_rank(d, _c, &[rank])),
                tp,
                LOCATION_EXTRA,
                0,
                1,
                c.into(),
            )))
        && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmrescon(ct)`: the `rescon` of the removal selection
fn rmrescon(d: &mut Duel, sg: &CardSet, ct: i64) -> bool {
    // if not (#sg==3 and sg:IsExists(Card.IsType,2,nil,TYPE_XYZ)
    //     and sg:IsExists(Card.IsType,1,nil,TYPE_FUSION)
    //     and sg:Filter(Card.IsType,nil,TYPE_XYZ):GetClassCount(Card.GetRank)==1) then return false end
    if !(sg.len() == 3
        && group::is_exists(d, sg, &|d, c| card::is_type(d, c, TYPE_XYZ), 2, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_type(d, c, TYPE_FUSION), 1, NO_EX)
        && {
            let xyz = group::filter(d, sg, &|d, c| card::is_type(d, c, TYPE_XYZ), NO_EX);
            group::get_class_count(d, &xyz, &|d, c| card::get_rank(d, c) as i64) == 1
        })
    {
        return false;
    }
    // local lvsum=sg:GetSum(Card.GetLevel)
    let lvsum = group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64);
    // local rnksum=sg:GetSum(Card.GetRank)
    let rnksum = group::get_sum(d, sg, &|d, c| card::get_rank(d, c) as i64);
    // return lvsum+rnksum==ct
    lvsum + rnksum == ct
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local ct=Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD|LOCATION_HAND,LOCATION_ONFIELD|LOCATION_HAND)
        let ct = duel::get_field_group_count(d, a.tp, LOCATION_ONFIELD | LOCATION_HAND, LOCATION_ONFIELD | LOCATION_HAND) as i64;
        // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_EXTRA,0,nil,tp)
        let tp = a.tp;
        let g = duel::get_matching_group(d, Some(&|d, c| rmfilter(d, c, tp)), a.tp, LOCATION_EXTRA, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,3,3,s.rmrescon(ct),0)
        let rs: aux::Rescon = &|d, sg, _e, _tp, _mg, _c| (rmrescon(d, sg, ct), false);
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(rs), 0, a.tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,3,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 3, a.tp, LOCATION_EXTRA as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOEXTRA,nil,2,tp,LOCATION_REMOVED)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOEXTRA, None, 2, a.tp, LOCATION_REMOVED as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.texfilter(c)`
fn texfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_FUSION|TYPE_XYZ) and c:IsFaceup() and c:IsAbleToExtra()
    card::is_type(d, c, TYPE_FUSION | TYPE_XYZ) && card::is_faceup(d, c) && card::is_able_to_extra(d, c)
}
/// `s.texrescon(sg,e,tp,mg)`
fn texrescon(d: &mut Duel, sg: &CardSet, tp: u8) -> bool {
    // if not (#sg==2 and sg:IsExists(Card.IsType,1,nil,TYPE_XYZ)
    //     and sg:IsExists(Card.IsType,1,nil,TYPE_FUSION)) then return false end
    if !(sg.len() == 2
        && group::is_exists(d, sg, &|d, c| card::is_type(d, c, TYPE_XYZ), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_type(d, c, TYPE_FUSION), 1, NO_EX))
    {
        return false;
    }
    // local lvsum=sg:GetSum(Card.GetLevel)
    let lvsum = group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64);
    // local rnksum=sg:GetSum(Card.GetRank)
    let rnksum = group::get_sum(d, sg, &|d, c| card::get_rank(d, c) as i64);
    // return Duel.IsExistingMatchingCard(s.lvrnkmatchfilter,tp,0,LOCATION_MZONE,1,nil,lvsum+rnksum)
    let v = lvsum + rnksum;
    duel::is_existing_matching_card(d, Some(&|d, c| lvrnkmatchfilter(d, c, v)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.lvrnkmatchfilter(c,lvrnk)`
fn lvrnkmatchfilter(d: &mut Duel, c: CardId, lvrnk: i64) -> bool {
    // return c:IsFaceup() and (c:IsLevel(lvrnk) or c:IsRank(lvrnk))
    card::is_faceup(d, c) && (card::is_level(d, c, &[lvrnk as u32]) || card::is_rank(d, c, &[lvrnk as u32]))
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ct=Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD|LOCATION_HAND,LOCATION_ONFIELD|LOCATION_HAND)
    let ct = duel::get_field_group_count(d, tp, LOCATION_ONFIELD | LOCATION_HAND, LOCATION_ONFIELD | LOCATION_HAND) as i64;
    // local exrmg=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_EXTRA,0,nil,tp)
    let exrmg = duel::get_matching_group(d, Some(&|d, c| rmfilter(d, c, tp)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #exrmg<3 then return end
    if exrmg.len() < 3 {
        return;
    }
    // local rg=aux.SelectUnselectGroup(exrmg,e,tp,3,3,s.rmrescon(ct),1,tp,HINTMSG_REMOVE)
    let rs: aux::Rescon = &|d, sg, _e, _tp, _mg, _c| (rmrescon(d, sg, ct), false);
    let rg = aux::select_unselect_group(d, &exrmg, Some(a.e), tp, Some(3), Some(3), Some(rs), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // if not (#rg==3 and Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)==3) then return end
    if !(rg.len() == 3 && duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None) == 3) {
        return;
    }
    // local opp_rg=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
    let opp_rg = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #opp_rg==0 then return end
    if opp_rg.is_empty() {
        return;
    }
    // local rmtexg=Duel.GetMatchingGroup(s.texfilter,tp,LOCATION_REMOVED,0,nil)
    let rmtexg = duel::get_matching_group(d, Some(&texfilter), tp, LOCATION_REMOVED, 0, NO_EX);
    // if not (#rmtexg>=2 and aux.SelectUnselectGroup(rmtexg,e,tp,2,2,s.texrescon,0) and Duel.SelectYesNo(tp,aux.Stringid(id,1))) then return end
    if rmtexg.len() < 2 {
        return;
    }
    let ts: aux::Rescon = &|d, sg, _e, tp2, _mg, _c| (texrescon(d, sg, tp2), false);
    if !matches!(aux::select_unselect_group(d, &rmtexg, Some(a.e), tp, Some(2), Some(2), Some(ts), 0, tp, None, None, None, false), Err(true)) {
        return;
    }
    if !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        return;
    }
    // local texg=aux.SelectUnselectGroup(rmtexg,e,tp,2,2,s.texrescon,1,tp,HINTMSG_TODECK)
    let ts: aux::Rescon = &|d, sg, _e, tp2, _mg, _c| (texrescon(d, sg, tp2), false);
    let texg = aux::select_unselect_group(d, &rmtexg, Some(a.e), tp, Some(2), Some(2), Some(ts), 1, tp, Some(HINTMSG_TODECK as u64), None, None, false).unwrap_or_default();
    // if #texg~=2 then return end
    if texg.len() != 2 {
        return;
    }
    // Duel.HintSelection(texg,true)
    duel::hint_selection(d, texg.clone(), true);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if Duel.SendtoDeck(texg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==2 then
    if duel::send_to_deck(d, texg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 2 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Remove(opp_rg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, opp_rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
