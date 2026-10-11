//! Dragunity Knight - Gormfaobhar
use super::prelude::*;

pub const ID: u32 = 36556781;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGUNITY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro summon procedure
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, _sc, _st, _tp| card::is_set_card(d, c, &[SET_DRAGUNITY as u16]))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Must be properly summoned before reviving
    card::enable_revive_limit(d, c);
    // Equip 1 "Dragunity" tuner from GY to this card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_LEAVE_GRAVE + CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(eqcon)));
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equip_op), e1, None, None, None);
    // Banish up to 2 cards from opponent's GY
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_REMOVE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(rmcost)));
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    effect::set_operation(d, e2, Some(operation(rmop)));
    card::register_effect(d, c, e2, false);
}
fn equip_op(d: &mut Duel, a: Args) -> bool {
    card::equip_by_effect_and_limit_register(d, a.c(0), a.e(1), a.p(2), a.c(3), None, None)
}
fn eqval(d: &mut Duel, a: Args) -> bool {
    let (ec, tp) = (a.c(0), a.p(2));
    card::is_controler(d, ec, tp) && card::is_set_card(d, ec, &[SET_DRAGUNITY as u16])
}
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_DRAGUNITY as u16]) && card::is_monster(d, c) && card::is_type(d, c, TYPE_TUNER) && !card::is_forbidden(d, c)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let g = duel::select_target(d, tp, Some(&filter), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g.clone(), 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, 0, 0);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc, a.e) {
            card::equip_by_effect_and_limit_register(d, c, a.e, tp, tc, None, None);
        }
    }
}
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        let eg = card::get_equip_group(d, h);
        let g = group::filter(d, &eg, &|d, c| card::is_controler(d, c, tp), NO_EX);
        return group::is_exists(d, &g, &|d, c| card::is_able_to_grave_as_cost(d, c), 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let eg = card::get_equip_group(d, h);
    let g = group::filter(d, &eg, &|d, c| card::is_controler(d, c, tp), NO_EX);
    let sel = group::filter_select(d, &g, tp, &|d, c| card::is_able_to_grave_as_cost(d, c), 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, sel, REASON_COST, None, None);
    true
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, false)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - tp) && rmfilter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, tp, Some(&rmfilter), tp, 0, LOCATION_MZONE | LOCATION_GRAVE, 1, 2, false, NO_EX).nil();
    let n = group::get_count(&g) as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_target_cards(d, a.e).nil();
    if !g.is_empty() {
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
