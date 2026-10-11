//! Putrid Pudding Body Buddies
//! プリン隊
use super::prelude::*;

pub const ID: u32 = 85101097;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot release
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UNRELEASABLE_SUM)
    effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UNRELEASABLE_NONSUM)
    effect::set_code(d, e2, EFFECT_UNRELEASABLE_NONSUM);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EFFECT_CANNOT_BE_FUSION_MATERIAL)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_FUSION_MATERIAL);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
    effect::set_code(d, e4, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    // e5:SetCode(EFFECT_CANNOT_BE_XYZ_MATERIAL)
    effect::set_code(d, e5, EFFECT_CANNOT_BE_XYZ_MATERIAL);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // control
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e6, aux::stringid(ID, 0));
    // e6:SetCategory(CATEGORY_DESTROY+CATEGORY_CONTROL)
    effect::set_category(d, e6, CATEGORY_DESTROY + CATEGORY_CONTROL);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e6:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e6, EVENT_PHASE + PHASE_END);
    // e6:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_CARD_TARGET, 0);
    // e6:SetRange(LOCATION_MZONE)
    effect::set_range(d, e6, LOCATION_MZONE);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetCondition(s.ctlcon)
    effect::set_condition(d, e6, Some(condition(ctlcon)));
    // e6:SetTarget(s.ctltg)
    effect::set_target(d, e6, Some(cost_target(ctltg)));
    // e6:SetOperation(s.ctlop)
    effect::set_operation(d, e6, Some(operation(ctlop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // damage
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e7, aux::stringid(ID, 1));
    // e7:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e7:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e7, CATEGORY_DAMAGE);
    // e7:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e7, EVENT_PHASE | PHASE_STANDBY);
    // e7:SetRange(LOCATION_MZONE)
    effect::set_range(d, e7, LOCATION_MZONE);
    // e7:SetCountLimit(1)
    effect::set_count_limit(d, e7, 1, 0, 0, 0);
    // e7:SetCondition(s.damcon)
    effect::set_condition(d, e7, Some(condition(damcon)));
    // e7:SetTarget(s.damtg)
    effect::set_target(d, e7, Some(cost_target(damtg)));
    // e7:SetOperation(s.damop)
    effect::set_operation(d, e7, Some(operation(damop)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
}
/// `s.ctlcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctlcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.ctltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn ctltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let h = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_PZONE) and chkc:IsControler(tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_PZONE) && card::is_controler(d, chkc, tp);
    }
    // if chk==0 then return e:GetHandler():IsControlerCanBeChanged()
    //     and Duel.IsExistingTarget(nil,tp,LOCATION_PZONE,0,1,nil) end
    if a.chk == Some(0) {
        return card::is_controler_can_be_changed(d, h, false, None) && duel::is_existing_target(d, None, tp, LOCATION_PZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_PZONE,0,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_PZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,e:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.ctlop(e,tp,eg,ep,ev,re,r,rp)`
fn ctlop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)~=0
    //     and c:IsRelateToEffect(e) and c:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e)
            && duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) != 0
            && card::is_relate_to_effect(d, c, e)
            && card::is_faceup(d, c)
        {
            // Duel.GetControl(c,1-tp)
            duel::get_control(d, CardSet::from_card(c), 1 - tp, 0, 0, None, None);
        }
    }
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,0,0,tp,300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 300);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // Duel.Damage(tp,300,REASON_EFFECT)
    duel::damage(d, a.tp, 300, REASON_EFFECT, false, None);
}
