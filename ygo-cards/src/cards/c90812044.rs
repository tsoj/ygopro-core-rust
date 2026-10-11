//! Chain Summon
use super::prelude::*;

pub const ID: u32 = 90812044;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.filter(c,e,tp,rk)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rk: i64) -> bool {
    // return c:IsType(TYPE_XYZ) and c:GetRank()<rk and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_XYZ)
        && (card::get_rank(d, c) as i64) < rk
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local rg,rk=g:GetMinGroup(Card.GetRank)
    let (rg, rk) = group::get_min_group(d, &g, &|d, c| card::get_rank(d, c) as i64).unwrap_or((CardSet::new(), 0));
    // if chkc then return rg:IsContains(chkc) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&rg, chkc);
    }
    // if chk==0 then return #g>=2 and rg:IsExists(Card.IsCanBeEffectTarget,1,nil,e)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_EXTRA,0,1,nil,e,tp,rk) end
    if a.chk == Some(0) {
        return g.len() >= 2
            && group::is_exists(d, &rg, &|d, c| card::is_can_be_effect_target(d, c, Some(a.e)), 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, a.tp, rk)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local sg=rg:FilterSelect(tp,Card.IsCanBeEffectTarget,1,1,nil,e)
    let sg = group::filter_select(d, &rg, a.tp, &|d, c| card::is_can_be_effect_target(d, c, Some(a.e)), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    if !card::is_relate_to_effect(d, tc, a.e) || card::is_facedown(d, tc) {
        return;
    }
    // local rk=tc:GetRank()
    let rk = card::get_rank(d, tc) as i64;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,rk)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| filter(d, c, a.e, a.tp, rk)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local sc=g:GetFirst()
    let sc = group::get_first(d, &g);
    // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)~=0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
            // Cannot attack directly
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(3207)
            effect::set_description(d, e1, 3207);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_DIRECT_ATTACK);
            // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1,true)
            card::register_effect(d, sc, e1, true);
            // sc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, sc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
            // Return it to extra deck during end phase
            // local e2=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e2 = effect::create_effect(d, h);
            // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e2:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
            // e2:SetCountLimit(1)
            effect::set_count_limit(d, e2, 1, 0, 0, 0);
            // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e2:SetLabelObject(sc)
            effect::set_label_object(d, e2, Value::Card(sc));
            // e2:SetCondition(s.retcon)
            effect::set_condition(d, e2, Some(condition(retcon)));
            // e2:SetOperation(s.retop)
            effect::set_operation(d, e2, Some(operation(retop)));
            // Duel.RegisterEffect(e2,tp)
            duel::register_effect(d, e2, a.tp);
        }
    }
}
/// `s.retcon(e,tp,eg,ep,ev,re,r,rp)`
fn retcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:GetFlagEffect(id)~=0 then
    if card::get_flag_effect(d, tc, ID) != 0 {
        // return true
        return true;
    }
    // e:Reset()
    effect::reset(d, a.e);
    // return false
    false
}
/// `s.retop(e,tp,eg,ep,ev,re,r,rp)`
fn retop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, CardSet::from_card(tc), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
