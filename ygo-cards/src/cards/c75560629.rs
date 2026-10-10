//! Flint
use super::prelude::*;

pub const ID: u32 = 75560629;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c,nil,s.filter,s.eqlimit)
    aux::add_equip_procedure(d, c, None, Some(aux::equip_filter_fn(|d, c, _e, _tp| filter(d, c))), Some(effect_card_fn(eqlimit)), None, None, None, None, None);
    // atkdown
    let e2 = effect::create_effect(d, c);
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    effect::set_condition(d, e2, Some(condition(flcon)));
    effect::set_value(d, e2, -300);
    card::register_effect(d, c, e2, false);
    // cannot attack
    let e3 = effect::create_effect(d, c);
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e3, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    effect::set_condition(d, e3, Some(condition(flcon)));
    card::register_effect(d, c, e3, false);
    // cannot change pos
    let e4 = effect::create_effect(d, c);
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    effect::set_code(d, e4, EFFECT_CANNOT_CHANGE_POSITION);
    effect::set_condition(d, e4, Some(condition(flcon)));
    card::register_effect(d, c, e4, false);
    // reequip
    let e6 = effect::create_effect(d, c);
    effect::set_description(d, e6, aux::stringid(ID, 0));
    effect::set_type(d, e6, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    effect::set_code(d, e6, EVENT_TO_GRAVE);
    effect::set_property(d, e6, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_condition(d, e6, Some(condition(eqcon)));
    effect::set_target(d, e6, Some(cost_target(target)));
    effect::set_operation(d, e6, Some(operation(op)));
    card::register_effect(d, c, e6, false);
    // local e7=e6:Clone()
    let e7 = effect::clone(d, e6);
    effect::set_code(d, e7, EVENT_REMOVE);
    card::register_effect(d, c, e7, false);
}
/// `function(e) local tc=e:GetHandler():GetEquipTarget() return tc:GetCode()~=83812099 or tc:IsDisabled() end`
fn flcon(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    let tc = card::get_equip_target(d, h).nil();
    card::get_code(d, tc).first().copied() != Some(83812099) || card::is_disabled(d, tc)
}
/// `c:GetEquipGroup():IsExists(Card.IsCode,1,ex,id)`
fn equip_has_other_flint(d: &mut Duel, c: CardId, ex: Option<CardId>) -> bool {
    let g = card::get_equip_group(d, c);
    group::is_exists(d, &g, &|d, x| card::is_code(d, x, &[ID]), 1, ex.into())
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetCode()~=83812099 or c:IsDisabled() or not c:GetEquipGroup():IsExists(Card.IsCode,1,e:GetHandler(),id)
    let h = effect::get_handler(d, e);
    card::get_code(d, c).first().copied() != Some(83812099) || card::is_disabled(d, c) || !equip_has_other_flint(d, c, h)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and (c:GetCode()~=83812099 or c:IsDisabled() or not c:GetEquipGroup():IsExists(Card.IsCode,1,nil,id))
    card::is_faceup(d, c) && (card::get_code(d, c).first().copied() != Some(83812099) || card::is_disabled(d, c) || !equip_has_other_flint(d, c, None))
}
/// `s.target`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let _g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e);
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, h.map(CardSet::from_card), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, h.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.operation`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) && card::is_faceup(d, tc.nil()) {
        // Duel.Equip(tp,c,tc)
        duel::equip(d, a.tp, c, tc.nil(), true, false);
    }
}
/// `s.eqcon`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetPreviousEquipTarget()
    let ec = card::get_previous_equip_target(d, c);
    // return c:IsReason(REASON_LOST_TARGET) and ec and ec:IsReason(REASON_DESTROY)
    card::is_reason(d, c, REASON_LOST_TARGET)
        && match ec {
            Some(ec) => card::is_reason(d, ec, REASON_DESTROY),
            None => false,
        }
}
