//! Sacrifice Level-Up
use super::prelude::*;

pub const ID: u32 = 94185340;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish 2 Xyz materials and Special Summon 1 Xyz mnster from the Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tgfilter(c,e,tp)
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsFaceup() and c:IsType(TYPE_XYZ)) then return false end
    if !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)) {
        return false;
    }
    // local og=c:GetOverlayGroup()
    let og = card::get_overlay_group(d, c);
    // return og:IsExists(Card.IsAbleToRemove,2,nil)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c:GetRank()+1)
    group::is_exists(d, &og, &|d, x| card::is_able_to_remove(d, x, None, None, None), 2, NO_EX)
        && {
            let rank = card::get_rank(d, c) + 1;
            duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, rank)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
        }
}
// function s.spfilter(c,e,tp,rank)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rank: u32) -> bool {
    // return c:IsType(TYPE_XYZ) and c:IsRank(rank)
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    card::is_type(d, c, TYPE_XYZ)
        && card::is_rank(d, c, &[rank])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, a.e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g:GetFirst():GetOverlayGroup(),2,tp,0)
    let first = group::get_first(d, &g).nil();
    let og = card::get_overlay_group(d, first);
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(og), 2, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local og=tc:GetOverlayGroup()
    let og = card::get_overlay_group(d, tc);
    // if #og<2 then return end
    if og.len() < 2 {
        return;
    }
    // local rank=tc:GetRank()+1
    let rank = card::get_rank(d, tc) + 1;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rg=og:FilterSelect(tp,Card.IsAbleToRemove,2,2,nil)
    let rg = group::filter_select(d, &og, tp, &|d, x| card::is_able_to_remove(d, x, None, None, None), 2, 2, false, NO_EX).nil();
    // if Duel.Remove(rg,nil,REASON_EFFECT)==2 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,rank) then
    if duel::remove(d, rg, 0, REASON_EFFECT, None, None) == 2
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, a.e, tp, rank)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,rank):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp, rank)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &sel);
        // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 then
        if let Some(sc) = sc {
            if duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
                // Cannot attack this turn
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetDescription(3206)
                effect::set_description(d, e1, 3206);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetCode(EFFECT_CANNOT_ATTACK)
                effect::set_code(d, e1, EFFECT_CANNOT_ATTACK);
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // sc:RegisterEffect(e1,true)
                card::register_effect(d, sc, e1, true);
            }
        }
    }
}
