//! Tri-Brigade Revolt
use super::prelude::*;

pub const ID: u32 = 40975243;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRI_BRIGADE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon from hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter2(c,fg,minmat,maxmat)`
fn spfilter2(d: &mut Duel, c: CardId, fg: &CardSet, minmat: u16, maxmat: u16) -> bool {
    // return c:IsSetCard(SET_TRI_BRIGADE) and c:IsLinkSummonable(nil,fg,minmat,maxmat)
    card::is_set_card(d, c, &[SET_TRI_BRIGADE as u16]) && card::is_link_summonable(d, c, None, Some(fg.clone()), minmat, maxmat)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMatchingGroupCount(s.spfilter2,tp,LOCATION_EXTRA,0,nil,sg,#sg,#sg)>0
    let n = sg.len() as u16;
    let count = duel::get_matching_group_count(d, Some(&|d, c| spfilter2(d, c, sg, n, n)), tp, LOCATION_EXTRA, 0, NO_EX);
    (count > 0, false)
}
/// `s.filtercheck(c,e,tp)`
fn filtercheck(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeLinkMaterial() and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsRace(RACES_BEAST_BWARRIOR_WINGB)
    //     and (c:IsLocation(LOCATION_GRAVE) or (c:IsLocation(LOCATION_REMOVED) and c:IsFaceup()))
    card::is_can_be_link_material(d, c, None, None)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_race(d, c, RACES_BEAST_BWARRIOR_WINGB as u64)
        && (card::is_location(d, c, LOCATION_GRAVE) || (card::is_location(d, c, LOCATION_REMOVED) && card::is_faceup(d, c)))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local fg=Duel.GetMatchingGroup(s.filtercheck,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil,e,tp)
    let fg = duel::get_matching_group(d, Some(&|d, c| filtercheck(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    if a.chk == Some(0) {
        // return ft>0 and Duel.IsPlayerCanSpecialSummonCount(tp,2)
        //     and Duel.GetLocationCountFromEx(tp,tp,nil,TYPE_LINK)>0
        //     and aux.SelectUnselectGroup(fg,e,tp,1,ft,s.rescon,0)
        return ft > 0
            && duel::is_player_can_special_summon_count(d, a.tp, 2)
            && duel::get_location_count_from_ex(d, a.tp, Some(a.tp), NO_EX, duel::ExtraCard::Type(TYPE_LINK), None).0 > 0
            && matches!(
                aux::select_unselect_group(d, &fg, Some(a.e), a.tp, Some(1), Some(ft as usize), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_REMOVED|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_REMOVED | LOCATION_EXTRA) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local fg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.filtercheck),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil,e,tp)
    let nvf = aux::necro_valley_filter(|d: &mut Duel, c: CardId| filtercheck(d, c, a.e, a.tp));
    let fg = duel::get_matching_group(d, Some(&nvf), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // if ft<1 or not Duel.IsPlayerCanSpecialSummonCount(tp,2) then return end
    if ft < 1 || !duel::is_player_can_special_summon_count(d, a.tp, 2) {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local g=aux.SelectUnselectGroup(fg,e,tp,1,ft,s.rescon,1,tp,HINTMSG_SPSUMMON,s.rescon,nil,false)
    let g = aux::select_unselect_group(
        d,
        &fg,
        Some(a.e),
        a.tp,
        Some(1),
        Some(ft as usize),
        Some(&rescon),
        1,
        a.tp,
        Some(HINTMSG_SPSUMMON as u64),
        Some(&rescon),
        None,
        false,
    )
    .unwrap_or_default();
    // if not g or #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
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
    // local tg=Duel.GetMatchingGroup(s.spfilter2,tp,LOCATION_EXTRA,0,nil,g,#g,#g)
    let n = g.len() as u16;
    let tg = duel::get_matching_group(d, Some(&|d, c| spfilter2(d, c, &g, n, n)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=tg:Select(tp,1,1,nil)
        let sg = group::select(d, &tg, a.tp, 1, 1, false, NO_EX).nil();
        // local sc=sg:GetFirst()
        let sc = group::get_first(d, &sg).nil();
        // Duel.LinkSummon(tp,sc,nil,g,#g,#g)
        duel::link_summon(d, a.tp, sc, None, Some(g.clone()), n, n);
    }
}
