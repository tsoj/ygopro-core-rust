//! The Eye of Timaeus
use super::prelude::*;

pub const ID: u32 = 1784686;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DARK_MAGICIAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card is also always treated as "Legendary Dragon Timaeus"
    let e0 = effect::create_effect(d, c);
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e0, EFFECT_ADD_CODE);
    effect::set_value(d, e0, 10000050);
    card::register_effect(d, c, e0, false);
    // Fusion Summon 1 Fusion Monster from your Extra Deck that mentions 1 "Dark Magician" monster you control as material, using it as the entire Fusion Material
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_DARK_MAGICIAN as u16])
        && card::is_can_be_fusion_material(d, c, None, None, None)
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, sc| spfilter(d, sc, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    if duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    let mustg = aux::get_must_be_material_group_opt(d, tp, None, tp, Some(c), None, REASON_FUSION);
    let mc_codes = card::get_code(d, mc);
    card::is_type(d, c, TYPE_FUSION)
        && card::lists_code_as_material(d, c, &mc_codes)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_FUSION, tp, false, false, None, None, None)
        && (mustg.is_empty() || (mustg.len() == 1 && group::is_contains(&mustg, mc)))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // `if chkc==0` compares a card or nil with 0 and is never true
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FMATERIAL as u64);
    duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, e) && card::is_can_be_fusion_material(d, tc, None, None, None) && !card::is_immune_to_effect(d, tc, e) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, tc)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        if let Some(sc) = group::get_first(d, &g) {
            card::set_material(d, sc, CardSet::from_card(tc));
            duel::send_to_grave(d, tc, REASON_EFFECT | REASON_MATERIAL | REASON_FUSION, None, None);
            duel::break_effect(d);
            if duel::special_summon(d, sc, SUMMON_TYPE_FUSION, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                card::complete_procedure(d, sc);
            }
        }
    }
}
