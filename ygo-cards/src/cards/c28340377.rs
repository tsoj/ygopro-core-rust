//! Court of Cards
use super::prelude::*;

pub const ID: u32 = 28340377;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_JACK_KNIGHT, CARD_QUEEN_KNIGHT, CARD_KING_KNIGHT],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Queen's Knight", "King's Knight", or "Jack's Knight" from your hand or GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Draw cards equal to the number of cards you banish to activate this effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_DRAW);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_cost(d, e3, Some(cost_target(drcost)));
    effect::set_operation(d, e3, Some(operation(drop_op)));
    card::register_effect(d, c, e3, false);
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() or not c:IsCode(CARD_JACK_KNIGHT,CARD_QUEEN_KNIGHT,CARD_KING_KNIGHT)
    card::is_facedown(d, c) || !card::is_code(d, c, &[CARD_JACK_KNIGHT, CARD_QUEEN_KNIGHT, CARD_KING_KNIGHT])
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&spconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_JACK_KNIGHT,CARD_QUEEN_KNIGHT,CARD_KING_KNIGHT) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_JACK_KNIGHT, CARD_QUEEN_KNIGHT, CARD_KING_KNIGHT]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) or Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.drcostfilter(c)`
fn drcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_JACK_KNIGHT,CARD_QUEEN_KNIGHT,CARD_KING_KNIGHT)
    //     and c:IsAbleToRemoveAsCost() and (c:IsLocation(LOCATION_HAND) or aux.SpElimFilter(c,true))
    card::is_code(d, c, &[CARD_JACK_KNIGHT, CARD_QUEEN_KNIGHT, CARD_KING_KNIGHT])
        && card::is_able_to_remove_as_cost(d, c, None)
        && (card::is_location(d, c, LOCATION_HAND) || aux::sp_elim_filter(d, c, true, false))
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.IsPlayerCanDraw(tp,#sg) and sg:GetClassCount(Card.GetCode)==#sg
    (duel::is_player_can_draw(d, tp, sg.len() as u32) && group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == sg.len(), false)
}
/// `s.drcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local cg=Duel.GetMatchingGroup(s.drcostfilter,tp,LOCATION_HAND|LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let cg = duel::get_matching_group(d, Some(&drcostfilter), a.tp, LOCATION_HAND | LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chk==0 then return aux.SelectUnselectGroup(cg,e,tp,1,1,s.rescon,0) end
    if a.chk == Some(0) {
        return matches!(aux::select_unselect_group(d, &cg, Some(a.e), a.tp, Some(1), Some(1), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // local g=aux.SelectUnselectGroup(cg,e,tp,1,3,s.rescon,1,tp,HINTMSG_REMOVE)
    let g = aux::select_unselect_group(d, &cg, Some(a.e), a.tp, Some(1), Some(3), Some(&rescon), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    let n = g.len();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(#g)
    duel::set_target_param(d, n as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,#g)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, n as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
