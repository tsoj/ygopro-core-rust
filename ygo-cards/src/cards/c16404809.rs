//! Kuribandit
use super::prelude::*;

pub const ID: u32 = 16404809;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // mill
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_operation(d, e1, Some(operation(sumsuc)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_DECKDES);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(lua_fn(cond)));
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(op)));
    card::register_effect(d, c, e2, false);
}
/// `s.sumsuc(e,...)`
fn sumsuc(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_TEMP_REMOVE|RESET_TURN_SET)|RESET_PHASE|PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | (RESETS_STANDARD & !(RESET_TEMP_REMOVE | RESET_TURN_SET)) | RESET_PHASE | PHASE_END, 0, 1, 0, 0);
}
/// `function(e) return e:GetHandler():HasFlagEffect(id) end`
fn cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::has_flag_effect(d, h, ID, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,5) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 5);
    }
    true
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToHand() and c:IsSpellTrap()
    card::is_able_to_hand(d, c, None) && card::is_spell_trap(d, c)
}
/// `s.operation(e,...)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.IsPlayerCanDiscardDeck(tp,5) then return end
    if !duel::is_player_can_discard_deck(d, tp, 5) {
        return;
    }
    // Duel.ConfirmDecktop(tp,5)
    duel::confirm_decktop(d, tp, 5);
    // local g=Duel.GetDecktopGroup(tp,5)
    let mut g = duel::get_decktop_group(d, tp, 5);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // if g:IsExists(s.filter,1,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if group::is_exists(d, &g, &filter, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
            // local sg=g:FilterSelect(tp,s.filter,1,1,nil)
            let sg = group::filter_select(d, &g, tp, &filter, 1, 1, false, NO_EX).nil();
            // Duel.SendtoHand(sg,nil,REASON_EFFECT)
            duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,sg)
            duel::confirm_cards(d, 1 - tp, &sg);
            // Duel.ShuffleHand(tp)
            duel::shuffle_hand(d, tp);
            // g:Sub(sg)
            group::remove_card(&mut g, &sg);
        }
        // Duel.SendtoGrave(g,REASON_EFFECT|REASON_EXCAVATE)
        duel::send_to_grave(d, g, REASON_EFFECT | REASON_EXCAVATE, None, None);
    }
}
