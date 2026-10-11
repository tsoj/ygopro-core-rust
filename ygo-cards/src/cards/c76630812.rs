//! Graceful & Skull Dice
use super::prelude::*;

pub const ID: u32 = 76630812;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_TIME_WIZARD], roll_dice: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Roll a six-sided die twice and apply these effects for the rest of this turn
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DICE | CATEGORY_ATKCHANGE | CATEGORY_DEFCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    card::register_effect(d, c, e1, false);
    // The group that keeps track of the summoned monsters is the label object of e2a
    let g = CardSet::new();
    // If your opponent Normal or Special Summons a monster(s) while this card is in your GY: You can banish this card, then target 1 of those monsters; roll a six-sided die twice, and if the total is 6 or more, destroy that monster
    let e2a = effect::create_effect(d, c);
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    effect::set_category(d, e2a, CATEGORY_DICE | CATEGORY_DESTROY);
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY | EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2a, EVENT_CUSTOM + ID);
    effect::set_range(d, e2a, LOCATION_GRAVE);
    effect::set_cost(d, e2a, Some(cost::self_banish()));
    effect::set_target(d, e2a, Some(cost_target(destg)));
    effect::set_operation(d, e2a, Some(operation(desop)));
    effect::set_label_object(d, e2a, Value::Group(g));
    card::register_effect(d, c, e2a, false);
    // Keep track of monsters the opponent Normal or Special Summons
    let e2b = effect::create_effect(d, c);
    effect::set_type(d, e2b, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e2b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e2b, EVENT_SUMMON_SUCCESS);
    effect::set_range(d, e2b, LOCATION_GRAVE);
    effect::set_label_object(d, e2b, Value::Effect(e2a));
    effect::set_operation(d, e2b, Some(operation(regop)));
    card::register_effect(d, c, e2b, false);
    let e2c = effect::clone(d, e2b);
    effect::set_code(d, e2c, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2c, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 2);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let dice = duel::toss_dice(d, a.tp, 2, 0);
    let total_atkdef = (dice[0] + dice[1]) as i64 * 200;
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // Monsters you control that mention "Dark Time Wizard" gain ATK/DEF equal to the total x 200
    let e1a = effect::create_effect(d, c);
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1a, EFFECT_UPDATE_ATTACK);
    effect::set_target_range(d, e1a, LOCATION_MZONE, 0);
    effect::set_target(d, e1a, Some(effect_card_fn(|d, _e, c| card::lists_code(d, c, &[CARD_DARK_TIME_WIZARD]))));
    effect::set_value(d, e1a, total_atkdef);
    effect::set_reset(d, e1a, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1a, a.tp);
    let e1b = effect::clone(d, e1a);
    effect::set_code(d, e1b, EFFECT_UPDATE_DEFENSE);
    duel::register_effect(d, e1b, a.tp);
    aux::register_client_hint(d, c, 0, a.tp, 0, 1, Some(aux::stringid(ID, 3)), 0, None);
    // Monsters your opponent controls lose ATK/DEF equal to the total x 200
    let e2a = effect::clone(d, e1a);
    effect::set_target_range(d, e2a, 0, LOCATION_MZONE);
    effect::set_target(d, e2a, Some(effect_card_fn(|_d, _e, _c| true)));
    effect::set_value(d, e2a, -total_atkdef);
    duel::register_effect(d, e2a, a.tp);
    let e2b = effect::clone(d, e2a);
    effect::set_code(d, e2b, EFFECT_UPDATE_DEFENSE);
    duel::register_effect(d, e2b, a.tp);
}
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, opp: u8) -> bool {
    card::is_summon_player(d, c, opp) && card::is_can_be_effect_target(d, c, Some(e)) && card::is_location(d, c, LOCATION_MZONE)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    if duel::is_damage_step(d, None) {
        return;
    }
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| desfilter(d, c, a.e, 1 - a.tp), NO_EX);
    if !tg.is_empty() {
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        let e2a = effect::get_label_object(d, a.e).as_effect().nil();
        let mut g = effect::get_label_object(d, e2a).as_group().cloned().nil();
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        group::add_card(&mut g, &tg);
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        effect::set_label_object(d, e2a, Value::Group(g));
        let h = effect::get_handler(d, a.e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
    }
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let opp = 1 - a.tp;
    let stored = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &stored, &|d, c| desfilter(d, c, a.e, opp), NO_EX);
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && desfilter(d, chkc, a.e, opp);
    }
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    let tc = if g.len() > 1 {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel)
    } else {
        group::get_first(d, &g)
    };
    if let Some(tc) = tc {
        duel::set_target_card(d, tc);
    }
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 2);
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, tc.map(CardSet::from_card), 1, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        let dice = duel::toss_dice(d, a.tp, 2, 0);
        if dice[0] + dice[1] >= 6 {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
