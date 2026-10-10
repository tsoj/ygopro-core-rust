//! My Body as a Shield
use super::prelude::*;

pub const ID: u32 = 69279219;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_cost(d, e1, Some(cost::pay_lp(cost::PayLp::Amount(1500))));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_on_field(d, c) && card::is_monster(d, c)
}
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if tp==ep or not Duel.IsChainNegatable(ev) then return false end
    if a.tp == a.ep || !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    // if not re:IsMonsterEffect() and not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return false end
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) && !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    // local ex,tg,tc=Duel.GetOperationInfo(ev,CATEGORY_DESTROY)
    // return ex and tg~=nil and tc+tg:FilterCount(s.cfilter,nil)-#tg>0
    let Some(info) = duel::get_operation_info(d, a.ev, CATEGORY_DESTROY) else {
        return false;
    };
    let Some(tg) = info.op_cards else {
        return false;
    };
    let n = group::filter_count(d, &tg, &cfilter, NO_EX) as i64;
    info.op_count as i64 + n - tg.len() as i64 > 0
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    let eg = a.eg.clone();
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, re) {
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, eg, 1, 0, 0);
    }
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then Duel.Destroy(eg,REASON_EFFECT) end
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(eg,...): an empty group when eg is nil (the chain's card group)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
