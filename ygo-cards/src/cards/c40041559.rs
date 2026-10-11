//! Genia of the Ring
use super::prelude::*;

pub const ID: u32 = 40041559;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If this card is in your hand: You can target 1 face-up monster on the field; Special Summon this card, also that target becomes a Spellcaster monster. The next time that target would be destroyed by card effect this turn, it is not destroyed, also you cannot Special Summon from the Extra Deck for the rest of this turn, except Spellcaster monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // If this card is Tributed, or banished, to activate a Spellcaster monster's effect: You can add this card to your hand
    let e2a = effect::create_effect(d, c);
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    effect::set_category(d, e2a, CATEGORY_TOHAND);
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2a, EVENT_RELEASE);
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    effect::set_condition(d, e2a, Some(condition(thcon)));
    effect::set_target(d, e2a, Some(cost_target(thtg)));
    effect::set_operation(d, e2a, Some(operation(thop)));
    card::register_effect(d, c, e2a, false);
    let e2b = effect::clone(d, e2a);
    effect::set_code(d, e2b, EVENT_REMOVE);
    card::register_effect(d, c, e2b, false);
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            && duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
    let _g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        if !card::is_race_except(d, tc, RACE_SPELLCASTER, None, 0, None) {
            // That target becomes a Spellcaster monster
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e1, EFFECT_CHANGE_RACE);
            effect::set_value(d, e1, RACE_SPELLCASTER as i64);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
        }
        // The next time that target would be destroyed by card effect this turn, it is not destroyed
        let e2 = effect::create_effect(d, c);
        effect::set_description(d, e2, aux::stringid(ID, 2));
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_COUNT);
        effect::set_count_limit(d, e2, 1, 0, 0, 0);
        effect::set_value_fn(d, e2, lua_fn(indestructable_value));
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
    }
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except Spellcaster monsters
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 3));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e3, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e3, 1, 0);
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_race_except(d, c, RACE_SPELLCASTER, None, 0, None))));
    effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e3, a.tp);
}
fn indestructable_value(d: &mut Duel, a: Args) -> Value {
    let e = a.e(0);
    if (a.int(2) as u32) & REASON_EFFECT > 0 {
        effect::reset(d, e);
        return Value::Bool(true);
    }
    Value::Nil
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if !(card::is_reason(d, h, REASON_COST) && a.re.is_some()) {
        return false;
    }
    let re = a.re.nil();
    if !(effect::is_activated(d, re) && effect::is_monster_effect(d, re)) {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rc, re) && card::is_faceup(d, rc) {
        card::is_race(d, rc, RACE_SPELLCASTER)
    } else {
        (duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_RACE])[0].int() as u64 & RACE_SPELLCASTER) > 0
    }
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
