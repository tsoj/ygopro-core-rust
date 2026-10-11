//! Malefic Paradigm Dragon
use super::prelude::*;

pub const ID: u32 = 16958382;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[27564031], listed_series: &[SET_MALEFIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // aux.AddMaleficSummonProcedure(c,nil,LOCATION_EXTRA,s.spcon)
    aux::add_malefic_summon_procedure(d, c, None, LOCATION_EXTRA, Some(lua_fn(spcon)));
    // Special Summon condition
    // local e1=Effect.CreateEffect(c)
    // e1:SetType(EFFECT_TYPE_SINGLE)
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    // e1:SetValue(aux.FALSE)
    // c:RegisterEffect(e1)
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, aux_false());
    card::register_effect(d, c, e1, false);
    // Self-destruct
    // local e2=Effect.CreateEffect(c)
    // e2:SetType(EFFECT_TYPE_SINGLE)
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    // e2:SetRange(LOCATION_MZONE)
    // e2:SetCode(EFFECT_SELF_DESTROY)
    // e2:SetCondition(s.descon)
    // c:RegisterEffect(e2)
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_SELF_DESTROY);
    effect::set_condition(d, e2, Some(condition(descon)));
    card::register_effect(d, c, e2, false);
    // Return to Extra Deck and Special Summon
    // local e3=Effect.CreateEffect(c)
    // e3:SetCategory(CATEGORY_TOEXTRA+CATEGORY_SPECIAL_SUMMON)
    // e3:SetType(EFFECT_TYPE_IGNITION)
    // e3:SetRange(LOCATION_MZONE)
    // e3:SetCountLimit(1)
    // e3:SetCost(s.retcost)
    // e3:SetTarget(s.rettg)
    // e3:SetOperation(s.retop)
    // c:RegisterEffect(e3)
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_TOEXTRA + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_cost(d, e3, Some(cost_target(retcost)));
    effect::set_target(d, e3, Some(cost_target(rettg)));
    effect::set_operation(d, e3, Some(operation(retop)));
    card::register_effect(d, c, e3, false);
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,id),e:GetHandlerPlayer(),LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
    let e = a.e(0);
    let p = effect::get_handler_player(d, e);
    !duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[ID])), p, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
}
// function s.descon(e)
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not (Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,27564031),e:GetHandlerPlayer(),LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) or Duel.IsEnvironment(27564031))
    let p = effect::get_handler_player(d, a.e);
    let b1 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[27564031])), p, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    !(b1 || duel::is_environment(d, 27564031, None, None))
}
// function s.costfilter(c)
fn costfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MALEFIC) and c:IsAbleToGraveAsCost()
    card::is_set_card(d, c, &[SET_MALEFIC as u16]) && card::is_able_to_grave_as_cost(d, c)
}
// function s.retcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn retcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.costfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&costfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.costfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&costfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
// function s.retfilter(c)
fn retfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_SYNCHRO) and c:IsLevel(8) and c:IsAbleToExtra() and c:IsFaceup()
    card::is_type(d, c, TYPE_SYNCHRO) && card::is_level(d, c, &[8]) && card::is_able_to_extra(d, c) && card::is_faceup(d, c)
}
// function s.rettg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rettg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.retfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&retfilter), tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,nil,1,tp,LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, None, 1, tp, LOCATION_REMOVED as i32);
    true
}
// function s.retop(e,tp,eg,ep,ev,re,r,rp)
fn retop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // Can only attack with "Malefic" monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    // e1:SetType(EFFECT_TYPE_FIELD)
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    // e1:SetTargetRange(LOCATION_MZONE,0)
    // e1:SetTarget(s.atktg)
    // e1:SetReset(RESET_PHASE|PHASE_END)
    // Duel.RegisterEffect(e1,tp)
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(atktg)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    // e2:SetDescription(aux.Stringid(id,0))
    // e2:SetReset(RESET_PHASE|PHASE_END)
    // e2:SetTargetRange(1,0)
    // Duel.RegisterEffect(e2,tp)
    let e2 = effect::create_effect(d, h);
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e2, 1, 0);
    duel::register_effect(d, e2, tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,s.retfilter,tp,LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&retfilter), tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // if #g>0 and Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0 and Duel.GetLocationCountFromEx(tp,tp,nil,g:GetFirst())>0 then
    if g.is_empty() {
        return;
    }
    if duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) <= 0 {
        return;
    }
    let first = group::get_first(d, &g);
    let loc_ok = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(first.nil()), None).0 > 0;
    if !loc_ok {
        return;
    }
    // local sc=Duel.GetOperatedGroup():GetFirst()
    let og = duel::get_operated_group(d);
    let sc = group::get_first(d, &og).nil();
    // if sc:IsLocation(LOCATION_EXTRA) and sc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if card::is_location(d, sc, LOCATION_EXTRA)
        && card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.atktg(e,c)
fn atktg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_MALEFIC)
    !card::is_set_card(d, c, &[SET_MALEFIC as u16])
}
