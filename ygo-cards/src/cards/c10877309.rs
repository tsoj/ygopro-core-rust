//! Time Thief Startup
//! クロノダイバー・スタートアップ
use super::prelude::*;

pub const ID: u32 = 10877309;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TIME_THIEF], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // spsummon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // attach
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.xtg)
    effect::set_target(d, e2, Some(cost_target(xtg)));
    // e2:SetOperation(s.xop)
    effect::set_operation(d, e2, Some(operation(xop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_TIME_THIEF) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_TIME_THIEF as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.xfilter(c)`
fn xfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_TIME_THIEF) and c:IsType(TYPE_XYZ)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_TIME_THIEF as u16]) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.xcheck(sg,e,tp)`
fn xcheck(d: &mut Duel, sg: &CardSet) -> bool {
    // return sg:FilterCount(Card.IsMonster,nil)==1
    //     and sg:FilterCount(Card.IsSpell,nil)==1
    //     and sg:FilterCount(Card.IsTrap,nil)==1
    group::filter_count(d, sg, &|d, c| card::is_monster(d, c), NO_EX) == 1
        && group::filter_count(d, sg, &|d, c| card::is_spell(d, c), NO_EX) == 1
        && group::filter_count(d, sg, &|d, c| card::is_trap(d, c), NO_EX) == 1
}
/// `s.xcheck` as the rescon of `aux.SelectUnselectGroup`: `(res, false)`.
fn xcheck_rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (xcheck(d, sg), false)
}
/// `s.xtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn xtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.xfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && xfilter(d, chkc);
    }
    // local g=Duel.GetMatchingGroup(Card.IsSetCard,tp,LOCATION_GRAVE,0,e:GetHandler(),SET_TIME_THIEF)
    let h = effect::get_handler(d, e).nil();
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_set_card(d, c, &[SET_TIME_THIEF as u16])), tp, LOCATION_GRAVE, 0, h.into());
    // if chk==0 then return Duel.IsExistingTarget(s.xfilter,tp,LOCATION_MZONE,0,1,nil)
    //     and aux.SelectUnselectGroup(g,e,tp,3,3,s.xcheck,0) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&xfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && matches!(
                aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&xcheck_rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.xfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, tp, Some(&xfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.xop(e,tp,eg,ep,ev,re,r,rp)`
fn xop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or tc:IsImmuneToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, e) || card::is_immune_to_effect(d, tc, e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(Card.IsSetCard),tp,LOCATION_GRAVE,0,nil,SET_TIME_THIEF)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| card::is_set_card(d, c, &[SET_TIME_THIEF as u16]))), tp, LOCATION_GRAVE, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,3,3,s.xcheck,1,tp,HINTMSG_XMATERIAL)
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&xcheck_rescon), 1, tp, Some(HINTMSG_XMATERIAL as u64), None, None, false).unwrap_or_default();
    // if #sg==3 then
    if sg.len() == 3 {
        // Duel.Overlay(tc,sg)
        duel::overlay(d, tc, sg, false);
    }
}
