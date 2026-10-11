//! Archfiend Playtime
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 87985506;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[58769832, 85154941, 11248645], listed_series: &[SET_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Level 7 or lower "Archfiend" monster from your hand or Deck, or if you have a face-up "Archfiend" Ritual Monster in your Extra Deck, you can Special Summon 1 "Royal Archfiend", 1 "Duke Archfiend", and/or 1 "Highness Archfiend" from your hand and/or Deck instead
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
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp,ex_chk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ex_chk: bool) -> bool {
    // return ((c:IsLevelBelow(7) and c:IsSetCard(SET_ARCHFIEND)) or (ex_chk and c:IsCode(58769832,85154941,11248645)))
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    ((card::is_level_below(d, c, 7) && card::is_set_card(d, c, &[SET_ARCHFIEND as u16])) || (ex_chk && card::is_code(d, c, &[58769832, 85154941, 11248645])))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.archritfilter(c)`
fn archritfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ARCHFIEND) and c:IsRitualMonster() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_ARCHFIEND as u16]) && card::is_ritual_monster(d, c) && card::is_faceup(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local ex_chk=Duel.IsExistingMatchingCard(s.archritfilter,tp,LOCATION_EXTRA,0,1,nil)
        let ex_chk = duel::is_existing_matching_card(d, Some(&archritfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp,ex_chk)
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, ex_chk)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // local ex_chk=Duel.IsExistingMatchingCard(s.archritfilter,tp,LOCATION_EXTRA,0,1,nil)
    let ex_chk = duel::is_existing_matching_card(d, Some(&archritfilter), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,nil,e,tp,ex_chk)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, ex_chk)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // if not ex_chk then
    if !ex_chk {
        // ft=1
        ft = 1;
    } else {
        // ft=math.min(ft,3)
        ft = ft.min(3);
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
    }
    // local rescon=s.rescon(ex_chk)
    // function(sg,e,tp,mg) return #sg==1 or (ex_chk and #sg<=3 and sg:FilterCount(Card.IsCode,nil,58769832,85154941,11248645)==#sg and sg:GetClassCount(Card.GetCode)==#sg) end
    let rescon = move |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| -> (bool, bool) {
        let n = sg.len();
        let res = n == 1
            || (ex_chk
                && n <= 3
                && group::filter_count(d, sg, &|d, x| card::is_code(d, x, &[58769832, 85154941, 11248645]), NO_EX) == n
                && group::get_class_count(d, sg, &|d, x| card::get_code(d, x)[0] as i64) == n);
        (res, false)
    };
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,rescon,1,tp,HINTMSG_SPSUMMON,rescon)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(ft as usize), Some(&rescon), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), Some(&rescon), None, false).unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
