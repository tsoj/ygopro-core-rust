//! Reversal Box
use super::prelude::*;

pub const ID: u32 = 2606417;

const COUNTER_REVERSAL_BOX: u16 = 0x21b;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_TIME_WIZARD], roll_dice: true, toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_counter_permit(d, c, COUNTER_REVERSAL_BOX, None, None);
    card::set_counter_limit(d, c, COUNTER_REVERSAL_BOX, 6);
    // Activate
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // Once per turn, during the Standby Phase: Roll a six-sided die and place counters on this card equal to the result (max. 6)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DICE + CATEGORY_COUNTER);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_STANDBY);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(countertg)));
    effect::set_operation(d, e1, Some(operation(counterop)));
    card::register_effect(d, c, e1, false);
    // When a monster effect is activated on your opponent's field, or once per battle, during damage calculation, if an opponent's monster attacks: You can remove 1 counter from this card, then target 1 of those monsters; toss a coin and call it. If you call it right, you can Special Summon 1 monster that mentions "Dark Time Wizard" from your Deck, and if you do, until the end of this turn, change the targeted monster's ATK to 0, also its effects are negated
    let e2a = effect::create_effect(d, c);
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    effect::set_category(d, e2a, CATEGORY_COIN + CATEGORY_SPECIAL_SUMMON + CATEGORY_ATKCHANGE + CATEGORY_DISABLE);
    effect::set_type(d, e2a, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2a, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2a, EVENT_CHAINING);
    effect::set_range(d, e2a, LOCATION_SZONE);
    effect::set_condition(d, e2a, Some(condition(coincon)));
    effect::set_cost(d, e2a, Some(cost::remove_counter_from_self(COUNTER_REVERSAL_BOX, 1)));
    effect::set_target(d, e2a, Some(cost_target(cointg)));
    effect::set_operation(d, e2a, Some(operation(coinop)));
    card::register_effect(d, c, e2a, false);
    let e2b = effect::clone(d, e2a);
    effect::set_code(d, e2b, EVENT_PRE_DAMAGE_CALCULATE);
    effect::set_condition(d, e2b, Some(lua_fn(e2b_cond)));
    effect::set_cost(
        d,
        e2b,
        Some(cost::and(vec![cost::remove_counter_from_self(COUNTER_REVERSAL_BOX, 1), cost::soft_once_per_battle(ID, None)])),
    );
    card::register_effect(d, c, e2b, false);
}
fn e2b_cond(d: &mut Duel, a: Args) -> bool {
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.p(1))
}
fn countertg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_DICE, None, 0, a.tp, 1);
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, a.tp, COUNTER_REVERSAL_BOX as i32);
    true
}
fn counterop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        let die_roll = duel::toss_dice(d, a.tp, 1, 0)[0];
        let mut ct = 1;
        if die_roll != 1 {
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
            ct = duel::announce_number_range(d, a.tp, Some(1), Some(die_roll as u32), &[]).nil().0 as i32;
        }
        card::add_counter(d, c, COUNTER_REVERSAL_BOX, ct as u16, true);
    }
}
fn coincon(d: &mut Duel, a: &EffArgs) -> bool {
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_CONTROLER]);
    let trig_loc = info[0].int();
    let trig_ctrl = info[1].int();
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    effect::is_monster_effect(d, re)
        && trig_ctrl == (1 - a.tp) as i64
        && trig_loc == LOCATION_MZONE as i64
        && card::is_relate_to_effect(d, h, re)
}
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    let opp_card = if effect::get_code(d, a.e) == EVENT_CHAINING {
        group::get_first(d, &a.eg.clone().nil())
    } else {
        None
    };
    let opp_card = opp_card.or_else(|| duel::get_attacker(d)).nil();
    if a.chk == Some(0) {
        return card::is_can_be_effect_target(d, opp_card, Some(a.e));
    }
    duel::set_target_card(d, opp_card);
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(opp_card)), 1, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(opp_card)), 1, a.tp, 0);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::lists_code(d, c, &[CARD_DARK_TIME_WIZARD]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn coinop(d: &mut Duel, a: &EffArgs) {
    if duel::call_coin(d, a.tp)
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3))
    {
        let tc = duel::first_target(d);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !g.is_empty() && duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 && card::is_relate_to_effect(d, tc.nil(), a.e) && card::is_faceup(d, tc.nil()) {
            let c = effect::get_handler(d, a.e).nil();
            let tc = tc.nil();
            // Until the end of this turn, change the targeted monster's ATK to 0
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e1, EFFECT_SET_ATTACK);
            effect::set_value(d, e1, 0);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e1, false);
            // Also its effects are negated
            card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), false, None);
        }
    }
}
