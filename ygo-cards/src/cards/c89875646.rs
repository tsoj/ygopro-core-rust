//! Triple Borrel Launch
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 89875646;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ROKKET],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects (but you can only use each effect of "Triple Borrel Launch" once per turn);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DRAGON) and c:IsAbleToDeck()
    card::is_race(d, c, RACE_DRAGON as u64) && card::is_able_to_deck(d, c)
}
/// `s.target(e,tp,...,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local dg=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE,0,nil)
    let dg = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // --Shuffle 1 Dragon monster from your GY into the Deck, then you can Special Summon 1 "Rokket" monster with a different name from your Deck
    // local b1=not Duel.HasFlagEffect(tp,id) and #dg>=1
    let b1 = !duel::has_flag_effect(d, tp, ID, None) && dg.len() >= 1;
    // --Shuffle 2 Dragon monsters from your GY into the Deck, then add 1 Field Spell from your GY to your hand
    // local b2=not Duel.HasFlagEffect(tp,id+1) and #dg>=2
    //     and Duel.IsExistingMatchingCard(aux.AND(Card.IsFieldSpell,Card.IsAbleToHand),tp,LOCATION_GRAVE,0,1,nil)
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None)
        && dg.len() >= 2
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_field_spell(d, c) && card::is_able_to_hand(d, c, None)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // --Shuffle 3 Dragon monsters from your GY into the Deck, then shuffle up to 3 monsters from your opponent's GY into the Deck
    // local b3=not Duel.HasFlagEffect(tp,id+2) and #dg>=3
    //     and Duel.IsExistingMatchingCard(aux.AND(Card.IsMonster,Card.IsAbleToDeck),tp,0,LOCATION_GRAVE,1,nil)
    let b3 = !duel::has_flag_effect(d, tp, ID + 2, None)
        && dg.len() >= 3
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_deck(d, c)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    // if chk==0 then return b1 or b2 or b3 end
    if a.chk == Some(0) {
        return b1 || b2 || b3;
    }
    // local op=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)}, {b3,aux.Stringid(id,3)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2)), (b3, aux::stringid(ID, 3))]);
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op.map_or(0, |v| v as i64)]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, LOCATION_GRAVE as i32);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    } else if op == Some(2) {
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TODECK + CATEGORY_TOHAND);
        // Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,2,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 2, tp, LOCATION_GRAVE as i32);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    } else if op == Some(3) {
        // e:SetCategory(CATEGORY_TODECK)
        effect::set_category(d, a.e, CATEGORY_TODECK);
        // Duel.RegisterFlagEffect(tp,id+2,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID + 2, RESET_PHASE | PHASE_END, 0, 1, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,4,PLAYER_ALL,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 4, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.spfilter(c,e,tp,code)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, code: u32) -> bool {
    // return c:IsSetCard(SET_ROKKET) and not c:IsCode(code) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ROKKET as u16]) && !card::is_code(d, c, &[code]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.activate(e,tp,...)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // local dg=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE,0,nil)
    let dg = duel::get_matching_group(d, Some(&tdfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // if op==1 then
    if op == 1 {
        // --Shuffle 1 Dragon monster from your GY into the Deck, then you can Special Summon 1 "Rokket" monster with a different name from your Deck
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local gc=dg:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &dg, tp, 1, 1, false, NO_EX).nil();
        // if not gc then return end
        let Some(gc) = group::get_first(d, &sel) else {
            return;
        };
        // Duel.HintSelection(gc)
        duel::hint_selection(d, CardSet::from_card(gc), true);
        // if Duel.SendtoDeck(gc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::send_to_deck(d, gc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0
            || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        {
            return;
        }
        // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp,gc:GetCode())
        let code = card::get_code(d, gc)[0];
        let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp, code)), tp, LOCATION_DECK, 0, NO_EX);
        // if #sg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
        if !sg.is_empty() && duel::select_yes_no(d, tp, aux::stringid(ID, 4)) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local sc=sg:Select(tp,1,1,nil)
            let sc = group::select(d, &sg, tp, 1, 1, false, NO_EX);
            // if sc then
            if let Some(sc) = sc {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
                duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    } else if op == 2 {
        // --Shuffle 2 Dragon monsters from your GY into the Deck, then add 1 Field Spell from your GY to your hand
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local g=dg:Select(tp,2,2,nil)
        let g = group::select(d, &dg, tp, 2, 2, false, NO_EX).nil();
        // if #g~=2 then return end
        if g.len() != 2 {
            return;
        }
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // if Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 then return end
        if duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local hg=Duel.SelectMatchingCard(tp,aux.AND(Card.IsFieldSpell,Card.IsAbleToHand),tp,LOCATION_GRAVE,0,1,1,nil)
        let hg = duel::select_matching_card(
            d,
            tp,
            Some(&|d, c| card::is_field_spell(d, c) && card::is_able_to_hand(d, c, None)),
            tp,
            LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #hg>0 then
        if !hg.is_empty() {
            // Duel.HintSelection(hg)
            duel::hint_selection(d, hg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(hg,nil,REASON_EFFECT)
            duel::send_to_hand(d, hg, None, REASON_EFFECT, None);
        }
    } else if op == 3 {
        // --Shuffle 3 Dragon monsters from your GY into the Deck, then shuffle up to 3 monsters from your opponent's GY into the Deck
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local g=dg:Select(tp,3,3,nil)
        let g = group::select(d, &dg, tp, 3, 3, false, NO_EX).nil();
        // if #g~=3 then return end
        if g.len() != 3 {
            return;
        }
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // if Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)==0 then return end
        if duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) == 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local sg=Duel.SelectMatchingCard(tp,aux.AND(Card.IsMonster,Card.IsAbleToDeck),tp,0,LOCATION_GRAVE,1,3,nil)
        let sg = duel::select_matching_card(
            d,
            tp,
            Some(&|d, c| card::is_monster(d, c) && card::is_able_to_deck(d, c)),
            tp,
            0,
            LOCATION_GRAVE,
            1,
            3,
            false,
            NO_EX,
        )
        .nil();
        // if #sg>0 then
        if !sg.is_empty() {
            // Duel.HintSelection(sg)
            duel::hint_selection(d, sg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoDeck(sg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
            duel::send_to_deck(d, sg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
        }
    }
}
