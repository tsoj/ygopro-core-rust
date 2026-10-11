//! Red Nova Dragon - Burning Soul
use super::prelude::*;

pub const ID: u32 = 65541655;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RED_DRAGON_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Synchro Summon procedue: 2 Tuners + 1 non-Tuner
    synchro::add_procedure(d, c, None, 2, 2, Some(synchro::non_tuner(None)), 1, 1, None, None, None, None, None);
    // Must be either Synchro Summoned, or Special Summoned (from your Extra Deck) by banishing 2 Tuners and 1 "Red Dragon Archfiend" from your GY
    card::add_must_be_synchro_summoned(d, c);
    let e0 = effect::create_effect(d, c);
    effect::set_description(d, e0, aux::stringid(ID, 0));
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e0, LOCATION_EXTRA);
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    effect::set_target(d, e0, Some(lua_fn(sptg)));
    effect::set_operation(d, e0, Some(operation(spop)));
    card::register_effect(d, c, e0, false);
    // If this card is Special Summoned: You can add 1 card from your GY to your hand, and if you do, this card gains 2000 ATK. You can only use this effect of "Red Nova Dragon - Burning Soul" once per Duel, and only during a Duel you Synchro Summoned "Red Dragon Archfiend"
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_ATKCHANGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_DUEL as u8);
    effect::set_condition(d, e1, Some(condition(|d, a| duel::has_flag_effect(d, a.tp, ID, None))));
    effect::set_target(d, e1, Some(cost_target(thtg)));
    effect::set_operation(d, e1, Some(operation(thop)));
    card::register_effect(d, c, e1, false);
    // Track Synchro Summons of "Red Dragon Archfiend"
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_SPSUMMON_SUCCESS);
        effect::set_operation(d, ge1, Some(operation(regop)));
        duel::register_effect(d, ge1, 0);
    });
    // Cannot be destroyed by card effects
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    // Multiple Tuners
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e3, EFFECT_MULTIPLE_TUNERS);
    card::register_effect(d, c, e3, false);
}
fn spcostfilter(d: &mut Duel, c: CardId) -> bool {
    (card::is_type(d, c, TYPE_TUNER) || card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND])) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let ok = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0
        && group::is_exists(d, sg, &|d, c| card::is_type(d, c, TYPE_TUNER), 2, NO_EX)
        && group::is_exists(d, sg, &|d, c| card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND]), 1, NO_EX);
    (ok, false)
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    let Some(_c) = a.card(1) else {
        return true;
    };
    let tp = effect::get_handler_player(d, a.e(0));
    let e = a.e(0);
    let g = duel::get_matching_group(d, Some(&spcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    g.len() >= 2
        && duel::get_m_zone_count(d, tp, (&g).into(), None, None, None).0 > 0
        && matches!(aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 0, tp, None, None, None, false), Err(true))
}
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let g = duel::get_matching_group(d, Some(&spcostfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(3), Some(3), Some(&rescon), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, true).unwrap_or_default();
    if !sg.is_empty() {
        effect::set_label_object(d, e, Value::Group(sg));
        return true;
    }
    false
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if let Some(sg) = effect::get_label_object(d, a.e).as_group().cloned() {
        duel::remove(d, sg, POS_FACEUP as u8, REASON_COST, None, None);
    }
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(h)), 1, a.tp, 2000);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let Some(sc) = group::get_first(d, &sel) else {
        return;
    };
    duel::hint_selection(d, sc, true);
    let c = effect::get_handler(d, a.e).nil();
    if duel::send_to_hand(d, sc, None, REASON_EFFECT, None) > 0 && card::is_location(d, sc, LOCATION_HAND) && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // This card gains 2000 ATK
        card::update_attack(d, c, 2000, None, None, None);
    }
}
fn regopfilter(d: &mut Duel, c: CardId, sp: u8) -> bool {
    card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND]) && card::is_summon_player(d, c, sp) && card::is_synchro_summoned(d, c) && card::is_faceup(d, c)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    if duel::has_flag_effect(d, 0, ID, None) && duel::has_flag_effect(d, 1, ID, None) {
        return;
    }
    let eg = a.eg.clone().nil();
    for sp in 0..=1u8 {
        if group::is_exists(d, &eg, &|d, c| regopfilter(d, c, sp), 1, NO_EX) {
            duel::register_flag_effect(d, sp, ID, 0, 0, 1, 0);
        }
    }
}
