//! Psychic Blade
use super::prelude::*;

pub const ID: u32 = 75539614;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Equip
    // local e1=aux.AddEquipProcedure(c,nil,nil,nil,s.cost)
    let e1 = aux::add_equip_procedure(d, c, None, None, None, Some(cost_target(cost)), None, None, None, None);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // Equipped monster gain ATK/DEF equal to the LP paid
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_value_fn(d, e2, effect_value(val));
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e3, EFFECT_UPDATE_DEFENSE);
    card::register_effect(d, c, e3, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // for _,te in ipairs({Duel.GetPlayerEffect(tp,EFFECT_LPCOST_CHANGE)}) do
        for te in duel::get_player_effect(d, a.tp, EFFECT_LPCOST_CHANGE) {
            // local value=te:GetValue()
            let Value::Fn(value) = effect::get_value(d, te) else {
                crate::lua_error("attempt to call a number value");
            };
            // if value(te,e,tp,100)~=100 then return false end
            let v = value.call(d, &[Value::Effect(te), Value::Effect(a.e), Value::Int(a.tp as i64), Value::Int(100)]);
            if v.int() != 100 {
                return false;
            }
        }
        // return Duel.CheckLPCost(tp,100)
        return duel::check_lp_cost(d, a.tp, 100);
    }
    // local lp=Duel.GetLP(tp)
    let lp = duel::get_lp(d, a.tp);
    // local m=math.floor(math.min(lp,2000)/100)
    let m = lp.min(2000).div_euclid(100);
    // local t={}
    // for i=1,m do t[i]=i*100 end
    let t: Vec<u64> = (1..=m).map(|i| (i * 100) as u64).collect();
    // local ac=Duel.AnnounceNumber(tp,table.unpack(t))
    let (ac, _) = duel::announce_number(d, a.tp, &t).nil();
    // Duel.PayLPCost(tp,ac)
    duel::pay_lp_cost(d, a.tp, ac as u32);
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,ac)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, ac as i64, 0);
    true
}
/// `s.val(e,c)`
fn val(d: &mut Duel, e: EffectId) -> i64 {
    // local ct=e:GetHandler():GetFlagEffectLabel(id)
    let h = effect::get_handler(d, e).nil();
    let ct = card::get_flag_effect_label(d, h, ID).first().copied();
    // if not ct then return 0 end
    // return ct
    ct.unwrap_or(0)
}
