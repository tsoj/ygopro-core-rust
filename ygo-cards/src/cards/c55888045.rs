//! CNo.106 溶岩掌ジャイアント・ハンド・レッド
//! Number C106: Giant Red Hand
use super::prelude::*;

pub const ID: u32 = 55888045;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NUMBER],
    xyz_number: Some(106),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure
    // Xyz.AddProcedure(c,nil,5,3)
    xyz::add_procedure(d, c, None, Some(5), 3, None, None, None, None, false, None);
    // Negate the effects of all face-up cards on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e1, CATEGORY_NEGATE);
    // e1:SetType(EFFECT_TYPE_QUICK_F)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_F);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION]);
    let loc = info[0].int() as u32;
    let h = effect::get_handler(d, a.e).nil();
    // return (loc&LOCATION_ONFIELD)~=0 and not e:GetHandler():IsStatus(STATUS_CHAINING)
    //     and e:GetHandler():GetOverlayGroup():IsExists(Card.IsSetCard,1,nil,SET_NUMBER)
    (loc & LOCATION_ONFIELD) != 0
        && !card::is_status(d, h, STATUS_CHAINING)
        && {
            let og = card::get_overlay_group(d, h);
            group::is_exists(d, &og, &|d, x| card::is_set_card(d, x, &[SET_NUMBER as u16]), 1, NO_EX)
        }
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.GetCurrentChain()~=ev+1 or c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)<1 then return end
    if !card::is_relate_to_effect(d, c, a.e)
        || duel::get_current_chain(d, false) != a.ev as i32 + 1
        || card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT) < 1
    {
        return;
    }
    // local g=Duel.GetMatchingGroup(Card.IsNegatable,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_negatable(d, x)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // g:ForEach(function(tc) tc:NegateEffects(c,RESET_PHASE|PHASE_END,true) end)
    for tc in group::iter(d, &g) {
        card::negate_effects(d, tc, c, Some(RESET_PHASE | PHASE_END), true, None);
    }
}
