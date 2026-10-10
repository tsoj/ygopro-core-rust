//! Raviel, Lord of Phantasms - Shimmering Scraper
use super::prelude::*;

pub const ID: u32 = 28651380;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[69890967], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Cannot be special summoned
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, aux_false());
    card::register_effect(d, c, e1, false);
    // Special summon procedure
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_range(d, e2, LOCATION_HAND);
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    card::register_effect(d, c, e2, false);
    // ATK change
    let e3 = effect::create_effect(d, c);
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_HAND);
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    effect::set_hint_timing(d, e3, TIMING_DAMAGE_STEP as u32, Some((TIMING_DAMAGE_STEP | TIMINGS_CHECK_MONSTER) as u32));
    effect::set_condition(d, e3, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    effect::set_cost(d, e3, Some(cost::self_discard()));
    effect::set_target(d, e3, Some(cost_target(adtg)));
    effect::set_operation(d, e3, Some(operation(adop)));
    card::register_effect(d, c, e3, false);
    // Add to hand
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_TOHAND);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e4, LOCATION_GRAVE);
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    effect::set_cost(d, e4, Some(cost_target(thcost)));
    effect::set_target(d, e4, Some(cost_target(thtg)));
    effect::set_operation(d, e4, Some(operation(thop)));
    card::register_effect(d, c, e4, false);
}
fn mzfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_controler(d, c, tp) && card::get_sequence(d, c) < 5
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let tp = card::get_controler(d, c);
    let rg = duel::get_release_group(d, tp, false, false, None);
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let ct = -ft + 1;
    ft > -3 && rg.len() > 2 && (ft > 0 || group::is_exists(d, &rg, &|d, x| mzfilter(d, x, tp), ct as u16, NO_EX))
}
fn spop(d: &mut Duel, a: Args) {
    let tp = a.p(1);
    let rg = duel::get_release_group(d, tp, false, false, None);
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let g = if ft > 0 {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
        group::select(d, &rg, tp, 3, 3, false, NO_EX).nil()
    } else if ft > -2 {
        let ct = -ft + 1;
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
        let mut g = group::filter_select(d, &rg, tp, &|d, x| mzfilter(d, x, tp), ct as u16, ct as u16, false, NO_EX).nil();
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
        let g2 = group::select(d, &rg, tp, (3 - ct) as u16, (3 - ct) as u16, false, (&g).into()).nil();
        group::add_card(&mut g, &g2);
        g
    } else {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
        group::filter_select(d, &rg, tp, &|d, x| mzfilter(d, x, tp), 3, 3, false, NO_EX).nil()
    };
    duel::release(d, g, REASON_COST, None);
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[69890967])
}
fn adtg(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn adop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, tc, a.e) {
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        let atk = card::get_attack(d, tc);
        effect::set_value(d, e1, atk as i64 * 2);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e1, false);
        let e2 = effect::create_effect(d, c);
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        effect::set_value(d, e2, 1);
        effect::set_code(d, e2, EFFECT_ATTACK_ALL);
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, tc, e2, false);
    }
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, None, 1, 1, false, None, NO_EX);
    }
    let sg = duel::select_release_group_cost(d, a.tp, None, 1, 1, false, None, NO_EX);
    duel::release(d, sg, REASON_COST, None);
    true
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, h, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, h, a.e) {
        duel::send_to_hand(d, h, None, REASON_EFFECT, None);
    }
}
