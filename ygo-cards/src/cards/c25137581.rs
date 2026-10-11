//! Sneaky "C"
use super::prelude::*;

pub const ID: u32 = 25137581;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand in face-down Defense Position
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_POSITION + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    effect::set_target(d, e1, Some(cost_target(sptg)));
    effect::set_operation(d, e1, Some(operation(spop)));
    card::register_effect(d, c, e1, false);
    // Destroy all Special Summoned monsters on the field
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_DESTROY);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e2, EVENT_FLIP);
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tp = 1 - a.tp;
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, tp), 1, NO_EX)
}
/// `function() return Duel.IsPhase(PHASE_END) end`
fn e2_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_phase(d, PHASE_END)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) > 0 {
        duel::confirm_cards(d, 1 - a.tp, c);
        // Change it to face-up Defense Position during the End Phase of this turn
        aux::delayed_operation(
            d,
            c,
            PHASE_END,
            ID,
            a.e,
            a.tp,
            Some(std::sync::Arc::new(change_pos)),
            None,
            None,
            Some(1),
            Some(aux::stringid(ID, 2)),
            None,
        );
    }
}
/// `function(cc) Duel.ChangePosition(cc,POS_FACEUP_DEFENSE) end`
fn change_pos(d: &mut Duel, ag: &CardSet, _a: &EffArgs) -> bool {
    duel::change_position(d, ag.clone(), POS_FACEUP_DEFENSE as u8, None, None, None, false);
    true
}
fn summon_special(d: &mut Duel, c: CardId) -> bool {
    card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL])
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let g = duel::get_matching_group(d, Some(&summon_special), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, a.tp, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&summon_special), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if !g.is_empty() {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
