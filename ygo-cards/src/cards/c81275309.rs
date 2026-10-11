//! General Raiho of the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 81275309;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // s[0]=0
    set_s_var(d, ID, 0, Value::Int(0));
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.handes)
    effect::set_operation(d, e1, Some(operation(handes)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.handes(e,tp,eg,ep,ev,re,r,rp)`
fn handes(d: &mut Duel, a: &EffArgs) {
    // local trig_loc,chain_id=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION,CHAININFO_CHAIN_ID)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_CHAIN_ID]);
    let trig_loc = info[0].int();
    let chain_id = info[1].int();
    // if not (ep==1-tp and trig_loc==LOCATION_MZONE and chain_id~=s[0] and re:IsMonsterEffect()) then return end
    if !(a.ep == 1 - a.tp
        && trig_loc == LOCATION_MZONE as i64
        && chain_id != s_var(d, ID, 0).int()
        && effect::is_monster_effect(d, a.re.nil()))
    {
        return;
    }
    // s[0]=chain_id
    set_s_var(d, ID, 0, Value::Int(chain_id));
    // if Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)>0 and Duel.SelectYesNo(1-tp,aux.Stringid(id,0)) then
    if duel::get_field_group_count(d, a.tp, 0, LOCATION_HAND) > 0
        && duel::select_yes_no(d, 1 - a.tp, aux::stringid(ID, 0))
    {
        // Duel.DiscardHand(1-tp,nil,1,1,REASON_EFFECT|REASON_DISCARD,nil)
        duel::discard_hand(d, 1 - a.tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
        // Duel.BreakEffect()
        duel::break_effect(d);
    // else Duel.NegateEffect(ev) end
    } else {
        duel::negate_effect(d, a.ev as u8);
    }
}
