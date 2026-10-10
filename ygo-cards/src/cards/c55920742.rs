//! Noh-P.U.N.K. Foxy Tune
use super::prelude::*;

pub const ID: u32 = 55920742;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PUNK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gain LP
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_RECOVER);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_condition(d, e1, Some(aux::bdocon()));
    effect::set_target(d, e1, Some(cost_target(lptg)));
    effect::set_operation(d, e1, Some(operation(lpop)));
    card::register_effect(d, c, e1, false);
    // Special Summon self
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    // Send to GY and Special Summon from Deck
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_HANDES + CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_HAND + LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_cost(d, e3, Some(cost::self_to_grave()));
    effect::set_target(d, e3, Some(cost_target(tgtg)));
    effect::set_operation(d, e3, Some(operation(tgop)));
    card::register_effect(d, c, e3, false);
}
fn lptg(d: &mut Duel, a: &EffArgs) -> bool {
    let handler = effect::get_handler(d, a.e).nil();
    let bt = card::get_battle_target(d, handler).nil();
    let dam = card::get_base_attack(d, bt);
    if a.chk == Some(0) {
        return dam > 0;
    }
    duel::set_target_player(d, a.tp);
    duel::set_target_param(d, dam as u32);
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, a.tp, dam);
    true
}
fn lpop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let amt = info[1].int();
    duel::recover(d, p, amt, REASON_EFFECT, false, None);
}
fn setpunk(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_PUNK as u16])
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, tp, Some(&setpunk), 1, 1, false, Some(&aux::release_check_mmz), NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    let g = duel::select_release_group_cost(d, tp, Some(&setpunk), 1, 1, false, Some(&aux::release_check_mmz), NO_EX);
    duel::release(d, g, REASON_COST, None);
    true
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, LOCATION_HAND as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
fn tgspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_PUNK as u16]) && !card::is_level(d, c, &[8]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if a.chk == Some(0) {
        return duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_grave(d, x, None)), tp, LOCATION_HAND, 0, 1, c.into())
            && duel::is_existing_matching_card(d, Some(&|d, x| tgspfilter(d, x, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() && duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None) > 0 {
        let first = group::get_first(d, &g).nil();
        if card::is_location(d, first, LOCATION_GRAVE)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| tgspfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX)
        {
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            let sg = duel::select_matching_card(d, tp, Some(&|d, c| tgspfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            if !sg.is_empty() {
                duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
