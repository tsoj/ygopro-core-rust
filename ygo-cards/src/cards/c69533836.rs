//! Soul Fist
use super::prelude::*;

pub const ID: u32 = 69533836;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RED_DRAGON_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip only to a Dragon Synchro Monster you control
    aux::add_equip_procedure(d, c, Some(0), Some(aux::equip_filter_fn(|d, c, _e, _tp| eqfilter(d, c))), None, None, None, None, None, None);
    // A "Red Dragon Archfiend" equipped with this card is unaffected by your opponent's activated effects
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_condition(d, e1, Some(condition(immcon)));
    effect::set_value_fn(d, e1, lua_fn(immvalue));
    card::register_effect(d, c, e1, false);
    // Change the ATK of all monsters your opponent currently controls to the equipped monster's
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_target(d, e2, Some(cost_target(oppatktg)));
    effect::set_operation(d, e2, Some(operation(oppatkop)));
    card::register_effect(d, c, e2, false);
    // Banish 1 monster from your opponent's GY, and if you do, the equipped monster gains ATK equal to the banished monster's
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 1));
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_ATKCHANGE);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_condition(d, e3, Some(condition(e3cond)));
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    effect::set_operation(d, e3, Some(operation(rmop)));
    card::register_effect(d, c, e3, false);
}
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_race(d, c, RACE_DRAGON) && card::is_type(d, c, TYPE_SYNCHRO)
}
fn immcon(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    match card::get_equip_target(d, effect::get_handler(d, e).nil()) {
        Some(ec) => card::is_code(d, ec, &[CARD_RED_DRAGON_ARCHFIEND]),
        None => false,
    }
}
fn immvalue(d: &mut Duel, a: Args) -> bool {
    let (e, te) = (a.e(0), a.e(1));
    effect::get_owner_player(d, te) == 1 - effect::get_handler_player(d, e) && effect::is_activated(d, te)
}
fn oppatktg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let Some(ec) = card::get_equip_target(d, effect::get_handler(d, a.e).nil()) else {
            return false;
        };
        let atk = card::get_attack(d, ec);
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && !card::is_attack(d, x, &[atk as u32])), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
fn oppatkop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, c).nil();
    let atk = card::get_attack(d, ec);
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x) && !card::is_attack(d, x, &[atk as u32])), a.tp, 0, LOCATION_MZONE, NO_EX);
    if g.is_empty() {
        return;
    }
    for tc in group::iter(d, &g) {
        // Change its ATK to the equipped monster's
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        effect::set_value(d, e1, atk as i64);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        card::register_effect(d, tc, e1, false);
    }
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
fn e3cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let ec = card::get_equip_target(d, c).nil();
    card::is_relate_to_battle(d, ec)
}
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_GRAVE) && card::is_monster(d, chkc) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmfilter), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    let g = duel::select_target(d, tp, Some(&rmfilter), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, tp, 0);
    true
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tc = duel::first_target(d).nil();
    if card::is_relate_to_effect(d, tc, a.e) && duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        let atk = card::get_attack(d, tc);
        if atk <= 0 {
            return;
        }
        let c = effect::get_handler(d, a.e).nil();
        // The equipped monster gains ATK equal to the banished monster's, until the end of this turn
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, atk as i64);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        let ec = card::get_equip_target(d, c).nil();
        card::register_effect(d, ec, e1, false);
    }
}
