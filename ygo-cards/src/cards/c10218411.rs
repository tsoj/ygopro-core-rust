//! Double Fusion
//! scripted by Naim
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 10218411;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Fusion.CreateSummonEff({handler=c,stage2=s.stage2})
    let e1 = fusion::create_summon_eff(
        d,
        c,
        fusion::SummonEff {
            stage2: Some(Arc::new(|d: &mut Duel, e: EffectId, tc: Option<CardId>, tp: u8, sg: Option<&CardSet>, chk: i64| stage2(d, e, tc, tp, sg, chk))),
            ..Default::default()
        },
    );
    // e1:SetCost(Cost.PayLP(500))
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(500))));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.stage2(e,tc,tp,sg,chk)
fn stage2(d: &mut Duel, e: EffectId, _tc: Option<CardId>, tp: u8, _sg: Option<&CardSet>, chk: i64) {
    // if chk==1 then
    if chk == 1 {
        // if Fusion.SummonEffTG()(e,tp,eg,ep,ev,re,r,rp,0) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        // (eg, ep, ev, re, r, rp are not parameters of s.stage2: they are nil globals in the Lua)
        let tg_args = [Value::Effect(e), Value::Int(tp as i64), Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Int(0)];
        let tg = fusion::summon_eff_tg(d, fusion::SummonEff::default()).call(d, &tg_args).truthy();
        if tg && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Fusion.SummonEffOP()(e,tp,eg,ep,ev,re,r,rp)
            let op_args = [Value::Effect(e), Value::Int(tp as i64), Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil];
            fusion::summon_eff_op(d, fusion::SummonEff::default()).call(d, &op_args);
        }
    }
}
