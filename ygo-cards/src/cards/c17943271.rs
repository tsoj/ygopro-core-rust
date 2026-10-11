//! Mementotlan Goblin
use super::prelude::*;

pub const ID: u32 = 17943271;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MEMENTO as u32],
    listed_names: &[CARD_MEMENTOAL_TECUHTLICA, 17943271],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Make your opponent cannot target "Memento" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some((TIMINGS_CHECK_MONSTER | TIMING_MAIN_END) as u32));
    // e1:SetCondition(s.untgcon)
    effect::set_condition(d, e1, Some(condition(untgcon)));
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetOperation(s.untgop)
    effect::set_operation(d, e1, Some(operation(untgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send up to 2 "Memento" cards with different names from your Deck to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.untgcon(e,tp,eg,ep,ev,re,r,rp)`
fn untgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsMainPhase() and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_MEMENTOAL_TECUHTLICA),tp,LOCATION_ONFIELD,0,1,nil)
    duel::is_main_phase(d, None)
        && duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_MEMENTOAL_TECUHTLICA]))), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `s.untgop(e,tp,eg,ep,ev,re,r,rp)`
fn untgop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Your opponent cannot target "Memento" monsters you control with card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MEMENTO))
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MEMENTO as u16]))));
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MEMENTO) and c:IsAbleToGrave() and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_MEMENTO as u16]) && card::is_able_to_grave(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_MEMENTO),tp,LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_MEMENTO as u16]))), a.tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return #g>0 and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return !g.is_empty() && duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, a.tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsSetCard,SET_MEMENTO),tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_MEMENTO as u16]))), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) > 0 {
        // local tg=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_DECK,0,nil)
        let tg = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_DECK, 0, NO_EX);
        // local sg=aux.SelectUnselectGroup(tg,e,tp,1,2,aux.dncheck,1,tp,HINTMSG_TOGRAVE)
        let sg = aux::select_unselect_group(d, &tg, Some(a.e), tp, Some(1), Some(2), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SendtoGrave(sg,REASON_EFFECT)
            duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
        }
    }
}
