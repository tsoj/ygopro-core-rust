//! Horoscope Sorcerer, the Stargazer Magician
use super::prelude::*;

pub const ID: u32 = 1186447;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGICIAN, SET_ODD_EYES, SET_PERFORMAPAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    pendulum::add_procedure(d, c, None, None);
    // When a Spell Card or effect activated by your opponent resolves, if you control a "Magician" Pendulum Monster or "Odd-Eyes" monster, you can negate that effect, then destroy this card
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_condition(d, e1, Some(condition(discon)));
    effect::set_operation(d, e1, Some(operation(disop)));
    card::register_effect(d, c, e1, false);
    // You can discard 1 other "Magician" Pendulum Monster, "Performapal" monster, or "Odd-Eyes" monster; Special Summon this card from your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::discard(Some(std::sync::Arc::new(|d, c, _e, _tp| spcostfilter(d, c))), true, None, None, None)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // If this card is Special Summoned: You can add 1 Pendulum Monster with 2500 ATK from your Deck to your hand
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_TOHAND | CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_target(d, e3, Some(cost_target(deckthtg)));
    effect::set_operation(d, e3, Some(operation(deckthop)));
    card::register_effect(d, c, e3, false);
    // You can target 1 Pendulum Monster Card you control; destroy it, then you can add 1 face-up Pendulum Monster from your Extra Deck to your hand
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_category(d, e4, CATEGORY_DESTROY | CATEGORY_TOHAND);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, ID, 2, 0);
    effect::set_target(d, e4, Some(cost_target(destg)));
    effect::set_operation(d, e4, Some(operation(desop)));
    card::register_effect(d, c, e4, false);
}
fn disconfilter(d: &mut Duel, c: CardId) -> bool {
    ((card::is_set_card(d, c, &[SET_MAGICIAN as u16]) && card::is_pendulum_monster(d, c)) || card::is_set_card(d, c, &[SET_ODD_EYES as u16]))
        && card::is_faceup(d, c)
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    a.rp == 1 - tp
        && effect::is_spell_effect(d, a.re.nil())
        && duel::is_chain_disablable(d, a.ev as u8)
        && !duel::has_flag_effect(d, tp, ID, None)
        && duel::is_existing_matching_card(d, Some(&disconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !duel::select_effect_yes_no(d, tp, c, Some(aux::stringid(ID, 3))) {
        return;
    }
    let _ = duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    if duel::negate_effect(d, a.ev as u8) {
        duel::break_effect(d);
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
/// `s.spcostfilter(c)`
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    ((card::is_set_card(d, c, &[SET_MAGICIAN as u16]) && card::is_pendulum_monster(d, c))
        || card::is_set_card(d, c, &[SET_ODD_EYES as u16, SET_PERFORMAPAL as u16]))
        && card::is_monster(d, c)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn deckthfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_pendulum_monster(d, c) && card::is_attack(d, c, &[2500]) && card::is_able_to_hand(d, c, None)
}
/// `s.deckthtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn deckthtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&deckthfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.deckthop(e,tp,eg,ep,ev,re,r,rp)`
fn deckthop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&deckthfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
    if let Some(g) = g {
        if !g.is_empty() {
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
fn pendulum_faceup_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_type(d, c, TYPE_PENDULUM) && card::is_faceup(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, tp) && card::is_type(d, chkc, TYPE_PENDULUM) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&pendulum_faceup_filter), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, Some(&pendulum_faceup_filter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn extrathfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_pendulum_monster(d, c) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e)
        && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
        && duel::is_existing_matching_card(d, Some(&extrathfilter), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 4))
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        let g = duel::select_matching_card(d, tp, Some(&extrathfilter), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX);
        if let Some(g) = g {
            if !g.is_empty() {
                duel::break_effect(d);
                duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
                duel::confirm_cards(d, 1 - tp, g);
            }
        }
    }
}
