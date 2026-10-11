//! Gimmick Puppet Rouge Doll
use super::prelude::*;

pub const ID: u32 = 63825486;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GIMMICK_PUPPET], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon both 1 "Gimmick Puppet" monster from your Deck and this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Add this card to your hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(thcond)));
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counter_filter));
}

fn counter_filter(d: &mut Duel, a: Args) -> bool {
    let c = a.c(0);
    !card::is_summon_location(d, c, LOCATION_EXTRA) || card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16])
}

fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !(card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16]) && card::is_type(d, c, TYPE_XYZ) && !card::is_public(d, c)) {
        return false;
    }
    let rank = card::get_rank(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, rank)), tp, LOCATION_DECK, 0, 1, NO_EX)
}

fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rank: u32) -> bool {
    card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16])
        && card::is_level(d, c, &[rank])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}

/// "Cannot Special Summon from the Extra Deck, except "Gimmick Puppet" monsters" (the same
/// effect both costs register).
fn cannot_sp_extra(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_OATH | EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(
        d,
        e1,
        Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_set_card(d, c, &[SET_GIMMICK_PUPPET as u16]))),
    );
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}

fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spcostfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spcostfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let rc = group::get_first(d, &g).nil();
    let rank = card::get_rank(d, rc) as i64;
    effect::set_label(d, a.e, &[rank]);
    duel::confirm_cards(d, 1 - a.tp, rc);
    duel::shuffle_extra(d, a.tp);
    cannot_sp_extra(d, a);
    true
}

fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 2, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}

fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2
    {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)) {
        return;
    }
    let rank = effect::get_label(d, a.e) as u32;
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, rank)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        let mut sg = g.clone();
        group::add_card(&mut sg, &CardSet::from_card(c));
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}

fn thcond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    !card::is_previous_location(d, h, LOCATION_HAND)
}

fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    cannot_sp_extra(d, a);
    true
}

fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}

fn thop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
