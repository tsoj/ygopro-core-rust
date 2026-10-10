//! Guiding Ariadne
use super::prelude::*;

pub const ID: u32 = 98301564;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --pendulum summon
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // --Cost Change
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_LPCOST_CHANGE)
    effect::set_code(d, e2, EFFECT_LPCOST_CHANGE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetValue(s.costchange)
    effect::set_value_fn(d, e2, lua_fn(costchange));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_DISCARD_COST_CHANGE)
    effect::set_code(d, e3, EFFECT_DISCARD_COST_CHANGE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetRange(LOCATION_PZONE)
    effect::set_range(d, e3, LOCATION_PZONE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // --search
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCondition(s.regcon)
    effect::set_condition(d, e4, Some(condition(regcon)));
    // e4:SetTarget(s.regtg)
    effect::set_target(d, e4, Some(cost_target(regtg)));
    // e4:SetOperation(s.regop)
    effect::set_operation(d, e4, Some(operation(regop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
// function s.costchange(e,re,rp,val)
fn costchange(d: &mut Duel, a: Args) -> i64 {
    // if re and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsTrap() and re:GetHandler():IsType(TYPE_COUNTER) then
    if let Some(re) = a.v(1).as_effect() {
        if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
            let h = effect::get_handler(d, re).nil();
            if card::is_trap(d, h) && card::is_type(d, h, TYPE_COUNTER) {
                // return 0
                return 0;
            }
        }
    }
    // else return val end
    a.int(3)
}
// function s.regcon(e,tp,eg,ep,ev,re,r,rp)
fn regcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_EFFECT+REASON_BATTLE)~=0
    (a.r & (REASON_EFFECT + REASON_BATTLE)) != 0
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsTrap() and c:IsType(TYPE_COUNTER) and c:IsAbleToHand()
    card::is_trap(d, c) && card::is_type(d, c, TYPE_COUNTER) && card::is_able_to_hand(d, c, None)
}
// function s.regtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn regtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return not e:GetHandler():IsLocation(LOCATION_DECK)
    //   and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_DECK,0,3,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return !card::is_location(d, h, LOCATION_DECK) && duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_DECK, 0, 3, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,0,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 0, LOCATION_DECK as i32);
    true
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g>=3 then
    if g.len() >= 3 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,3,3,nil)
        let sg = group::select(d, &g, a.tp, 3, 3, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg.clone());
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_ATOHAND as u64);
        // local tg=sg:Select(1-tp,1,1,nil)
        let tg = group::select(d, &sg, 1 - a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(tg,nil,REASON_EFFECT)
        duel::send_to_hand(d, tg, None, REASON_EFFECT, None);
    }
}
