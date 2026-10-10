//! Rank-Up-Magic Astral Force
use super::prelude::*;

pub const ID: u32 = 45950291;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Return this card from your GY to your hand during the Draw Phase
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_PREDRAW);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_condition(d, e2, Some(condition(thcon)));
    effect::set_cost(d, e2, Some(cost_target(thcost)));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn cfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_can_be_effect_target(d, c, Some(e))
}
fn filter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    let rk = card::get_rank(d, c);
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    (pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c)))
        && (rk > 0 || card::is_status(d, c, STATUS_NO_LEVEL))
        && duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, e, tp, c, rk + 2)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
fn filter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, rk: u32) -> bool {
    if let Some(rum_limit) = crate::card_info(d, c).rum_limit {
        if !rum_limit(d, mc, e) {
            return false;
        }
    }
    card::is_type_ctx(d, mc, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_rank(d, c, &[rk])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && {
            let race = card::get_race_ctx(d, mc, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp));
            card::is_race(d, c, race)
        }
        && {
            let attr = card::get_attribute_ctx(d, mc, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp));
            card::is_attribute(d, c, attr)
        }
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    let base = duel::get_matching_group(d, Some(&|d, c| cfilter(d, c, e)), tp, LOCATION_MZONE, 0, NO_EX);
    let g = match group::get_max_group(d, &base, &|d, c| card::get_rank(d, c) as i64) {
        Some((mx, _)) => group::filter(d, &mx, &|d, c| filter1(d, c, e, tp), NO_EX),
        None => CardSet::new(),
    };
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc);
    }
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let tc = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    duel::set_target_card(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let Some(tc) = duel::first_target(d) else {
        return;
    };
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(tc)), tp, None, None, REASON_XYZ);
    if card::is_facedown(d, tc)
        || !card::is_relate_to_effect(d, tc, a.e)
        || card::is_controler(d, tc, 1 - tp)
        || card::is_immune_to_effect(d, tc, a.e)
        || pg.len() > 1
        || (pg.len() == 1 && !group::is_contains(&pg, tc))
    {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let rk = card::get_rank(d, tc) + 2;
    let e = a.e;
    let g = duel::select_matching_card(d, tp, Some(&|d, x| filter2(d, x, e, tp, tc, rk)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    if let Some(sc) = group::get_first(d, &g) {
        card::set_material(d, sc, CardSet::from_card(tc));
        duel::overlay(d, sc, tc, false);
        duel::special_summon(d, sc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None);
        card::complete_procedure(d, sc);
    }
}
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    tp == duel::get_turn_player(d) && duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 0 && duel::get_draw_count(d, tp) > 0
}
/// `function(e,c,sump,sumtype,sumpos,targetp,se) return not se:GetHandler():IsCode(id) end`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let se = a.e(6);
    let h = effect::get_handler(d, se).nil();
    !card::is_code(d, h, &[ID])
}
fn thcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON]).first().copied().unwrap_or(0) == 0;
    }
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    effect::set_target_range(d, e1, 1, 0);
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    duel::register_effect(d, e1, tp);
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    true
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    let dt = duel::get_draw_count(d, tp);
    if dt != 0 {
        set_lua_global(d, "_replace_count", Value::Int(0));
        set_lua_global(d, "_replace_max", Value::Int(dt as i64));
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        effect::set_code(d, e1, EFFECT_DRAW_COUNT);
        effect::set_target_range(d, e1, 1, 0);
        effect::set_reset(d, e1, RESET_PHASE | PHASE_DRAW, 1);
        effect::set_value(d, e1, 0);
        duel::register_effect(d, e1, tp);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let cnt = lua_global(d, "_replace_count");
    if cnt.is_nil() {
        crate::lua_error("attempt to perform arithmetic on a nil value (global '_replace_count')");
    }
    let n = cnt.int() + 1;
    set_lua_global(d, "_replace_count", Value::Int(n));
    let max = lua_global(d, "_replace_max");
    if max.is_nil() {
        crate::lua_error("attempt to compare number with nil");
    }
    if n <= max.int() && card::is_relate_to_effect(d, c, a.e) {
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, c);
    }
}
