//! Ninjitsu Art of Duplication
use super::prelude::*;

pub const ID: u32 = 50766506;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NINJA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute 1 "Ninja" and Special Summon "Ninja" monsters from the Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy the Special Summoned monster(s) when leaving the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return lv>0 and c:IsSetCard(SET_NINJA) and Duel.GetMZoneCount(tp,c)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp,lv)
    lv > 0
        && card::is_set_card(d, c, &[SET_NINJA as u16])
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,lv)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:IsLevelBelow(lv) and c:IsSetCard(SET_NINJA) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE)
    card::is_level_below(d, c, lv)
        && card::is_set_card(d, c, &[SET_NINJA as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some((POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8), None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    }
    // local rg=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,e,tp)
    let rg = duel::select_release_group_cost(d, tp, Some(&|d, c| cfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    // e:SetLabel(rg:GetFirst():GetLevel())
    let first = group::get_first(d, &rg).nil();
    let lvl = card::get_level(d, first) as i64;
    effect::set_label(d, e, &[lvl]);
    // Duel.Release(rg,REASON_COST)
    duel::release(d, rg, REASON_COST, None);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetSum(Card.GetLevel)<=e:GetLabel()
    let e = e.nil();
    let sum = group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64);
    (sum <= effect::get_label(d, e), false)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local lv=e:GetLabel()
    let lv = effect::get_label(d, a.e) as u32;
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp,lv)
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, tp, lv)), tp, LOCATION_DECK, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,s.rescon,1,tp,HINTMSG_SPSUMMON)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(ft as usize), Some(&rescon), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local fdg=Group.CreateGroup()
    let mut fdg = CardSet::new();
    // for tc in sg:Iter() do
    for tc in group::iter(d, &sg) {
        // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE) and tc:IsFacedown() then
        if duel::special_summon_step(d, tc, 0, tp, tp, false, false, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8, None) && card::is_facedown(d, tc) {
            // fdg:AddCard(tc)
            group::add_card(&mut fdg, &CardSet::from_card(tc));
        }
        // c:SetCardTarget(tc)
        card::set_card_target(d, c, tc);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // if #fdg>0 then Duel.ConfirmCards(1-tp,fdg) end
    if !fdg.is_empty() {
        duel::confirm_cards(d, 1 - tp, fdg);
    }
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetHandler():GetCardTarget():Filter(Card.IsLocation,nil,LOCATION_MZONE)
    let h = effect::get_handler(d, a.e).nil();
    let t = card::get_card_target(d, h);
    let g = group::filter(d, &t, &|d, c| card::is_location(d, c, LOCATION_MZONE), NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
