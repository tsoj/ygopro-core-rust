//! 三幻魔の神淵
use super::prelude::*;

pub const ID: u32 = 86132414;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // You can send 3 face-up "Divine Abyss of the Sacred Beast" you control to the GY, including this card, then target 1 face-up monster your opponent controls; change it to face-down Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCost(s.poscost)
    effect::set_cost(d, e1, Some(cost_target(poscost)));
    // e1:SetTarget(s.postg)
    effect::set_target(d, e1, Some(cost_target(postg)));
    // e1:SetOperation(s.posop)
    effect::set_operation(d, e1, Some(operation(posop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // During the Main Phase: You can place 2 "Divine Abyss of the Sacred Beast" from your hand, Deck, and/or GY face-up on your field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(main_phase_cond)));
    // e2:SetTarget(s.pltg)
    effect::set_target(d, e2, Some(cost_target(pltg)));
    // e2:SetOperation(s.plop)
    effect::set_operation(d, e2, Some(operation(plop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // During your opponent's End Phase, if this card is in your GY: You can place it on the bottom of the Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e3, CATEGORY_TODECK);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsEndPhase(1-tp) end)
    effect::set_condition(d, e3, Some(condition(end_phase_cond)));
    // e3:SetTarget(s.tdtg)
    effect::set_target(d, e3, Some(cost_target(tdtg)));
    // e3:SetOperation(s.tdop)
    effect::set_operation(d, e3, Some(operation(tdop)));
    // e3:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e3, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn main_phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
fn end_phase_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // Duel.IsEndPhase(1-tp)
    duel::is_end_phase(d, Some(1 - a.tp))
}
fn poscostfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsCode(id) and c:IsAbleToGraveAsCost() and c:IsFaceup()
    card::is_code(d, c, &[ID]) && card::is_able_to_grave_as_cost(d, c) && card::is_faceup(d, c)
}
fn poscost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED)
    //     and Duel.IsExistingMatchingCard(s.poscostfilter,tp,LOCATION_ONFIELD,0,2,c) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c)
            && card::is_status(d, c, STATUS_EFFECT_ENABLED)
            && duel::is_existing_matching_card(d, Some(&poscostfilter), tp, LOCATION_ONFIELD, 0, 2, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.poscostfilter,tp,LOCATION_ONFIELD,0,2,2,c)
    let mut g = duel::select_matching_card(d, tp, Some(&poscostfilter), tp, LOCATION_ONFIELD, 0, 2, 2, false, c.into()).nil();
    // Duel.SendtoGrave(g+c,REASON_COST)
    group::add_card(&mut g, &CardSet::from_card(c));
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsCanTurnSet() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_can_turn_set(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsCanTurnSet,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_can_turn_set(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,Card.IsCanTurnSet,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_can_turn_set(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,tp,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, g, 1, tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
fn posop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsMonster() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_monster(d, tc) {
        // Duel.ChangePosition(tc,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
fn selfplfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // c:IsCode(id) and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_code(d, c, &[ID]) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>=2
    //     and Duel.IsExistingMatchingCard(s.selfplfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,2,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 >= 2
            && duel::is_existing_matching_card(d, Some(&|d, c| selfplfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 2, NO_EX);
    }
    true
}
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<2 then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOFIELD as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.selfplfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,2,2,nil,tp)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| selfplfilter(d, c, tp))),
        tp,
        LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
        0,
        2,
        2,
        false,
        NO_EX,
    )
    .nil();
    // if #g==2 then
    if g.len() == 2 {
        let order = group::iter(d, &g);
        // Duel.MoveToField(g:GetFirst(),tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, order[0], tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
        // Duel.MoveToField(g:GetNext(),tp,tp,LOCATION_SZONE,POS_FACEUP,true)
        duel::move_to_field(d, order[1], tp, tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
    }
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToDeck() end
    if a.chk == Some(0) {
        return card::is_able_to_deck(d, c);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoDeck(c,nil,SEQ_DECKBOTTOM,REASON_EFFECT)
        duel::send_to_deck(d, c, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
    }
}
