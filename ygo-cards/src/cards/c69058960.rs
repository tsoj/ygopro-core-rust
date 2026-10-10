//! Number 13: Embodiment of Crime
use super::prelude::*;

pub const ID: u32 = 69058960;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(13), listed_names: &[95442074], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,1,2)
    xyz::add_procedure(d, c, None, Some(1), 2, None, None, None, None, false, None);
    card::enable_revive_limit(d, c);
    // change battle position
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_POSITION);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    card::register_effect(d, c, e1, false);
    // cannot be destroyed
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    effect::set_condition(d, e2, Some(condition(indcon)));
    effect::set_value(d, e2, 1);
    card::register_effect(d, c, e2, false);
    let e3 = effect::clone(d, e2);
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    card::register_effect(d, c, e3, false);
    // reflect battle damage
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_REFLECT_BATTLE_DAMAGE);
    effect::set_condition(d, e4, Some(condition(refcon)));
    effect::set_value(d, e4, 1);
    card::register_effect(d, c, e4, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return ... end end` (no operation when the check fails)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::get_field_group_count(d, a.tp, 0, LOCATION_MZONE as u32) > 0;
    }
    true
}
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_MZONE as u32);
    if group::get_count(&g) > 0 {
        duel::change_position(d, g.clone(), POS_FACEUP_ATTACK as u8, None, None, None, false);
        let fid = card::get_real_field_id(d, c);
        for tc in group::iter(d, &g) {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_MUST_ATTACK);
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            card::register_effect(d, tc, e1, false);
            let e2 = effect::clone(d, e1);
            effect::set_code(d, e2, EFFECT_MUST_ATTACK_MONSTER);
            effect::set_value_fn(d, e2, lua_fn(atklimit));
            effect::set_label(d, e2, &[fid as i64]);
            card::register_effect(d, tc, e2, false);
        }
    }
}
/// `function(e,c) return c:GetRealFieldID()==e:GetLabel() end`
fn atklimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let c = a.c(1);
    card::get_real_field_id(d, c) as i64 == effect::get_label(d, e)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[95442074])
}
/// `function(e) return Duel.IsExistingMatchingCard(s.filter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil) and e:GetHandler():GetOverlayCount()~=0 end`
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&filter), p, LOCATION_MZONE, 0, 1, NO_EX) && card::get_overlay_count(d, h) != 0
}
/// `function(e) return Duel.IsExistingMatchingCard(s.filter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil) and Duel.GetAttackTarget()==e:GetHandler() end`
fn refcon(d: &mut Duel, a: &EffArgs) -> bool {
    let p = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&filter), p, LOCATION_MZONE, 0, 1, NO_EX) && duel::get_attack_target(d) == Some(h)
}
