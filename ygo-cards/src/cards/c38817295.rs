//! Ultimate Sky
use super::prelude::*;

pub const ID: u32 = 38817295;

/// `e:GetChainData().choice`
const CHOICE: u8 = chain::field::CARD;
/// `e:GetChainData().event_value`
const EVENT_VALUE: u8 = chain::field::CARD + 1;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If your opponent controls more monsters than you do: Activate 1 of these effects (but you can only use each effect of "Ultimate Sky" once per turn);
    // Target face-up cards on the field up to the number of face-up monsters your opponent controls and pay 800 LP for each target; negate their effects until the end of this turn
    // When your opponent activates a monster effect in the hand or GY: Negate that effect
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DISABLE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let opp = duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE);
    let own = duel::get_field_group_count(d, a.tp, LOCATION_MZONE, 0);
    opp > own
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (duel::check_lp_cost(d, tp, 800 * sg.len() as u32), false)
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if let Some(chkc) = a.chkc() {
        return chain::effect_data(d, e, None, CHOICE).int() == 1 && card::is_negatable(d, chkc) && card::is_on_field(d, chkc) && chkc != c;
    }
    // Target face-up cards on the field up to the number of face-up monsters your opponent controls and pay 800 LP for each target; negate their effects until the end of this turn
    let negate_group = duel::get_target_group(d, Some(&card::is_negatable), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    let opp_faceup_monster_count = duel::get_matching_group_count(d, Some(&|d, x| card::is_faceup(d, x)), tp, 0, LOCATION_MZONE, NO_EX);
    let option_1 = !duel::has_flag_effect(d, tp, ID, None)
        && !negate_group.is_empty()
        && opp_faceup_monster_count > 0
        && duel::check_lp_cost(d, tp, 800);
    // When your opponent activates a monster effect in the hand or GY: Negate that effect
    let evt = duel::check_event(d, EVENT_CHAINING);
    let option_2 = match &evt {
        Some(ev) => {
            !duel::has_flag_effect(d, tp, ID + 1, None)
                && ev.event_player == 1 - tp
                && effect::is_monster_effect(d, ev.reason_effect.nil())
                && chain::is_triggering(d, ev.event_value as u8, chain::Prop::Location, &[(LOCATION_HAND | LOCATION_GRAVE) as i64]).truthy()
                && duel::is_chain_disablable(d, ev.event_value as u8)
        }
        None => false,
    };
    if a.chk == Some(0) {
        return option_1 || option_2;
    }
    let choice = duel::select_effect(d, tp, &[(option_1, aux::stringid(ID, 1)), (option_2, aux::stringid(ID, 2))]);
    chain::set_effect_data(d, e, None, CHOICE, match choice {
        Some(x) => Value::Int(x as i64),
        None => Value::Nil,
    });
    if choice == Some(1) {
        // Target face-up cards on the field up to the number of face-up monsters your opponent controls and pay 800 LP for each target; negate their effects until the end of this turn
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        effect::set_property(d, e, EFFECT_FLAG_CARD_TARGET, 0);
        let lp = duel::get_lp(d, tp).max(0) as usize;
        let max_target_count = negate_group.len().min(opp_faceup_monster_count).min(lp / 800);
        let tg = aux::select_unselect_group(
            d,
            &negate_group,
            Some(e),
            tp,
            Some(1),
            Some(max_target_count),
            Some(&rescon),
            1,
            tp,
            Some(HINTMSG_NEGATE as u64),
            Some(&rescon),
            None,
            false,
        )
        .unwrap_or_default();
        duel::set_target_card(d, tg.clone());
        duel::pay_lp_cost(d, tp, 800 * tg.len() as u32);
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(tg.clone()), tg.len() as u8, tp, 0);
    } else if choice == Some(2) {
        // When your opponent activates a monster effect in the hand or GY: Negate that effect
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        effect::set_property(d, e, 0, 0);
        let ev_value = evt.as_ref().map_or(0, |x| x.event_value as i64);
        chain::set_effect_data(d, e, None, EVENT_VALUE, Value::Int(ev_value));
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, tp, 0);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let choice = chain::effect_data(d, a.e, None, CHOICE).int();
    if choice == 1 {
        // Target face-up cards on the field up to the number of face-up monsters your opponent controls and pay 800 LP for each target; negate their effects until the end of this turn
        let tg = duel::get_target_cards(d, a.e).nil();
        if tg.is_empty() {
            return;
        }
        let c = effect::get_handler(d, a.e).nil();
        for tc in group::iter(d, &tg) {
            // Negate their effects until the end of this turn
            card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), true, None);
        }
    } else if choice == 2 {
        // When your opponent activates a monster effect in the hand or GY: Negate that effect
        let ev = chain::effect_data(d, a.e, None, EVENT_VALUE).int();
        duel::negate_effect(d, ev as u8);
    }
}
