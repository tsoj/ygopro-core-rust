//! Update Jammer
use super::prelude::*;

pub const ID: u32 = 88093706;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Must be properly summoned before reviving
    link::add_procedure(d, c, Some(mat_filter(matfilter)), 2, Some(2), None, None, None);
    // Link summon procedure
    card::enable_revive_limit(d, c);
    // If your cyberse monster battles, negate all other card effects on the field, also use original ATK/DEF
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
    // If sent to GY as link material, that link monster can make a second attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_condition(d, e2, Some(condition(matcon)));
    effect::set_operation(d, e2, Some(operation(matop)));
    card::register_effect(d, c, e2, false);
}
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    card::is_level_above(d, c, 2) && card::is_race_ctx(d, c, RACE_CYBERSE, lc, sumtype, Some(tp))
}
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    let mut atk = duel::get_attacker(d);
    let mut at = duel::get_attack_target(d);
    if at.is_none() {
        return false;
    }
    // if a:IsControler(1-tp) then a,at=at,a end
    if card::is_controler(d, atk.nil(), 1 - a.tp) {
        std::mem::swap(&mut atk, &mut at);
    }
    let x = atk.nil();
    card::is_faceup(d, x) && card::is_race(d, x, RACE_CYBERSE)
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let atk = duel::get_attacker(d);
    let tgt = duel::get_attack_target(d);
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    effect::set_code(d, e1, EFFECT_DISABLE);
    effect::set_target_range(d, e1, LOCATION_ONFIELD, LOCATION_ONFIELD);
    effect::set_target(d, e1, Some(effect_card_fn(distg)));
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    duel::register_effect(d, e1, a.tp);
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e2, EVENT_CHAIN_SOLVING);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_operation(d, e2, Some(operation(disop)));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
    duel::register_effect(d, e2, a.tp);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    effect::set_code(d, e3, EFFECT_DISABLE_TRAPMONSTER);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    effect::set_reset(d, e3, RESET_PHASE | PHASE_DAMAGE, 1);
    duel::register_effect(d, e3, a.tp);
    // damage after destroying
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYED);
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_reset(d, e4, RESET_PHASE | PHASE_DAMAGE, 1);
    effect::set_condition(d, e4, Some(condition(damcon)));
    effect::set_target(d, e4, Some(cost_target(damtg)));
    effect::set_operation(d, e4, Some(operation(damop)));
    duel::register_effect(d, e4, a.tp);
    let att = atk.nil();
    if card::is_relate_to_battle(d, att) {
        let mut aa = card::get_text_attack(d, att);
        let mut ad = card::get_text_defense(d, att);
        if card::is_immune_to_effect(d, att, a.e) {
            aa = card::get_base_attack(d, att);
            ad = card::get_base_defense(d, att);
        }
        if aa < 0 {
            aa = 0;
        }
        if ad < 0 {
            ad = 0;
        }
        let e4 = effect::create_effect(d, c);
        effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e4, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
        effect::set_range(d, e4, LOCATION_MZONE);
        effect::set_code(d, e4, EFFECT_SET_ATTACK_FINAL);
        effect::set_reset(d, e4, RESET_PHASE | PHASE_DAMAGE, 1);
        effect::set_value(d, e4, aa as i64);
        card::register_effect(d, att, e4, true);
        let e5 = effect::create_effect(d, c);
        effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
        effect::set_range(d, e5, LOCATION_MZONE);
        effect::set_code(d, e5, EFFECT_SET_DEFENSE_FINAL);
        effect::set_reset(d, e5, RESET_PHASE | PHASE_DAMAGE, 1);
        effect::set_value(d, e5, ad as i64);
        card::register_effect(d, att, e5, true);
    }
    if let Some(dd) = tgt {
        if card::is_relate_to_battle(d, dd) {
            let mut da = card::get_text_attack(d, dd);
            let mut dv = card::get_text_defense(d, dd);
            if card::is_immune_to_effect(d, dd, a.e) {
                da = card::get_base_attack(d, dd);
                dv = card::get_base_defense(d, dd);
            }
            if da < 0 {
                da = 0;
            }
            if dv < 0 {
                dv = 0;
            }
            let e6 = effect::create_effect(d, c);
            effect::set_type(d, e6, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e6, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
            effect::set_range(d, e6, LOCATION_MZONE);
            effect::set_code(d, e6, EFFECT_SET_ATTACK_FINAL);
            effect::set_value(d, e6, da as i64);
            effect::set_reset(d, e6, RESET_PHASE | PHASE_DAMAGE, 1);
            card::register_effect(d, dd, e6, true);
            let e7 = effect::create_effect(d, c);
            effect::set_type(d, e7, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e7, EFFECT_FLAG_SINGLE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
            effect::set_range(d, e7, LOCATION_MZONE);
            effect::set_code(d, e7, EFFECT_SET_DEFENSE_FINAL);
            effect::set_value(d, e7, dv as i64);
            effect::set_reset(d, e7, RESET_PHASE | PHASE_DAMAGE, 1);
            card::register_effect(d, dd, e7, true);
        }
    }
}
/// `function(e,c) return c~=e:GetOwner() end`
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    Some(c) != effect::get_owner(d, e)
}
fn disop(d: &mut Duel, a: &EffArgs) {
    let v = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_EFFECT]);
    let loc = v.first().map_or(0, |x| x.int());
    let ce = v.get(1).and_then(|x| x.as_effect()).nil();
    if loc & LOCATION_ONFIELD as i64 == LOCATION_ONFIELD as i64 && effect::get_owner(d, ce) != effect::get_owner(d, a.e) {
        duel::negate_effect(d, a.ev as u8);
    }
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    card::is_location(d, tc, LOCATION_GRAVE) && card::is_monster(d, tc) && !card::is_previous_controler(d, tc, a.tp)
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_target_player(d, 1 - a.tp);
    duel::set_target_param(d, 1000);
    true
}
fn damop(d: &mut Duel, _a: &EffArgs) {
    let v = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, v[0].int() as u8, v[1].int(), REASON_EFFECT, false, None);
}
fn matcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_location(d, c, LOCATION_GRAVE) && (a.r & REASON_LINK) == REASON_LINK && {
        let rc = card::get_reason_card(d, c).nil();
        card::is_link_monster(d, rc)
    }
}
fn matop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(sc) = card::get_reason_card(d, c) {
        if card::is_faceup(d, sc) && card::is_on_field(d, sc) {
            let e1 = effect::create_effect(d, c);
            effect::set_description(d, e1, 3201);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            effect::set_value(d, e1, 1);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, sc, e1, false);
        }
    }
}
