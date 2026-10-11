//! Summoner of the Sacred Beasts
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 22734799;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SACRED_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can reveal this card in your hand; discard 1 card, and if you do, Special Summon 1 "Sacred Beast" monster from your hand in Defense Position, except a Level 8 monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_HANDES+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(Cost.SelfReveal)
    effect::set_cost(d, e1, Some(cost::self_reveal()));
    // e1:SetTarget(s.sptg(LOCATION_HAND))
    effect::set_target(d, e1, Some(sptg(LOCATION_HAND)));
    // e1:SetOperation(s.spop(LOCATION_HAND))
    effect::set_operation(d, e1, Some(spop(LOCATION_HAND)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can discard 1 card; Special Summon 1 "Sacred Beast" monster from your hand or GY in Defense Position, except a Level 8 monster or the discarded card
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.Discard(nil,false,1,1,function(e,tp,og) local oc=og:GetFirst() oc:CreateEffectRelation(e) e:SetLabelObject(oc) end))
    effect::set_cost(
        d,
        e2,
        Some(cost::discard(
            None,
            false,
            Some(cost::Count::N(1)),
            Some(cost::Count::N(1)),
            Some(Arc::new(|d: &mut Duel, e: EffectId, _tp: u8, og: &CardSet| {
                let oc = group::get_first(d, og).nil();
                card::create_effect_relation(d, oc, e);
                effect::set_label_object(d, e, Value::Card(oc));
            })),
        )),
    );
    // e2:SetTarget(s.sptg(LOCATION_HAND|LOCATION_GRAVE))
    effect::set_target(d, e2, Some(sptg(LOCATION_HAND | LOCATION_GRAVE)));
    // e2:SetOperation(s.spop(LOCATION_HAND|LOCATION_GRAVE))
    effect::set_operation(d, e2, Some(spop(LOCATION_HAND | LOCATION_GRAVE)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // You can banish this card from your GY; Special Summon 1 "Sacred Beast" monster from your GY in Defense Postion, except a Level 8 monster
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.sptg(LOCATION_GRAVE))
    effect::set_target(d, e3, Some(sptg(LOCATION_GRAVE)));
    // e3:SetOperation(s.spop(LOCATION_GRAVE))
    effect::set_operation(d, e3, Some(spop(LOCATION_GRAVE)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.discardfilter(c,e,tp)`
fn discardfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsDiscardable(REASON_EFFECT) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,c,e,tp)
    card::is_discardable(d, c, Some(REASON_EFFECT), None) && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_HAND, 0, 1, c.into())
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_SACRED_BEAST) and not c:IsLevel(8) and c:IsCanBeSpecialSummoned(e,0,tp,false,true,POS_FACEUP_DEFENSE)
    card::is_set_card(d, c, &[SET_SACRED_BEAST as u16])
        && !card::is_level(d, c, &[8])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, true, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(summon_location)`: the returned `function(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(summon_location: u32) -> LuaFn {
    cost_target(move |d, a| sptg_body(d, a, summon_location))
}
fn sptg_body(d: &mut Duel, a: &EffArgs, summon_location: u32) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return false;
        }
        if summon_location == LOCATION_HAND {
            // return Duel.IsExistingMatchingCard(s.discardfilter,tp,LOCATION_HAND,0,1,nil,e,tp)
            return duel::is_existing_matching_card(d, Some(&|d, c| discardfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
        } else {
            // return Duel.IsExistingMatchingCard(s.spfilter,tp,summon_location,0,1,e:GetHandler(),e,tp)
            let h = effect::get_handler(d, e).nil();
            return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, summon_location, 0, 1, h.into());
        }
    }
    // if summon_location==LOCATION_HAND then
    if summon_location == LOCATION_HAND {
        // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,summon_location)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, summon_location as i32);
    true
}
/// `s.spop(summon_location)`: the returned `function(e,tp,eg,ep,ev,re,r,rp)`
fn spop(summon_location: u32) -> LuaFn {
    operation(move |d, a| spop_body(d, a, summon_location))
}
fn spop_body(d: &mut Duel, a: &EffArgs, summon_location: u32) {
    let (e, tp) = (a.e, a.tp);
    // if summon_location==LOCATION_HAND and Duel.DiscardHand(tp,s.discardfilter,1,1,REASON_EFFECT|REASON_DISCARD,nil,e,tp)<0 then return end
    if summon_location == LOCATION_HAND
        && duel::discard_hand(d, tp, Some(&|d, c| discardfilter(d, c, e, tp)), 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX) < 0
    {
        return;
    }
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local exc=nil
    let mut exc: Option<CardId> = None;
    // if summon_location==LOCATION_HAND|LOCATION_GRAVE then
    if summon_location == LOCATION_HAND | LOCATION_GRAVE {
        // local cost_card=e:GetLabelObject()
        let cost_card = effect::get_label_object(d, e).as_card().nil();
        // exc=cost_card:IsRelateToEffect(e) and cost_card or nil
        if card::is_relate_to_effect(d, cost_card, e) {
            exc = Some(cost_card);
        }
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,summon_location,0,1,1,exc,e,tp):GetFirst()
    let f = aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp));
    let ex = match exc {
        Some(x) => x.into(),
        None => NO_EX,
    };
    let sg = duel::select_matching_card(d, tp, Some(&f), tp, summon_location, 0, 1, 1, false, ex).nil();
    if let Some(sc) = group::get_first(d, &sg) {
        // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,true,POS_FACEUP_DEFENSE)>0 then
        if duel::special_summon(d, sc, 0, tp, tp, false, true, POS_FACEUP_DEFENSE as u8, None) > 0 {
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
        }
    }
}
