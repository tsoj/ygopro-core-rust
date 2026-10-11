//! Stake your Soul!
use super::prelude::*;

pub const ID: u32 = 54562327;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VANQUISH_SOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}

fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[-1]);
    true
}

fn cfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !(card::is_monster(d, c) && !card::is_public(d, c)) {
        return false;
    }
    let attr = card::get_attribute(d, c);
    let code = card::get_code(d, c)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, attr, code)), tp, LOCATION_DECK, 0, 1, NO_EX)
}

fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, attr: u32, code: u32) -> bool {
    card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16])
        && card::is_attribute(d, c, attr)
        && !card::is_code(d, c, &[code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}

fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let res = effect::get_label(d, a.e) == -1
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
        effect::set_label(d, a.e, &[0]);
        return res;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| cfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &g).nil();
    let attr = card::get_attribute(d, rc);
    let codes = card::get_code(d, rc);
    let mut labels = vec![attr as i64];
    labels.extend(codes.iter().map(|x| *x as i64));
    effect::set_label(d, a.e, &labels);
    duel::confirm_cards(d, 1 - a.tp, rc);
    duel::shuffle_hand(d, a.tp);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}

fn ret_to_hand(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::send_to_hand(d, ag.clone(), None, REASON_EFFECT, None);
    true
}

fn activate(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let labels = effect::get_labels(d, a.e);
    let attr = labels.first().copied().unwrap_or(0) as u32;
    let code = labels.get(1).copied().unwrap_or(0) as u32;
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp, attr, code)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX)
        .nil();
    let tc = group::get_first(d, &g);
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // Return it to the hand during the End Phase
            aux::delayed_operation(
                d,
                CardSet::from_card(tc),
                PHASE_END,
                ID,
                a.e,
                a.tp,
                Some(std::sync::Arc::new(ret_to_hand)),
                None,
                Some(0),
                Some(1),
                Some(aux::stringid(ID, 1)),
                None,
            );
        }
    }
    duel::special_summon_complete(d);
}
