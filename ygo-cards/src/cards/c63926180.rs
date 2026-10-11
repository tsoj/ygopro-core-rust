//! Sacred Spirit Sword Aiwass
use super::prelude::*;

pub const ID: u32 = 63926180;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[84288367], listed_series: &[SET_ALEISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects; you cannot declare attacks for the rest of this turn, except with Fusion Monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(efftg)));
    effect::set_operation(d, e1, Some(operation(effop)));
    card::register_effect(d, c, e1, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[84288367]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn tgbanfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ALEISTER as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_grave(d, c, None) || card::is_able_to_remove(d, c, None, None, None))
}
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // Special Summon 1 "Aiwass, Divine Spirit of the Law" from your Deck or GY
    let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    // Send to the GY, or banish, 1 "Aleister" monster from your Deck
    let b2 = duel::is_existing_matching_card(d, Some(&tgbanfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // Look at 3 random face-down cards in your opponent's Extra Deck, and if you do, banish 1 of them
    let b3 = duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_facedown(d, c) && card::is_able_to_remove(d, c, None, None, None)),
        tp,
        0,
        LOCATION_EXTRA,
        3,
        NO_EX,
    );
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    let op = duel::select_effect(
        d,
        tp,
        &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))],
    )
    .unwrap_or(0) as i64;
    effect::set_label(d, e, &[op]);
    if op == 1 {
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    } else if op == 2 {
        effect::set_category(d, e, CATEGORY_TOGRAVE + CATEGORY_REMOVE);
        duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
        duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_DECK as i32);
    } else if op == 3 {
        effect::set_category(d, e, CATEGORY_REMOVE);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, LOCATION_EXTRA as i32);
    }
    true
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    let op = effect::get_label(d, e);
    if op == 1 {
        // Special Summon 1 "Aiwass, Divine Spirit of the Law" from your Deck or GY
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let g = duel::select_matching_card(
                d,
                tp,
                Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))),
                tp,
                LOCATION_DECK | LOCATION_GRAVE,
                0,
                1,
                1,
                false,
                NO_EX,
            )
            .nil();
            if !g.is_empty() {
                duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    } else if op == 2 {
        // Send to the GY, or banish, 1 "Aleister" monster from your Deck
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 4));
        let g = duel::select_matching_card(d, tp, Some(&tgbanfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if let Some(sc) = group::get_first(d, &g) {
            let b_grave = card::is_able_to_grave(d, sc, None);
            let b_remove = card::is_able_to_remove(d, sc, None, None, None);
            let op2 = duel::select_effect(d, tp, &[(b_grave, aux::stringid(ID, 5)), (b_remove, aux::stringid(ID, 6))]).unwrap_or(0);
            if op2 == 1 {
                duel::send_to_grave(d, sc, REASON_EFFECT, None, None);
            } else if op2 == 2 {
                duel::remove(d, sc, POS_FACEUP as u8, REASON_EFFECT, None, None);
            }
        }
    } else if op == 3 {
        // Look at 3 random face-down cards in your opponent's Extra Deck, and if you do, banish 1 of them
        let field = duel::get_field_group(d, tp, 0, LOCATION_EXTRA);
        let g = group::random_select(d, &field, tp, 3);
        if !g.is_empty() {
            duel::confirm_cards(d, tp, &g);
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
            let rg = group::filter_select(d, &g, tp, &|d, c| card::is_able_to_remove(d, c, None, None, None), 1, 1, false, NO_EX).nil();
            if !rg.is_empty() {
                duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
            }
            duel::shuffle_extra(d, 1 - tp);
        }
    }
    let c = effect::get_handler(d, e).nil();
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 7)), 0, None);
    // You cannot declare attacks for the rest of this turn, except with Fusion Monsters
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_fusion_monster(d, c))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
}
