//! Ghostrick Angel of Mischief
use super::prelude::*;

pub const ID: u32 = 53334641;

// s.listed_series={SET_GHOSTRICK}
// s.listed_names={id}
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_GHOSTRICK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --xyz summon
    // Xyz.AddProcedure(c,nil,4,2,s.ovfilter,aux.Stringid(id,0))
    xyz::add_procedure(d, c, None, Some(4), 2, Some(xyz::alter_filter(ovfilter)), Some(aux::stringid(ID, 0)), None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --win
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EVENT_ADJUST)
    effect::set_code(d, e1, EVENT_ADJUST);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetOperation(s.winop)
    effect::set_operation(d, e1, Some(operation(winop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --search
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --material
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.mttg)
    effect::set_target(d, e3, Some(cost_target(mttg)));
    // e3:SetOperation(s.mtop)
    effect::set_operation(d, e3, Some(operation(mtop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.ovfilter(c,tp,xyzc)
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_GHOSTRICK,xyzc,SUMMON_TYPE_XYZ,tp) and c:IsType(TYPE_XYZ,xyzc,SUMMON_TYPE_XYZ,tp) and not c:IsSummonCode(xyzc,SUMMON_TYPE_XYZ,tp,id)
    card::is_faceup(d, c)
        && card::is_set_card_ctx(d, c, &[SET_GHOSTRICK as u16], Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && !card::is_summon_code(d, c, Some(xyzc), SUMMON_TYPE_XYZ as u64, tp, &[ID])
}
// function s.winop(e,tp,eg,ep,ev,re,r,rp)
fn winop(d: &mut Duel, a: &EffArgs) {
    // if e:GetHandler():GetOverlayCount()==10 then
    let h = effect::get_handler(d, a.e).nil();
    if card::get_overlay_count(d, h) == 10 {
        // Duel.Win(tp,WIN_REASON_GHOSTRICK_MISCHIEF)
        duel::win(d, a.tp, WIN_REASON_GHOSTRICK_MISCHIEF as u32);
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GHOSTRICK) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_GHOSTRICK as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, &g);
    }
}
// function s.mtfilter(c)
fn mtfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GHOSTRICK)
    card::is_set_card(d, c, &[SET_GHOSTRICK as u16])
}
// function s.mttg(e,tp,eg,ep,ev,re,r,rp,chk)
fn mttg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsType(TYPE_XYZ) and Duel.IsExistingMatchingCard(s.mtfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_type(d, h, TYPE_XYZ) && duel::is_existing_matching_card(d, Some(&mtfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    true
}
// function s.mtop(e,tp,eg,ep,ev,re,r,rp)
fn mtop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
    // local g=Duel.SelectMatchingCard(tp,s.mtfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&mtfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Overlay(c,g)
        duel::overlay(d, c, g, false);
    }
}
