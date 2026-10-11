//! Gladiator Taming
use super::prelude::*;

pub const ID: u32 = 9780364;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GLADIATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_GLADIATOR),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_GLADIATOR as u16]))), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.filter(c,e)`
fn filter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsFaceup() and c:IsCanBeEffectTarget(e) and c:IsCanChangePosition()
    card::is_faceup(d, c) && card::is_can_be_effect_target(d, c, Some(e)) && card::is_can_change_position(d, c)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GLADIATOR) and c:IsAbleToChangeControler()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_GLADIATOR as u16]) && card::is_able_to_change_controler(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - a.tp) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| card::is_faceup(d, x)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,0,LOCATION_MZONE,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, x| filter(d, x, a.e)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local cg=Duel.GetMatchingGroup(s.filter2,tp,0,LOCATION_MZONE,nil)
    let cg = duel::get_matching_group(d, Some(&filter2), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EFFECT)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EFFECT as u64);
    // local sel=0
    // if #cg==0 then sel=Duel.SelectOption(tp,aux.Stringid(id,0))
    // else sel=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1)) end
    let sel: i32 = if cg.len() == 0 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0)])
    } else {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)])
    };
    // if sel==0 then
    if sel == 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_POSCHANGE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_POSCHANGE as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(sg)
        duel::set_target_card(d, sg);
        // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g.clone()), 1, 0, 0);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
        // local sg=cg:Select(tp,1,1,nil)
        let sg = group::select(d, &cg, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(sg)
        duel::set_target_card(d, sg);
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(g.clone()), 1, 0, 0);
    }
    // e:SetLabel(sel)
    effect::set_label(d, a.e, &[sel as i64]);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // if e:GetLabel()==0 then
            if effect::get_label(d, a.e) == 0 {
                // Duel.ChangePosition(tc,POS_FACEUP_DEFENSE,0,POS_FACEUP_ATTACK,0)
                duel::change_position(d, CardSet::from_card(tc), POS_FACEUP_DEFENSE as u8, Some(0), Some(POS_FACEUP_ATTACK as u8), Some(0), false);
            } else {
                // Duel.GetControl(tc,tp,PHASE_END,1)
                duel::get_control(d, CardSet::from_card(tc), a.tp, PHASE_END as u16, 1, None, None);
            }
        }
    }
}
