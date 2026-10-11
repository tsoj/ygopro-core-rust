//! Reincarnation of the Seventh Emperors
//! Scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 57499304;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
fn confilter(d: &mut Duel, c: CardId) -> bool {
    if !card::is_type(d, c, TYPE_XYZ) {
        return false;
    }
    // local no=c.xyz_number
    let no = crate::card_info(d, c).xyz_number;
    (card::is_set_card(d, c, &[SET_NUMBER as u16]) && no.is_some_and(|n| (101..=107).contains(&n)))
        || {
            let g = card::get_overlay_group(d, c);
            group::is_exists(d, &g, &confilter, 1, NO_EX)
        }
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let (bc, _) = duel::get_battle_monster(d, a.tp);
    match bc {
        Some(bc) => confilter(d, bc),
        None => false,
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let bc = duel::get_battle_monster(d, a.tp).0.nil();
    let g = card::get_overlay_group(d, bc);
    if a.chk == Some(0) {
        let removable = card::is_able_to_remove(d, bc, None, None, None);
        return removable && (g.is_empty() || g.len() == group::filter_count(d, &g, &|d, c| card::is_able_to_remove(d, c, None, None, None), NO_EX));
    }
    effect::set_label_object(d, a.e, Value::Card(bc));
    card::create_effect_relation(d, bc, a.e);
    let mut all = CardSet::from_card(bc);
    group::add_card(&mut all, &g);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(all), g.len() as u8, a.tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let bc = effect::get_label_object(d, a.e).as_card().nil();
    if card::is_relate_to_effect(d, bc, a.e) && card::is_controler(d, bc, tp) {
        let g = card::get_overlay_group(d, bc);
        let mut all = CardSet::from_card(bc);
        group::add_card(&mut all, &g);
        duel::remove(d, &all, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
    if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // During the End Phase of the turn you activated this card, Special Summon 1 Rank 3 or lower Xyz Monser from your Extra Deck, and if you do, inflict damage to your opponent equal to its original ATK
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_operation(d, e1, Some(operation(spop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_rank_below(d, c, 3) && card::is_type(d, c, TYPE_XYZ) && !card::is_set_card(d, c, &[SET_NUMBER as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if let Some(sc) = group::get_first(d, &g) {
        if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            let atk = card::get_base_attack(d, sc);
            duel::damage(d, 1 - tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
