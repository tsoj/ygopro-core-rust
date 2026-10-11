//! Evo-Singularity
use super::prelude::*;

pub const ID: u32 = 74100225;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_EVOLTILE, SET_EVOLSAUR, SET_EVOLZAR],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Evolzaur" from the Extra Deck and attach 1 "Evoltile" and 1 "Evolsaur" to it
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
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsMonster() and c:IsSetCard({SET_EVOLTILE,SET_EVOLSAUR}) and c:IsCanBeEffectTarget(e)
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_EVOLTILE as u16, SET_EVOLSAUR as u16]) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_EVOLZAR) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_EVOLZAR as u16])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_EVOLTILE) and sg:IsExists(Card.IsSetCard,1,nil,SET_EVOLSAUR)
    let has_tiltle = group::is_exists(d, sg, &|d: &mut Duel, x| card::is_set_card(d, x, &[SET_EVOLTILE as u16]), 1, NO_EX);
    (has_tiltle && group::is_exists(d, sg, &|d: &mut Duel, x| card::is_set_card(d, x, &[SET_EVOLSAUR as u16]), 1, NO_EX), false)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d: &mut Duel, x| filter(d, x, a.e)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #g>=2 and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return g.len() >= 2
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            )
            && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_ATTACH)
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(2),
        Some(2),
        Some(&rescon),
        1,
        a.tp,
        Some(HINTMSG_ATTACH as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // Duel.SetTargetCard(sg)
    duel::set_target_card(d, sg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(sg), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d: &mut Duel, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    if let Some(sc) = group::get_first(d, &sel) {
        // if Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 then
        if duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // local mg=Duel.GetTargetCards(e)
            let mg = duel::get_target_cards(d, a.e).nil();
            // if #mg>0 then Duel.Overlay(sc,mg) end
            if !mg.is_empty() {
                duel::overlay(d, sc, mg, false);
            }
        }
    }
}
