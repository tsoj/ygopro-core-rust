//! Rebirth of Parshath
use super::prelude::*;

pub const ID: u32 = 42444868;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PARSHATH], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE)) and Duel.IsChainNegatable(ev)
    (effect::is_monster_effect(d, a.re.nil()) || effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE))
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_COUNTER) and not c:IsPublic()
    card::is_type(d, c, TYPE_COUNTER) && !card::is_public(d, c)
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local h=e:GetHandler() (the Lua reads it per use; the card is the same)
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,e:GetHandler())
    //     and (Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,e:GetHandler())
    //     or Duel.IsPlayerAffectedByEffect(tp,EFFECT_DISCARD_COST_CHANGE))
    //     and Duel.CheckLPCost(tp,1000) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c)), a.tp, LOCATION_HAND, 0, 1, h.into())
            && (duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, h.into())
                || duel::is_player_affected_by_effect(d, a.tp, EFFECT_DISCARD_COST_CHANGE).is_some())
            && duel::check_lp_cost(d, a.tp, 1000);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil)
    let cg = duel::select_matching_card(d, a.tp, Some(&|d, c| cfilter(d, c)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tc=cg:GetFirst()
    let tc = group::get_first(d, &cg).nil();
    // Duel.ConfirmCards(1-tp,cg)
    duel::confirm_cards(d, 1 - a.tp, cg);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_PUBLIC)
    effect::set_code(d, e1, EFFECT_PUBLIC);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CHAIN)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetOperation(s.clearop)
    effect::set_operation(d, e2, Some(operation(clearop)));
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CHAIN)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
    // e2:SetLabel(Duel.GetCurrentChain())
    effect::set_label(d, e2, &[duel::get_current_chain(d, false) as i64]);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
    // if not Duel.IsPlayerAffectedByEffect(tp,EFFECT_DISCARD_COST_CHANGE) then
    if duel::is_player_affected_by_effect(d, a.tp, EFFECT_DISCARD_COST_CHANGE).is_none() {
        // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD)
        duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    }
    // Duel.PayLPCost(tp,1000)
    duel::pay_lp_cost(d, a.tp, 1000);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsAbleToDeck() and re:GetHandler():IsRelateToEffect(re) then
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_able_to_deck(d, rh) && card::is_relate_to_effect(d, rh, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, a.eg.clone(), 1, 0, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    }
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_PARSHATH) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    // and ((c:IsLocation(LOCATION_DECK) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
    // or (c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0))
    card::is_set_card(d, c, &[SET_PARSHATH as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && ((card::is_location(d, c, LOCATION_DECK) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0)
            || (card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0))
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local ec=re:GetHandler()
    let ec = effect::get_handler(d, a.re.nil()).nil();
    // if Duel.NegateActivation(ev) and ec:IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, ec, a.re.nil()) {
        // ec:CancelToGrave()
        card::cancel_to_grave(d, ec, true);
        // if Duel.SendtoDeck(ec,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)~=0 and ec:IsLocation(LOCATION_DECK|LOCATION_EXTRA) then
        if duel::send_to_deck(d, ec, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) != 0 && card::is_location(d, ec, LOCATION_DECK | LOCATION_EXTRA) {
            // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,nil,e,tp)
            let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, NO_EX);
            // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
            if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
                // local sg=g:Select(tp,1,1,nil)
                let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
/// `s.clearop(e,tp,eg,ep,ev,re,r,rp)`
fn clearop(d: &mut Duel, a: &EffArgs) {
    // if ev~=e:GetLabel() then return end
    if a.ev as i64 != effect::get_label(d, a.e) {
        return;
    }
    // e:GetLabelObject():Reset()
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::reset(d, lo);
    // e:Reset()
    effect::reset(d, a.e);
}
