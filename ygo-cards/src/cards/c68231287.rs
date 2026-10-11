//! Jupiter the Power Patron of Destruction
use super::prelude::*;

pub const ID: u32 = 68231287;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_DOOMZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);

    // Xyz Summon procedure: 3 Level 10 monsters
    // Xyz.AddProcedure(c,nil,10,3,function(c) return c:GetEquipCount()>=3 end,aux.Stringid(id,0),3,s.altprocop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(10),
        3,
        Some(xyz::alter_filter(|d, c, _tp, _xyzc| card::get_equip_count(d, c) >= 3)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(3)),
        Some(lua_fn(altprocop)),
        false,
        None,
    );

    // You can equip any number of appropriate Equip Spells from your GY to this card
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_category(d, e1, CATEGORY_EQUIP);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(xyz_summoned)));
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    effect::set_operation(d, e1, Some(operation(eqop)));
    card::register_effect(d, c, e1, false);

    // Gains 3000 ATK while it has an Xyz Monster as material
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_condition(d, e2, Some(condition(atkcon)));
    effect::set_value(d, e2, 3000);
    card::register_effect(d, c, e2, false);

    // Special Summon 1 "DoomZ" monster from your GY, then you can destroy 1 card on the field
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 2));
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e3, LOCATION_MZONE);
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    effect::set_target(d, e3, Some(cost_target(sptg)));
    effect::set_operation(d, e3, Some(operation(spop)));
    card::register_effect(d, c, e3, false);
}
/// `s.altprocop(e,tp,chk)` (and `op(e,tp,chk,c)` of the Xyz procedure)
fn altprocop(d: &mut Duel, a: Args) -> Value {
    let tp = a.p(1);
    if a.int(2) == 0 {
        return (!duel::has_flag_effect(d, tp, ID, None)).into();
    }
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, EFFECT_FLAG_OATH, 1, 0)
        .map_or(Value::Nil, Value::Effect)
}
fn xyz_summoned(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h)
}
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let og = card::get_overlay_group(d, h);
    group::is_exists(d, &og, &|d, c| card::is_type(d, c, TYPE_XYZ), 1, NO_EX)
}
fn eqfilter(d: &mut Duel, c: CardId, tp: u8, ec: CardId) -> bool {
    card::is_equip_spell(d, c) && card::check_equip_target(d, c, ec) && card::check_unique_on_field(d, c, tp, None, None)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| eqfilter(d, c, tp, h)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, 0, LOCATION_GRAVE as i32);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let c = effect::get_handler(d, a.e).nil();
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    if ft == 0 {
        return;
    }
    let nv = aux::necro_valley_filter(|d, x| eqfilter(d, x, tp, c));
    let g = duel::get_matching_group(d, Some(&nv), tp, LOCATION_GRAVE, 0, NO_EX);
    if g.is_empty() {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    let max = (ft.max(0) as usize).min(g.len());
    let sg = group::select(d, &g, tp, 1, max as u16, false, NO_EX).nil();
    if !sg.is_empty() {
        for tc in group::iter(d, &sg) {
            duel::equip(d, tp, tc, c, true, true);
        }
        duel::equip_complete(d);
    }
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_DOOMZ as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    if !g.is_empty()
        && duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::get_field_group_count(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD) > 0
        && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
    {
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        let dg = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
        if !dg.is_empty() {
            duel::hint_selection(d, dg.clone(), true);
            duel::break_effect(d);
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}
