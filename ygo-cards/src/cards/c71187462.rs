//! BF - Vata the Emblem of Wandering
//! ＢＦ－無頼のヴァータ
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 71187462;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID, CARD_BLACK_WINGED_DRAGON], listed_series: &[SET_BLACKWING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send this card and "Blackwing" non-tuners to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId, id: u32) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_BLACKWING) and not c:IsCode(id)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_BLACKWING as u16]) && !card::is_code(d, c, &[id])
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    let _ = c;
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, cc| spconfilter(d, cc, ID)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,ec)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ec: Option<CardId>) -> bool {
    // return c:IsCode(CARD_BLACK_WINGED_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,ec,c)>0
    card::is_code(d, c, &[CARD_BLACK_WINGED_DRAGON])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), ec.into(), duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BLACKWING) and not c:IsType(TYPE_TUNER) and c:HasLevel() and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_BLACKWING as u16]) && !card::is_type(d, c, TYPE_TUNER) && card::has_level(d, c) && card::is_able_to_grave(d, c, None)
}
/// `s.tgrescon(clv)`: the returned function of `function(sg)`; `clv` is the level of the card that
/// is sent with the group.
fn tgrescon(d: &mut Duel, sg: &CardSet, clv: i64) -> (bool, bool) {
    // local sum=clv+sg:GetSum(Card.GetLevel)
    let sum = clv + group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64);
    // return sum==8,sum>8
    (sum == 8, sum > 8)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if not c:IsAbleToGrave() or c:IsLevelAbove(8)
        //     or not Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c) then return false end
        if !card::is_able_to_grave(d, c, None)
            || card::is_level_above(d, c, 8)
            || !duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, a.e, tp, Some(c))), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        {
            return false;
        }
        // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_DECK, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,1,#g,s.tgrescon(c:GetLevel()),0)
        let clv = card::get_level(d, c) as i64;
        let n = g.len();
        let rs = move |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| tgrescon(d, sg, clv);
        return matches!(aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(n), Some(&rs), 0, tp, None, None, None, false), Err(true));
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsAbleToGrave() and c:IsLevelBelow(7) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_able_to_grave(d, c, None) && card::is_level_below(d, c, 7) {
        // local g=Duel.GetMatchingGroup(s.tgfilter,tp,LOCATION_DECK,0,nil)
        let g = duel::get_matching_group(d, Some(&tgfilter), tp, LOCATION_DECK, 0, NO_EX);
        // local rescon=s.tgrescon(c:GetLevel())
        let clv = card::get_level(d, c) as i64;
        let n = g.len();
        let rs = move |d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>| tgrescon(d, sg, clv);
        // local mg=aux.SelectUnselectGroup(g,e,tp,1,#g,rescon,1,tp,HINTMSG_TOGRAVE,rescon)
        let mg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(n), Some(&rs), 1, tp, Some(HINTMSG_TOGRAVE as u64), Some(&rs), None, false)
            .unwrap_or_default();
        // if #mg>0 and Duel.SendtoGrave(c+mg,REASON_EFFECT)>0 then
        if !mg.is_empty() {
            let mut cg = CardSet::from_card(c);
            group::add_card(&mut cg, &mg);
            if duel::send_to_grave(d, cg, REASON_EFFECT, None, None) > 0 {
                // local og=Duel.GetOperatedGroup()
                let og = duel::get_operated_group(d);
                // if og:FilterCount(Card.IsLocation,nil,LOCATION_GRAVE)~=#og then return end
                if group::filter_count(d, &og, &|d, cc| card::is_location(d, cc, LOCATION_GRAVE), NO_EX) != og.len() {
                    return;
                }
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
                // local sg=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
                let sg = duel::select_matching_card(d, tp, Some(&|d, cc| spfilter(d, cc, a.e, tp, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
                // if #sg>0 then
                if !sg.is_empty() {
                    // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                    duel::special_summon(d, sg, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                }
            }
        }
    }
    // Cannot Special Summon monsters from Extra Deck, except DARK monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsAttribute(ATTRIBUTE_DARK) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_attribute(d, c, ATTRIBUTE_DARK))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Clock Lizard check
    // aux.addTempLizardCheck(e:GetHandler(),tp,s.lizfilter)
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(lizfilter)), None, None, None, None);
}
/// `s.lizfilter(e,c)`: `not c:GetOriginalAttribute()==ATTRIBUTE_DARK` compares a boolean with a
/// number, so it is always false.
fn lizfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    let _ = card::get_original_attribute(d, c);
    false
}
