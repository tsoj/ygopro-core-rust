//! Ante
use super::prelude::*;

pub const ID: u32 = 34236961;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local h1=Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)
        let mut h1 = duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0);
        // if e:GetHandler():IsLocation(LOCATION_HAND) then h1=h1-1 end
        let c = effect::get_handler(d, a.e).nil();
        if card::is_location(d, c, LOCATION_HAND) {
            h1 -= 1;
        }
        // local h2=Duel.GetFieldGroupCount(1-tp,LOCATION_HAND,0)
        let h2 = duel::get_field_group_count(d, 1 - a.tp, LOCATION_HAND, 0);
        // return (h1>0 and h2>0)
        return h1 > 0 && h2 > 0;
    }
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0 or Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)==0 then return end
    if duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) == 0 || duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) == 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local g1=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_HAND,0,1,1,nil)
    let g1 = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_CONFIRM as u64);
    // local g2=Duel.SelectMatchingCard(1-tp,nil,1-tp,LOCATION_HAND,0,1,1,nil)
    let g2 = duel::select_matching_card(d, 1 - a.tp, None, 1 - a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g1)
    duel::confirm_cards(d, 1 - a.tp, &g1);
    // Duel.ConfirmCards(tp,g2)
    duel::confirm_cards(d, a.tp, &g2);
    // local atpsl=g1:GetFirst()
    let atpsl = group::get_first(d, &g1).nil();
    // local ntpsl=g2:GetFirst()
    let ntpsl = group::get_first(d, &g2).nil();
    // local atplv=atpsl:IsMonster() and atpsl:GetLevel() or 0
    let atplv = if card::is_monster(d, atpsl) { card::get_level(d, atpsl) } else { 0 };
    // local ntplv=ntpsl:IsMonster() and ntpsl:GetLevel() or 0
    let ntplv = if card::is_monster(d, ntpsl) { card::get_level(d, ntpsl) } else { 0 };
    if atplv == ntplv {
        duel::shuffle_hand(d, a.tp);
        duel::shuffle_hand(d, 1 - a.tp);
    } else if atplv > ntplv {
        duel::damage(d, 1 - a.tp, 1000, REASON_EFFECT, false, None);
        duel::send_to_grave(d, &g2, REASON_EFFECT, None, None);
        duel::shuffle_hand(d, a.tp);
    } else {
        duel::damage(d, a.tp, 1000, REASON_EFFECT, false, None);
        duel::send_to_grave(d, &g1, REASON_EFFECT, None, None);
        duel::shuffle_hand(d, 1 - a.tp);
    }
}
