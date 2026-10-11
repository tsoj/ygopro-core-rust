//! Xeno Meteorus
//! Scripted by Satella
use super::prelude::*;

pub const ID: u32 = 5852388;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_DESTROYED);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(selfspcon)));
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    effect::set_operation(d, e1, Some(operation(selfspop)));
    card::register_effect(d, c, e1, false);
    // Destroy 1 Dinosaur you control or in your hand, then Special Summon 1 Dinosaur Normal Monster from your hand or Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn selfspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsReason,1,nil,REASON_BATTLE|REASON_EFFECT)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT), 1, NO_EX)
}
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, CardSet::from_card(c), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_race(d, c, RACE_DINOSAUR)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_type(d, c, TYPE_NORMAL) && card::is_race(d, c, RACE_DINOSAUR) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c, a.tp)), a.tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, a.tp)), a.tp, LOCATION_MZONE | LOCATION_HAND, 0, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
fn lizard(d: &mut Duel, a: Args) -> bool {
    // function(_,c) return not c:IsOriginalRace(RACE_DRAGON|RACE_DINOSAUR|RACE_SEASERPENT|RACE_WYRM) end
    let c = a.c(1);
    !card::is_original_race(d, c, RACE_DRAGON | RACE_DINOSAUR | RACE_SEASERPENT | RACE_WYRM)
}
fn cannot_special_summon_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // function(_,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsRace(RACE_DRAGON|RACE_DINOSAUR|RACE_SEASERPENT|RACE_WYRM) end
    card::is_location(d, c, LOCATION_EXTRA) && !card::is_race(d, c, RACE_DRAGON | RACE_DINOSAUR | RACE_SEASERPENT | RACE_WYRM)
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // Cannot Special Summon from the Extra Deck, except Dragon, Dinosaur, Sea Serpent, and Wyrm monsters
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(cannot_special_summon_target)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    // Clock Lizard check
    aux::add_temp_lizard_check(d, c, a.tp, Some(lua_fn(lizard)), None, None, None, None);
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let dg = duel::select_matching_card(d, a.tp, Some(&|d, c| desfilter(d, c, a.tp)), a.tp, LOCATION_MZONE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !dg.is_empty() && duel::destroy(d, dg, REASON_EFFECT, None, None) > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sg = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if !sg.is_empty() {
            duel::break_effect(d);
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
