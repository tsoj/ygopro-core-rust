//! Cybernetic Overflow
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 82428674;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CYBERNETIC, SET_CYBER], listed_names: &[CARD_CYBER_DRAGON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish "Cyber Dragon(s)" then destroy an equal number of cards your opponent controls
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_REMOVE);
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
    // Add 1 "Cyber" Spell/Trap from your Deck to your hand
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.rmfilter(c)
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return (not c:IsLocation(LOCATION_MZONE) or c:IsFaceup()) and c:IsCode(CARD_CYBER_DRAGON) and c:IsLevelAbove(1) and c:IsAbleToRemove()
    (!card::is_location(d, c, LOCATION_MZONE) || card::is_faceup(d, c))
        && card::is_code(d, c, &[CARD_CYBER_DRAGON])
        && card::is_level_above(d, c, 1)
        && card::is_able_to_remove(d, c, None, None, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local dg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let dg = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE,0,1,nil) and #dg>0 end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmfilter), a.tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE, 0, 1, NO_EX) && !dg.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE) as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,1,1-tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
// dpcheck(Card.GetLevel)
fn get_level_value(d: &mut Duel, c: CardId) -> i64 {
    card::get_level(d, c) as i64
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local dg=Duel.GetMatchingGroup(nil,tp,0,LOCATION_ONFIELD,nil)
    let dg = duel::get_matching_group(d, None, a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_HAND|LOCATION_GRAVE|LOCATION_MZONE,0,nil)
    let g = duel::get_matching_group(d, Some(&rmfilter), a.tp, LOCATION_HAND | LOCATION_GRAVE | LOCATION_MZONE, 0, NO_EX);
    // if #dg==0 or #g==0 then return end
    if dg.is_empty() || g.is_empty() {
        return;
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,1,#dg,aux.dpcheck(Card.GetLevel),1,tp,HINTMSG_REMOVE)
    let rescon_fn = aux::dpcheck(get_level_value);
    let rescon: aux::Rescon = &rescon_fn;
    let dgn = group::get_count(&dg);
    let rg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(dgn), Some(rescon), 1, a.tp, Some(HINTMSG_REMOVE as u64), None, None, false).unwrap_or_default();
    // local rc=Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
    let rc = duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
    // if rc>0 then
    if rc > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local sg=dg:Select(tp,rc,rc,nil)
        let sg = group::select(d, &dg, a.tp, rc as u16, rc as u16, false, NO_EX).nil();
        // Duel.Destroy(sg,REASON_EFFECT)
        duel::destroy(d, sg, REASON_EFFECT, None, None);
    }
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and r&REASON_EFFECT~=0
    card::is_previous_location(d, c, LOCATION_ONFIELD) && (a.r & REASON_EFFECT) != 0
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsSetCard({SET_CYBERNETIC,SET_CYBER}) and c:IsAbleToHand()
    card::is_spell_trap(d, c) && card::is_set_card(d, c, &[SET_CYBERNETIC as u16, SET_CYBER as u16]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
