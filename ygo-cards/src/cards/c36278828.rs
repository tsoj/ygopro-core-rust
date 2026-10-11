//! Venom Serpent
use super::prelude::*;

pub const ID: u32 = 36278828;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[COUNTER_VENOM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // add counter
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsCanAddCounter(COUNTER_VENOM,1) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_can_add_counter(d, chkc, COUNTER_VENOM as u16, 1, false, 0);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsCanAddCounter,tp,0,LOCATION_MZONE,1,nil,COUNTER_VENOM,1) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_VENOM as u16, 1, false, 0)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,Card.IsCanAddCounter,tp,0,LOCATION_MZONE,1,1,nil,COUNTER_VENOM,1)
    let _g = duel::select_target(d, tp, Some(&|d, c| card::is_can_add_counter(d, c, COUNTER_VENOM as u16, 1, false, 0)), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, None, 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsCanAddCounter(COUNTER_VENOM,1) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_can_add_counter(d, tc, COUNTER_VENOM as u16, 1, false, 0) {
        // local atk=tc:GetAttack()
        let atk = card::get_attack(d, tc);
        // tc:AddCounter(COUNTER_VENOM,1)
        card::add_counter(d, tc, COUNTER_VENOM as u16, 1, false);
        // if atk>0 and tc:GetAttack()==0 then
        if atk > 0 && card::get_attack(d, tc) == 0 {
            // Duel.RaiseEvent(tc,EVENT_CUSTOM+54306223,e,0,0,0,0)
            duel::raise_event(d, CardSet::from_card(tc), EVENT_CUSTOM + 54306223, Some(a.e), 0, 0, 0, 0);
        }
    }
}
