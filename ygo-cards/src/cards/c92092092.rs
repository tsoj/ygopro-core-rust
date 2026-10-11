//! Space Dragster
use super::prelude::*;

pub const ID: u32 = 92092092;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card as an Effect Monster (Machine-Type/Tuner/FIRE/Level 1/ATK 0/DEF 1800). (This card is also still a Trap Card.)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // If Summoned this way, other Tuners you control cannot be destroyed by battle or your opponent's card effects while this card is in the Monster Zone
    let e2a = effect::create_effect(d, c);
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2a, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_range(d, e2a, LOCATION_MZONE);
    effect::set_target_range(d, e2a, LOCATION_MZONE, 0);
    effect::set_condition(d, e2a, Some(lua_fn(e2a_cond)));
    effect::set_target(d, e2a, Some(effect_card_fn(e2a_target)));
    effect::set_value(d, e2a, 1);
    card::register_effect(d, c, e2a, false);
    let e2b = effect::clone(d, e2a);
    effect::set_code(d, e2b, EFFECT_INDESTRUCTABLE_EFFECT);
    effect::set_value_fn(d, e2b, aux::tgoval());
    card::register_effect(d, c, e2b, false);
}
fn monster_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPE_MONSTER | TYPE_EFFECT | TYPE_TUNER),
        attack: Some(0),
        defense: Some(1800),
        level: Some(1),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_FIRE),
    }
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, ID, monster_data(), None, None, 0);
    }
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
fn activate(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) && duel::is_player_can_special_summon_monster(d, a.tp, ID, monster_data(), None, None, 0) {
        card::add_monster_attribute(d, c, TYPE_EFFECT | TYPE_TUNER | TYPE_TRAP, 0, 0, 0, 0, 0);
        duel::special_summon_step(d, c, 1, a.tp, a.tp, true, false, POS_FACEUP as u8, None);
        card::add_monster_attribute_complete(d, c);
        duel::special_summon_complete(d);
    }
}
/// `function(e) return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+1) end`
fn e2a_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_summon_type(d, h, &[SUMMON_TYPE_SPECIAL + 1])
}
/// `function(e,c) return c~=e:GetHandler() and c:IsType(TYPE_TUNER) end`
fn e2a_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    Some(c) != effect::get_handler(d, e) && card::is_type(d, c, TYPE_TUNER)
}
