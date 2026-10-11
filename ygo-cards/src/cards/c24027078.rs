//! Morgan, the Enchantress of Avalon
use super::prelude::*;

pub const ID: u32 = 24027078;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NOBLE_ARMS, SET_NOBLE_KNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // c:IsFaceup() and c:IsSetCard(SET_NOBLE_ARMS) and c:IsType(TYPE_EQUIP)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_NOBLE_ARMS as u16]) && card::is_type(d, c, TYPE_EQUIP)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep~=tp and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_NOBLE_KNIGHT),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_ONFIELD,0,1,nil)
    a.ep != a.tp
        && duel::is_existing_matching_card(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_NOBLE_KNIGHT as u16]))),
            a.tp,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
        && duel::is_existing_matching_card(d, Some(&filter2), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.filter2,tp,LOCATION_ONFIELD,0,nil)
    let g = duel::get_matching_group(d, Some(&filter2), a.tp, LOCATION_ONFIELD, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&filter2), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.Destroy(g,REASON_EFFECT)~=0 then
    if !g.is_empty() && duel::destroy(d, g, REASON_EFFECT, None, None) != 0 {
        // if Duel.NegateActivation(ev) and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsRelateToEffect(re) then
        if duel::negate_activation(d, a.ev as u8) {
            let re = a.re.nil();
            if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
                let rh = effect::get_handler(d, re).nil();
                if card::is_relate_to_effect(d, rh, re) {
                    // Duel.SendtoGrave(eg,REASON_EFFECT)
                    duel::send_to_grave(d, a.eg(), REASON_EFFECT, None, None);
                }
            }
        }
    }
}
