//! Power Patron Shade of the Final Hour
use super::prelude::*;

pub const ID: u32 = 22404570;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_POWER_PATRON as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Power Patron" monster from your hand, face-up Extra Deck, or GY, then you can apply any of these effects, in sequence, based on the number of different card types (Fusion, Synchro, Xyz) you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsSetCard(SET_POWER_PATRON) and (c:IsFaceup() or c:IsLocation(LOCATION_HAND)) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    if !(card::is_set_card(d, c, &[SET_POWER_PATRON as u16])
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_HAND))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
    {
        return false;
    }
    // if c:IsLocation(LOCATION_HAND|LOCATION_GRAVE) then
    if card::is_location(d, c, LOCATION_HAND | LOCATION_GRAVE) {
        // return Duel.GetMZoneCount(tp)>0
        duel::get_m_zone_count(d, tp, NO_EX, None, None, None).0 > 0
    // elseif c:IsLocation(LOCATION_EXTRA) then
    } else if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        false
    }
}
/// `s.sptg(e,tp,...,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_EXTRA|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| spfilter(d, c, a.e, tp)),
            tp,
            LOCATION_HAND | LOCATION_EXTRA | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_EXTRA|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_EXTRA | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_DECK|LOCATION_ONFIELD|LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - tp, (LOCATION_DECK | LOCATION_ONFIELD | LOCATION_HAND) as i32);
    true
}
/// `s.spop(e,tp,...)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_EXTRA|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, tp))),
        tp,
        LOCATION_HAND | LOCATION_EXTRA | LOCATION_GRAVE,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local card_type_count=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsMonster),tp,LOCATION_MZONE,0,nil):GetBinClassCount(function(c) return c:GetType()&(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ) end)
        let mzone = duel::get_matching_group(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_monster(d, c)),
            tp,
            LOCATION_MZONE,
            0,
            NO_EX,
        );
        let card_type_count = group::get_bin_class_count(d, &mzone, &|d, c| (card::get_type(d, c) & (TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ)) as i64);
        // if card_type_count==0 then return end
        if card_type_count == 0 {
            return;
        }
        // local opp=1-tp
        let opp = 1 - tp;
        // local break_chk=false
        let mut break_chk = false;
        // local opp_decktop3=Duel.GetDecktopGroup(opp,3)
        let opp_decktop3 = duel::get_decktop_group(d, opp, 3);
        // local opp_field=Duel.GetFieldGroup(opp,LOCATION_ONFIELD,0)
        let opp_field = duel::get_field_group(d, opp, LOCATION_ONFIELD, 0);
        // local opp_hand=Duel.GetFieldGroup(opp,LOCATION_HAND,0)
        let opp_hand = duel::get_field_group(d, opp, LOCATION_HAND, 0);
        let removable_facedown = |d: &mut Duel, c: CardId| card::is_able_to_remove(d, c, Some(tp), Some(POS_FACEDOWN as u8), None);
        // --● 1+: Banish (face-down) the top 3 cards of your opponent's Deck
        // if card_type_count>=1 and #opp_decktop3==3 and opp_decktop3:FilterCount(Card.IsAbleToRemove,nil,tp,POS_FACEDOWN)==3
        //   and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
        if card_type_count >= 1
            && opp_decktop3.len() == 3
            && group::filter_count(d, &opp_decktop3, &removable_facedown, NO_EX) == 3
            && duel::select_yes_no(d, tp, aux::stringid(ID, 1))
        {
            // Duel.DisableShuffleCheck()
            duel::disable_shuffle_check(d, true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // if Duel.Remove(opp_decktop3,POS_FACEDOWN,REASON_EFFECT)>0 then break_chk=true end
            if duel::remove(d, opp_decktop3, POS_FACEDOWN as u8, REASON_EFFECT, None, None) > 0 {
                break_chk = true;
            }
        }
        // --● 2+: Banish (face-down) 1 card your opponent controls
        // if card_type_count>=2 and #opp_field>0 and opp_field:IsExists(Card.IsAbleToRemove,1,nil,tp,POS_FACEDOWN)
        //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if card_type_count >= 2
            && !opp_field.is_empty()
            && group::is_exists(d, &opp_field, &removable_facedown, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
            // local g=opp_field:FilterSelect(tp,Card.IsAbleToRemove,1,1,nil,tp,POS_FACEDOWN)
            let g = group::filter_select(d, &opp_field, tp, &removable_facedown, 1, 1, false, NO_EX).nil();
            // if #g>0 then
            if !g.is_empty() {
                // Duel.HintSelection(g)
                duel::hint_selection(d, g.clone(), true);
                // if break_chk then Duel.BreakEffect() end
                if break_chk {
                    duel::break_effect(d);
                }
                // if Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)>0 then break_chk=true end
                if duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None) > 0 {
                    break_chk = true;
                }
            }
        }
        // --● 3: Banish (face-down) 1 random card from your opponent's hand
        // if card_type_count==3 and #opp_hand>0 and opp_hand:IsExists(Card.IsAbleToRemove,1,nil,tp,POS_FACEDOWN)
        //   and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if card_type_count == 3
            && !opp_hand.is_empty()
            && group::is_exists(d, &opp_hand, &removable_facedown, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
        {
            // local g=opp_hand:Match(Card.IsAbleToRemove,nil,tp,POS_FACEDOWN):RandomSelect(tp,1)
            let mut matching = opp_hand.clone();
            group::match_(d, &mut matching, &removable_facedown, NO_EX);
            let g = group::random_select(d, &matching, tp, 1);
            // if #g>0 then
            if !g.is_empty() {
                // if break_chk then Duel.BreakEffect() end
                if break_chk {
                    duel::break_effect(d);
                }
                // Duel.Remove(g,POS_FACEDOWN,REASON_EFFECT)
                duel::remove(d, g, POS_FACEDOWN as u8, REASON_EFFECT, None, None);
            }
        }
    }
}
