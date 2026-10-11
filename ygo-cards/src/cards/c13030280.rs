//! CXyz Comics Hero Legend Arthur
use super::prelude::*;

pub const ID: u32 = 13030280;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[77631175], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon procedure: 3 Level 5 monsters
    xyz::add_procedure(d, c, None, Some(5), 3, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // Once per turn, this card cannot be destroyed by battle
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_value_fn(d, e1, lua_fn(valcon));
    card::register_effect(d, c, e1, false);
    // Banish a monster destroyed by battle with this card and inflict damage to your opponent equal to its original ATK
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_DAMAGE);
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_condition(d, e2, Some(condition(damcon)));
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    effect::set_target(d, e2, Some(cost_target(damtg)));
    effect::set_operation(d, e2, Some(operation(damop)));
    card::register_effect(d, c, e2, false);
}
/// `function(e,re,r,rp)`
fn valcon(_d: &mut Duel, a: Args) -> bool {
    (a.int(2) & REASON_BATTLE as i64) != 0
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    let overlay = card::get_overlay_group(d, c);
    group::is_exists(d, &overlay, &|d, x| card::is_code(d, x, &[77631175]), 1, NO_EX)
        && card::is_relate_to_battle(d, c)
        && card::is_location(d, bc.nil(), LOCATION_GRAVE)
        && card::is_monster(d, bc.nil())
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c).nil();
    if a.chk == Some(0) {
        let p = card::get_controler(d, bc);
        return duel::is_player_affected_by_effect(d, p, CARD_SPIRIT_ELIMINATION).is_none()
            && card::is_able_to_remove(d, bc, None, None, None);
    }
    duel::set_target_card(d, bc);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(bc)), 1, 0, 0);
    let atk = card::get_base_attack(d, bc);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, atk);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    let bc = duel::first_target(d);
    if let Some(bc) = bc {
        if card::is_relate_to_effect(d, bc, a.e) && duel::remove(d, bc, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
            let atk = card::get_base_attack(d, bc);
            duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
        }
    }
}
