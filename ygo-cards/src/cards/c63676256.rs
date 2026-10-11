//! Buster Blaster
use super::prelude::*;

pub const ID: u32 = 63676256;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddUnionProcedure(c,nil,true,false)
    aux::add_union_procedure(d, c, None, true, Some(false));
    // atk,def
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_condition(d, e1, Some(aux::is_union_state()));
    effect::set_value(d, e1, -500);
    card::register_effect(d, c, e1, false);
    let e2 = effect::clone(d, e1);
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e2, false);
    // search
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    effect::set_condition(d, e3, Some(condition(descon)));
    effect::set_target(d, e3, Some(cost_target(destg)));
    effect::set_operation(d, e3, Some(operation(desop)));
    card::register_effect(d, c, e3, false);
}
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return aux.IsUnionState(e) and #eg==1 and eg:GetFirst()==e:GetHandler():GetEquipTarget()
    if !aux::is_union_state().call(d, &[Value::Effect(a.e)]).truthy() {
        return false;
    }
    let eg = a.eg.clone().nil();
    if eg.len() != 1 {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    group::get_first(d, &eg) == card::get_equip_target(d, h)
}
/// `s.dfilter(c,rac)`
fn dfilter(d: &mut Duel, c: CardId, rac: u64) -> bool {
    card::is_faceup(d, c) && card::is_race(d, c, rac)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    // local tc=eg:GetFirst():GetBattleTarget()
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    let tc = card::get_battle_target(d, first).nil();
    // local desg=Duel.GetMatchingGroup(s.dfilter,tp,LOCATION_MZONE,LOCATION_MZONE,nil,tc:GetRace())
    let rac = card::get_race(d, tc);
    let desg = duel::get_matching_group(d, Some(&|d, c| dfilter(d, c, rac)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(desg.clone()), desg.len() as u8, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let eg = a.eg.clone().nil();
    let first = group::get_first(d, &eg).nil();
    let tc = card::get_battle_target(d, first).nil();
    let rac = card::get_race(d, tc);
    let desg = duel::get_matching_group(d, Some(&|d, c| dfilter(d, c, rac)), a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    duel::destroy(d, desg, REASON_EFFECT, None, None);
}
