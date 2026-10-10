//! Shining Sarcophagus
use super::prelude::*;

pub const ID: u32 = 79791878;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed by monster effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_value_fn(d, e2, lua_fn(|d, a: Args| effect::is_monster_effect(d, a.e(1))));
    card::register_effect(d, c, e2, false);
    // Search 1 card that mentions "Shining Sarcophagus"
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_category(d, e3, CATEGORY_TOHAND | CATEGORY_SEARCH);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_target(d, e3, Some(cost_target(thtg)));
    effect::set_operation(d, e3, Some(operation(thop)));
    card::register_effect(d, c, e3, false);
    // Send 1 monster back to the Graveyard
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_TOGRAVE);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET | EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e4, EVENT_CUSTOM + ID);
    effect::set_range(d, e4, LOCATION_SZONE);
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    effect::set_condition(d, e4, Some(condition(|d, _a| duel::get_current_phase(d) != PHASE_DAMAGE)));
    effect::set_cost(d, e4, Some(cost_target(tgcost)));
    effect::set_target(d, e4, Some(cost_target(tgtg)));
    effect::set_operation(d, e4, Some(operation(tgop)));
    card::register_effect(d, c, e4, false);
    let g = CardSet::new();
    effect::set_label_object(d, e4, Value::Group(g));
    // Mass register
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_type(d, e5, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e5, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e5, LOCATION_SZONE);
    effect::set_label_object(d, e5, Value::Effect(e4));
    effect::set_operation(d, e5, Some(operation(regop)));
    card::register_effect(d, c, e5, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::lists_code(d, c, &[ID]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, p: u8) -> bool {
    card::is_summon_player(d, c, p)
        && card::is_summon_location(d, c, LOCATION_GRAVE)
        && card::is_able_to_grave(d, c, None)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_can_be_effect_target(d, c, Some(e))
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, x| tgfilter(d, x, e, 1 - tp), NO_EX);
    if !tg.is_empty() {
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        let e4 = effect::get_label_object(d, e).as_effect().nil();
        let mut g = effect::get_label_object(d, e4).as_group().cloned().nil();
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        group::add_card(&mut g, &tg);
        group::remove(d, &mut g, &|d, x| card::get_flag_effect(d, x, ID) == 0, NO_EX);
        effect::set_label_object(d, e4, Value::Group(g));
        let h = effect::get_handler(d, e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(e), 0, tp, tp, 0);
    }
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell(d, c) && card::is_discardable(d, c, None, None)
}
fn tgcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, a.tp, Some(&cfilter), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let lo = effect::get_label_object(d, e).as_group().cloned().nil();
    let g = group::filter(d, &lo, &|d, x| tgfilter(d, x, e, 1 - tp), NO_EX);
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && tgfilter(d, chkc, e, 1 - tp);
    }
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let mut tg = g.clone();
    if g.len() > 1 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        tg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    }
    duel::set_target_card(d, tg.clone());
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(tg), 1, 0, 0);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    }
}
