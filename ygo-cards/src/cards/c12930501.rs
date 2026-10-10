//! Dark Spell Regeneration
//! Scripted by ahtelel
use super::prelude::*;

pub const ID: u32 = 12930501;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_MONSTER_REBORN, CARD_RA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_LEAVE_GRAVE + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Special Summon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOGRAVE);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let at = duel::get_attacker(d).nil();
    card::is_controler(d, at, 1 - a.tp)
}
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_spell(d, c) && card::is_s_setable(d, c, false, None)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_GRAVE) && setfilter(d, chkc);
    }
    let h = effect::get_handler(d, a.e).nil();
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    if card::is_location(d, h, LOCATION_HAND) {
        ft -= 1;
    }
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_target(d, Some(&setfilter), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    let g = duel::select_target(d, tp, Some(&setfilter), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && card::is_s_setable(d, tc, false, None) {
        duel::s_set(d, a.tp, tc, None, true);
    }
}
fn costfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_MONSTER_REBORN])
        && card::is_able_to_grave_as_cost(d, c)
        && (card::is_facedown(d, c) || !card::is_on_field(d, c))
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)` called from a cost with the same arguments
fn bfgcost(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    aux::bfgcost()
        .call(
            d,
            &[
                Value::Effect(a.e),
                Value::Int(a.tp as i64),
                a.eg.clone().map_or(Value::Nil, Value::Group),
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                Value::effect(a.re),
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
                Value::Int(chk),
            ],
        )
        .truthy()
}
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return bfgcost(d, a, 0) && duel::is_existing_matching_card(d, Some(&|d, c| costfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    bfgcost(d, a, a.chk.unwrap_or(1));
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| costfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_code(d, c, &[CARD_RA]) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, LOCATION_GRAVE as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if g.is_empty() || duel::special_summon(d, &g, 0, tp, tp, true, false, POS_FACEUP as u8, None) < 1 {
        return;
    }
    let tc = group::get_first(d, &g).nil();
    card::register_flag_effect(d, tc, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 3));
    // Send to GY in the End Phase
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_label_object(d, e1, Value::Card(tc));
    effect::set_condition(d, e1, Some(condition(descon)));
    effect::set_operation(d, e1, Some(operation(desop)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e1, tp);
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_MZONE, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        let sg = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        if !sg.is_empty() {
            duel::break_effect(d);
            duel::send_to_grave(d, &sg, REASON_EFFECT, None, None);
        }
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    card::get_flag_effect(d, tc, ID) > 0
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::send_to_grave(d, tc, REASON_EFFECT, None, None);
}
