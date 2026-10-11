//! Hatsugai
use super::prelude::*;

pub const ID: u32 = 43618262;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RECOVER+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_RECOVER + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)>=5 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_DECK) >= 5;
    }
    true
}
/// `declare_lp_table` (`i*100` for `i=1..30`)
const DECLARE_LP_TABLE: [u64; 30] = [
    100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000, 2100, 2200, 2300, 2400, 2500, 2600, 2700, 2800, 2900, 3000,
];
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.ConfirmDecktop(1-tp,5)
    let _ = duel::confirm_decktop(d, 1 - tp, 5);
    // local g=Duel.GetDecktopGroup(1-tp,5)
    let g = duel::get_decktop_group(d, 1 - tp, 5);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel).nil();
    // Duel.ConfirmCards(1-tp,sc)
    duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
    // local ac=Duel.AnnounceNumber(tp,declare_lp_table)
    let ac = duel::announce_number(d, tp, &DECLARE_LP_TABLE).nil().0 as i64;
    // if sc:IsAbleToHand() and Duel.SelectYesNo(1-tp,aux.Stringid(id,1))
    //     and Duel.Recover(1-tp,ac,REASON_EFFECT)>0 then
    if card::is_able_to_hand(d, sc, None)
        && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 1))
        && duel::recover(d, 1 - tp, ac, REASON_EFFECT, false, None) > 0
    {
        // Duel.SetLP(tp,Duel.GetLP(tp)-ac)
        let lp = duel::get_lp(d, tp) - ac as i32;
        duel::set_lp(d, tp, lp);
        // Duel.DisableShuffleCheck(true)
        duel::disable_shuffle_check(d, true);
        // Duel.SendtoHand(sc,tp,REASON_EFFECT)
        duel::send_to_hand(d, CardSet::from_card(sc), Some(tp), REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sc)
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
    }
}
