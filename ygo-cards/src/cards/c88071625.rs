//! The Tyrant Neptune
use super::prelude::*;

pub const ID: u32 = 88071625;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    let e1 = effect::create_effect(d, c);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    effect::set_value_fn(d, e1, aux_false());
    card::register_effect(d, c, e1, false);
    // summon/set with 1 tribute
    // local e2=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0))
    let _e2 = aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE as u32), Some(aux::stringid(ID, 0)), None, None);
    // local e3=aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0))
    let _e3 = aux::add_normal_set_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE as u32), Some(aux::stringid(ID, 0)), None, None);
    // tribute check
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e4, effect_card_value(valcheck));
    card::register_effect(d, c, e4, false);
    // give atk effect only when summon
    let e5 = effect::create_effect(d, c);
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e5, EFFECT_SUMMON_COST);
    effect::set_operation(d, e5, Some(operation(facechk)));
    effect::set_label_object(d, e5, Value::Effect(e4));
    card::register_effect(d, c, e5, false);
    // copy
    let e6 = effect::create_effect(d, c);
    effect::set_description(d, e6, aux::stringid(ID, 1));
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e6, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e6, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e6, Some(condition(copycon)));
    effect::set_target(d, e6, Some(cost_target(copytg)));
    effect::set_operation(d, e6, Some(operation(copyop)));
    card::register_effect(d, c, e6, false);
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    let g = card::get_material(d, c);
    let _ = group::get_first(d, &g);
    let mut atk: i64 = 0;
    let mut def: i64 = 0;
    for tc in group::iter(d, &g) {
        let catk = card::get_text_attack(d, tc);
        let cdef = card::get_text_defense(d, tc);
        atk += if catk >= 0 { catk as i64 } else { 0 };
        def += if cdef >= 0 { cdef as i64 } else { 0 };
    }
    if effect::get_label(d, e) == 1 {
        effect::set_label(d, e, &[0]);
        // atk continuous effect
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        effect::set_range(d, e1, LOCATION_MZONE);
        effect::set_value(d, e1, atk);
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
        card::register_effect(d, c, e1, false);
        // def continuous effect
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
        effect::set_value(d, e2, def);
        card::register_effect(d, c, e2, false);
    }
    // the function returns nothing: the core reads nil as 0
    0
}
/// `s.facechk(e,tp,eg,ep,ev,re,r,rp)`
fn facechk(d: &mut Duel, a: &EffArgs) {
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, lo, &[1]);
}
/// `s.copycon(e,tp,eg,ep,ev,re,r,rp)`
fn copycon(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, c)
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    card::is_type(d, c, TYPE_EFFECT) && card::is_location(d, c, LOCATION_GRAVE) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.copytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn copytg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    if let Some(chkc) = a.chkc() {
        let c = effect::get_handler(d, e).nil();
        let mat = card::get_material(d, c);
        return group::is_contains(&mat, chkc) && filter(d, chkc, e);
    }
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    let c = effect::get_handler(d, e).nil();
    let mat = card::get_material(d, c);
    let g = group::filter_select(d, &mat, tp, &|d, tc| filter(d, tc, e), 1, 1, false, NO_EX).nil();
    duel::set_target_card(d, g);
    true
}
/// `s.copyop(e,tp,eg,ep,ev,re,r,rp)`
fn copyop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let c = effect::get_handler(d, e).nil();
    let tc = duel::first_target(d);
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e) && card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) {
            let code = card::get_original_code(d, tc);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            effect::set_code(d, e1, EFFECT_CHANGE_CODE);
            effect::set_value(d, e1, code as i64);
            card::register_effect(d, c, e1, false);
            // c:CopyEffect(code,RESET_EVENT|RESETS_STANDARD,1)
            let _ = card::copy_effect(d, c, code, RESET_EVENT | RESETS_STANDARD, 1);
        }
    }
}
