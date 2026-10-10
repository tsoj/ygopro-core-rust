//! Gameciel, the Sea Turtle Kaiju
use super::prelude::*;

pub const ID: u32 = 55063751;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_KAIJU], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1,e2=aux.AddKaijuProcedure(c)
    aux::add_kaiju_procedure(d, c);
    // negate
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_NEGATE+CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_NEGATE + CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.negcon)
    effect::set_condition(d, e3, Some(condition(negcon)));
    // e3:SetCost(s.negcost)
    effect::set_cost(d, e3, Some(cost_target(negcost)));
    // e3:SetTarget(s.negtg)
    effect::set_target(d, e3, Some(cost_target(negtg)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if ep==tp or e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if a.ep == a.tp {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if card::is_status(d, h, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // return not re:GetHandler():IsCode(id) and Duel.IsChainNegatable(ev)
    let rc = effect::get_handler(d, a.re.nil()).nil();
    !card::is_code(d, rc, &[ID]) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.negcost(e,tp,...,chk)`
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsCanRemoveCounter(tp,1,1,COUNTER_KAIJU,2,REASON_COST) end
    if a.chk == Some(0) {
        return duel::is_can_remove_counter(d, a.tp, 1, 1, COUNTER_KAIJU as u16, 2, REASON_COST);
    }
    // Duel.RemoveCounter(tp,1,1,COUNTER_KAIJU,2,REASON_COST)
    duel::remove_counter(d, a.tp, 1, 1, COUNTER_KAIJU as u16, 2, REASON_COST);
    true
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // local relation=rc:IsRelateToEffect(re)
    let relation = card::is_relate_to_effect(d, rc, a.re.nil());
    // if chk==0 then return rc:IsAbleToRemove(tp) or (not relation and Duel.IsPlayerCanRemove(tp)) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, rc, Some(a.tp), None, None) || (!relation && duel::is_player_can_remove(d, a.tp, None));
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    if relation {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rc,1,rc:GetControler(),rc:GetLocation())
        let controler = card::get_controler(d, rc);
        let location = card::get_location(d, rc) as i32;
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, controler, location);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,0,rc:GetPreviousLocation())
        let previous = card::get_previous_location(d, rc) as i32;
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, 0, previous);
    }
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        card::is_relate_to_effect(d, rc, a.re.nil())
    } {
        // Duel.Remove(eg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, a.eg(), POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
