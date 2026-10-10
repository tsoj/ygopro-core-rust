//! Caius the Shadow Monarch
use super::prelude::*;

pub const ID: u32 = 9748752;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // remove
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    effect::set_condition(d, e1, Some(condition(cond)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and tc:IsAbleToRemove() then
    if let Some(tc) = tc {
        if card::is_able_to_remove(d, tc, None, None, None) {
            // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,0,0)
            duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(tc)), 1, 0, 0);
            // if tc:IsFaceup() and tc:IsAttribute(ATTRIBUTE_DARK) then
            if card::is_faceup(d, tc) && card::is_attribute(d, tc, ATTRIBUTE_DARK) {
                // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1000)
                duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 1000);
            }
        }
    }
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
            // if tc:IsLocation(LOCATION_REMOVED) and tc:IsMonster() and tc:IsAttribute(ATTRIBUTE_DARK) then
            if card::is_location(d, tc, LOCATION_REMOVED) && card::is_monster(d, tc) && card::is_attribute(d, tc, ATTRIBUTE_DARK) {
                // Duel.Damage(1-tp,1000,REASON_EFFECT)
                duel::damage(d, 1 - tp, 1000, REASON_EFFECT, false, None);
            }
        }
    }
}
