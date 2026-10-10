//! Chain Destruction
use super::prelude::*;

pub const ID: u32 = 1248895;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e1);
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    card::register_effect(d, c, e3, false);
}
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !(card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_attack_below(d, c, 2000) && card::is_can_be_effect_target(d, c, Some(e))) {
        return false;
    }
    if card::is_controler(d, c, tp) {
        let codes = card::get_code(d, c);
        duel::is_existing_matching_card(d, Some(&|d, x| card::is_code(d, x, &codes)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
    } else {
        let ctrl = card::get_controler(d, c);
        duel::get_field_group_count(d, ctrl, LOCATION_HAND | LOCATION_DECK, 0) > 0 && mdnamecheck(d, c)
    }
}
fn mdnamecheck(d: &mut Duel, c: CardId) -> bool {
    let codes = card::get_code(d, c);
    for code in codes {
        let typ = duel::get_card_type_from_code(d, code).nil();
        if (typ & (TYPE_TOKEN | TYPE_EXTRA)) == 0 {
            return true;
        }
    }
    false
}
fn publicfilter(d: &mut Duel, c: CardId, convulsion: bool, top_card: Option<CardId>, codes: &[u32]) -> bool {
    card::is_code(d, c, codes) && (card::is_public(d, c) || (convulsion && Some(c) == top_card))
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tg = group::filter(d, &a.eg.clone().nil(), &|d, c| tgfilter(d, c, a.e, a.tp), NO_EX);
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&tg, chkc);
    }
    if a.chk == Some(0) {
        return tg.len() > 0;
    }
    let tc = if tg.len() == 1 {
        group::get_first(d, &tg).nil()
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
        let sel = group::filter_select(d, &tg, a.tp, &|_, _| true, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel).nil()
    };
    duel::set_target_card(d, tc);
    let ctrl_player = card::get_controler(d, tc);
    let decktop = duel::get_decktop_group(d, ctrl_player, 1);
    let top_card = group::get_first(d, &decktop);
    let convulsion = duel::is_player_affected_by_effect(d, ctrl_player, EFFECT_REVERSE_DECK).is_some();
    let codes = card::get_code(d, tc);
    let dg = duel::get_matching_group(d, Some(&|d, x| publicfilter(d, x, convulsion, top_card, &codes)), ctrl_player, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    if dg.len() > 0 {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg.clone()), dg.len() as u8, a.tp, 0);
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, ctrl_player, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc)) {
        return;
    }
    let ctrl = card::get_controler(d, tc);
    let codes = card::get_code(d, tc);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_code(d, x, &codes)), ctrl, LOCATION_HAND | LOCATION_DECK, 0, NO_EX);
    if g.len() > 0 {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
