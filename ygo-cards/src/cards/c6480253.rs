//! Wroughtweiler
use super::prelude::*;

pub const ID: u32 = 6480253;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_series={SET_ELEMENTAL_HERO}
    listed_series: &[SET_ELEMENTAL_HERO],
    // s.listed_names={CARD_POLYMERIZATION}
    listed_names: &[CARD_POLYMERIZATION],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Elemental HERO" card and 1 "Polymerization" from your GY to your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetCondition(function(e) return e:GetHandler():IsLocation(LOCATION_GRAVE) end)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function(e) return e:GetHandler():IsLocation(LOCATION_GRAVE) end
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_location(d, h, LOCATION_GRAVE)
}
// function s.thfilter(c,e)
fn thfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return (c:IsSetCard(SET_ELEMENTAL_HERO) or c:IsCode(CARD_POLYMERIZATION)) and c:IsAbleToHand() and c:IsCanBeEffectTarget(e)
    (card::is_set_card(d, c, &[SET_ELEMENTAL_HERO as u16]) || card::is_code(d, c, &[CARD_POLYMERIZATION]))
        && card::is_able_to_hand(d, c, None)
        && card::is_can_be_effect_target(d, c, Some(e))
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:IsExists(Card.IsSetCard,1,nil,SET_ELEMENTAL_HERO) and sg:IsExists(Card.IsCode,1,nil,CARD_POLYMERIZATION)
    let a = group::is_exists(d, sg, &|d, x| card::is_set_card(d, x, &[SET_ELEMENTAL_HERO as u16]), 1, NO_EX);
    let b = group::is_exists(d, sg, &|d, x| card::is_code(d, x, &[CARD_POLYMERIZATION]), 1, NO_EX);
    (a && b, false)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.thfilter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, x| thfilter(d, x, a.e)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_ATOHAND)
    let tg = aux::select_unselect_group(
        d,
        &g,
        Some(a.e),
        a.tp,
        Some(2),
        Some(2),
        Some(&rescon),
        1,
        a.tp,
        Some(HINTMSG_ATOHAND as u64),
        None,
        None,
        false,
    )
    .unwrap_or_default();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SetTargetCard(tg)
        duel::set_target_card(d, tg.clone());
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tg,2,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(tg), 2, a.tp, 0);
    }
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    // if tg and #tg>0 then
    if let Some(tg) = duel::get_target_cards(d, a.e) {
        if !tg.is_empty() {
            // Duel.SendtoHand(tg,nil,REASON_EFFECT)
            duel::send_to_hand(d, tg.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,tg)
            duel::confirm_cards(d, 1 - a.tp, tg);
        }
    }
}
