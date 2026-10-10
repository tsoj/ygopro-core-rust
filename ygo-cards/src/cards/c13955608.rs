//! Stronghold the Moving Fortress
use super::prelude::*;

pub const ID: u32 = 13955608;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[41172955, 86445415, 13839120], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
// The monster data of Stronghold (Duel.IsPlayerCanSpecialSummonMonster(tp,id,0,TYPE_MONSTER|TYPE_EFFECT,0,2000,4,RACE_MACHINE,ATTRIBUTE_EARTH))
fn stronghold_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPE_MONSTER | TYPE_EFFECT),
        attack: Some(0),
        defense: Some(2000),
        level: Some(4),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID, stronghold_data(), None, None, 0);
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, c, e) {
        return;
    }
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !duel::is_player_can_special_summon_monster(d, tp, ID, stronghold_data(), None, None, 0)
    {
        return;
    }
    card::add_monster_attribute(d, c, TYPE_TRAP + TYPE_EFFECT, 0, 0, 0, 0, 0);
    duel::special_summon_step(d, c, 0, tp, tp, true, false, POS_FACEUP_DEFENSE as u8, None);
    card::add_monster_attribute_complete(d, c);
    // update attack
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, 3000);
    effect::set_condition(d, e1, Some(condition(atkcon)));
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    card::register_effect(d, c, e1, true);
    duel::special_summon_complete(d);
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[41172955])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[86445415])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[13839120])), tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
