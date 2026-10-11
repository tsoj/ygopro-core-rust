//! Pendulum Xyz
use super::prelude::*;

pub const ID: u32 = 46005939;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.filter(c,e,tp)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:GetLevel()>0 and c:IsCanBeEffectTarget(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_faceup(d, c)
        && card::get_level(d, c) > 0
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// local function reglevel(c,tc,lv)
fn reglevel(d: &mut Duel, c: CardId, tc: CardId, lv: i64) -> EffectId {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_XYZ_LEVEL)
    effect::set_code(d, e1, EFFECT_XYZ_LEVEL);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(lv)
    effect::set_value(d, e1, lv);
    // tc:RegisterEffect(e1,true)
    card::register_effect(d, tc, e1, true);
    // return e1
    e1
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_PZONE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| filter(d, x, a.e, a.tp)), a.tp, LOCATION_PZONE, 0, NO_EX);
    // if chk==0 then
    if a.chk == Some(0) {
        // if not (#g==2 and Duel.IsPlayerCanSpecialSummonCount(tp,2)
        //   and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>1) then return false end
        if !(g.len() == 2
            && duel::is_player_can_special_summon_count(d, a.tp, 2)
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1)
        {
            return false;
        }
        let order = group::iter(d, &g);
        let (tc1, tc2) = (order[0], order[1]);
        // local tc1=g:GetFirst()
        // local tc2=g:GetNext()
        // local e1=reglevel(e:GetHandler(),tc1,tc2:GetLevel())
        let h = effect::get_handler(d, a.e).nil();
        let lv2 = card::get_level(d, tc2) as i64;
        let e1 = reglevel(d, h, tc1, lv2);
        // local e2=reglevel(e:GetHandler(),tc2,tc1:GetLevel())
        let lv1 = card::get_level(d, tc1) as i64;
        let e2 = reglevel(d, h, tc2, lv1);
        // local res=Duel.IsExistingMatchingCard(Card.IsXyzSummonable,tp,LOCATION_EXTRA,0,1,nil,g,g)
        let res = duel::is_existing_matching_card(
            d,
            Some(&|d, x| card::is_xyz_summonable(d, x, Some(g.clone()), Some(g.clone()), 0, 0)),
            a.tp,
            LOCATION_EXTRA,
            0,
            1,
            NO_EX,
        );
        // if e1 then e1:Reset() end
        effect::reset(d, e1);
        // if e2 then e2:Reset() end
        effect::reset(d, e2);
        // return res
        return res;
    }
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 2, 0, 0);
    true
}
// function s.filter2(c,e,tp)
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRelateToEffect(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_relate_to_effect(d, c, e) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS):Filter(s.filter2,nil,e,tp)
    let targets = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    let g = group::filter(d, &targets, &|d, x| filter2(d, x, a.e, a.tp), NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    let h = effect::get_handler(d, a.e).nil();
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
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
    // local tc1=g:GetFirst()
    // local tc2=g:GetNext()
    let order = group::iter(d, &g);
    let (tc1, tc2) = (order[0], order[1]);
    // local e1=reglevel(e:GetHandler(),tc1,tc2:GetLevel())
    let lv2 = card::get_level(d, tc2) as i64;
    let e1 = reglevel(d, h, tc1, lv2);
    // local e2=reglevel(e:GetHandler(),tc2,tc1:GetLevel())
    let lv1 = card::get_level(d, tc1) as i64;
    let e2 = reglevel(d, h, tc2, lv1);
    // local xyzg=Duel.GetMatchingGroup(Card.IsXyzSummonable,tp,LOCATION_EXTRA,0,nil,g,g)
    let xyzg = duel::get_matching_group(
        d,
        Some(&|d, x| card::is_xyz_summonable(d, x, Some(g.clone()), Some(g.clone()), 0, 0)),
        a.tp,
        LOCATION_EXTRA,
        0,
        NO_EX,
    );
    // if xyzg and #xyzg>0 then
    if !xyzg.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local xyz=xyzg:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &xyzg, a.tp, 1, 1, false, NO_EX).nil();
        let xyz = group::get_first(d, &sel).nil();
        // Duel.XyzSummon(tp,xyz,g,nil,2,2)
        duel::xyz_summon(d, a.tp, xyz, Some(g), None, 2, 2);
    } else {
        // if e1 then e1:Reset() end
        effect::reset(d, e1);
        // if e2 then e2:Reset() end
        effect::reset(d, e2);
    }
}
