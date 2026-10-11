//! Icejade Manifestation
use super::prelude::*;

pub const ID: u32 = 72776252;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ICEJADE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon, or send to the GY, 1 "Icejade" monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE | CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(tgspcon)));
    effect::set_target(d, e1, Some(cost_target(tgsptg)));
    effect::set_operation(d, e1, Some(operation(tgspop)));
    card::register_effect(d, c, e1, false);
    // Banish 1 card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY | EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(condition(rmcon)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    effect::set_operation(d, e2, Some(operation(rmop)));
    card::register_effect(d, c, e2, false);
}
fn tgspcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&card::is_monster), a.tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER)),
            a.tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
}
fn tgspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    card::is_set_card(d, c, &[SET_ICEJADE as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_grave(d, c, None)
            || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
fn tgsptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        return duel::is_existing_matching_card(d, Some(&|d, c| tgspfilter(d, c, a.e, a.tp, ft)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn tgspop(d: &mut Duel, a: &EffArgs) {
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| tgspfilter(d, c, a.e, a.tp, ft)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = match group::get_first(d, &g) {
        Some(tc) => tc,
        None => return,
    };
    let b1 = card::is_able_to_grave(d, tc, None);
    let b2 = ft > 0 && card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None);
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
    if op == Some(1) {
        duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
    } else if op == Some(2) {
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn rmconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_controler(d, c, tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_set_card(d, c, &[SET_ICEJADE as u16])
        && card::get_reason_player(d, c) == 1 - tp
        && !card::is_reason(d, c, REASON_DESTROY)
}
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| rmconfilter(d, c, a.tp), 1, NO_EX)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_on_field(d, chkc) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, 0, 0);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
