//! Absolute King Back Jack
use super::prelude::*;

pub const ID: u32 = 60990740;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Excavate the top card of your Deck and set it if it is a Normal Trap
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_GRAVE);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    effect::set_cost(d, e1, Some(cost::self_banish()));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
    // Excavate the top 3 cards of your Deck
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(sdtg)));
    effect::set_operation(d, e2, Some(operation(sdop)));
    card::register_effect(d, c, e2, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1)
    //     and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.CanPlayerSetSpellTrap(tp) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1)
            && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::can_player_set_spell_trap(d, a.tp, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsPlayerCanDiscardDeck(tp,1) or not Duel.CanPlayerSetSpellTrap(tp) then return end
    if !duel::is_player_can_discard_deck(d, a.tp, 1) || !duel::can_player_set_spell_trap(d, a.tp, None) {
        return;
    }
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, a.tp, 1);
    // local tc=Duel.GetDecktopGroup(tp,1):GetFirst()
    let tc = group::get_first(d, &duel::get_decktop_group(d, a.tp, 1)).nil();
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // if tc:IsNormalTrap() and tc:IsSSetable() then
    if card::is_normal_trap(d, tc) && card::is_s_setable(d, tc, false, None) {
        // Duel.SSet(tp,tc)
        duel::s_set(d, a.tp, tc, None, true);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_TRAP_ACT_IN_SET_TURN)
        effect::set_code(d, e1, EFFECT_TRAP_ACT_IN_SET_TURN);
        // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE)
        effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    } else {
        // Duel.SendtoGrave(tc,REASON_EFFECT|REASON_EXCAVATE)
        duel::send_to_grave(d, tc, REASON_EFFECT | REASON_EXCAVATE, None, None);
    }
}
/// `s.sdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>2 end
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 2;
    }
    true
}
/// `s.sdop(e,tp,eg,ep,ev,re,r,rp)`
fn sdop(d: &mut Duel, a: &EffArgs) {
    // Duel.SortDecktop(tp,tp,3)
    duel::sort_decktop(d, a.tp, a.tp, 3);
}
