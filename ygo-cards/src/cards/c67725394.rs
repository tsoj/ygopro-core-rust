//! Doodle Beast - Tyranno
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 67725394;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DOODLE_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Summon with 1 tribute
    // local ea=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _ea = aux::add_normal_summon_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE as u32), Some(aux::stringid(ID, 0)), Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, tp: u8| otfilter(d, c, tp))), None);
    // local eb=aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _eb = aux::add_normal_set_procedure(d, c, true, true, Some(1), Some(1), Some(SUMMON_TYPE_TRIBUTE as u32), Some(aux::stringid(ID, 0)), Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, tp: u8| otfilter(d, c, tp))), None);
    // Summon itself by Tribute Summon (Quick)
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 1));
    effect::set_category(d, e1, CATEGORY_SUMMON + CATEGORY_SET);
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_hint_timing(d, e1, TIMINGS_CHECK_MONSTER, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(tscon)));
    effect::set_target(d, e1, Some(cost_target(tstg)));
    effect::set_operation(d, e1, Some(operation(tsop)));
    card::register_effect(d, c, e1, false);
    // Destroy 1 monster if Normal Summoned then increase its own ATK if the tribute was a "Doodle Beast" monster
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 2));
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_ATKCHANGE);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    effect::set_target(d, e2, Some(cost_target(destg)));
    effect::set_operation(d, e2, Some(operation(desop)));
    card::register_effect(d, c, e2, false);
    // Tribute check
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e3, effect_card_value(valcheck));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
}
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    let g = card::get_material(d, c);
    if group::is_exists(d, &g, &|d, x| card::is_set_card(d, x, &[SET_DOODLE_BEAST as u16]), 1, NO_EX) {
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, lo, &[1]);
    } else {
        let lo = effect::get_label_object(d, e).as_effect().nil();
        effect::set_label(d, lo, &[0]);
    }
    0
}
fn otfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_race(d, c, RACE_DINOSAUR) && (card::is_controler(d, c, tp) || card::is_faceup(d, c))
}
fn tscon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsMainPhase() and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_DINOSAUR),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    duel::is_main_phase(d, None)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_DINOSAUR)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
fn tstg(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::can_summon_or_set(d, c, true, None, 1, None);
    }
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, LOCATION_HAND as i32);
    true
}
fn tsop(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    if card::is_relate_to_effect(d, c, a.e) {
        duel::summon_or_set(d, a.tp, c, true, None, 1, None);
    }
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_matching_card(d, a.tp, None, a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    if !g.is_empty() {
        duel::hint_selection(d, g.clone(), true);
        let tc = group::get_first(d, &g).nil();
        // local atk=0 ... atk=tc:GetAttack()/2 (a float in Lua)
        let mut atk: f64 = 0.0;
        if card::is_faceup(d, tc) {
            atk = card::get_attack(d, tc) as f64 / 2.0;
        }
        let c = effect::get_handler(d, a.e).nil();
        if duel::destroy(d, tc, REASON_EFFECT, None, None) > 0
            && card::is_relate_to_effect(d, c, a.e)
            && card::is_tribute_summoned(d, c)
            && card::is_faceup(d, c)
            && effect::get_label(d, a.e) == 1
            && atk > 0.0
        {
            duel::break_effect(d);
            // Increase ATK
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_value(d, e1, lua_int(atk));
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
            card::register_effect(d, c, e1, false);
        }
    }
}
