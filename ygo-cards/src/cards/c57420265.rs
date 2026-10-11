//! Fishborg Harpooner
//! フィッシュボーグ－ハープナー
use super::prelude::*;

pub const ID: u32 = 57420265;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon and discard this card and 1 other WATER monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_HANDES)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_HANDES);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the effects of 1 Effect Monster your opponent controls
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcostfilter(c,e,tp,dc_chk,sp_chk)`
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, dc_chk: bool, sp_chk: bool) -> bool {
    // return c:IsAttribute(ATTRIBUTE_WATER) and not c:IsPublic()
    //     and ((sp_chk and c:IsDiscardable(REASON_EFFECT)) or (dc_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_attribute(d, c, ATTRIBUTE_WATER)
        && !card::is_public(d, c)
        && ((sp_chk && card::is_discardable(d, c, Some(REASON_EFFECT), None))
            || (dc_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local dc_chk=c:IsDiscardable(REASON_EFFECT)
    let dc_chk = card::is_discardable(d, c, Some(REASON_EFFECT), None);
    // local sp_chk=c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let sp_chk = card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    // if chk==0 then return (dc_chk or sp_chk) and not c:IsPublic()
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spcostfilter,tp,LOCATION_HAND,0,1,c,e,tp,dc_chk,sp_chk) end
    if a.chk == Some(0) {
        return (dc_chk || sp_chk)
            && !card::is_public(d, c)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spcostfilter(d, x, a.e, a.tp, dc_chk, sp_chk)), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcostfilter,tp,LOCATION_HAND,0,1,1,c,e,tp,dc_chk,sp_chk)
    let mut g = duel::select_matching_card(d, a.tp, Some(&|d, x| spcostfilter(d, x, a.e, a.tp, dc_chk, sp_chk)), a.tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, g.clone());
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g.clone()), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, Some(g), 1, 0, 0);
    true
}
/// `s.spfilter(c,e,tp,g)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, g: &CardSet) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and g:IsExists(Card.IsDiscardable,1,c,REASON_EFFECT)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && group::is_exists(d, g, &|d, x| card::is_discardable(d, x, Some(REASON_EFFECT), None), 1, c.into())
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g~=2 then return end
    if g.len() != 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sg=g:FilterSelect(tp,s.spfilter,1,1,nil,e,tp,g)
    let gc = g.clone();
    let sg = group::filter_select(d, &g, a.tp, &|d, x| spfilter(d, x, a.e, a.tp, &gc), 1, 1, false, NO_EX).nil();
    // if #sg==1 then
    if sg.len() == 1 {
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg.clone(), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // Duel.SendtoGrave(g:Sub(sg),REASON_EFFECT|REASON_DISCARD)
        let mut rest = g.clone();
        group::remove_card(&mut rest, &sg);
        duel::send_to_grave(d, rest, REASON_EFFECT | REASON_DISCARD, None, None);
    }
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLocation(LOCATION_GRAVE) and r==REASON_SYNCHRO and c:GetReasonCard():IsAttribute(ATTRIBUTE_WATER)
    if !card::is_location(d, c, LOCATION_GRAVE) || a.r != REASON_SYNCHRO {
        return false;
    }
    let rc = card::get_reason_card(d, c).nil();
    card::is_attribute(d, rc, ATTRIBUTE_WATER)
}
/// `s.disfilter(c)`
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsNegatableMonster() and c:IsType(TYPE_EFFECT)
    card::is_negatable_monster(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.disfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_NEGATE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.disfilter,tp,0,LOCATION_MZONE,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&disfilter), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // Duel.HintSelection(tc,true)
        duel::hint_selection(d, tc, true);
        // tc:NegateEffects(e:GetHandler(),RESET_PHASE|PHASE_END)
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, Some(RESET_PHASE | PHASE_END), false, None);
    }
}
