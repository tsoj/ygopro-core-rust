//! Virtual World Mai-Hime - Lulu
//! Scripted by Hel
use super::prelude::*;

pub const ID: u32 = 49088914;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_VIRTUAL_WORLD], ..ScriptInfo::DEFAULT };

/// `local key=TYPE_MONSTER+TYPE_SPELL+TYPE_TRAP`
const KEY: u32 = TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 card to the GY, Special Summon, and search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.togravefilter(c,ctype)`
fn togravefilter(d: &mut Duel, c: CardId, ctype: u32) -> bool {
    // return c:IsSetCard(SET_VIRTUAL_WORLD) and not c:IsType(ctype&key) and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16]) && !card::is_type(d, c, ctype & KEY) && card::is_able_to_grave(d, c, None)
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VIRTUAL_WORLD) and Duel.IsExistingMatchingCard(s.togravefilter,tp,LOCATION_DECK,0,1,nil,c:GetType())
    if !card::is_faceup(d, c) || !card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16]) {
        return false;
    }
    let t = card::get_type(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| togravefilter(d, x, t)), tp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) and chkc:IsControler(tp) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_ONFIELD) && card::is_controler(d, chkc, a.tp) && tgfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_ONFIELD,0,1,nil,tp) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and e:GetHandler():IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| tgfilter(d, x, a.tp)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && {
                let c = effect::get_handler(d, a.e).nil();
                card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            };
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_ONFIELD,0,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, x| tgfilter(d, x, a.tp)), a.tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,e:GetHandler(),1,0,LOCATION_HAND)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, LOCATION_HAND as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tohandfilter(c,type1,type2)`
fn tohandfilter(d: &mut Duel, c: CardId, type1: u32, type2: u32) -> bool {
    // return c:IsSetCard(SET_VIRTUAL_WORLD) and not c:IsType(type1&key) and not c:IsType(type2&key) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16])
        && !card::is_type(d, c, type1 & KEY)
        && !card::is_type(d, c, type2 & KEY)
        && !card::is_code(d, c, &[ID])
        && card::is_able_to_hand(d, c, None)
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not (c:IsLevelAbove(3) or c:IsRankAbove(3))
    !(card::is_level_above(d, c, 3) || card::is_rank_above(d, c, 3))
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
    // Send to GY and Special Summon
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
            // local g=Duel.SelectMatchingCard(tp,s.togravefilter,tp,LOCATION_DECK,0,1,1,nil,tc:GetType())
            let t1 = card::get_type(d, tc);
            let g = duel::select_matching_card(d, a.tp, Some(&|d, x| togravefilter(d, x, t1)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT)>0 then
            if !g.is_empty() && duel::send_to_grave(d, &g, REASON_EFFECT, None, None) > 0 {
                // local ogc=Duel.GetOperatedGroup():GetFirst()
                let og = duel::get_operated_group(d);
                let ogc = group::get_first(d, &og).nil();
                // if ogc:IsLocation(LOCATION_GRAVE) and c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
                if card::is_location(d, ogc, LOCATION_GRAVE)
                    && card::is_relate_to_effect(d, c, a.e)
                    && duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0
                {
                    // Search
                    // local gth=Duel.GetMatchingGroup(s.tohandfilter,tp,LOCATION_DECK,0,nil,tc:GetType(),g:GetFirst():GetType())
                    let t2 = card::get_type(d, tc);
                    let g1 = group::get_first(d, &g).nil();
                    let t3 = card::get_type(d, g1);
                    let gth = duel::get_matching_group(d, Some(&|d, x| tohandfilter(d, x, t2, t3)), a.tp, LOCATION_DECK, 0, NO_EX);
                    // if #gth>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
                    if !gth.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
                        // Duel.BreakEffect()
                        duel::break_effect(d);
                        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
                        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
                        // local sg=gth:Select(tp,1,1,nil)
                        let sg = group::select(d, &gth, a.tp, 1, 1, false, NO_EX).nil();
                        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
                        duel::send_to_hand(d, &sg, None, REASON_EFFECT, None);
                        // Duel.ConfirmCards(1-tp,sg)
                        duel::confirm_cards(d, 1 - a.tp, &sg);
                    }
                }
            }
        }
    }
}
