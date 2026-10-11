//! Puppet Shark
//! 傀儡流儀－パペット・シャーク
use super::prelude::*;

pub const ID: u32 = 53323475;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=4
    //   and Duel.CheckRemoveOverlayCard(tp,1,1,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) >= 4
            && duel::check_remove_overlay_card(d, a.tp, 1, 1, 1, REASON_EFFECT, None);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return ((c:IsMonster() or c:IsSpell()) and c:IsAbleToHand()) or (c:IsTrap() and c:IsSSetable())
    ((card::is_monster(d, c) || card::is_spell(d, c)) && card::is_able_to_hand(d, c, None))
        || (card::is_trap(d, c) && card::is_s_setable(d, c, false, None))
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.RemoveOverlayCard(tp,1,1,1,1,REASON_EFFECT)>0 and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=4 then
    if duel::remove_overlay_card(d, a.tp, 1, 1, 1, 1, REASON_EFFECT, None) > 0 && duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) >= 4 {
        // Duel.ConfirmDecktop(tp,4)
        duel::confirm_decktop(d, a.tp, 4);
        // local g=Duel.GetDecktopGroup(tp,4)
        let g = duel::get_decktop_group(d, a.tp, 4);
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
        // local sc=g:FilterSelect(tp,s.filter,1,1,nil):GetFirst()
        let sel = group::filter_select(d, &g, a.tp, &filter, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if not sc then return end
        let Some(sc) = sc else { return };
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // if (sc:IsMonster() or sc:IsSpell()) and Duel.SendtoHand(sc,nil,REASON_EFFECT)>0
        //   and sc:IsLocation(LOCATION_HAND) then
        if (card::is_monster(d, sc) || card::is_spell(d, sc))
            && duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0
            && card::is_location(d, sc, LOCATION_HAND)
        {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - a.tp, sc);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, a.tp);
        } else if card::is_trap(d, sc) && duel::s_set(d, a.tp, sc, None, true) > 0 {
            // sc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
            card::register_flag_effect(d, sc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
            // It can be activated this turn
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(aux.Stringid(id,3))
            effect::set_description(d, e1, aux::stringid(ID, 3));
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
            effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
            // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
            effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
        }
    }
}
