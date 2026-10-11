//! Metaltron XII, the True Dracombatant
use super::prelude::*;

pub const ID: u32 = 57761191;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with s/t
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_ADD_EXTRA_TRIBUTE);
    effect::set_target_range(d, e1, LOCATION_SZONE, 0);
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_type(d, c, TYPE_CONTINUOUS))));
    effect::set_value(d, e1, POS_FACEUP as i64);
    card::register_effect(d, c, e1, false);
    // summon with 3 tribute
    let _e1 = aux::add_normal_summon_procedure(d, c, true, false, Some(3), Some(3), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), None, None);
    let _e2 = aux::add_normal_set_procedure(d, c, false, false, None, None, None, None, None, None);
    // tribute check
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e4, lua_fn(valcheck));
    card::register_effect(d, c, e4, false);
    // immune reg
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE, 0);
    effect::set_code(d, e5, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e5, Some(condition(regcon)));
    effect::set_operation(d, e5, Some(operation(regop)));
    effect::set_label_object(d, e5, Value::from(e4));
    card::register_effect(d, c, e5, false);
    // special summon
    let e6 = effect::create_effect(d, c);
    effect::set_description(d, e6, aux::stringid(ID, 5));
    effect::set_category(d, e6, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e6, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e6, EVENT_DESTROYED);
    effect::set_condition(d, e6, Some(condition(spcon)));
    effect::set_target(d, e6, Some(cost_target(sptg)));
    effect::set_operation(d, e6, Some(operation(spop)));
    card::register_effect(d, c, e6, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(1);
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local typ=0
    let mut typ: u32 = 0;
    // for tc in aux.Next(g) do typ=typ|tc:GetOriginalType()&(TYPE_MONSTER|TYPE_SPELL|TYPE_TRAP) end
    for tc in group::iter(d, &g) {
        typ |= card::get_original_type(d, tc) & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP);
    }
    // e:SetLabel(typ)
    effect::set_label(d, e, &[typ as i64]);
}
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, c)
}
/// `function s.econ(e) return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) end`
fn econ(d: &mut Duel, a: Args) -> bool {
    let c = effect::get_handler(d, a.e(0)).nil();
    !card::is_status(d, c, STATUS_BATTLE_DESTROYED)
}
/// `s.efilter(e,te)`
fn efilter(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    effect::is_active_type(d, te, effect::get_label(d, e) as u32) && effect::get_owner(d, te) != effect::get_owner(d, e)
}
fn regop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    // local typ=e:GetLabelObject():GetLabel()
    let label_obj = effect::get_label_object(d, a.e).as_effect().nil();
    let typ = effect::get_label(d, label_obj) as u32;
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    effect::set_range(d, e1, LOCATION_MZONE);
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    effect::set_condition(d, e1, Some(lua_fn(econ)));
    effect::set_value_fn(d, e1, lua_fn(efilter));
    effect::set_label(d, e1, &[typ as i64]);
    card::register_effect(d, c, e1, false);
    if typ & TYPE_MONSTER != 0 {
        card::register_flag_effect(d, c, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
    }
    if typ & TYPE_SPELL != 0 {
        card::register_flag_effect(d, c, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 3));
    }
    if typ & TYPE_TRAP != 0 {
        card::register_flag_effect(d, c, 0, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 4));
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return ((rp==1-tp) or (r&REASON_BATTLE)>0) and c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
    //   and c:IsTributeSummoned()
    ((a.rp == 1 - a.tp) || (a.r & REASON_BATTLE) > 0)
        && card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_tribute_summoned(d, c)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_attribute(d, c, ATTRIBUTE_EARTH | ATTRIBUTE_WATER | ATTRIBUTE_FIRE | ATTRIBUTE_WIND)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_type(d, c, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX);
    if let Some(g) = g {
        if !g.is_empty() {
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
