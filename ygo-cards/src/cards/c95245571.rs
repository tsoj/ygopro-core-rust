//! Scareclaw Twinsaw
use super::prelude::*;

pub const ID: u32 = 95245571;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_VISAS_STARFROST], listed_series: &[SET_SCARECLAW], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute 1 "Scareclaw" monster to destroy 2 targets
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_REMOVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Prevent the activation of effect of Link monsters
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_condition(d, e2, Some(condition(gycond)));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(gytg)));
    effect::set_operation(d, e2, Some(operation(gyop)));
    card::register_effect(d, c, e2, false);
}
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let _ = d;
    effect::set_label(d, a.e, &[1]);
    true
}
/// `s.spcheck(sg,tp,exg,dg)`: at least 2 targets are left after the Tributes
fn spcheck(d: &mut Duel, sg: &CardSet, dg: &CardSet) -> bool {
    let mut a: i64 = 0;
    for c in group::iter(d, sg) {
        if dg.contains(c) {
            a += 1;
        }
        for tc in group::iter(d, &card::get_equip_group(d, c)) {
            if dg.contains(tc) {
                a += 1;
            }
        }
    }
    dg.len() as i64 - a >= 2
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_set_card(d, c, &[SET_SCARECLAW as u16])
}
fn visas_on_field(d: &mut Duel, tp: u8) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_VISAS_STARFROST])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    if !card::is_can_be_effect_target(d, c, Some(e)) {
        return false;
    }
    let visas = visas_on_field(d, tp);
    !visas || card::is_able_to_remove(d, c, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp) && tgfilter(d, chkc, e, tp);
    }
    let dg = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, 0, LOCATION_ONFIELD, NO_EX);
    if a.chk == Some(0) {
        if effect::get_label(d, e) == 1 {
            effect::set_label(d, e, &[0]);
            let dg2 = dg.clone();
            return duel::check_release_group_cost(
                d,
                tp,
                Some(&cfilter),
                1,
                1,
                false,
                Some(&|d: &mut Duel, sg: &CardSet, _tp: u8, _exg: &CardSet| (spcheck(d, sg, &dg2), false)),
                NO_EX,
            );
        } else {
            return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, e, tp)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
        }
    }
    if effect::get_label(d, e) == 1 {
        effect::set_label(d, e, &[0]);
        let dg2 = dg.clone();
        let sg = duel::select_release_group_cost(
            d,
            tp,
            Some(&cfilter),
            1,
            1,
            false,
            Some(&|d: &mut Duel, sg: &CardSet, _tp: u8, _exg: &CardSet| (spcheck(d, sg, &dg2), false)),
            NO_EX,
        );
        duel::release(d, sg, REASON_COST, None);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, e, tp)), tp, 0, LOCATION_ONFIELD, 2, 2, false, NO_EX).nil();
    let n = g.len() as i32;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), n as u8, tp, 0);
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n as u8, tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tg = duel::get_target_cards(d, a.e).nil();
    if tg.is_empty() {
        return;
    }
    let loc = if visas_on_field(d, a.tp) { LOCATION_REMOVED } else { LOCATION_GRAVE };
    duel::destroy(d, tg, REASON_EFFECT, Some(loc), None);
}
fn gycond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_link_above(d, c, 3)),
        a.tp,
        LOCATION_MZONE,
        LOCATION_MZONE,
        1,
        NO_EX,
    )
}
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_flag_effect(d, 0, ID) == 0;
    }
    true
}
fn actlimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    let loc = effect::get_activate_location(d, re);
    let h = effect::get_handler(d, re).nil();
    loc == LOCATION_MZONE && effect::is_monster_effect(d, re) && card::is_type(d, h, TYPE_LINK)
}
fn gyop(d: &mut Duel, a: &EffArgs) {
    duel::register_flag_effect(d, 0, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
    effect::set_target_range(d, e1, 1, 1);
    effect::set_value_fn(d, e1, lua_fn(actlimit));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, a.tp);
}
