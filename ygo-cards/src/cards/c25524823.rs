//! Gravekeeper's Oracle
use super::prelude::*;

pub const ID: u32 = 25524823;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GRAVEKEEPERS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // summon with 3 tribute
    let e1 = aux::add_normal_summon_procedure(d, c, true, true, Some(3), Some(3), Some(SUMMON_TYPE_TRIBUTE), Some(aux::stringid(ID, 0)), None, None);
    // summon with 1 tribute
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e2, EFFECT_SUMMON_PROC);
    effect::set_condition(d, e2, Some(lua_fn(otcon)));
    effect::set_target(d, e2, Some(lua_fn(ottg)));
    effect::set_operation(d, e2, Some(lua_fn(otop)));
    effect::set_value(d, e2, SUMMON_TYPE_TRIBUTE as i64);
    card::register_effect(d, c, e2, false);
    // summon success
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 6));
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e3, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e3, Some(condition(cond)));
    effect::set_target(d, e3, Some(cost_target(target)));
    effect::set_operation(d, e3, Some(operation(op)));
    card::register_effect(d, c, e3, false);
    // tribute check
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e4, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e4, lua_fn(valcheck));
    card::register_effect(d, c, e4, false);
    effect::set_label_object(d, e3, Value::Effect(e4));
    effect::set_label_object(d, e4, Value::Effect(e3));
}
fn otfilter(d: &mut Duel, c: CardId, tp: u8, relzone: u32) -> bool {
    // return (c:IsControler(tp) or c:IsFaceup()) and c:IsSetCard(SET_GRAVEKEEPERS) and aux.IsZone(c,relzone,tp)
    (card::is_controler(d, c, tp) || card::is_faceup(d, c)) && card::is_set_card(d, c, &[SET_GRAVEKEEPERS as u16]) && aux::is_zone(d, c, relzone, tp)
}
/// `s.otcon(e,c,minc,zone,relzone,exeff)`
fn otcon(d: &mut Duel, a: Args) -> bool {
    let Some(c) = a.card(1) else { return true };
    let minc = a.int(2);
    let zone = number(a.v(3));
    let relzone = a.int(4) as u32;
    if minc > 1 || card::get_level(d, c) <= 6 {
        return false;
    }
    let tp = card::get_controler(d, c);
    let mg = duel::get_matching_group(d, Some(&|d, x| otfilter(d, x, tp, relzone)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    duel::check_tribute(d, c, 1, Some(1), Some(&mg), Some(tp), zone)
}
/// `s.ottg(e,tp,eg,ep,ev,re,r,rp,chk,c,minc,zone,relzone,exeff)`
fn ottg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    let c = a.c(9);
    let zone = number(a.v(11));
    let relzone = a.int(12) as u32;
    let mg = duel::get_matching_group(d, Some(&|d, x| otfilter(d, x, tp, relzone)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    let sg = duel::select_tribute(d, tp, c, 1, 1, Some(&mg), Some(tp), zone, true);
    if let Some(sg) = sg {
        if !sg.is_empty() {
            // sg:KeepAlive() does nothing here (groups are values)
            effect::set_label_object(d, e, Value::Group(sg));
            return true;
        }
    }
    false
}
/// `s.otop(e,tp,eg,ep,ev,re,r,rp,c)`
fn otop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(8);
    let sg = effect::get_label_object(d, e).as_group().cloned().nil();
    card::set_material(d, c, sg.clone());
    duel::release(d, sg, REASON_SUMMON | REASON_MATERIAL, None);
    // sg:DeleteGroup() does nothing here
}
/// `s.valcheck(e,c)`
fn valcheck(d: &mut Duel, a: Args) {
    let (e, c) = (a.e(0), a.c(1));
    let g = card::get_material(d, c);
    let ct = group::filter_count(d, &g, &|d, x| card::is_set_card(d, x, &[SET_GRAVEKEEPERS as u16]), NO_EX);
    let lv = group::get_sum(d, &g, &|d, x| card::get_level(d, x) as i64);
    effect::set_label(d, e, &[lv]);
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, lo, &[ct as i64]);
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown()
    card::is_facedown(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let mut ct = effect::get_label(d, a.e);
    let lo = effect::get_label_object(d, a.e).as_effect().nil();
    let mut b1 = effect::get_label(d, lo) > 0;
    let mut b2 = duel::is_existing_matching_card(d, Some(&filter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    let mut b3 = duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    if a.chk == Some(0) {
        return ct > 0 && (b1 || b2 || b3);
    }
    let mut sel: i64 = 0;
    loop {
        let mut ops: Vec<u64> = Vec::new();
        // opval[k]: the option at index k (0-based) is 1, 2 or 3
        let mut opval: Vec<i64> = Vec::new();
        if b1 {
            ops.push(aux::stringid(ID, 2));
            opval.push(1);
        }
        if b2 {
            ops.push(aux::stringid(ID, 3));
            opval.push(2);
        }
        if b3 {
            ops.push(aux::stringid(ID, 4));
            opval.push(3);
        }
        let off = ops.len() as i64 + 1;
        let op = duel::select_option(d, a.tp, true, &ops);
        let v = opval[op as usize];
        if v == 1 {
            sel += 1;
            b1 = false;
        } else if v == 2 {
            sel += 2;
            b2 = false;
        } else {
            sel += 4;
            b3 = false;
        }
        ct -= 1;
        if ct == 0 || off < 3 || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 5)) {
            break;
        }
    }
    effect::set_label(d, a.e, &[sel]);
    if (sel & 2) != 0 {
        let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
        let n = g.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let c = effect::get_handler(d, a.e).nil();
    let sel = effect::get_label(d, a.e);
    if (sel & 1) != 0 && card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        let lo = effect::get_label_object(d, a.e).as_effect().nil();
        let lv = effect::get_label(d, lo);
        let e1 = effect::create_effect(d, c);
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        effect::set_value(d, e1, lv * 100);
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        card::register_effect(d, c, e1, false);
    }
    if (sel & 2) != 0 {
        let g = duel::get_matching_group(d, Some(&filter), a.tp, 0, LOCATION_MZONE, NO_EX);
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
    if (sel & 4) != 0 {
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
        let _ = group::get_first(d, &g);
        for tc in group::iter(d, &g) {
            let e1 = effect::create_effect(d, c);
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            effect::set_value(d, e1, -2000);
            card::register_effect(d, tc, e1, false);
            let e2 = effect::clone(d, e1);
            effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
            card::register_effect(d, tc, e2, false);
        }
    }
}
/// `aux.NormalSummonCondition`-style zone argument: `zone` nil stays nil
fn number(v: &Value) -> Option<u32> {
    if v.is_nil() { None } else { Some(v.int() as u32) }
}
