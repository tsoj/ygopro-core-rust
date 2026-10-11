//! Unchained Soul Rage Abominator
use super::prelude::*;

pub const ID: u32 = 21419436;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_UNCHAINED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, including a Fiend monster
    link::add_procedure(d, c, None, 2, Some(3), Some(group_check(matcheck)), None, None);
    // If your opponent Special Summons a monster(s) face-up: You can target 1 of them; destroy 1 Fiend monster you control, and if you do, negate the targeted monster's effects until the end of this turn
    let e1a = effect::create_effect(d, c);
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    effect::set_category(d, e1a, CATEGORY_DESTROY + CATEGORY_DISABLE);
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY | EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1a, EVENT_CUSTOM + ID);
    effect::set_range(d, e1a, LOCATION_MZONE);
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    effect::set_target(d, e1a, Some(cost_target(distg)));
    effect::set_operation(d, e1a, Some(operation(disop)));
    effect::set_label_object(d, e1a, Value::Group(CardSet::new()));
    card::register_effect(d, c, e1a, false);
    // Keep track of monsters the opponent Special Summoned face-up
    let e1b = effect::create_effect(d, c);
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e1b, LOCATION_MZONE);
    effect::set_label_object(d, e1b, Value::Effect(e1a));
    effect::set_operation(d, e1b, Some(operation(regop)));
    card::register_effect(d, c, e1b, false);
    // When a monster effect is activated on the field and you control a Link-4 or higher "Unchained" Link Monster (Quick Effect): You can banish this card from your GY; destroy the monster that activated that effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_CHAINING);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(descon)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
fn matcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    group::is_exists(d, g, &|d, c| card::is_race_ctx(d, c, RACE_FIEND, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
fn disfilter(d: &mut Duel, c: CardId, e: EffectId, opp: u8) -> bool {
    card::is_summon_player(d, c, opp) && card::is_negatable_monster(d, c) && card::is_can_be_effect_target(d, c, Some(e)) && card::is_location(d, c, LOCATION_MZONE)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    if duel::is_damage_step(d, None) {
        return;
    }
    let (e, tp) = (a.e, a.tp);
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| disfilter(d, c, e, 1 - tp), NO_EX);
    if !tg.is_empty() {
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        let e1a = effect::get_label_object(d, e).as_effect().nil();
        let mut g = effect::get_label_object(d, e1a).as_group().cloned().nil();
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        group::add_card(&mut g, &tg);
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        effect::set_label_object(d, e1a, Value::Group(g));
        let h = effect::get_handler(d, e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(e), 0, tp, tp, 0);
    }
}
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let opp = 1 - tp;
    let stored = effect::get_label_object(d, e).as_group().cloned().nil();
    let g = group::filter(d, &stored, &|d, c| disfilter(d, c, e, opp), NO_EX);
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && disfilter(d, chkc, e, opp);
    }
    let dg = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_FIEND)), tp, LOCATION_MZONE, 0, NO_EX);
    if a.chk == Some(0) {
        return !dg.is_empty() && !g.is_empty();
    }
    let tc = if g.len() > 1 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel).nil()
    } else {
        group::get_first(d, &g).nil()
    };
    duel::set_target_card(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, tp, 0);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(tc)), 1, tp, 0);
    true
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_FIEND)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    duel::hint_selection(d, g.clone(), true);
    if duel::destroy(d, g, REASON_EFFECT, None, None) == 0 {
        return;
    }
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) {
        // Negate the targeted monster's effects until the end of this turn
        let h = effect::get_handler(d, e).nil();
        card::negate_effects(d, tc, h, Some(RESET_PHASE | PHASE_END), false, None);
    }
}
fn desconfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_link_above(d, c, 4) && card::is_set_card(d, c, &[SET_UNCHAINED as u16]) && card::is_faceup(d, c)
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    if duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int() != LOCATION_MZONE as i64 {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    card::is_relate_to_effect(d, rc, re) && duel::is_existing_matching_card(d, Some(&desconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let rc = effect::get_handler(d, a.re.nil()).nil();
    card::create_effect_relation(d, rc, a.e);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(rc)), 1, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rc, re) {
        duel::destroy(d, rc, REASON_EFFECT, None, None);
    }
}
