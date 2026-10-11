//! Frightfur Jar
use super::prelude::*;

pub const ID: u32 = 18138630;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EDGE_IMP], listed_names: &[70245411], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Changes name to "Toy Vendor" while on the field/in the GY
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_CHANGE_CODE);
    effect::set_range(d, e2, LOCATION_SZONE | LOCATION_GRAVE);
    effect::set_value(d, e2, 70245411);
    card::register_effect(d, c, e2, false);
    // Draw 1 card. If "Edge Imp" monster, destroy 1 card, else return 1 card from hand to deck
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_DRAW + CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER));
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_cost(d, e3, Some(cost_target(descost)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
    // If sent to the GY, halve the ATK of opponent's monster
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_ATKCHANGE);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_target(d, e4, Some(cost_target(atktg)));
    effect::set_operation(d, e4, Some(operation(atkop)));
    card::register_effect(d, c, e4, false);
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, cc| card::is_discardable(d, cc, None, None)), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    duel::discard_hand(d, a.tp, Some(&|d, cc| card::is_discardable(d, cc, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local dg=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)
    let dg = duel::get_field_group(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // if Duel.Draw(tp,1,REASON_EFFECT)==0 then return end
    if duel::draw(d, tp, 1, REASON_EFFECT) == 0 {
        return;
    }
    // local tc=Duel.GetOperatedGroup():GetFirst()
    let og = duel::get_operated_group(d);
    let tc = group::get_first(d, &og).nil();
    // Duel.ConfirmCards(1-tp,tc)
    duel::confirm_cards(d, 1 - tp, tc);
    if card::is_set_card(d, tc, &[SET_EDGE_IMP as u16]) && card::is_monster(d, tc) {
        if duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            let tg = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
            duel::hint_selection(d, tg.clone(), true);
            duel::destroy(d, tg, REASON_EFFECT, None, None);
        }
    } else {
        duel::shuffle_hand(d, tp);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, cc| card::is_able_to_deck(d, cc)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        if duel::select_option(d, tp, true, &[aux::stringid(ID, 3), aux::stringid(ID, 4)]) == 0 {
            duel::send_to_deck(d, g, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
        } else {
            duel::send_to_deck(d, g, None, SEQ_DECKBOTTOM as i32, REASON_EFFECT, None);
        }
    }
    duel::shuffle_hand(d, tp);
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_faceup(d, chkc) && card::is_location(d, chkc, LOCATION_MZONE);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, cc| card::is_faceup(d, cc)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, a.tp, Some(&|d, cc| card::is_faceup(d, cc)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:GetAttack()/2 (Lua float division, rounded by the core)
        let atk = card::get_attack(d, tc);
        effect::set_value(d, e1, lua_int(atk as f64 / 2.0));
        card::register_effect(d, tc, e1, false);
    }
}
