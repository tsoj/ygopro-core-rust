//! Tyler the Great Warrior
use super::prelude::*;

pub const ID: u32 = 68811206;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::add_cannot_be_special_summoned(d, c);
    // Inflict damage to your opponent's Life Points equal to the ATK of the destroyed monster
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e1, Some(condition(damcon)));
    effect::set_target(d, e1, Some(cost_target(damtg)));
    effect::set_operation(d, e1, Some(operation(damop)));
    card::register_effect(d, c, e1, false);
}
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c);
    card::is_relate_to_battle(d, c) && {
        let bc = bc.nil();
        card::is_location(d, bc, LOCATION_GRAVE) && card::is_monster(d, bc)
    }
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, c).nil();
    let mut dam = card::get_attack(d, bc);
    if dam < 0 {
        dam = 0;
    }
    duel::set_target_player(d, 1 - tp);
    duel::set_target_param(d, dam as u32);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, dam);
    true
}
fn damop(d: &mut Duel, _a: &EffArgs) {
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    duel::damage(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
