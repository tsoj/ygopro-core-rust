//! H.E.R.O. Flash!
use super::prelude::*;

pub const ID: u32 = 18482473;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ELEMENTAL_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
}
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None) || (mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let mmz_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    let max_count = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16])), a.tp, LOCATION_MZONE, 0, NO_EX);
    let b1 = !duel::has_flag_effect(d, a.tp, ID, None);
    let b2 = max_count > 0 && duel::is_existing_target(d, None, a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    let b3 = duel::is_existing_matching_card(d, Some(&|d, c| thspfilter(d, c, a.e, a.tp, mmz_chk)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    let b4 = !duel::has_flag_effect(d, a.tp, ID + 100, None);
    if a.chk == Some(0) {
        return b1 || b2 || b3 || b4;
    }
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3)), (b4, aux::stringid(ID, 4))]);
    effect::set_label(d, a.e, &[op.map_or(0, |x| x as i64)]);
    match op {
        Some(1) => {
            effect::set_category(d, a.e, 0);
            effect::set_property(d, a.e, 0, 0);
        }
        Some(2) => {
            effect::set_category(d, a.e, CATEGORY_DESTROY);
            effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
            let g = duel::select_target(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, max_count as u16, false, NO_EX).nil();
            let n = g.len() as u8;
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
        }
        Some(3) => {
            effect::set_category(d, a.e, CATEGORY_TOHAND | CATEGORY_SPECIAL_SUMMON);
            effect::set_property(d, a.e, 0, 0);
            duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, 0);
            duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
            duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
        }
        Some(4) => {
            effect::set_category(d, a.e, 0);
            effect::set_property(d, a.e, 0, 0);
        }
        _ => {}
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let op = effect::get_label(d, a.e);
    let c = effect::get_handler(d, a.e).nil();
    if op == 1 {
        // If your "Elemental HERO" monster attacks a Defense Position monster this turn, inflict piercing battle damage to your opponent
        duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 5)), 0, None);
        // Your "Elemental HERO" monsters inflict piercing battle damage
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_code(d, e1, EFFECT_PIERCE);
        effect::set_target_range(d, e1, LOCATION_MZONE, 0);
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]))));
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e1, a.tp);
    } else if op == 2 {
        // Target cards your opponent controls up to the number of "Elemental HERO" monsters you control; destroy them
        let tg = duel::get_target_cards(d, a.e).nil();
        if !tg.is_empty() {
            duel::destroy(d, tg, REASON_EFFECT, None, None);
        }
    } else if op == 3 {
        // Add to your hand, or Special Summon, 1 "Elemental HERO" monster from your GY
        let mmz_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 6));
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thspfilter(d, c, a.e, a.tp, mmz_chk)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
        let Some(sc) = group::get_first(d, &g) else { return };
        aux::to_hand_or_else(
            d,
            sc,
            a.tp,
            Some(&|d, _c| mmz_chk && card::is_can_be_special_summoned(d, sc, a.e, 0, a.tp, false, false, None, None, None)),
            Some(&|d, _g| {
                duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
                0
            }),
            Some(aux::stringid(ID, 7)),
        );
    } else if op == 4 {
        // "Elemental HERO" monsters can attack directly this turn
        duel::register_flag_effect(d, a.tp, ID + 100, RESET_PHASE | PHASE_END, 0, 1, 0);
        aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 8)), 0, None);
        // "Elemental HERO" monsters can attack directly this turn
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
        effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
        effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]))));
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        duel::register_effect(d, e2, a.tp);
    }
}
