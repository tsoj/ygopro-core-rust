//! Dinovatus Docus
use super::prelude::*;

pub const ID: u32 = 84631951;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can only Special Summon "Dinovader Docus" once per turn
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Special Summon itself from the hand
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
    // e1:SetCondition(function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_DINOSAUR),tp,LOCATION_MZONE,0,2,nil) end)
    effect::set_condition(d, e1, Some(lua_fn(selfspcond)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Dinosaur monster from your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.dspcost)
    effect::set_cost(d, e2, Some(cost_target(dspcost)));
    // e2:SetTarget(s.dsptg)
    effect::set_target(d, e2, Some(cost_target(dsptg)));
    // e2:SetOperation(s.dspop)
    effect::set_operation(d, e2, Some(operation(dspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp) return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_DINOSAUR),tp,LOCATION_MZONE,0,2,nil) end`
fn selfspcond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_DINOSAUR)), tp, LOCATION_MZONE, 0, 2, NO_EX)
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c.nil(), a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c.nil())), 1, a.tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.dspcostfilter(c,e,tp)`
fn dspcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingMatchingCard(s.dspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetLevel())
    if !card::is_level_below(d, c, 4) {
        return false;
    }
    if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 <= 0 {
        return false;
    }
    let lv = card::get_level(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| dspfilter(d, x, e, tp, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.dspfilter(c,e,tp,lv)`
fn dspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: u32) -> bool {
    // return c:IsRace(RACE_DINOSAUR) and c:IsLevel(lv-2,lv+2) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_DINOSAUR)
        && card::is_level(d, c, &[lv.wrapping_sub(2), lv.wrapping_add(2)])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.dspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.dspcostfilter,1,false,nil,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| dspcostfilter(d, c, a.e, a.tp)), 1, 1, false, None, NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.dspcostfilter,1,1,false,nil,nil,e,tp)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| dspcostfilter(d, c, a.e, a.tp)), 1, 1, false, None, NO_EX);
    // e:SetLabel(g:GetFirst():GetLevel())
    let first = group::get_first(d, &g).nil();
    let lv = card::get_level(d, first);
    effect::set_label(d, a.e, &[lv as i64]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.dsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn dsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.destroy_dg`: `function(dg) Duel.Destroy(dg,REASON_EFFECT) end`
fn destroy_dg(d: &mut Duel, dg: &CardSet, _a: &EffArgs) -> bool {
    duel::destroy(d, dg, REASON_EFFECT, None, None);
    true
}
/// `s.dspop(e,tp,eg,ep,ev,re,r,rp)`
fn dspop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.dspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,e:GetLabel()):GetFirst()
    let lv = effect::get_label(d, a.e) as u32;
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, c| dspfilter(d, c, a.e, a.tp, lv)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // Destroy it during the End Phase
            // aux.DelayedOperation(tc,PHASE_END,id,e,tp,function(dg) Duel.Destroy(dg,REASON_EFFECT) end,nil,0,nil,aux.Stringid(id,2))
            aux::delayed_operation(
                d,
                tc,
                PHASE_END,
                ID,
                a.e,
                a.tp,
                Some(std::sync::Arc::new(destroy_dg)),
                None,
                Some(0),
                None,
                Some(aux::stringid(ID, 2)),
                None,
            );
        }
    }
}
