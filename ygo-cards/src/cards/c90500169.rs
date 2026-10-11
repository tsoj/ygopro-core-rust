//! Level Down!?
use super::prelude::*;

pub const ID: u32 = 90500169;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LV], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `c:GetMetatable(true)`: the `LVnum` and `LVset` the card's script declares.
fn lv_of(d: &Duel, c: CardId) -> Option<(u32, u32)> {
    crate::card_info(d, c).lv
}
/// `s.tdfilter(c,e,tp)`
fn tdfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsSetCard(SET_LV) and c:IsFaceup() and c:IsAbleToDeck()) then return false end
    if !(card::is_set_card(d, c, &[SET_LV as u16]) && card::is_faceup(d, c) && card::is_able_to_deck(d, c)) {
        return false;
    }
    // local owner=c:GetOwner()
    let owner = card::get_owner(d, c);
    // if Duel.GetMZoneCount(owner,c,tp)<=0 then return false end
    if duel::get_m_zone_count(d, owner, c.into(), Some(tp), None, None).0 <= 0 {
        return false;
    }
    // local class=c:GetMetatable(true)
    // return class and class.LVnum and class.LVset
    //     and Duel.IsExistingMatchingCard(s.spfilter,owner,LOCATION_GRAVE,0,1,nil,e,tp,owner,class.LVnum,class.LVset)
    let Some((lvnum, lvset)) = lv_of(d, c) else {
        return false;
    };
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, owner, lvnum, lvset)), owner, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,owner,lvnum,lvset)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, owner: u8, lvnum: u32, lvset: u32) -> bool {
    // local class=c:GetMetatable(true)
    // return class and class.LVnum and class.LVnum<lvnum and class.LVset==lvset
    //     and c:IsCanBeSpecialSummoned(e,0,tp,true,false,POS_FACEUP,owner)
    let Some((cl_num, cl_set)) = lv_of(d, c) else {
        return false;
    };
    cl_num < lvnum
        && cl_set == lvset
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, Some(POS_FACEUP as u8), Some(owner), None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.tdfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && tdfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tdfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tdfilter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local tc=Duel.SelectTarget(tp,s.tdfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,e,tp):GetFirst()
    let sel = duel::select_target(d, tp, Some(&|d, c| tdfilter(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,tc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, tc.map(CardSet::from_card), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tc:GetOwner(),LOCATION_GRAVE)
    let owner = card::get_owner(d, tc.nil());
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, owner, LOCATION_GRAVE as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsFaceup()) then return end
    if !(card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc)) {
        return;
    }
    // local owner=tc:GetOwner()
    let owner = card::get_owner(d, tc);
    // local class=tc:GetMetatable(true)
    let lv = lv_of(d, tc);
    // if not (Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and tc:IsLocation(LOCATION_DECK|LOCATION_EXTRA)
    //     and Duel.GetLocationCount(owner,LOCATION_MZONE)>0 and class and class.LVnum and class.LVset) then return end
    let sent = duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0;
    if !(sent
        && card::is_location(d, tc, LOCATION_DECK | LOCATION_EXTRA)
        && duel::get_location_count(d, owner, LOCATION_MZONE as u8, None, None, None).0 > 0)
    {
        return;
    }
    let Some((lvnum, lvset)) = lv else {
        return;
    };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,owner,LOCATION_GRAVE,0,1,1,nil,e,tp,owner,class.LVnum,class.LVset)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp, owner, lvnum, lvset)), owner, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,owner,true,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, owner, true, false, POS_FACEUP as u8, None);
    }
}
