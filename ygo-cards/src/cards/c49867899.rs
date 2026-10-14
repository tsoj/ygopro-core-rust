//! Fiendsmith's Sequence
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 49867899;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    link::add_procedure(d, c, None, 2, Some(2), Some(group_check(|d, g, lc, st, tp| lcheck(d, g, lc, st, tp))), None, None);
    // Fusion Summon 1 Fiend Fusion Monster by shuffling its materials from your GY into the Deck
    let params = fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_FIEND))),
        matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _chk| false)),
        // s.fextra: Duel.GetMatchingGroup(aux.NecroValleyFilter(Fusion.IsMonsterFilter(Card.IsAbleToDeck)),tp,LOCATION_GRAVE,0,nil)
        extrafil: Some(Arc::new(|d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet| {
            let f = aux::necro_valley_filter(|d, c| card::is_monster(d, c) && card::is_able_to_deck(d, c));
            (Some(duel::get_matching_group(d, Some(&f), tp, LOCATION_GRAVE, 0, NO_EX)), None)
        })),
        extraop: Some(fusion::shuffle_material()),
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    };
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    let tg = fusion::summon_eff_tg(d, params.clone());
    effect::set_target(d, e1, Some(tg));
    let op = fusion::summon_eff_op(d, params);
    effect::set_operation(d, e1, Some(op));
    card::register_effect(d, c, e1, false);
    // Equip this card to 1 non-Link LIGHT Fiend monster you control
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_EQUIP);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE + LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    effect::set_operation(d, e2, Some(operation(eqop)));
    card::register_effect(d, c, e2, false);
}
fn matfilter(d: &mut Duel, c: CardId, lc: CardId, sumtype: u64, tp: u8) -> bool {
    card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, Some(lc), sumtype, Some(tp)) && card::is_race_ctx(d, c, RACE_FIEND, Some(lc), sumtype, Some(tp))
}
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    group::is_exists(d, g, &|d, c| matfilter(d, c, lc, sumtype, tp), 1, NO_EX)
}
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    !card::is_link_monster(d, c) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_FIEND) && card::is_faceup(d, c)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && eqfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 && duel::is_existing_target(d, Some(&eqfilter), tp, LOCATION_MZONE, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let _ = duel::select_target(d, tp, Some(&eqfilter), tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(c)), 1, tp, 0);
    if card::is_location(d, c, LOCATION_GRAVE) {
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, tp, 0);
    }
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let tc = duel::first_target(d);
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        && card::is_faceup(d, tc.nil())
        && card::is_relate_to_effect(d, tc.nil(), a.e)
        && card::is_controler(d, tc.nil(), tp)
        && duel::equip(d, tp, c, tc.nil(), true, false)
    {
        let tc = tc.nil();
        // Equip limit
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        effect::set_value_fn(d, e1, effect_card_fn(move |_d, _e, x| x == tc));
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
        // Your opponent cannot target the equipped monster with card effects
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
        effect::set_value_fn(d, e2, aux::tgoval());
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e2, false);
    } else if card::is_location(d, c, LOCATION_MZONE) {
        duel::send_to_grave(d, c, REASON_RULE, None, Some(PLAYER_NONE as u8));
    }
}
