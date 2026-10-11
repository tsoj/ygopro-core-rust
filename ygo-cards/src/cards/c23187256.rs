//! No.93 Utopia Kaiser (Number 93: Utopia Kaiser)
use super::prelude::*;

pub const ID: u32 = 23187256;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(93), listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,s.xyzfilter,nil,2,nil,nil,Xyz.InfiniteMats,nil,false,s.xyzcheck)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, x, sc, st, tp| xyzfilter(d, x, sc, st, tp))),
        None,
        2,
        None,
        None,
        Some(xyz::MaxCt::Infinite),
        None,
        false,
        Some(xyz::ex_chk(|d, g, tp, xyz| xyzcheck(d, g, tp, xyz))),
    );
    // Special Summon
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_condition(d, e2, Some(condition(indcon)));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    card::register_effect(d, c, e3, false);
}
fn xyzfilter(d: &mut Duel, c: CardId, xyz: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    card::is_type_ctx(d, c, TYPE_XYZ, xyz, sumtype, Some(tp))
        && card::is_set_card_ctx(d, c, &[SET_NUMBER as u16], xyz, sumtype, Some(tp))
        && card::get_overlay_count(d, c) > 0
}
fn xyzcheck(d: &mut Duel, g: &CardSet, _tp: u8, _xyz: CardId) -> bool {
    let mg = group::filter(d, g, &|d, c| card::is_has_effect(d, c, EFFECT_EQUIP_SPELL_XYZ_MAT, None).is_empty(), NO_EX);
    group::get_class_count(d, &mg, &|d, c| card::get_rank(d, c) as i64) == 1
}
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rp: u8) -> bool {
    card::is_rank_below(d, c, 9)
        && card::is_attack_below(d, c, 3000)
        && card::is_set_card(d, c, &[SET_NUMBER as u16])
        && duel::get_location_count_from_ex(d, tp, Some(rp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, a.e, a.tp, a.rp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX)
            && card::get_overlay_count(d, c) > 0;
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn gfilter(d: &mut Duel, c: CardId, tp: u8, rp: u8, rank: u32) -> bool {
    !card::is_rank(d, c, &[rank]) && duel::get_location_count_from_ex(d, tp, Some(rp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let overlay = card::get_overlay_group(d, c);
    let mut ct = group::get_class_count(d, &overlay, &|d, x| card::get_code(d, x)[0] as i64) as i64;
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ct = 1;
    }
    let gate = aux::check_summon_gate_limit(d, tp).unwrap_or(ct);
    ct = ct.min(gate);
    if ct > 0 {
        let mut g1 = duel::get_matching_group(d, Some(&|d, x| filter(d, x, a.e, tp, a.rp)), tp, LOCATION_EXTRA, 0, NO_EX);
        if !g1.is_empty() {
            loop {
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                let sel = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
                let tc = group::get_first(d, &sel).nil();
                duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                let e1 = effect::create_effect(d, c);
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e1, EFFECT_DISABLE);
                effect::set_reset(d, e1, RESET_EVENT + RESETS_STANDARD, 1);
                card::register_effect(d, tc, e1, false);
                let e2 = effect::create_effect(d, c);
                effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
                effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
                effect::set_reset(d, e2, RESET_EVENT + RESETS_STANDARD, 1);
                card::register_effect(d, tc, e2, false);
                let rank = card::get_rank(d, tc);
                group::match_(d, &mut g1, &|d, x| gfilter(d, x, tp, a.rp, rank), NO_EX);
                ct -= 1;
                if g1.is_empty() || ct == 0 || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
                    break;
                }
            }
            duel::special_summon_complete(d);
            duel::break_effect(d);
            card::remove_overlay_card(d, c, tp, 1, 1, REASON_EFFECT);
        }
    }
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_CHANGE_BATTLE_DAMAGE);
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_target_range(d, e3, 0, 1);
    effect::set_value(d, e3, HALF_DAMAGE as i64);
    effect::set_reset(d, e3, RESET_PHASE + PHASE_END, 1);
    duel::register_effect(d, e3, tp);
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    effect::set_description(d, e4, aux::stringid(ID, 2));
    effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_reset(d, e4, RESET_PHASE + PHASE_END, 1);
    effect::set_target_range(d, e4, 1, 0);
    duel::register_effect(d, e4, tp);
}
fn indfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_NUMBER as u16])
}
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let hp = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&indfilter), hp, LOCATION_MZONE, 0, 1, c.into())
}
