//! Ascending Soul
use super::prelude::*;

pub const ID: u32 = 88301833;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // salvage
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOHAND);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRitualSummoned()
    card::is_ritual_summoned(d, c)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c), 1, NO_EX)
}
/// `s.thfilter(c,e,tp)`
fn thfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_GRAVE) and c:IsReason(REASON_RELEASE) and c:IsControler(tp)
    //     and c:IsAbleToHand() and c:IsCanBeEffectTarget(e)
    card::is_location(d, c, LOCATION_GRAVE)
        && card::is_reason(d, c, REASON_RELEASE)
        && card::is_controler(d, c, tp)
        && card::is_able_to_hand(d, c, None)
        && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let tc = group::get_first(d, &eg).nil();
    // local mat=tc:GetMaterial()
    let mat = card::get_material(d, tc);
    // if chkc then return mat:IsContains(chkc) and s.thfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&mat, chkc) && thfilter(d, chkc, e, tp);
    }
    // if chk==0 then return mat:IsExists(s.thfilter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &mat, &|d, c| thfilter(d, c, e, tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local g=mat:FilterSelect(tp,s.thfilter,1,1,nil,e,tp)
    let g = group::filter_select(d, &mat, tp, &|d, c| thfilter(d, c, e, tp), 1, 1, false, NO_EX).nil();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, e).nil();
    if !card::is_relate_to_effect(d, h, e) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,tc)
            duel::confirm_cards(d, 1 - tp, tc);
        }
    }
}
