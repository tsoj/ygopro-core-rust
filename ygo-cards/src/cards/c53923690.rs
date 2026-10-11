//! Omega Judgment
use super::prelude::*;

pub const ID: u32 = 53923690;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Destroy 1 monster card in the S/T and 2 cards the opponent controls
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,e,tp)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return ((c:IsFaceup() and c:IsOriginalType(TYPE_MONSTER) and c:IsControler(tp) and c:GetSequence()<5) or c:IsControler(1-tp))
    //   and c:IsCanBeEffectTarget(e)
    ((card::is_faceup(d, c) && card::is_original_type(d, c, TYPE_MONSTER) && card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5)
        || card::is_controler(d, c, 1 - tp))
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.rescon(sg,e,tp,mg)`: `sg:FilterCount(Card.IsControler,nil,tp)==1`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX) == 1, false)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local rg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_SZONE,LOCATION_ONFIELD,nil,e,tp)
    let rg = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, a.e, a.tp)), a.tp, LOCATION_SZONE, LOCATION_ONFIELD, NO_EX);
    let rs: aux::Rescon = &rescon;
    // if chk==0 then return aux.SelectUnselectGroup(rg,e,tp,3,3,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(3), Some(3), Some(rs), 0, a.tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(rg,e,tp,3,3,s.rescon,1,tp,HINTMSG_DESTROY)
    let tg = aux::select_unselect_group(d, &rg, Some(a.e), a.tp, Some(3), Some(3), Some(rs), 1, a.tp, Some(HINTMSG_DESTROY as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tg,3,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(tg), 3, 0, 0);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Destroy(tg,REASON_EFFECT)
        duel::destroy(d, tg, REASON_EFFECT, None, None);
    }
}
