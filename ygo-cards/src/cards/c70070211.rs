//! Joruri-P.U.N.K. Nashiwari Surprise
//! scripted by Rundas
use super::prelude::*;

pub const ID: u32 = 70070211;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_PUNK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pop
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_DESTROY);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PUNK) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_PUNK as u16]) && card::is_faceup(d, c)
}
/// `s.tgfilter(c,punk)`
fn tgfilter(d: &mut Duel, c: CardId, punk: bool) -> bool {
    // return c:IsFacedown() or punk
    card::is_facedown(d, c) || punk
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local punk=Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_MZONE,0,1,nil)
    let punk = duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    // if chkc then return chkc:IsOnField() and s.tgfilter(chkc,punk) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && tgfilter(d, chkc, punk);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,0,LOCATION_ONFIELD,1,nil,punk) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, punk)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local tc=Duel.SelectTarget(tp,s.tgfilter,tp,0,LOCATION_ONFIELD,1,1,nil,punk)
    let tc = duel::select_target(d, a.tp, Some(&|d, c| tgfilter(d, c, punk)), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,1-tp,LOCATION_ONFIELD)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, tc, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) then Duel.Destroy(tc,REASON_EFFECT) end
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) {
            duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
        }
    }
}
