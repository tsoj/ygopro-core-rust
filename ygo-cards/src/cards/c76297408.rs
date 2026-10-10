//! Soul Demolition
use super::prelude::*;

pub const ID: u32 = 76297408;

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
    // Each player banished a card from their GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetCondition(s.rmcon)
    effect::set_condition(d, e2, Some(condition(rmcon)));
    // e2:SetCost(Cost.PayLP(500))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_FIEND),tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_FIEND)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.rfilter(c)`
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemove() and aux.SpElimFilter(c)
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, false)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.rfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,nil)
    //     and Duel.IsExistingTarget(s.rfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rfilter), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&rfilter), a.tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // local sel_player=Duel.GetTurnPlayer()
    let sel_player = duel::get_turn_player(d);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g1=Duel.SelectTarget(sel_player,s.rfilter,sel_player,0,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil)
    let mut g1 = duel::select_target(d, sel_player, Some(&rfilter), sel_player, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_REMOVE as u64);
    // local g2=Duel.SelectTarget(1-sel_player,s.rfilter,1-sel_player,0,LOCATION_MZONE|LOCATION_GRAVE,1,1,nil)
    let g2 = duel::select_target(d, 1 - sel_player, Some(&rfilter), 1 - sel_player, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g1,#g1,0,0)
    let n = g1.len();
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g1), n as u8, 0, 0);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // g1,g2=g:Split(Card.IsControler,nil,tp)
    let (g1, g2) = group::split(d, &g, &|d, c| card::is_controler(d, c, a.tp), NO_EX);
    // if #g1>0 then
    if !g1.is_empty() {
        // Duel.Remove(g1,POS_FACEUP,REASON_EFFECT,PLAYER_NONE,1-tp)
        duel::remove(d, g1, POS_FACEUP as u8, REASON_EFFECT, None, Some(1 - a.tp));
    }
    // if #g2>0 then
    if !g2.is_empty() {
        // Duel.Remove(g2,POS_FACEUP,REASON_EFFECT,PLAYER_NONE,tp)
        duel::remove(d, g2, POS_FACEUP as u8, REASON_EFFECT, None, Some(a.tp));
    }
}
