//! Metamorphosis
use super::prelude::*;

pub const ID: u32 = 46411259;

/// `e:GetChainData().level_of_the_tribute`
const LEVEL_OF_THE_TRIBUTE: u8 = chain::field::CARD;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute 1 monster. Special Summon 1 Fusion Monster from your Extra Deck with the same Level as the Tributed monster.
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_label(d, e1, &[0]);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    effect::set_label(d, a.e, &[100]);
    true
}
fn costfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let lv = card::get_level(d, c);
    lv > 0 && duel::is_existing_matching_card(d, Some(&|d, sc| spfilter(d, sc, lv, e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn spfilter(d: &mut Duel, c: CardId, lv: u32, e: EffectId, tp: u8, mc: Option<CardId>) -> bool {
    card::is_fusion_monster(d, c)
        && card::is_level(d, c, &[lv])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        if effect::get_label(d, e) != 100 {
            return false;
        }
        effect::set_label(d, e, &[0]);
        // CheckReleaseGroupCost(tp,f,1,false,nil,nil,e,tp): the old form (minc = maxc = 1)
        return duel::check_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    }
    let rg = duel::select_release_group_cost(d, tp, Some(&|d, c| costfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    let first = group::get_first(d, &rg).nil();
    let lv = card::get_level(d, first);
    chain::set_effect_data(d, e, None, LEVEL_OF_THE_TRIBUTE, Value::Int(lv as i64));
    duel::release(d, rg, REASON_COST, None);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let lv = chain::effect_data(d, e, None, LEVEL_OF_THE_TRIBUTE).int() as u32;
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, lv, e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
