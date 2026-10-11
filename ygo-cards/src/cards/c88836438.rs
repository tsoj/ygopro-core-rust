//! Spright Smashers
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 88836438;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SPRINGANS, SET_THERION, SET_SPRIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.actcost)
    effect::set_target(d, e1, Some(cost_target(actcost)));
    // e1:SetOperation(s.actop)
    effect::set_operation(d, e1, Some(operation(actop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `g1 - g2` (the Lua `-` of two groups)
fn minus(g1: &CardSet, g2: &CardSet) -> CardSet {
    let mut x = g1.clone();
    group::remove_card(&mut x, g2);
    x
}
/// `g1 + g2` (the Lua `+` of two groups)
fn plus(g1: &CardSet, g2: &CardSet) -> CardSet {
    let mut x = g1.clone();
    group::add_card(&mut x, g2);
    x
}
/// `s.actcostfilter(c,tp,dsp,gg,rg)`
fn actcostfilter(d: &mut Duel, c: CardId, tp: u8, dsp: bool, gg: &CardSet, rg: Option<&CardSet>) -> bool {
    // if not ((c:IsSetCard(SET_SPRINGANS) or c:IsSetCard(SET_THERION) or c:IsSetCard(SET_SPRIGHT)) and c:IsAbleToRemoveAsCost()
    //   and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true))) then return false end
    if !((card::is_set_card(d, c, &[SET_SPRINGANS as u16]) || card::is_set_card(d, c, &[SET_THERION as u16]) || card::is_set_card(d, c, &[SET_SPRIGHT as u16]))
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, false)))
    {
        return false;
    }
    let cc = CardSet::from_card(c);
    // return (Duel.GetMZoneCount(tp,c)>0 and (dsp or #(gg-c)>0)) or (rg and #(rg-c)>0)
    (duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0 && (dsp || group::get_count(&minus(gg, &cc)) > 0))
        || rg.is_some_and(|rg| group::get_count(&minus(rg, &cc)) > 0)
}
/// `s.spfilter(c,e,tp,set)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, set: u32) -> bool {
    // return c:IsSetCard(set) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[set as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.rmfilter(c,tp)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and c:IsFaceup() and (c:IsLevel(2) or c:IsRank(2) or c:IsLink(2))
    card::is_able_to_remove(d, c, None, None, None) && card::is_faceup(d, c) && (card::is_level(d, c, &[2]) || card::is_rank(d, c, &[2]) || card::is_link(d, c, &[2]))
}
/// `s.actcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn actcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local dsp=Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,SET_SPRINGANS)
    let dsp = duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, SET_SPRINGANS)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // local gg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp,SET_THERION)
    let gg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, SET_THERION)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local rg=Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,1,nil)
    //   and Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE,0,nil) or nil
    let rg = if duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX) {
        Some(duel::get_matching_group(d, Some(&rmfilter), a.tp, LOCATION_MZONE, 0, NO_EX))
    } else {
        None
    };
    // if chk==0 then return Duel.IsExistingMatchingCard(s.actcostfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE,0,1,c,tp,dsp,gg,rg) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, x| actcostfilter(d, x, a.tp, dsp, &gg, rg.as_ref())),
            a.tp,
            LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE,
            0,
            1,
            c.into(),
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.actcostfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE,0,1,1,c,tp,dsp,gg,rg)
    let cg = duel::select_matching_card(
        d,
        a.tp,
        Some(&|d, x| actcostfilter(d, x, a.tp, dsp, &gg, rg.as_ref())),
        a.tp,
        LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE,
        0,
        1,
        1,
        false,
        c.into(),
    )
    .nil();
    // Duel.Remove(cg,POS_FACEUP,REASON_COST)
    duel::remove(d, cg.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local op=Duel.SelectEffect(tp,
    //   {ft>0 and dsp,aux.Stringid(id,1)},
    //   {ft>0 and #(gg-cg)>0,aux.Stringid(id,2)},
    //   {rg and #(rg-cg)>0,aux.Stringid(id,3)})
    let opt2 = ft > 0 && group::get_count(&minus(&gg, &cg)) > 0;
    let opt3 = rg.as_ref().is_some_and(|rg| group::get_count(&minus(rg, &cg)) > 0);
    let op = duel::select_effect(d, a.tp, &[(ft > 0 && dsp, aux::stringid(ID, 1)), (opt2, aux::stringid(ID, 2)), (opt3, aux::stringid(ID, 3))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    // if op==1 or op==2 then
    if op == 1 || op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,op==1 and LOCATION_DECK or LOCATION_GRAVE)
        let loc = if op == 1 { LOCATION_DECK } else { LOCATION_GRAVE };
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, loc as i32);
    // elseif op==3 then
    } else if op == 3 {
        // e:SetCategory(CATEGORY_REMOVE)
        effect::set_category(d, a.e, CATEGORY_REMOVE);
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rg-cg,2,PLAYER_ALL,LOCATION_ONFIELD)
        let rg_cg = minus(&rg.clone().nil(), &cg);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(rg_cg), 2, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    // else
    } else {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    }
    true
}
/// `s.actop(e,tp,eg,ep,ev,re,r,rp)`
fn actop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 or op==2 then
    if op == 1 || op == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Group.CreateGroup()
        // (the Lua group is replaced in both branches below)
        let g = if op == 1 {
            // g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,SET_SPRINGANS)
            duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, SET_SPRINGANS)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil()
        } else {
            // g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp,SET_THERION)
            duel::select_matching_card(
                d,
                a.tp,
                Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, a.tp, SET_THERION))),
                a.tp,
                LOCATION_GRAVE,
                0,
                1,
                1,
                false,
                NO_EX,
            )
            .nil()
        };
        // if #g>0 then
        if group::get_count(&g) > 0 {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    // elseif op==3 then
    } else if op == 3 {
        // local rg1=Duel.GetMatchingGroup(Card.IsAbleToRemove,tp,0,LOCATION_ONFIELD,nil)
        let rg1 = duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
        // if #rg1<=0 then return end
        if group::get_count(&rg1) == 0 {
            return;
        }
        // local rg2=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE,0,nil)
        let rg2 = duel::get_matching_group(d, Some(&rmfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
        // if #rg2<=0 then return end
        if group::get_count(&rg2) == 0 {
            return;
        }
        // local g=aux.SelectUnselectGroup(rg1+rg2,e,tp,2,2,function(sg) return sg:GetClassCount(Card.GetControler)==2 end,1,tp,HINTMSG_REMOVE)
        let rescon = |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| -> (bool, bool) {
            (group::get_class_count(d, sg, &|d, c| card::get_controler(d, c) as i64) == 2, false)
        };
        let g = aux::select_unselect_group(d, &plus(&rg1, &rg2), Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
        // if #g>0 then
        if group::get_count(&g) > 0 {
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
