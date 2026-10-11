//! Xyz Universe
use super::prelude::*;

pub const ID: u32 = 11109820;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c,e,tp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ)
    //     and Duel.IsExistingTarget(s.filter2,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,e,tp,c,c:GetRank())
    if !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)) {
        return false;
    }
    let rk = card::get_rank(d, c) as i64;
    duel::is_existing_target(d, Some(&|d, x| filter2(d, x, e, tp, c, rk)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
/// `s.filter2(c,e,tp,mc,rk)`
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, rk: i64) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,rk+c:GetRank(),Group.FromCards(c,mc))
    if !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)) {
        return false;
    }
    let sum = rk + card::get_rank(d, c) as i64;
    let mut sg = CardSet::from_card(c);
    group::add_card(&mut sg, &CardSet::from_card(mc));
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, sum, Some(&sg))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,rk,sg)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rk: i64, sg: Option<&CardSet>) -> bool {
    // local crk=c:GetRank()
    let crk = card::get_rank(d, c) as i64;
    // return (crk==rk or crk==rk-1) and not c:IsSetCard(SET_NUMBER) and Duel.GetLocationCountFromEx(tp,tp,sg,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (crk == rk || crk == rk - 1)
        && !card::is_set_card(d, c, &[SET_NUMBER as u16])
        && duel::get_location_count_from_ex(d, tp, Some(tp), Exclude::from(sg), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE)
    //     and Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE)
            && duel::is_existing_target(d, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,e,tp)
    let mut g1 = duel::select_target(d, tp, Some(&|d, c| filter1(d, c, e, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // local tc=g1:GetFirst()
    let tc = group::get_first(d, &g1).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g2=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,tc,e,tp,tc,tc:GetRank())
    let rk = card::get_rank(d, tc) as i64;
    let g2 = duel::select_target(d, tp, Some(&|d, c| filter2(d, c, e, tp, tc, rk)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, tc.into()).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g1,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g1), 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.operation(e,tp,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // change damage
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(0)
    effect::set_value(d, e1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_NO_EFFECT_DAMAGE)
    effect::set_code(d, e2, EFFECT_NO_EFFECT_DAMAGE);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // sp_summon
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    // local tc1=g:GetFirst()
    // local tc2=g:GetNext()
    let order = group::iter(d, &g);
    let tc1 = order.first().copied();
    let tc2 = order.get(1).copied();
    // if not tc1:IsRelateToEffect(e) or not tc2:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc1.nil(), e) || !card::is_relate_to_effect(d, tc2.nil(), e) {
        return;
    }
    // Duel.SendtoGrave(g,REASON_EFFECT)
    duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None);
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // if og:FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)<2 then return end
    if group::filter_count(d, &og, &|d, x| card::is_location(d, x, LOCATION_GRAVE), NO_EX) < 2 {
        return;
    }
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,e,tp,tc1:GetRank()+tc2:GetRank())
    let rk = card::get_rank(d, tc1.nil()) as i64 + card::get_rank(d, tc2.nil()) as i64;
    let sg = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp, rk, None)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local ssg=sg:Select(tp,1,1,nil)
    let ssg = group::select(d, &sg, tp, 1, 1, false, NO_EX).nil();
    // local sc=ssg:GetFirst()
    let sc = group::get_first(d, &ssg);
    // if sc then
    if let Some(sc) = sc {
        // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // if c:IsRelateToEffect(e) then
        if card::is_relate_to_effect(d, c, e) {
            // c:CancelToGrave()
            card::cancel_to_grave(d, c, true);
            // Duel.Overlay(sc,c)
            duel::overlay(d, sc, c, false);
        }
    }
}
