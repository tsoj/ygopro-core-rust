//! Altergeist Marionetter
use super::prelude::*;

pub const ID: u32 = 53143898;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ALTERGEIST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Set 1 "Altergeist" Trap from the Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(settg)));
    effect::set_operation(d, e1, Some(operation(setop)));
    card::register_effect(d, c, e1, false);
    // Send 1 face-up "Altergeist" card to the GY and Special Summon 1 "Altergeist" monster from the GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}

fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_ALTERGEIST as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}

fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}

fn setop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::s_set(d, a.tp, g, None, true);
    }
}

fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_ALTERGEIST as u16])
        && card::is_able_to_grave(d, c, None)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}

fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_ALTERGEIST as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}

fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chkc().is_some() {
        return false;
    }
    let _ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None);
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    let g1 = duel::select_target(d, a.tp, Some(&|d, c| tgfilter(d, c, a.tp)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g2 = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g1.clone()), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g2.clone()), 1, 0, 0);
    let label_obj = match group::get_first(d, &g1) {
        Some(first) => Value::Card(first),
        None => Value::Nil,
    };
    effect::set_label_object(d, a.e, label_obj);
    true
}

fn spop(d: &mut Duel, a: &EffArgs) {
    let targets = duel::get_first_target(d);
    let mut tc1 = targets.first().copied();
    let mut tc2 = targets.get(1).copied();
    let label_obj = effect::get_label_object(d, a.e).as_card();
    if tc1 != label_obj {
        std::mem::swap(&mut tc1, &mut tc2);
    }
    if card::is_relate_to_effect(d, tc1.nil(), a.e)
        && duel::send_to_grave(d, tc1.nil(), REASON_EFFECT, None, None) > 0
        && card::is_location(d, tc1.nil(), LOCATION_GRAVE)
        && card::is_relate_to_effect(d, tc2.nil(), a.e)
        && aux::nvfilter(d, tc2.nil())
    {
        duel::special_summon(d, tc2.nil(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
