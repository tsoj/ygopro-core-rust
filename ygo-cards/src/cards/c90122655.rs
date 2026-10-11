//! Tenmataitei
use super::prelude::*;

pub const ID: u32 = 90122655;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // act limit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD | EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.chainop)
    effect::set_operation(d, e1, Some(operation(chainop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.chainop(e,tp,eg,ep,ev,re,r,rp)`
fn chainop(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // local loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // if re:IsMonsterEffect() and loc==LOCATION_MZONE and rc:IsNormalSummoned() then
    if effect::is_monster_effect(d, a.re.nil()) && loc == LOCATION_MZONE as i64 && card::is_normal_summoned(d, rc) {
        // Duel.SetChainLimit(s.chainlm)
        duel::set_chain_limit(d, lua_fn(chainlm));
    }
}
/// `s.chainlm(e,rp,tp)`
fn chainlm(_d: &mut Duel, a: Args) -> bool {
    // return tp==rp
    a.p(2) == a.p(1)
}
