//! Yu-Jo Friendship
use super::prelude::*;

pub const ID: u32 = 81332143;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[14731897], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local opt=0
    let mut opt: i64 = 0;
    // if Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_HAND,0,1,nil,14731897) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[14731897])), tp, LOCATION_HAND, 0, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsCode,tp,LOCATION_HAND,0,1,1,nil,14731897)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_code(d, c, &[14731897])), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // opt=1
        opt = 1;
    }
    // if opt==0 then
    if opt == 0 {
        // opt=Duel.SelectOption(1-tp,aux.Stringid(id,0),aux.Stringid(id,1))
        opt = duel::select_option(d, 1 - tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]) as i64;
    } else {
        // opt=Duel.SelectOption(1-tp,aux.Stringid(id,0))
        opt = duel::select_option(d, 1 - tp, true, &[aux::stringid(ID, 0)]) as i64;
    }
    // if opt==0 then
    if opt == 0 {
        // local lp=(Duel.GetLP(tp)+Duel.GetLP(1-tp))/2
        let sum = duel::get_lp(d, tp) + duel::get_lp(d, 1 - tp);
        // (Lua `/` is float division: the core rounds the half away from zero)
        let lp = lua_int(sum as f64 / 2.0) as i32;
        // Duel.SetLP(tp,lp)
        duel::set_lp(d, tp, lp);
        // Duel.SetLP(1-tp,lp)
        duel::set_lp(d, 1 - tp, lp);
    }
}
