//! Ｒ．Ｂ．ネクスト・フェーズ
//! R.B. Next Phase
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 79859067;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_RB], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate the activation of a monster effect and destroy it, then gain 2000LP
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY+CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY + CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
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
    // return rp==1-tp and re:IsMonsterEffect() and Duel.IsChainNegatable(ev) and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_RB),tp,LOCATION_MZONE,0,1,nil)
    if a.rp as i64 != 1 - a.tp as i64 {
        return false;
    }
    if !effect::is_monster_effect(d, a.re.nil()) {
        return false;
    }
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, x| card::is_set_card(d, x, &[SET_RB as u16]))),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, tp, 0);
    // if rc:IsDestructable() and rc:IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g+eg,2,tp,0)
        let mut ge = g.clone();
        group::add_card(&mut ge, &a.eg.clone().nil());
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(ge), 2, tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,2000)
        duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, 2000);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_MZONE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, None, tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // if Duel.Destroy(g,REASON_EFFECT)>0 and Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re)
    //     and Duel.Destroy(eg,REASON_EFFECT)>0 then
    if duel::destroy(d, g, REASON_EFFECT, None, None) > 0 && duel::negate_activation(d, a.ev as u8) {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rc, a.re.nil()) && duel::destroy(d, a.eg(), REASON_EFFECT, None, None) > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Recover(tp,2000,REASON_EFFECT)
            duel::recover(d, tp, 2000, REASON_EFFECT, false, None);
        }
    }
}
