//! Trishula, Dragon of the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 52687916;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 2, 99, None, None, None, None, None);
    card::enable_revive_limit(d, c);
    // remove
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(remcon)));
    effect::set_target(d, e1, Some(cost_target(remtg)));
    effect::set_operation(d, e1, Some(operation(remop)));
    card::register_effect(d, c, e1, false);
}
fn remcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, c)
}
fn remtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        if duel::is_player_affected_by_effect(d, 1 - tp, CARD_SPIRIT_ELIMINATION).is_some() {
            return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_HAND | LOCATION_ONFIELD, 1, NO_EX);
        } else {
            return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE, 1, NO_EX);
        }
    }
    if duel::is_player_affected_by_effect(d, 1 - tp, CARD_SPIRIT_ELIMINATION).is_some() {
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 0, (LOCATION_HAND | LOCATION_ONFIELD) as i32);
    } else {
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 0, (LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    }
    true
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and aux.SpElimFilter(c)
    card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, false)
}
fn remop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g1 = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_ONFIELD, NO_EX);
    let g2 = duel::get_matching_group(d, Some(&|d, c| rmfilter(d, c)), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, NO_EX);
    let g3 = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, 0, LOCATION_HAND, NO_EX);
    let mut sg = CardSet::new();
    if !g1.is_empty() && ((g2.is_empty() && g3.is_empty()) || duel::select_yes_no(d, tp, aux::stringid(ID, 1))) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        let sg1 = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
        duel::hint_selection(d, sg1.clone(), true);
        group::add_card(&mut sg, &sg1);
    }
    if !g2.is_empty() && ((sg.is_empty() && g3.is_empty()) || duel::select_yes_no(d, tp, aux::stringid(ID, 2))) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        let sg2 = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
        duel::hint_selection(d, sg2.clone(), true);
        group::add_card(&mut sg, &sg2);
    }
    if !g3.is_empty() && (sg.is_empty() || duel::select_yes_no(d, tp, aux::stringid(ID, 3))) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        let sg3 = group::random_select(d, &g3, tp, 1);
        group::add_card(&mut sg, &sg3);
    }
    duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
