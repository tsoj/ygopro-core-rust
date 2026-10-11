//! SPYRAL Double Agent
use super::prelude::*;

pub const ID: u32 = 88124568;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[41091257], listed_series: &[SET_SPYRAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself to the opponent's field
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcond)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // "SPYRAL" monsters gain 500 ATK
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(atk_target)));
    effect::set_value(d, e2, 500);
    card::register_effect(d, c, e2, false);
    // "SPYRAL Super Agent" can attack directly
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_DIRECT_ATTACK);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e3, Some(effect_card_fn(direct_target)));
    card::register_effect(d, c, e3, false);
}
fn atk_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SPYRAL as u16])
}
fn direct_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_code(d, c, &[41091257])
}
fn spcond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_SPYRAL as u16]))),
        a.tp,
        LOCATION_ONFIELD | LOCATION_GRAVE,
        0,
        1,
        NO_EX,
    )
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let sdg = duel::get_decktop_group(d, a.tp, 1);
    let odg = duel::get_decktop_group(d, 1 - a.tp, 1);
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(a.tp), None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), Some(1 - a.tp), None)
            && sdg.len() > 0
            && odg.len() > 0
            && !card::is_public(d, group::get_first(d, &sdg).nil())
            && !card::is_public(d, group::get_first(d, &odg).nil());
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e)
        && duel::special_summon(d, c, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP_DEFENSE as u8, None) > 0
    {
        // Cannot be Tributed
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
        effect::set_value(d, e1, 1);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e1, false);
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_UNRELEASABLE_NONSUM);
        card::register_effect(d, c, e2, false);
        // Cannot be used as material for a Fusion, Synchro, Xyz, or Link Summon
        let e3 = effect::create_effect(d, c);
        effect::set_description(d, e3, aux::stringid(ID, 1));
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e3, EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e3, EFFECT_CANNOT_BE_MATERIAL);
        effect::set_value_fn(d, e3, aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK]));
        effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, c, e3, false);
        if duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) == 0 || duel::get_field_group_count(d, 1 - a.tp, LOCATION_DECK, 0) == 0 {
            return;
        }
        duel::break_effect(d);
        duel::confirm_decktop(d, a.tp, 1);
        duel::confirm_decktop(d, 1 - a.tp, 1);
    }
}
