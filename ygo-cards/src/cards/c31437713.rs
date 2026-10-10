//! Number 82: Heartlandraco
use super::prelude::*;

pub const ID: u32 = 31437713;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(82), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // While you control a face-up Spell, your opponent cannot target this card for attacks
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_BATTLE_TARGET);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_condition(d, e1, Some(condition(cond1)));
    effect::set_value_fn(d, e1, aux::imval1());
    card::register_effect(d, c, e1, false);
    // Apply a "this turn, this card can attack your opponent directly, but other monsters cannot attack" effect
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    effect::set_condition(d, e2, Some(condition(cond2)));
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_operation(d, e2, Some(operation(effop)));
    card::register_effect(d, c, e2, false);
}
/// `function(e) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSpell),e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil) end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_spell(d, c)), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
/// `function(e) return Duel.IsAbleToEnterBP() and not e:GetHandler():IsHasEffect(EFFECT_DIRECT_ATTACK) end`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    duel::is_able_to_enter_bp(d) && card::is_has_effect(d, h, EFFECT_DIRECT_ATTACK, None).is_empty()
}
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    let mut fid: i64 = -1;
    let c = effect::get_handler(d, e).nil();
    if card::is_relate_to_effect(d, c, e) {
        fid = card::get_field_id(d, c) as i64;
        // This turn, this card can attack your opponent directly
        let e1 = effect::create_effect(d, c);
        effect::set_description(d, e1, 3205);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        effect::set_code(d, e1, EFFECT_DIRECT_ATTACK);
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        card::register_effect(d, c, e1, false);
    }
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // But other monsters cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    effect::set_target(d, e2, Some(effect_card_fn(move |d, _e, tc| card::get_field_id(d, tc) as i64 != fid)));
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    duel::register_effect(d, e2, tp);
}
