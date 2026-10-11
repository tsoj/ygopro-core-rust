//! Silent Sea Nettle
use super::prelude::*;

pub const ID: u32 = 31059809;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon from hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Shuffle up 3 WATER monsters from GY into the Deck
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    card::register_effect(d, c, e2, false);
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
// function s.counterfilter(c) return c:IsAttribute(ATTRIBUTE_WATER) end
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    card::is_attribute(d, a.c(0), ATTRIBUTE_WATER)
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT + EFFECT_FLAG_OATH, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
    true
}
// function s.splimit(e,c) return not c:IsAttribute(ATTRIBUTE_WATER) end
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    !card::is_attribute(d, c, ATTRIBUTE_WATER)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_HAND as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.tdfilter(c) return c:IsAttribute(ATTRIBUTE_WATER) and c:IsAbleToDeck() end
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_WATER) && card::is_able_to_deck(d, c)
}
// function s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && chkc != c && tdfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TODECK as u64);
    let g = duel::select_target(d, a.tp, Some(&tdfilter), a.tp, LOCATION_GRAVE, 0, 1, 3, false, c.into()).nil();
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
// function s.tdop(e,tp,eg,ep,ev,re,r,rp)
fn tdop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_target_cards(d, a.e).nil();
    if !g.is_empty() {
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
