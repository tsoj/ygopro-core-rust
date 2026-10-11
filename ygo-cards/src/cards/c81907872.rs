//! Ghostrick Specter
use super::prelude::*;

pub const ID: u32 = 81907872;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GHOSTRICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon limit
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    effect::set_condition(d, e1, Some(condition(sumcon)));
    card::register_effect(d, c, e1, false);
    // turn set
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_target(d, e2, Some(cost_target(postg)));
    effect::set_operation(d, e2, Some(operation(posop)));
    card::register_effect(d, c, e2, false);
    // spsummon
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_DRAW + CATEGORY_SET);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    effect::set_range(d, e3, LOCATION_HAND);
    effect::set_condition(d, e3, Some(condition(spcon)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
/// `s.sumcon(e)`
fn sumcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = effect::get_handler_player(d, a.e);
    !duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GHOSTRICK as u16])),
        tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_can_turn_set(d, c) && card::get_flag_effect(d, c, ID) == 0;
    }
    card::register_flag_effect(d, c, ID, RESET_EVENT | (RESETS_STANDARD_PHASE_END & !RESET_TURN_SET), 0, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(c)), 1, a.tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        duel::change_position(d, CardSet::from_card(c), POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_controler(d, c, tp)
        && card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_DESTROY)
        && card::get_reason_player(d, c) != tp
        && card::is_set_card(d, c, &[SET_GHOSTRICK as u16])
        && card::is_monster(d, c)
        && (card::is_reason(d, c, REASON_EFFECT)
            || (card::is_reason(d, c, REASON_BATTLE) && duel::get_attack_target(d) == Some(c)))
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, tp), 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
            && duel::is_player_can_draw(d, tp, 1);
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) != 0 {
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(c));
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
