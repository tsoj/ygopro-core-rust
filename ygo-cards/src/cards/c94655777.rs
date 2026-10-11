//! Nebulus, Dragon Ruler of Mishaps
use super::prelude::*;

pub const ID: u32 = 94655777;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID, 30350202],
    listed_series: &[SET_DRAGON_RULER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Special Summon 2 of your banished "Dragon Ruler" monsters with different Attributes
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(DragonRuler.SelfDiscardCost(ATTRIBUTE_DARK))
    effect::set_cost(d, e1, Some(aux::dragon_ruler::self_discard_cost(ATTRIBUTE_DARK)));
    // e1:SetTarget(s.rmsptg)
    effect::set_target(d, e1, Some(cost_target(rmsptg)));
    // e1:SetOperation(s.rmspop)
    effect::set_operation(d, e1, Some(operation(rmspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Special Summon 1 "Eclepsis, Dragon Ruler of Woes" from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.gyspcost)
    effect::set_cost(d, e2, Some(cost_target(gyspcost)));
    // e2:SetTarget(s.gysptg)
    effect::set_target(d, e2, Some(cost_target(gysptg)));
    // e2:SetOperation(s.gyspop)
    effect::set_operation(d, e2, Some(operation(gyspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.rmspfilter(c,e,tp)
fn rmspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DRAGON_RULER) and c:IsFaceup() and c:IsCanBeEffectTarget(e) and not c:IsCode(id)
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DRAGON_RULER as u16])
        && card::is_faceup(d, c)
        && card::is_can_be_effect_target(d, c, Some(e))
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.rmsptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn rmsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.rmspfilter,tp,LOCATION_REMOVED,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| rmspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, NO_EX);
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>=2 and g:GetClassCount(Card.GetAttribute)>=2 end
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && group::get_class_count(d, &g, &|d, c| card::get_attribute(d, c) as i64) >= 2;
    }
    // local tg=aux.SelectUnselectGroup(g,e,tp,2,2,aux.dpcheck(Card.GetAttribute),1,tp,HINTMSG_SPSUMMON)
    let dp = aux::dpcheck(|d, c| card::get_attribute(d, c) as i64);
    let tg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&dp), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tg,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(tg), 2, a.tp, 0);
    true
}
// function s.rmspop(e,tp,eg,ep,ev,re,r,rp)
fn rmspop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // local tg=Duel.GetTargetCards(e)
    let mut tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and #tg>1 then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() && tg.len() > 1 {
        return;
    }
    // if #tg>ft then
    if tg.len() > ft as usize {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // tg=tg:Select(tp,ft,ft,nil)
        tg = group::select(d, &tg, a.tp, ft as u16, ft as u16, false, NO_EX).nil();
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in tg:Iter() do
    for tc in group::iter(d, &tg) {
        // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) {
            // --It cannot attack this turn
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3206)
            effect::set_description(d, e1, 3206);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.gyspcostfilter(c,e,tp)
fn gyspcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsAttribute(ATTRIBUTE_DARK) or c:IsRace(RACE_DRAGON)) and c:IsAbleToRemoveAsCost()
    //   and Duel.IsExistingTarget(s.gyspfilter,tp,LOCATION_GRAVE,0,1,c,e,tp)
    (card::is_attribute(d, c, ATTRIBUTE_DARK) || card::is_race(d, c, RACE_DRAGON))
        && card::is_able_to_remove_as_cost(d, c, None)
        && duel::is_existing_target(d, Some(&|d, cc| gyspfilter(d, cc, e, tp)), tp, LOCATION_GRAVE, 0, 1, c.into())
}
// function s.gyspfilter(c,e,tp)
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(30350202) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[30350202]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.gyspcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn gyspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemoveAsCost()
    //   and Duel.IsExistingMatchingCard(s.gyspcostfilter,tp,LOCATION_GRAVE,0,1,c,e,tp) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && duel::is_existing_matching_card(d, Some(&|d, cc| gyspcostfilter(d, cc, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.gyspcostfilter,tp,LOCATION_GRAVE,0,1,1,c,e,tp)
    let mut g = duel::select_matching_card(d, a.tp, Some(&|d, cc| gyspcostfilter(d, cc, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, c.into()).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
// function s.gysptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn gysptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.gyspfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && gyspfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.gyspfilter,tp,LOCATION_GRAVE,0,1,c,e,tp) end
    // (the Lua `c` here is not a local: it is the global nil)
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, cc| gyspfilter(d, cc, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.gyspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, cc| gyspfilter(d, cc, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, 0);
    true
}
// function s.gyspop(e,tp,eg,ep,ev,re,r,rp)
fn gyspop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
