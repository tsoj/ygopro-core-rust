//! Oracle of the Herald
use super::prelude::*;

pub const ID: u32 = 79306385;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Ritual.AddProcGreaterCode(c,12,nil,48546368)
    let e1 = ritual::add_proc_greater_code(d, c, Some(ritual::Lv::N(12)), None, &[48546368]);
    // e1:SetTarget(s.target(e1))
    let tg = effect::get_target(d, e1);
    effect::set_target(d, e1, Some(lua_fn(move |d: &mut Duel, a: Args| target_wrap(d, a, tg.clone()))));
}
/// `s.target(eff)` (the project's override, core_ext/lua/cards/c79306385.lua: the chain limit only
/// when the target runs for real, chk~=0): the Ritual target, then the chain limit for Activate
fn target_wrap(d: &mut Duel, a: Args, tg: Option<LuaFn>) -> Value {
    // local ret = tg(e,tp,eg,ep,ev,re,r,rp,chk,...)
    let ret = tg.nil().call(d, a.0);
    // if chk==0 then return ret end
    if a.int(8) == 0 {
        return ret;
    }
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) then Duel.SetChainLimit(s.chlimit) end
    let e = a.e(0);
    if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        duel::set_chain_limit(d, lua_fn(chlimit));
    }
    // return ret
    ret
}
/// `s.chlimit(e,ep,tp)`
fn chlimit(_d: &mut Duel, a: Args) -> bool {
    // return tp==ep
    a.p(2) == a.p(1)
}
