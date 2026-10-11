//! Meltiel, Sage of the Sky
use super::prelude::*;

pub const ID: u32 = 49905576;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SANCTUARY_SKY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // recover&destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not re:IsHasType(EFFECT_TYPE_ACTIVATE) or not re:IsActiveType(TYPE_COUNTER) or not c:IsLocation(LOCATION_MZONE) or not c:IsFaceup() then return end
    if !effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)
        || !effect::is_active_type(d, a.re.nil(), TYPE_COUNTER)
        || !card::is_location(d, c, LOCATION_MZONE)
        || !card::is_faceup(d, c)
    {
        return;
    }
    // Duel.Recover(tp,1000,REASON_EFFECT)
    duel::recover(d, a.tp, 1000, REASON_EFFECT, false, None);
    // if not Duel.IsEnvironment(CARD_SANCTUARY_SKY) then return end
    if !duel::is_environment(d, CARD_SANCTUARY_SKY, None, None) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.TRUE,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, None, a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
