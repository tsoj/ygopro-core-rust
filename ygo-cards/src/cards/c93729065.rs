//! Starving Venemy Dragon
use super::prelude::*;

pub const ID: u32 = 93729065;

pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[0x1149], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Pendulum Summon procedure
    pendulum::add_procedure(d, c, Some(false), None);
    // Fusion Summon procedure: 1 Pendulum Monster + 1 DARK monster
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_type_ctx(d, c, TYPE_PENDULUM, Some(fc), st, Some(tp)))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, Some(fc), st, Some(tp)))),
        ],
    );
    // Place 1 Venemy Counter on this card for each card sent from the field to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e1, EVENT_TO_GRAVE);
    effect::set_range(d, e1, LOCATION_PZONE);
    effect::set_operation(d, e1, Some(operation(counterplaceop)));
    card::register_effect(d, c, e1, false);
    // Monsters on the field lose 200 ATK for each Venemy Counter on this card, except DARK Dragon monsters
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_PZONE);
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| !(card::is_attribute(d, c, ATTRIBUTE_DARK) && card::is_race(d, c, RACE_DRAGON)))));
    effect::set_value_fn(d, e2, effect_card_value(|d, _e, _c| duel::get_counter(d, 0, 1, 1, 0x1149) as i64 * -200));
    card::register_effect(d, c, e2, false);
    // Once per turn, you can reduce battle damage you would take to 0
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 0));
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e3, EVENT_PRE_BATTLE_DAMAGE);
    effect::set_range(d, e3, LOCATION_PZONE);
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        a.ep == a.tp && !card::has_flag_effect(d, h, ID, None)
    })));
    effect::set_operation(d, e3, Some(operation(nodamop)));
    card::register_effect(d, c, e3, false);
    // This card gains the name and original effects of 1 monster your opponent controls, then that monster loses 500 ATK/DEF,
    // its effects are negated and your opponent takes 500 damage
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_DISABLE + CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE + CATEGORY_DAMAGE);
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    effect::set_target(d, e4, Some(cost_target(copytg)));
    effect::set_operation(d, e4, Some(operation(copyop)));
    card::register_effect(d, c, e4, false);
    // If this card in the Monster Zone is destroyed: You can place it in your Pendulum Zone
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 2));
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e5, EVENT_DESTROYED);
    effect::set_property(d, e5, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    effect::set_condition(d, e5, Some(condition(pencon)));
    effect::set_target(d, e5, Some(cost_target(pentg)));
    effect::set_operation(d, e5, Some(operation(penop)));
    card::register_effect(d, c, e5, false);
}
fn counterplaceop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &|d, c| card::is_previous_location(d, c, LOCATION_ONFIELD), NO_EX);
    if ct > 0 {
        let h = effect::get_handler(d, a.e).nil();
        card::add_counter(d, h, 0x1149, ct as u16, false);
    }
}
fn nodamop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let tp = a.tp;
    if duel::select_effect_yes_no(d, tp, c, None) {
        card::register_flag_effect(d, c, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        duel::hint(d, HINT_CARD as u8, 0, ID as u64);
        duel::change_battle_damage(d, tp, 0, true);
    }
}
fn copytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_negatable_monster(d, chkc);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_negatable_monster(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_negatable_monster(d, c)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 500);
    true
}
fn copyop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    let tc = duel::first_target(d);
    let Some(tc) = tc else { return };
    if card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) && card::is_relate_to_effect(d, tc, e) && card::is_faceup(d, tc) && !card::is_type(d, tc, TYPE_TOKEN) {
        let code = card::get_original_code_rule(d, tc)[0];
        // This card's name becomes the targeted monster's name and replaces this card's effects with that target's original effects
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_CHANGE_CODE);
        effect::set_value(d, e1, code as i64);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
        if card::copy_effect(d, c, code, RESETS_STANDARD_PHASE_END, 1) > 0 {
            duel::break_effect(d);
            // It loses 500 ATK and DEF
            card::update_attack(d, tc, -500, Some(RESET_EVENT | RESETS_STANDARD), Some(c), None);
            card::update_defense(d, tc, -500, Some(RESET_EVENT | RESETS_STANDARD), Some(c), None);
            // Negate its effects
            card::negate_effects(d, tc, c, None, false, None);
            // Inflict 500 damage to your opponent
            duel::damage(d, 1 - tp, 500, REASON_EFFECT, false, None);
        }
    }
}
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)
}
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    true
}
fn penop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if !duel::check_pendulum_zones(d, tp) {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::move_to_field(d, c, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
