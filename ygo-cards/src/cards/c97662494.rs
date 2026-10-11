//! Heraldic Beast Stad Whale
//! 紋章獣スタット・ホエール
use super::prelude::*;

pub const ID: u32 = 97662494;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_HERALDRY, SET_HERALDIC_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 2 "Heraldry" Spells/Traps from your Deck to your hand.
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost::discard(None, false, None, None, None)));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    // Special Summon in Defense Position 2 "Heraldic Beast" monsters with the same name from your GY
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e3, LOCATION_GRAVE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_cost(d, e3, Some(cost::self_banish()));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_HERALDRY as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 2, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 2, 2, false, NO_EX).nil();
    if g.len() == 2 {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - tp, g);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_HERALDIC_BEAST as u16])
        && card::is_can_be_effect_target(d, c, Some(e))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
fn spcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (group::get_class_count(d, sg, &|d, c| card::get_code(d, c)[0] as i64) == 1, false)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, c.into());
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&spcheck), 0, tp, None, None, None, false), Err(true));
    }
    let tg = aux::select_unselect_group(d, &g, Some(e), tp, Some(2), Some(2), Some(&spcheck), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    duel::set_target_card(d, tg.clone());
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), 2, tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tg = duel::get_target_cards(d, e).nil();
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    if !tg.is_empty() && ft > 0 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none() {
        let mut tg = tg;
        if ft == 1 {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            tg = group::filter_select(d, &tg, tp, &|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None), 1, 1, false, NO_EX).nil();
        }
        duel::special_summon(d, tg, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
    let c = effect::get_handler(d, e).nil();
    // Cannot Special Summon from the Extra Deck for the rest of this turn, except by Xyz Summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(lua_fn(|d, a: Args| {
        let c = a.c(1);
        let sumtype = a.int(3) as u64;
        card::is_location(d, c, LOCATION_EXTRA) && (sumtype & SUMMON_TYPE_XYZ as u64) != SUMMON_TYPE_XYZ as u64
    })));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    // Can only use monsters with "Heraldic Beast" and/or "Number" in their original names as material for an Xyz Summon this turn
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e2, EFFECT_CANNOT_BE_XYZ_MATERIAL);
    effect::set_target_range(d, e2, LOCATION_ALL, LOCATION_ALL);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| !card::is_original_set_card(d, c, &[SET_HERALDIC_BEAST as u16, SET_NUMBER as u16]))));
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, c| card::is_controler(d, c, effect::get_handler_player(d, e))));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, tp);
}
