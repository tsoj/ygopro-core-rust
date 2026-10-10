//! Frozen Rose
use super::prelude::*;

pub const ID: u32 = 53503015;

/// `s.check` (a variable on the script table, initially `false`)
const S_CHECK: u64 = 0;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate (draw and discard)
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, _a: &EffArgs) -> bool {
    // s.check=true
    set_s_var(d, ID, S_CHECK, Value::Bool(true));
    // if chk==0 then return true end
    true
}
/// `s.cfilter(c,chk,p,chk1,chk2)`
fn cfilter(d: &mut Duel, c: CardId, chk: i64, p: bool, chk1: bool, chk2: bool) -> bool {
    // return c:IsFaceup() and c:IsAbleToGraveAsCost() and ((chk == 0 and c:IsRace(RACE_PLANT)==p)
    //     or ((c:IsRace(RACE_PLANT) and chk1) or (not c:IsRace(RACE_PLANT) and chk2)))
    card::is_faceup(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && ((chk == 0 && card::is_race(d, c, RACE_PLANT) == p)
            || ((card::is_race(d, c, RACE_PLANT) && chk1) || (!card::is_race(d, c, RACE_PLANT) && chk2)))
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PLANT) and c:IsLevelBelow(4) and c:IsAbleToHand()
    card::is_race(d, c, RACE_PLANT) && card::is_level_below(d, c, 4) && card::is_able_to_hand(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local chk1 = Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil,0,true)
    let chk1 = duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, 0, true, false, false)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    // local chk2 = Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil,0,false)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
    let chk2 = duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, 0, false, false, false)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then
    if a.chk == Some(0) {
        // if not s.check then return false end
        if !s_var(d, ID, S_CHECK).truthy() {
            return false;
        }
        // s.check=false
        set_s_var(d, ID, S_CHECK, Value::Bool(false));
        // return chk1 or chk2
        return chk1 || chk2;
    }
    // e:SetLabel(0)
    effect::set_label(d, e, &[0]);
    // e:SetCategory(0)
    effect::set_category(d, e, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_MZONE,0,1,1,nil,1,nil,chk1,chk2)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| cfilter(d, c, 1, false, chk1, chk2)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // local opt=g:GetFirst():IsRace(RACE_PLANT) and 0 or 1
    let first = group::get_first(d, &g).nil();
    let opt = if card::is_race(d, first, RACE_PLANT) { 0 } else { 1 };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // if opt == 0 then
    if opt == 0 {
        // e:SetLabel(1)
        effect::set_label(d, e, &[1]);
        // e:SetCategory(CATEGORY_DRAW+CATEGORY_HANDES)
        effect::set_category(d, e, CATEGORY_DRAW + CATEGORY_HANDES);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,2)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 2);
        // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    } else {
        // e:SetLabel(2)
        effect::set_label(d, e, &[2]);
        // e:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, e, CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,0,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 0, tp, LOCATION_DECK as i32);
    }
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_CARD,PLAYER_ALL,id)
    duel::hint(d, HINT_CARD as u8, PLAYER_ALL as u8, ID as u64);
    // if Duel.Draw(tp,2,REASON_EFFECT)==2 then
    if duel::draw(d, tp, 2, REASON_EFFECT) == 2 {
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)
        duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
    }
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local opt=e:GetLabel()
    let opt = effect::get_label(d, e);
    // if opt==1 then
    if opt == 1 {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        // e1:SetOperation(s.drop)
        effect::set_operation(d, e1, Some(operation(drop_op)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    } else if opt == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, &g);
        }
    }
}
