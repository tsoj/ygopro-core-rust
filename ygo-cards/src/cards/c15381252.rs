//! Berserker Soul
//! 狂戦士の魂
use super::prelude::*;

pub const ID: u32 = 15381252;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e1, CATEGORY_DECKDES);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // return ep==1-tp and eg:GetFirst():IsControler(tp) and ev<=1500 and Duel.GetAttackTarget()==nil
    if a.ep as i64 != 1 - tp as i64 {
        return false;
    }
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    if !card::is_controler(d, first, tp) {
        return false;
    }
    a.ev <= 1500 && duel::get_attack_target(d).is_none()
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
        let mut g = duel::get_field_group(d, tp, LOCATION_HAND, 0);
        // g:RemoveCard(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        group::remove_card(&mut g, &CardSet::from_card(h));
        // return #g>0 and g:FilterCount(Card.IsDiscardable,nil)==#g
        if g.is_empty() {
            return false;
        }
        let n = g.len();
        return group::filter_count(d, &g, &|d, c| card::is_discardable(d, c, None, None), NO_EX) == n;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let g = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    // Duel.SendtoGrave(g,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, g, REASON_COST | REASON_DISCARD, None, None);
    true
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`: `if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1) end`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local count=8
    let mut count: i64 = 8;
    // while count>0 and Duel.IsPlayerCanDiscardDeck(tp,1) and Duel.GetLP(1-tp)>0 do
    while count > 0 && duel::is_player_can_discard_deck(d, tp, 1) && duel::get_lp(d, 1 - tp) > 0 {
        // if count<8 then Duel.BreakEffect() end
        if count < 8 {
            duel::break_effect(d);
        }
        // Duel.ConfirmDecktop(tp,1)
        duel::confirm_decktop(d, tp, 1);
        // local g=Duel.GetDecktopGroup(tp,1)
        let g = duel::get_decktop_group(d, tp, 1);
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g).nil();
        // if tc:IsMonster() then
        if card::is_monster(d, tc) {
            // Duel.DisableShuffleCheck()
            duel::disable_shuffle_check(d, true);
            // Duel.SendtoGrave(tc,REASON_EFFECT|REASON_EXCAVATE)
            duel::send_to_grave(d, tc, REASON_EFFECT | REASON_EXCAVATE, None, None);
            // if tc:IsLocation(LOCATION_GRAVE) then
            if card::is_location(d, tc, LOCATION_GRAVE) {
                // Duel.Damage(1-tp,500,REASON_EFFECT)
                duel::damage(d, 1 - tp, 500, REASON_EFFECT, false, None);
                // count=count-1
                count -= 1;
            } else {
                count = 0;
            }
        } else {
            // count=0
            count = 0;
        }
    }
}
