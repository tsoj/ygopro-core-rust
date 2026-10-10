//! Danger!? Jackalope?
use super::prelude::*;

pub const ID: u32 = 43694650;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_DANGER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Discard 1 card, Special Summon itself and draw 1 card
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
    // Special Summon 1 "Danger" monster from the Deck
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon2)
    effect::set_condition(d, e2, Some(condition(spcon2)));
    // e2:SetTarget(s.sptg2)
    effect::set_target(d, e2, Some(cost_target(sptg2)));
    // e2:SetOperation(s.spop2)
    effect::set_operation(d, e2, Some(operation(spop2)));
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
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetFieldGroup(tp,LOCATION_HAND,0)
    let g = duel::get_field_group(d, tp, LOCATION_HAND, 0);
    // if #g<1 then return end
    if g.len() < 1 {
        return;
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_DISCARD as u64);
    // local tc=g:RandomSelect(1-tp,1)
    let tc = group::random_select(d, &g, 1 - tp, 1);
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.SendtoGrave(tc,REASON_EFFECT|REASON_DISCARD)
    duel::send_to_grave(d, tc.clone(), REASON_EFFECT | REASON_DISCARD, None, None);
    // if not Duel.IsPlayerCanSpecialSummon(tp) or Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if !duel::is_player_can_special_summon(d, tp, None) || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // if not tc:GetFirst():IsCode(id) then
    let first = group::get_first(d, &tc).nil();
    if !card::is_code(d, first, &[ID]) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
        let sc = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if #sc>0 and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 then
        if !sc.is_empty() && duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
    }
}
/// `s.spcon2(e,tp,eg,ep,ev,re,r,rp)`
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetPreviousLocation()==LOCATION_HAND and (r&REASON_DISCARD)~=0
    let h = effect::get_handler(d, a.e).nil();
    card::get_previous_location(d, h) == LOCATION_HAND && (a.r & REASON_DISCARD) != 0
}
/// `s.spfilter2(c,e,tp)`
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DANGER) and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DANGER as u16]) && !card::is_code(d, c, &[ID]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg2(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter2(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop2(e,tp,eg,ep,ev,re,r,rp)`
fn spop2(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let _c = effect::get_handler(d, e);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter2,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter2(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &g) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
