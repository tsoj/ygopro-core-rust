//! Appliancer Laundry Dragon
use super::prelude::*;

pub const ID: u32 = 77102944;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_APPLIANCER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    card::enable_revive_limit(d, c);
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_set_card_ctx(d, c, &[SET_APPLIANCER as u16], sc, st, Some(tp)))), 1, None, None, None, None);
    // cannot link material
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
    effect::set_condition(d, e1, Some(condition(lkcon)));
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // avoid battle damage
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_NO_BATTLE_DAMAGE);
    card::register_effect(d, c, e3, false);
    // banish
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 0));
    effect::set_category(d, e4, CATEGORY_REMOVE);
    effect::set_type(d, e4, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EVENT_BATTLED);
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    effect::set_condition(d, e4, Some(condition(rmcon)));
    effect::set_operation(d, e4, Some(operation(rmop)));
    card::register_effect(d, c, e4, false);
    // destroy
    let e5 = effect::create_effect(d, c);
    effect::set_description(d, e5, aux::stringid(ID, 1));
    effect::set_category(d, e5, CATEGORY_DESTROY);
    effect::set_type(d, e5, EFFECT_TYPE_TRIGGER_O + EFFECT_TYPE_SINGLE);
    effect::set_code(d, e5, EVENT_BATTLED);
    effect::set_count_limit(d, e5, 1, 0, 0, 0);
    effect::set_condition(d, e5, Some(condition(descon)));
    effect::set_operation(d, e5, Some(operation(desop)));
    card::register_effect(d, c, e5, false);
}
fn lkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_status(d, c, STATUS_SPSUMMON_TURN) && card::is_link_summoned(d, c)
}
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    match card::get_battle_target(d, c) {
        Some(bc) => card::is_controler(d, bc, 1 - tp) && card::get_mutual_linked_group_count(d, c) > 0,
        None => false,
    }
}
fn rmop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if let Some(tc) = card::get_battle_target(d, c) {
        if card::is_relate_to_battle(d, tc) {
            duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    match card::get_battle_target(d, c) {
        Some(bc) => card::is_controler(d, bc, 1 - tp) && card::get_mutual_linked_group_count(d, c) == 0,
        None => false,
    }
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if let Some(tc) = card::get_battle_target(d, c) {
        if card::is_relate_to_battle(d, tc) {
            let mut atk = card::get_base_attack(d, tc);
            if atk < 0 {
                atk = 0;
            }
            if duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
                duel::damage(d, 1 - tp, atk as i64, REASON_EFFECT, false, None);
            }
        }
    }
}
