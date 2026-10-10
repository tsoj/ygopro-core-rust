//! Vampire Lady
use super::prelude::*;

pub const ID: u32 = 26495087;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Your opponent sends 1 card of the declared type from their Deck to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e1, EVENT_BATTLE_DAMAGE);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return ep==1-tp end)
    effect::set_condition(d, e1, Some(condition(|_d, a| a.ep == 1 - a.tp)));
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    effect::set_operation(d, e1, Some(operation(tgop)));
    card::register_effect(d, c, e1, false);
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_OPTION)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_OPTION as u64);
    // local op=Duel.SelectOption(tp,DECLTYPE_MONSTER,DECLTYPE_SPELL,DECLTYPE_TRAP)
    let op = duel::select_option(d, a.tp, true, &[DECLTYPE_MONSTER as u64, DECLTYPE_SPELL as u64, DECLTYPE_TRAP as u64]);
    // e:SetLabel(1<<op)
    effect::set_label(d, a.e, &[1i64 << op]);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgfilter(c,ty)`
fn tgfilter(d: &mut Duel, c: CardId, ty: u32) -> bool {
    // return c:IsType(ty) and c:IsAbleToGrave()
    card::is_type(d, c, ty) && card::is_able_to_grave(d, c, None)
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(1-tp,s.tgfilter,1-tp,LOCATION_DECK,0,1,1,nil,e:GetLabel())
    let label = effect::get_label(d, a.e);
    let g = duel::select_matching_card(d, 1 - a.tp, Some(&|d, c| tgfilter(d, c, label as u32)), 1 - a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
