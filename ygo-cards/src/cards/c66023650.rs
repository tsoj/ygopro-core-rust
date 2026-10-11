//! Folgo, Justice Fur Hire
use super::prelude::*;

pub const ID: u32 = 66023650;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FUR_HIRE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,nil,3,3,s.lcheck)
    link::add_procedure(d, c, None, 3, Some(3), Some(group_check(lcheck)), None, None);
    // cannot link material
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_UNCOPYABLE, 0);
    effect::set_code(d, e1, EFFECT_CANNOT_BE_LINK_MATERIAL);
    effect::set_value(d, e1, 1);
    card::register_effect(d, c, e1, false);
    // spsummon
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 0));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_condition(d, e2, Some(condition(spcon)));
    effect::set_target(d, e2, Some(cost_target(sptg)));
    effect::set_operation(d, e2, Some(operation(spop)));
    card::register_effect(d, c, e2, false);
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e3, EFFECT_MATERIAL_CHECK);
    effect::set_value_fn(d, e3, effect_card_value(valcheck));
    effect::set_label_object(d, e3, Value::Effect(e2));
    card::register_effect(d, c, e3, false);
    // draw
    let e4 = effect::create_effect(d, c);
    effect::set_description(d, e4, aux::stringid(ID, 1));
    effect::set_category(d, e4, CATEGORY_DRAW);
    effect::set_type(d, e4, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    effect::set_code(d, e4, EVENT_DESTROYED);
    effect::set_property(d, e4, EFFECT_FLAG_DELAY | EFFECT_FLAG_DAMAGE_STEP | EFFECT_FLAG_PLAYER_TARGET, 0);
    effect::set_range(d, e4, LOCATION_MZONE);
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    effect::set_condition(d, e4, Some(condition(drcon)));
    effect::set_target(d, e4, Some(cost_target(drtg)));
    effect::set_operation(d, e4, Some(operation(drop_op)));
    card::register_effect(d, c, e4, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:CheckDifferentPropertyBinary(Card.GetRace,lc,sumtype,tp)
    group::check_different_property_binary(d, g, &|d, c| card::get_race_ctx(d, c, Some(lc), sumtype, Some(tp)))
}
/// `s.valcheck(e,c)`: the races of the materials, stored in the label of `e:GetLabelObject()`
fn valcheck(d: &mut Duel, e: EffectId, c: CardId) -> i64 {
    let g = card::get_material(d, c);
    let mut val: u64 = 0;
    for tc in group::iter(d, &g) {
        val |= card::get_race(d, tc);
    }
    let e2 = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, e2, &[val as i64]);
    0
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h) && card::get_material(d, h).len() == 3
}
/// `s.spfilter(c,e,tp,rc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rc: i64) -> bool {
    card::is_set_card(d, c, &[SET_FUR_HIRE as u16])
        && !card::is_race(d, c, rc as u64)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        let rc = effect::get_label(d, e);
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, rc)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    let rc = effect::get_label(d, e);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, rc)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX);
    if let Some(g) = g {
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
        }
    }
}
/// `s.drcfilter(c,tp)`
fn drcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    card::is_previous_location(d, c, LOCATION_ONFIELD)
        && !card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    group::is_exists(d, &a.eg(), &|d, c| drcfilter(d, c, tp), 1, NO_EX)
}
/// `s.drfilter(c)`
fn drfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_FUR_HIRE as u16])
}
fn class_count_by_code(d: &mut Duel, g: &CardSet) -> usize {
    group::get_class_count(d, g, &|d, c| card::get_code(d, c)[0] as i64)
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&drfilter), tp, LOCATION_MZONE, 0, NO_EX);
    let mut ct: u32 = 1;
    if class_count_by_code(d, &g) >= 3 {
        ct = 3;
    }
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, ct);
    }
    duel::set_target_player(d, tp);
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, ct as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let g = duel::get_matching_group(d, Some(&drfilter), a.tp, LOCATION_MZONE, 0, NO_EX);
    let p = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER])[0].int() as u8;
    if duel::draw(d, p, 1, REASON_EFFECT) > 0 && class_count_by_code(d, &g) >= 3 {
        duel::break_effect(d);
        duel::draw(d, p, 2, REASON_EFFECT);
    }
}
