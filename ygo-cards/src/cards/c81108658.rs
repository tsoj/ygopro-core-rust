//! Ursarctic Megabilis
use super::prelude::*;

pub const ID: u32 = 81108658;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_URSARCTIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // Ursarctic.AddSpSummonQuickEffect(c,id)
    aux::ursarctic::add_sp_summon_quick_effect(d, c, ID);
    // Banish
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.bancon)
    effect::set_condition(d, e2, Some(condition(bancon)));
    // e2:SetTarget(s.bantg)
    effect::set_target(d, e2, Some(cost_target(bantg)));
    // e2:SetOperation(s.banop)
    effect::set_operation(d, e2, Some(operation(banop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_URSARCTIC)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_URSARCTIC as u16])
}
// function s.bfilter(c)
fn bfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove()
    card::is_able_to_remove(d, c, None, None, None)
}
// function s.bancon(e,tp,eg,ep,ev,re,r,rp)
fn bancon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, h.into())
}
// function s.bantg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn bantg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return s.bfilter(chkc) and chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(1-tp) end
    if let Some(chkc) = a.chkc() {
        return bfilter(d, chkc) && card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.bfilter,tp,0,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&bfilter), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local tc=Duel.SelectTarget(tp,s.bfilter,tp,0,LOCATION_GRAVE,1,1,nil)
    let tc = duel::select_target(d, a.tp, Some(&bfilter), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, tc, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.banop(e,tp,eg,ep,ev,re,r,rp)
fn banop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
