//! Avalon
use super::prelude::*;

pub const ID: u32 = 82140600;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NOBLE_KNIGHT, SET_ARTORIGUS, SET_LAUNDSALLYN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 5 "Noble Knight" monsters in your Graveyard, including at least 1 "Artorigus" monster and at least 1 "Laundsallyn" monster; banish those targets, and if you do, destroy all cards on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_DESTROY);
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
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.banfilter(c)`
fn banfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NOBLE_KNIGHT) and c:IsMonster() and c:IsAbleToRemove()
    card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16]) && card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_ARTORIGUS) and sg:IsExists(Card.IsSetCard,1,nil,SET_LAUNDSALLYN)
    let res = group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_ARTORIGUS as u16]), 1, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_set_card(d, c, &[SET_LAUNDSALLYN as u16]), 1, NO_EX);
    (res, false)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetTargetGroup(s.banfilter,tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_target_group(d, Some(&banfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return #g>=5 and aux.SelectUnselectGroup(g,e,tp,5,5,s.rescon,0)
    //     and Duel.IsExistingMatchingCard(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c) end
    if a.chk == Some(0) {
        return g.len() >= 5
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), tp, Some(5), Some(5), Some(&rescon), 0, tp, None, None, None, false), Err(true))
            && duel::is_existing_matching_card(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,5,5,s.rescon,1,tp,HINTMSG_REMOVE)
    let tg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(5), Some(5), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // local dg=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
    let dg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tg,5,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(tg), 5, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,tp,0)
    let n = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), n, tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #tg>0 and Duel.Remove(tg,POS_FACEUP,REASON_EFFECT)>0 then
    if !tg.is_empty() && duel::remove(d, tg, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local exc=c:IsRelateToEffect(e) and c or nil
        let exc = if card::is_relate_to_effect(d, c, a.e) { c.into() } else { NO_EX };
        // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,exc)
        let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, exc);
        // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
        if !g.is_empty() {
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
