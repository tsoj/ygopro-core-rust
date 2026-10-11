//! Minimalian
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 21147203;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e1, LOCATION_HAND);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_cost(d, e1, Some(cost_target(selfspcost)));
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    effect::set_operation(d, e1, Some(operation(selfspop)));
    card::register_effect(d, c, e1, false);
    // Special Summon 1 monster from your Deck with the same original Type and Attribute that the banished monster had on the field, but 1 or 2 Levels lower than its original Level
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    effect::set_range(d, e2, LOCATION_MZONE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost_target(deckspcost)));
    effect::set_target(d, e2, Some(cost_target(decksptg)));
    effect::set_operation(d, e2, Some(operation(deckspop)));
    card::register_effect(d, c, e2, false);
}
/// `s.selfspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove_as_cost(d, x, None)), a.tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND,0,1,1,c)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| card::is_able_to_remove_as_cost(d, x, None)), a.tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
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
/// `s.deckspcostfilter(c,e,tp)`
fn deckspcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsAbleToRemoveAsCost() and c:IsLevelBelow(4) and c:IsFaceup() and Duel.GetMZoneCount(tp,c)>0
    //     and Duel.IsExistingMatchingCard(s.deckspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,c:GetOriginalRace(),c:GetOriginalAttribute(),c:GetOriginalLevel())
    if !card::is_able_to_remove_as_cost(d, c, None) || !card::is_level_below(d, c, 4) || !card::is_faceup(d, c) {
        return false;
    }
    if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 <= 0 {
        return false;
    }
    let rac = card::get_original_race(d, c);
    let att = card::get_original_attribute(d, c);
    let lvl = card::get_original_level(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| deckspfilter(d, x, e, tp, rac, att, lvl)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.deckspfilter(c,e,tp,rac,att,lvl)`
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rac: u64, att: u32, lvl: u32) -> bool {
    // return c:IsOriginalRace(rac) and c:IsOriginalAttribute(att) and c:IsLevel(lvl-1,lvl-2)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    // (a level of 0 or 1 gives a negative Lua value that no card has: wrapping_sub never matches a real level either)
    card::is_original_race(d, c, rac)
        && card::is_original_attribute(d, c, att)
        && card::is_level(d, c, &[lvl.wrapping_sub(1), lvl.wrapping_sub(2)])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.deckspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn deckspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.deckspcostfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| deckspcostfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.deckspcostfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| deckspcostfilter(d, c, a.e, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g).nil();
    // local cd=e:GetChainData()
    // cd.race=sc:GetOriginalRace()
    let race = card::get_original_race(d, sc);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(race as i64));
    // cd.attribute=sc:GetOriginalAttribute()
    let attribute = card::get_original_attribute(d, sc);
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 1, Value::Int(attribute as i64));
    // cd.level=sc:GetOriginalLevel()
    let level = card::get_original_level(d, sc);
    chain::set_effect_data(d, a.e, None, chain::field::CARD + 2, Value::Int(level as i64));
    // Duel.Remove(sc,POS_FACEUP,REASON_COST)
    duel::remove(d, sc, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.decksptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn decksptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.deckspop(e,tp,eg,ep,ev,re,r,rp)`
fn deckspop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local cd=e:GetChainData()
    let race = chain::effect_data(d, a.e, None, chain::field::CARD).int() as u64;
    let attribute = chain::effect_data(d, a.e, None, chain::field::CARD + 1).int() as u32;
    let level = chain::effect_data(d, a.e, None, chain::field::CARD + 2).int() as u32;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,cd.race,cd.attribute,cd.level)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| deckspfilter(d, c, a.e, a.tp, race, attribute, level)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
