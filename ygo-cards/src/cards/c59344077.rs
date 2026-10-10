//! Magic Drain
use super::prelude::*;

pub const ID: u32 = 59344077;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CHAINING);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp~=tp and re:IsSpellEffect() and re:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsChainNegatable(ev)
    let re = a.re.nil();
    a.rp != a.tp && effect::is_spell_effect(d, re) && effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsChainDisablable(0) then
    if duel::is_chain_disablable(d, 0) {
        // local g=Duel.GetMatchingGroup(Card.IsSpell,tp,0,LOCATION_HAND,nil)
        let g = duel::get_matching_group(d, Some(&card::is_spell), a.tp, 0, LOCATION_HAND, NO_EX);
        // Duel.Hint(HINT_SELECTMSG,1-tp,aux.Stringid(id,0))
        duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, aux::stringid(ID, 0));
        // if #g>0 then sel=Duel.SelectOption(1-tp,1213,1214) else sel=Duel.SelectOption(1-tp,1214)+1 end
        let sel = if !g.is_empty() {
            duel::select_option(d, 1 - a.tp, true, &[1213, 1214])
        } else {
            duel::select_option(d, 1 - a.tp, true, &[1214]) + 1
        };
        // if sel==0 then
        if sel == 0 {
            // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_DISCARD)
            duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_DISCARD as u64);
            // local sg=g:Select(1-tp,1,1,nil)
            let sg = group::select(d, &g, 1 - a.tp, 1, 1, false, NO_EX).nil();
            // Duel.SendtoGrave(sg,REASON_EFFECT|REASON_DISCARD)
            duel::send_to_grave(d, sg, REASON_EFFECT | REASON_DISCARD, None, None);
            // Duel.NegateEffect(0)
            duel::negate_effect(d, 0);
            // return
            return;
        }
    }
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let h = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, h, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
