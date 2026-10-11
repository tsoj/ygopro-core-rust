//! Dance Princess of the Ice Barrier
//! 氷結界の舞姫
use super::prelude::*;

pub const ID: u32 = 59546528;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ICE_BARRIER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ICE_BARRIER)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, h.into())
}
// function s.cfilter2(c)
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ICE_BARRIER) and c:IsMonster() and not c:IsPublic()
    card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])
        && card::is_monster(d, c)
        && !card::is_public(d, c)
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsAbleToHand()
    card::is_facedown(d, c) && card::is_able_to_hand(d, c, None)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter2,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&cfilter2),
            a.tp,
            LOCATION_HAND,
            0,
            1,
            NO_EX,
        );
    }
    // local ct=Duel.GetTargetCount(s.filter,tp,0,LOCATION_SZONE,nil)
    let ct = duel::get_target_count(d, Some(&filter), a.tp, 0, LOCATION_SZONE, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.cfilter2,tp,LOCATION_HAND,0,1,ct,nil)
    let cg = duel::select_matching_card(
        d,
        a.tp,
        Some(&cfilter2),
        a.tp,
        LOCATION_HAND,
        0,
        1,
        ct as u16,
        false,
        NO_EX,
    )
    .nil();
    // Duel.ConfirmCards(1-tp,cg)
    duel::confirm_cards(d, 1 - a.tp, &cg);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // e:SetLabel(#cg)
    effect::set_label(d, a.e, &[cg.len() as i64]);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE)
            && card::is_controler(d, chkc, 1 - a.tp)
            && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_SZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), a.tp, 0, LOCATION_SZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_SZONE,e:GetLabel(),e:GetLabel(),nil)
    let lbl = effect::get_label(d, a.e) as u16;
    let g = duel::select_target(
        d,
        a.tp,
        Some(&filter),
        a.tp,
        0,
        LOCATION_SZONE,
        lbl,
        lbl,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
    let n = g.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, n, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp,chk)
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
