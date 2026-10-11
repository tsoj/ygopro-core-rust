//! Spellbook Library of the Crescent
use super::prelude::*;

pub const ID: u32 = 40230018;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SPELLBOOK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Reveal 3 "Spellbook" Spell Cards and make the opponent randomly add 1 of them to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,s.chainfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(chainfilter));
}
/// `s.chainfilter(re,tp,cid)`
fn chainfilter(d: &mut Duel, a: Args) -> bool {
    let re = a.e(0);
    // return not (re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and not re:GetHandler():IsSetCard(SET_SPELLBOOK))
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return true;
    }
    if !effect::is_spell_effect(d, re) {
        return true;
    }
    let rc = effect::get_handler(d, re).nil();
    !card::is_set_card(d, rc, &[SET_SPELLBOOK as u16])
}
/// `s.confilter(c)`
fn confilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPELLBOOK) and c:IsSpell()
    card::is_set_card(d, c, &[SET_SPELLBOOK as u16]) && card::is_spell(d, c)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.confilter,tp,LOCATION_GRAVE,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&confilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.value(e,re,tp)` (the value of EFFECT_CANNOT_ACTIVATE)
fn cannot_activate_value(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:IsSpellEffect() and not re:GetHandler():IsSetCard(SET_SPELLBOOK)
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    if !effect::is_spell_effect(d, re) {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    !card::is_set_card(d, rc, &[SET_SPELLBOOK as u16])
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_CHAIN)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, tp, ACTIVITY_CHAIN) == 0;
    }
    // You cannot activate any Spells the turn you activate this card, except "Spellbook" Spells
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetValue(function(e,re,tp) return ... end)
    effect::set_value_fn(d, e1, lua_fn(cannot_activate_value));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SPELLBOOK) and c:IsSpell() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_SPELLBOOK as u16]) && card::is_spell(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
        // return g:GetClassCount(Card.GetCode)>=3
        return group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&thfilter), tp, LOCATION_DECK, 0, NO_EX);
    // if g:GetClassCount(Card.GetCode)>=3 then
    if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3 {
        // local revg=aux.SelectUnselectGroup(g,e,tp,3,3,aux.dncheck,1,tp,HINTMSG_CONFIRM)
        let revg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&aux::dncheck()), 1, tp, Some(HINTMSG_CONFIRM as u64), None, None, false)
            .unwrap_or_default();
        // Duel.ConfirmCards(1-tp,revg)
        duel::confirm_cards(d, 1 - tp, revg.clone());
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_ATOHAND as u64);
        // local sg=revg:Select(1-tp,1,1,nil)
        let sg = group::select(d, &revg, 1 - tp, 1, 1, false, NO_EX).nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
        }
    }
}
