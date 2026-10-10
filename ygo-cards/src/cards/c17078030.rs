//! Wall of Revealing Light
use super::prelude::*;

pub const ID: u32 = 17078030;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    let e1 = effect::create_effect(d, c);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_cost(d, e1, Some(cost_target(cost)));
    card::register_effect(d, c, e1, false);
    // cannot attack
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
    effect::set_range(d, e2, LOCATION_SZONE);
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    effect::set_target(d, e2, Some(effect_card_fn(atktarget)));
    card::register_effect(d, c, e2, false);
    // e1:SetLabelObject(e2)
    effect::set_label_object(d, e1, Value::Effect(e2));
}
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // for _,eff in pairs({Duel.IsPlayerAffectedByEffect(tp,EFFECT_LPCOST_CHANGE)}) do
        if let Some(eff) = duel::is_player_affected_by_effect(d, a.tp, EFFECT_LPCOST_CHANGE) {
            // local val=eff:GetValue()
            let val = effect::get_value(d, eff);
            // if (type(val)=='integer' and val==0) or (type(val)=='function' and (val(eff,e,tp,1000)~=1000)) then
            let stop = match &val {
                Value::Int(v) => *v == 0,
                Value::Fn(f) => f.call(d, &[Value::Effect(eff), Value::Effect(a.e), Value::Int(a.tp as i64), Value::Int(1000)]).int() != 1000,
                _ => false,
            };
            if stop {
                // return false
                return false;
            }
        }
        // return Duel.CheckLPCost(tp,1000)
        return duel::check_lp_cost(d, a.tp, 1000);
    }
    // local lp=Duel.GetLP(tp)
    let lp = duel::get_lp(d, a.tp);
    // local t={}
    // for i=1,math.floor((lp)/1000) do t[i]=i*1000 end
    let mut t: Vec<u64> = Vec::new();
    for i in 1..=(lp / 1000) {
        t.push((i * 1000) as u64);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // local announce=Duel.AnnounceNumber(tp,table.unpack(t))
    let (announce, _) = duel::announce_number(d, a.tp, &t).nil();
    // Duel.PayLPCost(tp,announce)
    duel::pay_lp_cost(d, a.tp, announce as u32);
    // e:GetLabelObject():SetLabel(announce)
    let label_obj = effect::get_label_object(d, a.e).as_effect().nil();
    effect::set_label(d, label_obj, &[announce as i64]);
    // e:GetHandler():SetHint(CHINT_NUMBER,announce)
    let h = effect::get_handler(d, a.e).nil();
    card::set_hint(d, h, CHINT_NUMBER as u8, announce);
    true
}
fn atktarget(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetAttack()<=e:GetLabel()
    (card::get_attack(d, c) as i64) <= effect::get_label(d, e)
}
