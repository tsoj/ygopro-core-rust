//! Secret Six Samurai - Fuma
use super::prelude::*;

pub const ID: u32 = 71207871;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_SIX_SAMURAI], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Six Samurai" monster from your Deck
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_DESTROYED);
    effect::set_condition(d, e1, Some(condition(spcon)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Destruction replacement for exactly 1 "Six Samurai" monster
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_target(d, e2, Some(cost_target(reptg)));
    effect::set_value_fn(d, e2, effect_card_fn(repval));
    effect::set_operation(d, e2, Some(operation(repop)));
    card::register_effect(d, c, e2, false);
}
fn spcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_EFFECT+REASON_BATTLE)~=0
    (a.r & (REASON_EFFECT + REASON_BATTLE)) != 0
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16]) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if let Some(tg) = group::get_first(d, &g) {
        duel::special_summon(d, CardSet::from_card(tg), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_SIX_SAMURAI as u16])
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_controler(d, c, tp)
        && card::is_reason(d, c, REASON_EFFECT)
        && !card::is_reason(d, c, REASON_REPLACE)
}
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None)
            && group::is_exists(d, &a.eg.clone().nil(), &|d, x| repfilter(d, x, a.tp), 1, NO_EX)
            && a.eg.clone().nil().len() == 1;
    }
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    let tp = effect::get_handler_player(d, e);
    repfilter(d, c, tp)
}
fn repop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    duel::remove(d, CardSet::from_card(h), POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
}
