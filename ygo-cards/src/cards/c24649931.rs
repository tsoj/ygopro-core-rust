//! Materialization
use super::prelude::*;

pub const ID: u32 = 24649931;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
}
/// `aux.FaceupFilter(Card.IsOriginalType,TYPE_MONSTER)`
fn faceup_monster(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_original_type(d, c, TYPE_MONSTER)
}
/// `s.target(e,tp,...,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // if not (chkc:IsLocation(LOCATION_STZONE) and chkc:IsOriginalType(TYPE_MONSTER) and chkc:IsFaceup()) then return false end
        if !(card::is_location(d, chkc, LOCATION_STZONE) && card::is_original_type(d, chkc, TYPE_MONSTER) && card::is_faceup(d, chkc)) {
            return false;
        }
        // local label=e:GetLabel()
        let label = effect::get_label(d, a.e);
        if label == 1 {
            // local owner=chkc:GetOwner()
            let owner = card::get_owner(d, chkc);
            // return Duel.GetLocationCount(owner,LOCATION_MZONE)>0
            //   and chkc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,owner)
            return duel::get_location_count(d, owner, LOCATION_MZONE as u8, None, None, None).0 > 0
                && card::is_can_be_special_summoned(d, chkc, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(owner), None);
        } else if label == 2 {
            // return chkc:IsAbleToHand()
            return card::is_able_to_hand(d, chkc, None);
        } else if label == 3 {
            // return true
            return true;
        }
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsOriginalType,TYPE_MONSTER),tp,LOCATION_STZONE,LOCATION_STZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&faceup_monster), tp, LOCATION_STZONE, LOCATION_STZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsOriginalType,TYPE_MONSTER),tp,LOCATION_STZONE,LOCATION_STZONE,1,1,nil):GetFirst()
    let g = duel::select_target(d, tp, Some(&faceup_monster), tp, LOCATION_STZONE, LOCATION_STZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // local owner=tc:GetOwner()
    let owner = card::get_owner(d, tc);
    // local b1=Duel.GetLocationCount(owner,LOCATION_MZONE)>0
    //   and tc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,owner)
    let b1 = duel::get_location_count(d, owner, LOCATION_MZONE as u8, None, None, None).0 > 0
        && card::is_can_be_special_summoned(d, tc, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(owner), None);
    // local b2=tc:IsAbleToHand()
    let b2 = card::is_able_to_hand(d, tc, None);
    // local b3=true
    let b3 = true;
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,1)},{b2,aux.Stringid(id,2)},{b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]).nil();
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op as i64]);
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(tc)), 1, tp, 0);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(tc)), 1, tp, 0);
    } else if op == 3 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, a.e, CATEGORY_DESTROY);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc)), 1, tp, 0);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,PLAYER_EITHER,LOCATION_MZONE)
        duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    }
    true
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Duel.SpecialSummon(tc,0,tp,tc:GetOwner(),false,false,POS_FACEUP)
        let owner = card::get_owner(d, tc);
        duel::special_summon(d, tc, 0, tp, owner, false, false, POS_FACEUP as u8, None);
    } else if op == 2 {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,tc)
        duel::confirm_cards(d, 1 - tp, tc);
    } else if op == 3 {
        // if Duel.Destroy(tc,REASON_EFFECT)==0 or not tc:HasLevel() then return end
        if duel::destroy(d, tc, REASON_EFFECT, None, None) == 0 || !card::has_level(d, tc) {
            return;
        }
        // local lv=tc:GetLevel()
        let lv = card::get_level(d, tc);
        // if Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsLevelBelow,lv),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
        //   and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
        if duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_level_below(d, c, lv))), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 4))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local g=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsLevelBelow,lv),tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
            let g = duel::select_matching_card(d, tp, Some(&aux::faceup_filter(|d, c| card::is_level_below(d, c, lv))), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
            // if #g==0 then return end
            if g.is_empty() {
                return;
            }
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
