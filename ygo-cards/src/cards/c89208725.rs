//! Metaverse
use super::prelude::*;

pub const ID: u32 = 89208725;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Take 1 Field Spell from your Deck, and either activate it or add it to your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `c:GetActivateEffect():IsActivatable(tp,true,true)` (the card has an activate effect)
fn activatable(d: &mut Duel, c: CardId, tp: u8) -> bool {
    let te = card::get_activate_effect(d, c).first().copied().nil();
    effect::is_activatable(d, te, tp, true, true)
}
/// `s.thactfilter(c,tp)`
fn thactfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFieldSpell() and (c:IsAbleToHand() or c:GetActivateEffect():IsActivatable(tp,true,true))
    card::is_field_spell(d, c) && (card::is_able_to_hand(d, c, None) || activatable(d, c, tp))
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thactfilter,tp,LOCATION_DECK,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thactfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // if not Duel.CheckPhaseActivity() then Duel.RegisterFlagEffect(tp,CARD_MAGICAL_MIDBREAKER,RESET_CHAIN,0,1) end
    if !duel::check_phase_activity(d) {
        duel::register_flag_effect(d, a.tp, CARD_MAGICAL_MIDBREAKER, RESET_CHAIN, 0, 1, 0);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 1));
    // local sc=Duel.SelectMatchingCard(tp,s.thactfilter,tp,LOCATION_DECK,0,1,1,nil,tp):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| thactfilter(d, c, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // aux.ToHandOrElse(sc,tp,function() return sc:GetActivateEffect():IsActivatable(tp,true,true) end,
    //   function() Duel.ActivateFieldSpell(sc,e,tp,eg,ep,ev,re,r,rp) end,aux.Stringid(id,2))
    // (`if card then` in ToHandOrElse: nothing happens without a card)
    if let Some(sc) = sc {
        aux::to_hand_or_else(
            d,
            sc,
            a.tp,
            Some(&|d, _c| activatable(d, sc, a.tp)),
            Some(&|d, _g| {
                duel::activate_field_spell(d, Some(sc), a, None);
                0
            }),
            Some(aux::stringid(ID, 2)),
        );
    }
}
