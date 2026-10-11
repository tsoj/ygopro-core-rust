//! Tellarknight Constellar Delteros
//! 星騎士 セイクリッド・デルタテロス
use super::prelude::*;

pub const ID: u32 = 32289031;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TELLARKNIGHT, SET_CONSTELLAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3+ Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,3,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, Some(xyz::MaxCt::Infinite), None, false, None);
    // Your opponent cannot target Rank 5 or higher "tellarknight" or "Constellar" Xyz Monsters you control with card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.tellarkR5filter)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| tellark_r5_filter(d, c))));
    // e1:SetValue(aux.tgoval)
    effect::set_value_fn(d, e1, aux::tgoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "tellarknight" or "Constellar" monster from your Deck to your hand, then immediately after this effect resolves, you can Normal Summon 1 LIGHT monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOHAND | CATEGORY_SEARCH | CATEGORY_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "tellarknight" or "Constellar" monster from your hand or banishment
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE | EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e3, EVENT_TO_GRAVE);
    // e3:SetCondition(function(e) return not e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(
        d,
        e3,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            !card::is_previous_location(d, h, LOCATION_ONFIELD)
        })),
    );
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.tellarkR5filter(e,c)`
fn tellark_r5_filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_TELLARKNIGHT,SET_CONSTELLAR}) and c:IsRankAbove(5) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16, SET_CONSTELLAR as u16]) && card::is_rank_above(d, c, 5) && card::is_faceup(d, c)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_TELLARKNIGHT,SET_CONSTELLAR}) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16, SET_CONSTELLAR as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    true
}
/// `s.nsfilter(c)`
fn nsfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsSummonable(true,nil)
    card::is_attribute(d, c, ATTRIBUTE_LIGHT as u32) && card::is_summonable(d, c, true, None, 0, None)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, tp);
        // if Duel.IsExistingMatchingCard(s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::is_existing_matching_card(d, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
            // local sc=Duel.SelectMatchingCard(tp,s.nsfilter,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil):GetFirst()
            let sel = duel::select_matching_card(d, tp, Some(&nsfilter), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
            let sc = group::get_first(d, &sel);
            // if sc then
            if let Some(sc) = sc {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Summon(tp,sc,true,nil)
                duel::summon(d, tp, sc, true, None, 0, None);
            }
        }
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard({SET_TELLARKNIGHT,SET_CONSTELLAR}) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and (c:IsFaceup() or c:IsLocation(LOCATION_HAND))
    card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16, SET_CONSTELLAR as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_HAND | LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_REMOVED) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| spfilter(d, cc, e, tp)), tp, LOCATION_HAND | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
