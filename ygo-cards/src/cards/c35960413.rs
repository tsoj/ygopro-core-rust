//! Cupid Fore
use super::prelude::*;

pub const ID: u32 = 35960413;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the hand if all monsters have 600 DEF
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change level of a LIGHT Fairy monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e2, CATEGORY_LVCHANGE);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some((TIMINGS_CHECK_MONSTER | TIMING_MAIN_END) as u32));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.lvcst)
    effect::set_cost(d, e2, Some(cost_target(lvcst)));
    // e2:SetTarget(s.lvtg)
    effect::set_target(d, e2, Some(cost_target(lvtg)));
    // e2:SetOperation(s.lvop)
    effect::set_operation(d, e2, Some(operation(lvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetDefense()==600
    card::is_faceup(d, c) && card::get_defense(d, c) == 600
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    // return #g>0 and g:FilterCount(s.cfilter,nil)==#g
    !g.is_empty() && group::filter_count(d, &g, &cfilter, NO_EX) == g.len()
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.lvcst(e,tp,eg,ep,ev,re,r,rp,chk)
fn lvcst(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST|REASON_DISCARD,nil)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST | REASON_DISCARD, NO_EX);
    true
}
// function s.filter1(c,tp)
fn filter1(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_FAIRY) and c:HasLevel()
    //     and Duel.IsExistingTarget(s.lvfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,c,c:GetLevel())
    if !(card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_FAIRY) && card::has_level(d, c)) {
        return false;
    }
    let lv = card::get_level(d, c);
    duel::is_existing_target(d, Some(&|d, x| lvfilter(d, x, lv)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into())
}
// function s.lvfilter(c,lv)
fn lvfilter(d: &mut Duel, c: CardId, lv: u32) -> bool {
    // return c:IsMonster() and c:IsFaceup() and c:HasLevel() and c:GetLevel()~=lv
    card::is_monster(d, c) && card::is_faceup(d, c) && card::has_level(d, c) && card::get_level(d, c) != lv
}
fn lvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local g1=Duel.SelectTarget(tp,s.filter1,tp,LOCATION_MZONE,0,1,1,nil,tp)
    let g1 = duel::select_target(d, tp, Some(&|d, c| filter1(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    let tc1 = group::get_first(d, &g1).nil();
    let lv1 = card::get_level(d, tc1);
    // local g2=Duel.SelectTarget(tp,s.lvfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,g1:GetFirst(),g1:GetFirst():GetLevel())
    let _g2 = duel::select_target(d, tp, Some(&|d, c| lvfilter(d, c, lv1)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, tc1.into());
    // e:SetLabelObject(g1:GetFirst())
    effect::set_label_object(d, a.e, Value::Card(tc1));
    // Duel.SetOperationInfo(0,CATEGORY_LVCHANGE,g1:GetFirst(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LVCHANGE, Some(CardSet::from_card(tc1)), 1, 0, 0);
    true
}
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c1=e:GetLabelObject()
    let c1 = effect::get_label_object(d, a.e).as_card().nil();
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS])[0].as_group().cloned().nil();
    // local tc=g:GetFirst()
    let list = group::iter(d, &g);
    let mut tc = list.first().copied();
    // if tc==c1 then tc=g:GetNext() end
    if tc == Some(c1) {
        tc = list.get(1).copied();
    }
    // if not c1:HasLevel() or c1:IsFacedown() or not c1:IsRelateToEffect(e) or not c1:IsControler(tp) then return end
    if !card::has_level(d, c1) || card::is_facedown(d, c1) || !card::is_relate_to_effect(d, c1, a.e) || !card::is_controler(d, c1, a.tp) {
        return;
    }
    let tc = tc.nil();
    // if tc:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
        // e1:SetValue(tc:GetLevel())
        let lv = card::get_level(d, tc) as i64;
        effect::set_value(d, e1, lv);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // c1:RegisterEffect(e1)
        card::register_effect(d, c1, e1, false);
    }
}
