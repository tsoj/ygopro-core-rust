//! Magician's Circle
use super::prelude::*;

pub const ID: u32 = 50755;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetAttacker():IsRace(RACE_SPELLCASTER)
    let atk = duel::get_attacker(d).nil();
    card::is_race(d, atk, RACE_SPELLCASTER)
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // c:IsAttackBelow(2000) and c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_attack_below(d, c, 2000)
        && card::is_race(d, c, RACE_SPELLCASTER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if let Some(tc) = group::get_first(d, &g) {
            duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP_ATTACK as u8, None);
        }
    }
    // if Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 then
    if duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_SPSUMMON as u64);
        let g = duel::select_matching_card(d, 1 - tp, Some(&|d, c| filter(d, c, e, 1 - tp)), 1 - tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        if let Some(tc) = group::get_first(d, &g) {
            duel::special_summon_step(d, tc, 0, 1 - tp, 1 - tp, false, false, POS_FACEUP_ATTACK as u8, None);
        }
    }
    duel::special_summon_complete(d);
}
