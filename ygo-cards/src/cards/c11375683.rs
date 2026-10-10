//! Tindangle Trinity
use super::prelude::*;

pub const ID: u32 = 11375683;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[94365540, 59490397], listed_series: &[SET_TINDANGLE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_FLIP + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // flip
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_FLIP);
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_operation(d, e2, Some(operation(flipop)));
    card::register_effect(d, c, e2, false);
    // indes
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    effect::set_target(d, e3, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_TINDANGLE as u16]))));
    effect::set_condition(d, e3, Some(condition(indcon)));
    effect::set_value(d, e3, 1);
    card::register_effect(d, c, e3, false);
    // search
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_TOGRAVE);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e4, EVENT_BE_MATERIAL);
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    effect::set_condition(d, e4, Some(condition(thcon)));
    effect::set_target(d, e4, Some(cost_target(thtg)));
    effect::set_operation(d, e4, Some(operation(thop)));
    card::register_effect(d, c, e4, false);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[94365540]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let tc = duel::get_first_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, NO_EX);
    if let Some(tc) = tc {
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn flipop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
}
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) != 0
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_location(d, c, LOCATION_GRAVE) || a.r != REASON_LINK {
        return false;
    }
    let rc = card::get_reason_card(d, c).nil();
    card::is_set_card(d, rc, &[SET_TINDANGLE as u16])
}
fn thfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_code(d, c, &[59490397]) && card::is_able_to_hand(d, c, None) && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, c.into())
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell_trap(d, c) && card::is_able_to_grave(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    let hg = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let mut ok = false;
    if hg.len() > 0 && duel::send_to_hand(d, hg.clone(), Some(tp), REASON_EFFECT, None) > 0 {
        let first = group::get_first(d, &hg).nil();
        ok = card::is_location(d, first, LOCATION_HAND);
    }
    if ok {
        duel::confirm_cards(d, 1 - tp, hg);
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if g.len() > 0 {
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    }
}
