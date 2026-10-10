//! Danger! Nessie!
use super::prelude::*;

pub const ID: u32 = 70711847;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_DANGER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_HANDES+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_HANDES + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCost(s.spcost)
    effect::set_cost(d, e1, Some(cost_target(spcost)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // to hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_REMOVE)
    effect::set_code(d, e3, EVENT_REMOVE);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsPublic() and c:GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        return !card::is_public(d, c) && card::get_flag_effect(d, c, ID) == 0;
    }
    // c:RegisterFlagEffect(id,RESET_CHAIN,0,1)
    card::register_flag_effect(d, c, ID, RESET_CHAIN, 0, 1, 0, 0);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 1);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_HAND, 0);
    // if #g<1 then return end
    if g.len() < 1 {
        return;
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - a.tp, HINTMSG_DISCARD as u64);
    // local tc=g:RandomSelect(1-tp,1,1,nil)
    let tc = group::random_select(d, &g, 1 - a.tp, 1);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.SendtoGrave(tc,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, &tc, REASON_EFFECT | REASON_DISCARD, None, None);
    // if not Duel.IsPlayerCanSpecialSummon(tp) or Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if !duel::is_player_can_special_summon(d, a.tp, None) || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // if not tc:GetFirst():IsCode(id) then
    let first = group::get_first(d, &tc).nil();
    if !card::is_code(d, first, &[ID]) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let sc = duel::select_matching_card(d, a.tp, Some(&|d, x| spfilter(d, x, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #sc>0 and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) then
        if !sc.is_empty() && duel::special_summon(d, &sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) != 0 {
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, a.tp, 1, REASON_EFFECT);
        }
    }
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetPreviousLocation()==LOCATION_HAND and (r&REASON_DISCARD)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_previous_location(d, h) == LOCATION_HAND && (a.r & REASON_DISCARD) != 0
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_DANGER) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_DANGER as u16]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
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
