//! Doodlebook - Uh uh uh!
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 94113093;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DOODLE_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate attack and Special Summon 1 "Doodle Beast" monster from the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_BE_BATTLE_TARGET);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Add 1 level 5 or higher Dinosaur to the hand
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SEARCH + CATEGORY_TOHAND);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_condition(d, e2, Some(aux::exccon()));
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(thtg)));
    effect::set_operation(d, e2, Some(operation(thop)));
    card::register_effect(d, c, e2, false);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttackTarget()
    // return at and at:IsFaceup() and at:IsRace(RACE_DINOSAUR) and at:IsControler(tp)
    let Some(at) = duel::get_attack_target(d) else { return false };
    card::is_faceup(d, at) && card::is_race(d, at, RACE_DINOSAUR) && card::is_controler(d, at, a.tp)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_DOODLE_BEAST as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not Duel.NegateAttack() or Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if !duel::negate_attack(d) || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, LOCATION_GRAVE, 0, NO_EX);
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        if sg.is_empty() {
            return;
        }
        duel::break_effect(d);
        let tc = group::get_first(d, &sg).nil();
        if duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            let c = effect::get_handler(d, a.e).nil();
            // Cannot be destroyed by battle
            let e1 = effect::create_effect(d, c);
            effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
            effect::set_value(d, e1, 1);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            card::register_effect(d, tc, e1, false);
            // Destroy it during the End Phase
            let fid = card::get_field_id(d, c);
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, fid as i64, aux::stringid(ID, 3));
            let e2 = effect::create_effect(d, c);
            effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
            effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            effect::set_count_limit(d, e2, 1, 0, 0, 0);
            effect::set_label(d, e2, &[fid as i64]);
            effect::set_label_object(d, e2, Value::Card(tc));
            effect::set_condition(d, e2, Some(condition(descon)));
            effect::set_operation(d, e2, Some(operation(desop)));
            duel::register_effect(d, e2, a.tp);
        }
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    // if tc:GetFlagEffectLabel(id)~=e:GetLabel() then e:Reset() return false else return true end
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    let flag = card::get_flag_effect_label(d, tc, ID).first().copied();
    if flag != Some(effect::get_label(d, a.e)) {
        effect::reset(d, a.e);
        return false;
    }
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    duel::destroy(d, tc, REASON_EFFECT, None, None);
}
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_DINOSAUR) && card::is_level_above(d, c, 5) && card::is_able_to_hand(d, c, None)
}
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn thop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
