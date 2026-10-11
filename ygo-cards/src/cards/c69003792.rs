//! Megalith Unformed
use super::prelude::*;

pub const ID: u32 = 69003792;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_MEGALITH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local ritparams={handler=c,lvtype=RITPROC_EQUAL,filter=aux.FilterBoolFunction(Card.IsSetCard,SET_MEGALITH),lv=s.ritlevel,location=LOCATION_DECK,sumpos=POS_FACEUP_DEFENSE}
    let ritparams = ritual::RitualProc {
        lvtype: RITPROC_EQUAL,
        filter: Some(std::sync::Arc::new(|d: &mut Duel, x: CardId| card::is_set_card(d, x, &[SET_MEGALITH as u16]))),
        lv: Some(ritual::Lv::F(std::sync::Arc::new(ritlevel))),
        location: Some(LOCATION_DECK),
        sumpos: Some(POS_FACEUP_DEFENSE as u8),
        ..Default::default()
    };
    let rtg = ritual::target(d, ritparams.clone());
    let rtop = ritual::operation(d, ritparams);
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP as u32, Some((TIMING_DAMAGE_STEP | TIMING_END_PHASE) as u32));
    effect::set_target(d, e1, Some(target_fn(rtg, rtop)));
    card::register_effect(d, c, e1, false);
}
/// `function s.ritlevel(c) return c:GetLevel()*2 end`
fn ritlevel(d: &mut Duel, c: CardId) -> i64 {
    card::get_level(d, c) as i64 * 2
}
/// `e,tp,eg,ep,ev,re,r,rp` followed by `chk`, as the core passes them to a target function
fn eff_values(a: &EffArgs, chk: i64) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        Value::Int(chk),
    ]
}
/// `s.target(rtg,rtop)`
fn target_fn(rtg: LuaFn, rtop: LuaFn) -> LuaFn {
    cost_target(move |d, a| {
        let (e, tp) = (a.e, a.tp);
        let b1 = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX) > 0
            && duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_RITUAL)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) > 0;
        let b2 = rtg.call(d, &eff_values(a, 0)).truthy();
        if a.chk == Some(0) {
            return b1 || b2;
        }
        let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 0)), (b2, aux::stringid(ID, 1))]);
        effect::set_label(d, e, &[op.map_or(0, |x| x as i64)]);
        if op == Some(1) {
            effect::set_category(d, e, CATEGORY_ATKCHANGE);
            effect::set_operation(d, e, Some(operation(atkop)));
        } else if op == Some(2) {
            effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
            effect::set_operation(d, e, Some(rtop.clone()));
            rtg.call(d, &eff_values(a, 1));
        }
        true
    })
}
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    let atk = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_RITUAL)), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX) as i64 * -500;
    if g.is_empty() || atk == 0 {
        return;
    }
    let c = effect::get_handler(d, a.e).nil();
    for tc in group::iter(d, &g) {
        op(d, tc, c, atk);
    }
}
/// `function s.op(tc,c,atk)`
fn op(d: &mut Duel, tc: CardId, c: CardId, atk: i64) {
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    effect::set_value(d, e1, atk);
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    card::register_effect(d, tc, e1, false);
}
