//! Ursarctic Megapolar
use super::prelude::*;

pub const ID: u32 = 28715905;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_URSARCTIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // Ursarctic.AddSpSummonQuickEffect(c,id)
    aux::ursarctic::add_sp_summon_quick_effect(d, c, ID);
    // Pop
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.popcon)
    effect::set_condition(d, e2, Some(condition(popcon)));
    // e2:SetTarget(s.poptg)
    effect::set_target(d, e2, Some(cost_target(poptg)));
    // e2:SetOperation(s.popop)
    effect::set_operation(d, e2, Some(operation(popop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_URSARCTIC)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_URSARCTIC as u16])
}
/// `s.pfilter(c)`
fn pfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsDestructable()
    card::is_spell_trap(d, c) && card::is_destructable(d, c, None)
}
/// `s.popcon(e,tp,eg,ep,ev,re,r,rp)`
fn popcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, h.into())
}
/// `s.poptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn poptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return s.pfilter(chkc) and chkc:IsLocation(LOCATION_ONFIELD) and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return pfilter(d, chkc) && card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.pfilter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&pfilter), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectTarget(tp,s.pfilter,tp,0,LOCATION_ONFIELD,1,1,nil)
    let tc = duel::select_target(d, a.tp, Some(&pfilter), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc, 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.popop(e,tp,eg,ep,ev,re,r,rp)`
fn popop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
