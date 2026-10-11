//! Dried Winds
use super::prelude::*;

pub const ID: u32 = 28265983;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_AROMA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e0 = effect::create_effect(d, c);
    effect::set_description(d, e0, aux::stringid(ID, 0));
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    card::register_effect(d, c, e0, false);
    // destroy
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 2));
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    effect::set_range(d, e1, LOCATION_SZONE);
    effect::set_code(d, e1, EVENT_RECOVER);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_condition(d, e1, Some(condition(descon1)));
    effect::set_target(d, e1, Some(cost_target(destg1)));
    effect::set_operation(d, e1, Some(operation(desop1)));
    card::register_effect(d, c, e1, false);
    // destroy
    let e3 = effect::create_effect(d, c);
    effect::set_description(d, e3, aux::stringid(ID, 4));
    effect::set_category(d, e3, CATEGORY_DESTROY);
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    effect::set_range(d, e3, LOCATION_SZONE);
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    effect::set_condition(d, e3, Some(condition(descon2)));
    effect::set_cost(d, e3, Some(cost_target(descost2)));
    effect::set_target(d, e3, Some(cost_target(destg2)));
    effect::set_operation(d, e3, Some(operation(desop2)));
    card::register_effect(d, c, e3, false);
}
// function s.descon1(e,tp,eg,ep,ev,re,r,rp)
fn descon1(_d: &mut Duel, a: &EffArgs) -> bool {
    a.ep == a.tp
}
// function s.destg1(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg1(d: &mut Duel, a: &EffArgs) -> bool {
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_faceup(d, chkc);
    }
    if a.chk == Some(0) {
        return true;
    }
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
// function s.desop1(e,tp,eg,ep,ev,re,r,rp)
fn desop1(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
// function s.descon2(e,tp,eg,ep,ev,re,r,rp)
fn descon2(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_AROMA as u16])), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::get_lp(d, a.tp) - duel::get_lp(d, 1 - a.tp) >= 3000
}
// function s.descost2(e,tp,eg,ep,ev,re,r,rp,chk)
fn descost2(d: &mut Duel, a: &EffArgs) -> bool {
    let lp = duel::get_lp(d, a.tp) - duel::get_lp(d, 1 - a.tp);
    if a.chk == Some(0) {
        // for _,eff in pairs({Duel.IsPlayerAffectedByEffect(tp,EFFECT_LPCOST_CHANGE)}) do
        for eff in duel::is_player_affected_by_effect(d, a.tp, EFFECT_LPCOST_CHANGE).into_iter() {
            // local val=eff:GetValue()
            let val = effect::get_value(d, eff);
            // if (type(val)=='integer' and val==0) or (type(val)=='function' and (val(eff,e,tp,lp)~=lp)) then return false end
            if matches!(val, Value::Int(0)) {
                return false;
            }
            if let Value::Fn(f) = &val {
                if f.call(d, &[Value::Effect(eff), Value::Effect(a.e), Value::Int(a.tp as i64), Value::Int(lp as i64)]).int() != lp as i64 {
                    return false;
                }
            }
            // if eff:GetOwner():IsCode(74936480) then return e:GetHandler():IsStatus(STATUS_EFFECT_ENABLED) end
            let owner = effect::get_owner(d, eff).nil();
            if card::is_code(d, owner, &[74936480]) {
                let h = effect::get_handler(d, a.e).nil();
                return card::is_status(d, h, STATUS_EFFECT_ENABLED);
            }
        }
        // return Duel.CheckLPCost(tp,lp)
        return duel::check_lp_cost(d, a.tp, lp as u32);
    }
    // Duel.PayLPCost(tp,lp)
    duel::pay_lp_cost(d, a.tp, lp as u32);
    // e:SetLabel(lp)
    effect::set_label(d, a.e, &[lp as i64]);
    true
}
// function s.desfilter2(c,num)
fn desfilter2(d: &mut Duel, c: CardId, num: i64) -> bool {
    card::is_faceup(d, c) && card::is_attack_below(d, c, num as i32)
}
// function s.destg2(e,tp,eg,ep,ev,re,r,rp,chk)
fn destg2(d: &mut Duel, a: &EffArgs) -> bool {
    let lp = duel::get_lp(d, a.tp) - duel::get_lp(d, 1 - a.tp);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| desfilter2(d, c, lp as i64)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    let num = effect::get_label(d, a.e);
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter2(d, c, num)), a.tp, 0, LOCATION_MZONE, NO_EX);
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
// function s.rescon(lp) return function(sg,e,tp,mg) return sg:GetSum(Card.GetAttack)<=lp end end
fn rescon_fn(lp: i64) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| (group::get_sum(d, sg, &|d, c| card::get_attack(d, c) as i64) <= lp, false)
}
// function s.desop2(e,tp,eg,ep,ev,re,r,rp)
fn desop2(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    let num = effect::get_label(d, a.e);
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter2(d, c, num)), a.tp, 0, LOCATION_MZONE, NO_EX);
    if g.is_empty() {
        return;
    }
    let rc = rescon_fn(num);
    let dg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), None, Some(&rc), 1, a.tp, Some(HINTMSG_DESTROY as u64), None, None, false).unwrap_or_default();
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
