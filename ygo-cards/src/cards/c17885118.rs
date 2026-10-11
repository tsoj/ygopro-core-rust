//! Pendulum Scale
use super::prelude::*;

pub const ID: u32 = 17885118;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFieldCard(tp,LOCATION_PZONE,0) and Duel.GetFieldCard(tp,LOCATION_PZONE,1)
    duel::get_field_card(d, a.tp, LOCATION_PZONE, 0).is_some() && duel::get_field_card(d, a.tp, LOCATION_PZONE, 1).is_some()
}
/// `s.thfilter(c,hl)`
fn thfilter(d: &mut Duel, c: CardId, hl: bool) -> bool {
    // if not (c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()) then return false end
    if !(card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)) {
        return false;
    }
    // if hl then return c:IsLevelAbove(5) and c:IsLevelBelow(7)
    if hl {
        card::is_level_above(d, c, 5) && card::is_level_below(d, c, 7)
    } else {
        // else return c:IsLevelAbove(2) and c:IsLevelBelow(4) end
        card::is_level_above(d, c, 2) && card::is_level_below(d, c, 4)
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local lc=Duel.GetFieldCard(tp,LOCATION_PZONE,0)
    let lc = duel::get_field_card(d, a.tp, LOCATION_PZONE, 0);
    // local rc=Duel.GetFieldCard(tp,LOCATION_PZONE,1)
    let rc = duel::get_field_card(d, a.tp, LOCATION_PZONE, 1);
    // if chk==0 then
    if a.chk == Some(0) {
        // if not lc or not rc then return false end
        let (Some(lc), Some(rc)) = (lc, rc) else { return false };
        // local diff=math.abs(lc:GetScale()-rc:GetScale())
        let diff = (card::get_scale(d, lc) as i64 - card::get_scale(d, rc) as i64).abs();
        let h = effect::get_handler(d, a.e).nil();
        // if diff==0 then
        if diff == 0 {
            // return Duel.IsExistingMatchingCard(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,2,e:GetHandler())
            return duel::is_existing_matching_card(d, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 2, h.into());
        } else if diff <= 3 {
            // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,false)
            return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, false)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
        } else if diff <= 6 {
            // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,true)
            return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, true)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
        } else if diff >= 7 {
            // return lc:IsAbleToHand() or rc:IsAbleToHand()
            return card::is_able_to_hand(d, lc, None) || card::is_able_to_hand(d, rc, None);
        }
    }
    // local diff=math.abs(lc:GetScale()-rc:GetScale())
    let lc = lc.nil();
    let rc = rc.nil();
    let diff = (card::get_scale(d, lc) as i64 - card::get_scale(d, rc) as i64).abs();
    // if diff==0 then
    if diff == 0 {
        // local g=Duel.GetMatchingGroup(Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let g = duel::get_matching_group(d, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,2,PLAYER_ALL,LOCATION_ONFIELD)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 2, PLAYER_ALL as u8, LOCATION_ONFIELD as i32);
    } else if diff <= 6 {
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    } else if diff >= 7 {
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_PZONE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_PZONE as i32);
    }
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsType(TYPE_PENDULUM) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_type(d, c, TYPE_PENDULUM) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local lc=Duel.GetFieldCard(tp,LOCATION_PZONE,0)
    let lc = duel::get_field_card(d, a.tp, LOCATION_PZONE, 0);
    // local rc=Duel.GetFieldCard(tp,LOCATION_PZONE,1)
    let rc = duel::get_field_card(d, a.tp, LOCATION_PZONE, 1);
    // if not lc or not rc then return end
    let (Some(lc), Some(rc)) = (lc, rc) else { return };
    // local diff=math.abs(lc:GetScale()-rc:GetScale())
    let diff = (card::get_scale(d, lc) as i64 - card::get_scale(d, rc) as i64).abs();
    let h = effect::get_handler(d, a.e).nil();
    // if diff==0 then
    if diff == 0 {
        // Duel.Hint(HINT_OPSELECTED,tp,aux.Stringid(id,1))
        duel::hint(d, HINT_OPSELECTED as u8, a.tp, aux::stringid(ID, 1));
        // Duel.Hint(HINT_OPSELECTED,1-tp,aux.Stringid(id,1))
        duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, aux::stringid(ID, 1));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsSpellTrap,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,2,2,e:GetHandler())
        let g = duel::select_matching_card(d, a.tp, Some(&card::is_spell_trap), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 2, 2, false, h.into()).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    } else if diff <= 6 {
        // local diffchk=diff>=4
        let diffchk = diff >= 4;
        // local str=diffchk and 3 or 2
        let str_id = if diffchk { 3 } else { 2 };
        // Duel.Hint(HINT_OPSELECTED,tp,aux.Stringid(id,str))
        duel::hint(d, HINT_OPSELECTED as u8, a.tp, aux::stringid(ID, str_id));
        // Duel.Hint(HINT_OPSELECTED,1-tp,aux.Stringid(id,str))
        duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, aux::stringid(ID, str_id));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,diffchk)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, diffchk)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    } else if diff >= 7 {
        // Duel.Hint(HINT_OPSELECTED,tp,aux.Stringid(id,4))
        duel::hint(d, HINT_OPSELECTED as u8, a.tp, aux::stringid(ID, 4));
        // Duel.Hint(HINT_OPSELECTED,1-tp,aux.Stringid(id,4))
        duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, aux::stringid(ID, 4));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
        // local hg=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_PZONE,0,1,2,nil)
        let hg = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, LOCATION_PZONE, 0, 1, 2, false, NO_EX).nil();
        // if #hg>0 and Duel.SendtoHand(hg,nil,REASON_EFFECT)>0 and hg:IsExists(Card.IsLocation,1,nil,LOCATION_HAND)
        //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
        if !hg.is_empty()
            && duel::send_to_hand(d, hg.clone(), None, REASON_EFFECT, None) > 0
            && group::is_exists(d, &hg, &|d, c| card::is_location(d, c, LOCATION_HAND), 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        {
            // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_HAND,0,nil,e,tp)
            let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, NO_EX);
            // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,5)) then
            if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 5)) {
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
                // local sg=g:Select(tp,1,1,nil)
                let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
                // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
