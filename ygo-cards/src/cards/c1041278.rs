//! Branded Expulsion
use super::prelude::*;

pub const ID: u32 = 1041278;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 2 non-Fusion monsters
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
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp,targetp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, targetp: u8) -> bool {
    // return c:IsFaceup() and not c:IsType(TYPE_FUSION)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,targetp) and c:IsCanBeEffectTarget(e)
    card::is_faceup(d, c)
        && !card::is_type(d, c, TYPE_FUSION)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(targetp), None)
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.spcostfilter(c,tp,b1,b2)`
fn spcostfilter(d: &mut Duel, c: CardId, tp: u8, b1: bool, b2: bool) -> bool {
    // local ft=Duel.GetMZoneCount(tp,c)
    let ft = duel::get_m_zone_count(d, tp, c.into(), None, None, None).0;
    // return ft>0 and c:IsType(TYPE_FUSION) and ((b1 and Duel.GetMZoneCount(1-tp,c)>0)
    //     or (b2 and ft>1 and c:ListsCodeAsMaterial(CARD_ALBAZ)))
    ft > 0
        && card::is_type(d, c, TYPE_FUSION)
        && ((b1 && duel::get_m_zone_count(d, 1 - tp, c.into(), None, None, None).0 > 0)
            || (b2 && ft > 1 && card::lists_code_as_material(d, c, &[CARD_ALBAZ])))
}
/// `s.rescon(ag,g1,g2)`: returns `function(sg,e,tp,mg)`
fn rescon_albaz(ag: CardSet, g1: CardSet, g2: CardSet) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>| {
        // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
        // local ft2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)
        let ft2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0;
        // local res1=ft1>=2 and #(ag&sg)>=2
        let res1 = ft1 >= 2 && group::intersection(&ag, sg).len() >= 2;
        // local res2=ft1>0 and ft2>0 and #(sg&g1)>0 and #(sg&g2)>0
        let res2 = ft1 > 0 && ft2 > 0 && !group::intersection(sg, &g1).is_empty() && !group::intersection(sg, &g2).is_empty();
        // return res1 or res2
        (res1 || res2, false)
    }
}
/// `s.spownfilter(c,e,tp,tg)`
fn spownfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tg: &CardSet) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and (tg-c):GetFirst():IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    if !card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None) {
        return false;
    }
    let mut rest = tg.clone();
    group::remove_card(&mut rest, &CardSet::from_card(c));
    let other = group::get_first(d, &rest).nil();
    card::is_can_be_special_summoned(d, other, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return false end
    if a.chk == Some(0) && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return false;
    }
    // local g1=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,LOCATION_GRAVE|LOCATION_REMOVED,nil,e,tp,tp)
    let g1 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp, tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, LOCATION_GRAVE | LOCATION_REMOVED, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,LOCATION_GRAVE|LOCATION_REMOVED,nil,e,tp,1-tp)
    let g2 = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp, 1 - tp)), tp, LOCATION_GRAVE | LOCATION_REMOVED, LOCATION_GRAVE | LOCATION_REMOVED, NO_EX);
    // local b1=(#g1>1 and #g2>1) or (#(g1&g2)~=#g1 and #(g1&g2)~=#g2)
    let inter = group::intersection(&g1, &g2).len();
    let mut b1 = (g1.len() > 1 && g2.len() > 1) || (inter != g1.len() && inter != g2.len());
    // local ag=g1:Filter(Card.IsCanBeSpecialSummoned,nil,e,0,tp,false,false,POS_FACEUP_DEFENSE)
    let ag = group::filter(d, &g1, &|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None), NO_EX);
    // local b2=#ag>1
    let mut b2 = ag.len() > 1;
    // if chk==0 then return (b1 or b2) and Duel.CheckReleaseGroupCost(tp,s.spcostfilter,1,false,nil,nil,tp,b1,b2) end
    if a.chk == Some(0) {
        return (b1 || b2) && duel::check_release_group_cost(d, tp, Some(&|d, c| spcostfilter(d, c, tp, b1, b2)), 1, 1, false, None, NO_EX);
    }
    // local rc=Duel.SelectReleaseGroupCost(tp,s.spcostfilter,1,1,false,nil,nil,tp,b1,b2):GetFirst()
    let rcs = duel::select_release_group_cost(d, tp, Some(&|d, c| spcostfilter(d, c, tp, b1, b2)), 1, 1, false, None, NO_EX);
    let rc = group::get_first(d, &rcs).nil();
    // local albaz=rc:ListsCodeAsMaterial(CARD_ALBAZ)
    let albaz = card::lists_code_as_material(d, rc, &[CARD_ALBAZ]);
    // Duel.Release(rc,REASON_COST)
    duel::release(d, rc, REASON_COST, None);
    // local tg=nil
    // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local ft2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)
    let ft2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0;
    // b1=b1 and ft1>0 and ft2>0
    b1 = b1 && ft1 > 0 && ft2 > 0;
    // b2=b2 and ft1>=2
    b2 = b2 && ft1 >= 2;
    let union = group::union(&g1, &g2);
    let both = |_d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| {
        (!group::intersection(sg, &g1).is_empty() && !group::intersection(sg, &g2).is_empty(), false)
    };
    let tg: CardSet = if b1 && !b2 {
        // tg=aux.SelectUnselectGroup(g1+g2,e,tp,2,2,function(sg) return #(sg&g1)>0 and #(sg&g2)>0 end,1,tp,HINTMSG_SPSUMMON)
        aux::select_unselect_group(d, &union, Some(a.e), tp, Some(2), Some(2), Some(&both), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
            .unwrap_or_default()
    } else if !b1 && b2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // tg=ag:Select(tp,2,2,nil)
        group::select(d, &ag, tp, 2, 2, false, NO_EX).nil()
    } else if !albaz {
        // tg=aux.SelectUnselectGroup(g1+g2,e,tp,2,2,function(sg) return #(sg&g1)>0 and #(sg&g2)>0 end,1,tp,HINTMSG_SPSUMMON)
        aux::select_unselect_group(d, &union, Some(a.e), tp, Some(2), Some(2), Some(&both), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
            .unwrap_or_default()
    } else {
        // tg=aux.SelectUnselectGroup(g1+g2,e,tp,2,2,s.rescon(ag,g1,g2),1,tp,HINTMSG_SPSUMMON)
        let rescon = rescon_albaz(ag.clone(), g1.clone(), g2.clone());
        aux::select_unselect_group(d, &union, Some(a.e), tp, Some(2), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
            .unwrap_or_default()
    };
    // if tg and #tg==2 then
    if tg.len() == 2 {
        // Duel.SetTargetCard(tg)
        duel::set_target_card(d, tg.clone());
        // e:SetLabel(albaz and 100 or 0)
        effect::set_label(d, a.e, &[if albaz { 100 } else { 0 }]);
    }
    // local locations=tg:GetBitwiseOr(Card.GetLocation)
    let locations = group::get_bitwise_or(d, &tg, &|d, c| card::get_location(d, c) as i64);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,#tg,0,locations)
    let n = tg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), n, 0, locations as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if #tg<2 or ft1<1 or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if tg.len() < 2 || ft1 < 1 || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // local b1=Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and tg:IsExists(s.spownfilter,1,nil,e,tp,tg)
    let b1 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && group::is_exists(d, &tg, &|d, c| spownfilter(d, c, a.e, tp, &tg), 1, NO_EX);
    // local b2=ft1>1 and e:GetLabel()==100
    //     and tg:FilterCount(Card.IsCanBeSpecialSummoned,nil,e,0,tp,false,false,POS_FACEUP_DEFENSE)==2
    let b2 = ft1 > 1
        && effect::get_label(d, a.e) == 100
        && group::filter_count(
            d,
            &tg,
            &|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None),
            NO_EX,
        ) == 2;
    // if b2 and (not b1 or Duel.SelectYesNo(tp,aux.Stringid(id,1))) then
    if b2 && (!b1 || duel::select_yes_no(d, tp, aux::stringid(ID, 1))) {
        // Duel.SpecialSummon(tg,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, tg, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    // elseif b1 then
    } else if b1 {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
        // local sc=tg:FilterSelect(tp,s.spownfilter,1,1,nil,e,tp,tg):GetFirst()
        let sel = group::filter_select(d, &tg, tp, &|d, c| spownfilter(d, c, a.e, tp, &tg), 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if not sc then return end
        let Some(sc) = sc else {
            return;
        };
        // Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // Duel.SpecialSummonStep((tg-sc):GetFirst(),0,tp,1-tp,false,false,POS_FACEUP)
        let mut rest = tg.clone();
        group::remove_card(&mut rest, &CardSet::from_card(sc));
        let other = group::get_first(d, &rest).nil();
        duel::special_summon_step(d, other, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None);
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
