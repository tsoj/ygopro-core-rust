//! The Bystial Aluber
use super::prelude::*;

pub const ID: u32 = 45005708;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card's name becomes "Fallen of Albaz" on the field or in the GY
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    effect::set_value(d, e1, CARD_ALBAZ as i64);
    card::register_effect(d, c, e1, false);
    // Take control of or Special Summon 1 Dragon monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(cost)));
    effect::set_target(d, e2, Some(cost_target(target)));
    effect::set_operation(d, e2, Some(operation(operation_fn)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sc: CardId) -> bool {
    if !(card::is_race(d, c, RACE_DRAGON) && card::is_faceup(d, c)) {
        return false;
    }
    if card::is_location(d, c, LOCATION_MZONE) {
        ctrlfilter(d, c, tp, sc)
    } else {
        spfilter(d, c, e, tp, sc)
    }
}
fn ctrlfilter(d: &mut Duel, c: CardId, tp: u8, sc: CardId) -> bool {
    // return Duel.GetMZoneCount(tp,sc,tp,LOCATION_REASON_CONTROL)>0 and c:IsAbleToChangeControler()
    duel::get_m_zone_count(d, tp, sc.into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0 && card::is_able_to_change_controler(d, c)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sc: CardId) -> bool {
    // return Duel.GetMZoneCount(tp,sc)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    duel::get_m_zone_count(d, tp, sc.into(), None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local label=e:GetLabel()
    let label = effect::get_label(d, e);
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp)
            && card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE)
            && ((label == 1 && ctrlfilter(d, chkc, tp, c)) || (label == 2 && spfilter(d, chkc, e, tp, c)));
    }
    if a.chk == Some(0) {
        return card::is_able_to_grave(d, c, None)
            && duel::is_existing_target(d, Some(&|d, tc| tgfilter(d, tc, e, tp, c)), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, tp, Some(&|d, tc| tgfilter(d, tc, e, tp, c)), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    if card::is_location(d, tc, LOCATION_MZONE) {
        effect::set_label(d, e, &[1]);
        effect::set_category(d, e, CATEGORY_TOGRAVE | CATEGORY_CONTROL);
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(tc)), 1, 0, 0);
    } else {
        effect::set_label(d, e, &[2]);
        effect::set_category(d, e, CATEGORY_TOGRAVE | CATEGORY_SPECIAL_SUMMON);
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(tc)), 1, tp, 0);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::send_to_grave(d, c, REASON_EFFECT, None, None) > 0 && card::is_location(d, c, LOCATION_GRAVE) {
        let tc = duel::first_target(d).nil();
        if !card::is_relate_to_effect(d, tc, a.e) {
            return;
        }
        let label = effect::get_label(d, a.e);
        if label == 1 {
            duel::get_control(d, tc, a.tp, PHASE_END as u16, 1, None, None);
        } else if label == 2 && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
