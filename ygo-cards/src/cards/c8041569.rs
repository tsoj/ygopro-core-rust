//! Laval Lakeside Lady
//! ラヴァル炎湖畔の淑女
use super::prelude::*;

pub const ID: u32 = 8041569;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetCost(s.descost)
    effect::set_cost(d, e1, Some(cost_target(descost)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetMatchingGroup(Card.IsSetCard,tp,LOCATION_GRAVE,0,nil,SET_LAVAL):GetClassCount(Card.GetCode)>=3
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_set_card(d, c, &[SET_LAVAL as u16])), a.tp, LOCATION_GRAVE, 0, NO_EX);
    group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LAVAL) and c:IsAbleToRemoveAsCost()
    card::is_set_card(d, c, &[SET_LAVAL as u16]) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    // local h=e:GetHandler() (the Lua reads it where it is used)
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,0) and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_GRAVE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let args = [
            Value::Effect(a.e),
            Value::Int(a.tp as i64),
            a.eg.clone().map_or(Value::Nil, Value::Group),
            Value::Int(a.ep as i64),
            Value::Int(a.ev as i64),
            Value::effect(a.re),
            Value::Int(a.r as i64),
            Value::Int(a.rp as i64),
            Value::Int(0),
        ];
        return aux::bfgcost().call(d, &args).truthy() && duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c)), a.tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_GRAVE,0,1,1,e:GetHandler())
    let mut g = duel::select_matching_card(d, a.tp, Some(&|d, c| cfilter(d, c)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, h.into()).nil();
    // g:AddCard(e:GetHandler())
    group::add_card(&mut g, &CardSet::from_card(h));
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and chkc:IsFacedown() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - a.tp) && card::is_facedown(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFacedown,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_facedown(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFacedown,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_facedown(d, c)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFacedown() and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_facedown(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
