//! Mother Spider Splitter
use super::prelude::*;

pub const ID: u32 = 34034150;

pub const INFO: ScriptInfo = ScriptInfo { listed_cards: &[60023855], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_condition(d, e1, Some(lua_fn(selfspcon)));
    card::register_effect(d, c, e1, false);
    // Special Summon up to 3 "Baby Spider"
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::self_tribute()));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
// function s.selfspcon(e,c)
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)==0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[60023855]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        if duel::get_m_zone_count(d, a.tp, h.into(), None, None, None).0 <= 0 {
            return false;
        }
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
// function s.xyzlimit(e,c)
fn xyzlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // if not c then return false end
    // return not c:IsAttribute(ATTRIBUTE_DARK)
    !card::is_attribute(d, c, ATTRIBUTE_DARK)
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // Can only Special Summon Xyz monster from the Extra Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_type(d, c, TYPE_XYZ) && card::is_location(d, c, LOCATION_EXTRA))));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    // Clock Lizard check
    aux::add_temp_lizard_check(d, c, a.tp, Some(effect_card_fn(|d, _e, c| !card::is_original_type(d, c, TYPE_XYZ))), None, None, None, None);
    // Special Summon "Baby Spider" from hand or Deck
    let mut ft = std::cmp::min(3, duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0);
    if ft <= 0 {
        return;
    }
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, ft as u16, false, NO_EX).nil();
    if g.is_empty() {
        return;
    }
    for tc in group::iter(d, &g) {
        if duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // Level becomes 5
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
            effect::set_value(d, e1, 5);
            effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
            // Can only be used as Xyz material for a DARK monster
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
            effect::set_code(d, e2, EFFECT_CANNOT_BE_XYZ_MATERIAL);
            effect::set_value_fn(d, e2, effect_card_fn(xyzlimit));
            effect::set_reset(d, e2, RESET_EVENT + RESETS_STANDARD, 1);
            card::register_effect(d, tc, e2, false);
        }
    }
    duel::special_summon_complete(d);
}
