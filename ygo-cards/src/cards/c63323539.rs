//! Trojan Blast
use super::prelude::*;

pub const ID: u32 = 63323539;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_CONTROL_CHANGED);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(op)));
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // if chk==0 then return r==REASON_EFFECT and rp==1-tp and eg:IsExists(Card.IsControler,1,nil,1-tp) end
    if a.chk == Some(0) {
        return a.r == REASON_EFFECT
            && a.rp as i64 == 1 - a.tp as i64
            && group::is_exists(d, &eg, &|d, c| card::is_controler(d, c, 1 - a.tp), 1, NO_EX);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, eg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    true
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRelateToEffect(e) and c:IsControler(1-tp)
    card::is_relate_to_effect(d, c, e) && card::is_controler(d, c, 1 - tp)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=eg:FilterSelect(tp,s.filter,1,1,nil,e,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter_select(d, &eg, a.tp, &|d, c| filter(d, c, a.e, a.tp), 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    // if not tc then return end
    let Some(tc) = group::get_first(d, &g) else { return };
    // local atk=tc:GetAttack()
    let mut atk = card::get_attack(d, tc);
    // if atk<0 or tc:IsFacedown() then atk=0 end
    if atk < 0 || card::is_facedown(d, tc) {
        atk = 0;
    }
    // if Duel.Destroy(tc,REASON_EFFECT)~=0 then
    if duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) != 0 {
        // Duel.Damage(1-tp,atk,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, atk as i64, REASON_EFFECT, false, None);
    }
}
