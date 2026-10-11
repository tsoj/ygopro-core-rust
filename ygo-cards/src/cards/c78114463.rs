//! Solemn Accusation
use super::prelude::*;

pub const ID: u32 = 78114463;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can activate this card the turn it was Set, by revealing 1 other Set Trap you control
    let e0 = effect::create_effect(d, c);
    effect::set_description(d, e0, aux::stringid(ID, 0));
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_SET_AVAILABLE, 0);
    effect::set_code(d, e0, EFFECT_TRAP_ACT_IN_SET_TURN);
    effect::set_value_fn(d, e0, lua_fn(|d, a: Args| {
        effect::set_label(d, a.e(0), &[1]);
        Value::Nil
    }));
    effect::set_condition(d, e0, Some(condition(|d, a| {
        let p = effect::get_handler_player(d, a.e);
        let c = effect::get_handler(d, a.e).nil();
        duel::is_existing_matching_card(d, Some(&|d, cc| card::is_facedown(d, cc) && card::is_trap(d, cc)), p, LOCATION_SZONE, 0, 1, c.into())
    })));
    card::register_effect(d, c, e0, false);
    // When a Spell/Trap Card is activated: Activate 1 of these effects;
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(|d, a| effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) && duel::is_chain_negatable(d, a.ev as u8))));
    effect::set_cost(d, e1, Some(cost::and(vec![cost_target(revealcost), paylpcost()])));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_label_object(d, e1, Value::Effect(e0));
    card::register_effect(d, c, e1, false);
}
/// `s.paylpcost`
fn paylpcost() -> LuaFn {
    cost::choice(vec![
        cost::CostChoice {
            cost: cost::pay_lp(cost::PayLp::Amount(1500)),
            desc: aux::stringid(ID, 2),
            check: Some(aux_true()),
        },
        cost::CostChoice {
            cost: cost::pay_lp(cost::PayLp::Amount(3000)),
            desc: aux::stringid(ID, 3),
            check: Some(lua_fn(|d, a: Args| banish_activation_chk(d, a.e(0), a.p(1)))),
        },
    ])
}
/// `s.revealcost`
fn revealcost(d: &mut Duel, a: &EffArgs) -> bool {
    let label_obj = effect::get_label_object(d, a.e).as_effect().nil();
    if a.chk == Some(0) {
        effect::set_label(d, label_obj, &[0]);
        return true;
    }
    if effect::get_label(d, label_obj) > 0 {
        effect::set_label(d, label_obj, &[0]);
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
        let c = effect::get_handler(d, a.e).nil();
        let g = duel::select_matching_card(d, a.tp, Some(&|d, cc| card::is_facedown(d, cc) && card::is_trap(d, cc)), a.tp, LOCATION_SZONE, 0, 1, 1, false, c.into()).nil();
        duel::confirm_cards(d, 1 - a.tp, g);
    }
    true
}
/// `Chain.GetTriggeringEffect()`
fn triggering_effect(d: &Duel) -> Option<EffectId> {
    duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect()
}
/// `s.banish_activation_chk(e, tp)`
fn banish_activation_chk(d: &mut Duel, _e: EffectId, tp: u8) -> bool {
    let re = triggering_effect(d).nil();
    let rc = effect::get_handler(d, re).nil();
    card::is_able_to_remove(d, rc, Some(tp), None, None) || (!card::is_relate_to_effect(d, rc, re) && duel::is_player_can_remove(d, tp, None))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    let relation = card::is_relate_to_effect(d, rc, re);
    let cost_choice = chain::effect_data(d, a.e, None, chain::field::COST_CHOICE).int();
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    if cost_choice == 1 {
        effect::set_category(d, a.e, CATEGORY_NEGATE + CATEGORY_DESTROY);
        if card::is_destructable(d, rc, None) && relation {
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, a.tp, 0);
        }
    } else if cost_choice == 2 {
        effect::set_category(d, a.e, CATEGORY_NEGATE + CATEGORY_REMOVE);
        if relation {
            duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, a.tp, 0);
        } else {
            let prev = card::get_previous_location(d, rc) as i32;
            duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, prev);
        }
    }
    true
}
/// `s.banfilter(c, code, opp)`
fn banfilter(d: &mut Duel, c: CardId, code: u32, opp: u8) -> bool {
    card::is_original_code_rule(d, c, &[code]) && card::is_able_to_remove(d, c, Some(opp), None, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    let code = card::get_original_code_rule(d, rc).first().copied().nil();
    let cost_choice = chain::effect_data(d, a.e, None, chain::field::COST_CHOICE).int();
    if cost_choice == 1 {
        // Pay 1500 LP; negate the activation, and if you do, destroy that card, and if you do that, for the rest of this turn, neither player can activate cards, or the effects of cards, with its same original name
        if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, rc, re) && duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None) > 0 {
            // For the rest of this turn, neither player can activate cards, or the effects of cards, with its same original name
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            effect::set_description(d, e1, aux::stringid(ID, 4));
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
            effect::set_target_range(d, e1, 1, 1);
            effect::set_value_fn(d, e1, lua_fn(move |d, args: Args| {
                let rc2 = effect::get_handler(d, args.e(1)).nil();
                card::is_original_code_rule(d, rc2, &[code])
            }));
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            duel::register_effect(d, e1, a.tp);
        }
    } else if cost_choice == 2 {
        // Pay 3000 LP; negate the activation, and if you do, banish that card, then your opponent banishes all cards with its same original name from their hand and Deck
        if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, rc, re) && duel::remove(d, a.eg.clone().nil(), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
            let opp = 1 - a.tp;
            let g = duel::get_matching_group(d, Some(&|d, cc| banfilter(d, cc, code, opp)), a.tp, 0, LOCATION_HAND | LOCATION_DECK, NO_EX);
            if !g.is_empty() {
                duel::break_effect(d);
                duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, Some(opp));
            }
        }
    }
}
