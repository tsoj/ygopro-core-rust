//! Hiita the Fire Charmer, Ablaze
use super::prelude::*;

pub const ID: u32 = 48815792;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,nil,2,2,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(2), Some(group_check(lcheck)), None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add to hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e3, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e3, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.lcheck(g,lc,sumtype,tp)`
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsAttribute,1,nil,ATTRIBUTE_FIRE,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_attribute_ctx(d, c, ATTRIBUTE_FIRE, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
/// `s.spfilter(c,e,tp,zone)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, zone: u32) -> bool {
    // return c:IsAttribute(ATTRIBUTE_FIRE) and (zone~=0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone))
    card::is_attribute(d, c, ATTRIBUTE_FIRE) && (zone != 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone)))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local lg=c:GetLinkedGroup()
    let _lg = card::get_linked_group(d, c);
    // local zone=c:GetFreeLinkedZone()&ZONES_MMZ
    let zone = card::get_free_linked_zone(d, c, None) & ZONES_MMZ;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(1-tp) and s.spfilter(chkc,e,tp,zone) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - a.tp) && spfilter(d, chkc, a.e, a.tp, zone);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,0,LOCATION_GRAVE,1,nil,e,tp,zone) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, a.e, a.tp, zone)), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,0,LOCATION_GRAVE,1,1,nil,e,tp,zone)
    let g = duel::select_target(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp, zone)), a.tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local zone=e:GetHandler():GetFreeLinkedZone()&ZONES_MMZ
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_free_linked_zone(d, h, None) & ZONES_MMZ;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) and zone~=0 then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && zone != 0 {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP,zone)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
        }
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_BATTLE|REASON_EFFECT) and rp==1-tp and c:IsLinkSummoned() and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT) && a.rp == 1 - a.tp && card::is_link_summoned(d, c) && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsDefenseBelow(1500) and c:IsMonster() and c:IsAttribute(ATTRIBUTE_FIRE) and c:IsAbleToHand()
    card::is_defense_below(d, c, 1500) && card::is_monster(d, c) && card::is_attribute(d, c, ATTRIBUTE_FIRE) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
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
