//! Stamping Destruction
use super::prelude::*;

pub const ID: u32 = 81385346;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_DRAGON),tp,LOCATION_MZONE,0,1,nil)
    let f = aux::faceup_filter(|d, c| card::is_race(d, c, RACE_DRAGON));
    duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and s.filter2(chkc) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_on_field(d, chkc) && filter2(d, chkc) && chkc != h;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter2,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter2), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,e:GetHandler())
    let g = duel::select_target(d, a.tp, Some(&filter2), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, h.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // local p=tc:GetControler()
            let p = card::get_controler(d, tc);
            // if Duel.Destroy(tc,REASON_EFFECT)==0 then return end
            if duel::destroy(d, tc, REASON_EFFECT, None, None) == 0 {
                return;
            }
            // Duel.Damage(p,500,REASON_EFFECT)
            duel::damage(d, p, 500, REASON_EFFECT, false, None);
        }
    }
}
