//! Dawn Angel Kambi
use super::prelude::*;

pub const ID: u32 = 12500059;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
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
    // e1:SetCondition(s.selfspcon)
    effect::set_condition(d, e1, Some(condition(selfspcon)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Fairy monster from your Deck whose Level equals the total Levels the Tributed monsters had on the field
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
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(s.deckspcost)
    effect::set_cost(d, e2, Some(cost_target(deckspcost)));
    // e2:SetTarget(s.decksptg)
    effect::set_target(d, e2, Some(cost_target(decksptg)));
    // e2:SetOperation(s.deckspop)
    effect::set_operation(d, e2, Some(operation(deckspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return c:IsRace(RACE_FAIRY) end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counter_filter));
}
/// `function(c) return c:IsRace(RACE_FAIRY) end` (the activity counter filter)
fn counter_filter(d: &mut Duel, a: Args) -> bool {
    card::is_race(d, a.c(0), RACE_FAIRY as u64)
}
/// `s.selfspconfilter(c)`
fn selfspconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() or not c:IsRace(RACE_FAIRY)
    card::is_facedown(d, c) || !card::is_race(d, c, RACE_FAIRY as u64)
}
/// `s.selfspcon(e,tp,eg,ep,ev,re,r,rp)`
fn selfspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.selfspconfilter,tp,LOCATION_MZONE,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&|d, c| selfspconfilter(d, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(c), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.deckspcostfilter(c)`
fn deckspcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FAIRY) and c:HasLevel()
    card::is_race(d, c, RACE_FAIRY as u64) && card::has_level(d, c)
}
/// `s.rescon(sg,tp,exg,e,handler)`: the release check, with `e` and `handler` passed as the extra arguments
fn rescon(d: &mut Duel, sg: &CardSet, tp: u8, e: EffectId, handler: CardId) -> (bool, bool) {
    // return sg:IsContains(handler) and Duel.GetMZoneCount(tp,sg)>0
    //     and Duel.IsExistingMatchingCard(s.deckspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,sg:GetSum(Card.GetLevel)),not sg:IsContains(handler)
    let res = group::is_contains(sg, handler) && duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0 && {
        let lv = group::get_sum(d, sg, &|d: &mut Duel, c: CardId| card::get_level(d, c) as i64);
        duel::is_existing_matching_card(d, Some(&|d, c| deckspfilter(d, c, e, tp, lv)), tp, LOCATION_DECK, 0, 1, NO_EX)
    };
    (res, !group::is_contains(sg, handler))
}
/// `s.deckspfilter(c,e,tp,lv)`
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    // return c:IsRace(RACE_FAIRY) and c:IsLevel(lv) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_FAIRY as u64)
        && card::is_level(d, c, &[lv as u32])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.deckspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn deckspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // e:SetLabel(-100)
    effect::set_label(d, e, &[-100]);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return c:IsReleasable() and c:HasLevel() and Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0
    //     and Duel.CheckReleaseGroupCost(tp,s.deckspcostfilter,1,99,false,s.rescon,nil,e,c) end
    if a.chk == Some(0) {
        return card::is_releasable(d, c, None, None)
            && card::has_level(d, c)
            && duel::get_custom_activity_count(d, ID, tp, ACTIVITY_SPSUMMON) == 0
            && duel::check_release_group_cost(d, tp, Some(&|d, cc| deckspcostfilter(d, cc)), 1, 99, false, Some(&|d, sg, tp2, _exg| rescon(d, sg, tp2, e, c)), NO_EX);
    }
    // local g=Duel.SelectReleaseGroupCost(tp,s.deckspcostfilter,1,99,false,s.rescon,nil,e,c)
    let mut g = duel::select_release_group_cost(d, tp, Some(&|d, cc| deckspcostfilter(d, cc)), 1, 99, false, Some(&|d, sg, tp2, _exg| rescon(d, sg, tp2, e, c)), NO_EX);
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // e:SetLabel(g:GetSum(Card.GetLevel))
    let lv = group::get_sum(d, &g, &|d: &mut Duel, cc: CardId| card::get_level(d, cc) as i64);
    effect::set_label(d, e, &[lv]);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // You cannot Special Summon the turn you activate this effect, except Fairy monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsRace(RACE_FAIRY) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, cc| !card::is_race(d, cc, RACE_FAIRY as u64))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
/// `s.decksptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn decksptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then
    if a.chk == Some(0) {
        // local res=e:GetLabel()==-100
        // e:SetLabel(0)
        // return res
        let res = effect::get_label(d, e) == -100;
        effect::set_label(d, e, &[0]);
        return res;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.deckspop(e,tp,eg,ep,ev,re,r,rp)`
fn deckspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.deckspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,e:GetLabel())
    let lv = effect::get_label(d, e);
    let g = duel::select_matching_card(d, tp, Some(&|d, c| deckspfilter(d, c, e, tp, lv)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
    if !g.is_empty() {
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
