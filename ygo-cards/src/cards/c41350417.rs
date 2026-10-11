//! Dark Magical Curtain
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 41350417;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL, ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Each player can Special Summon 1 DARK Spellcaster monster from their hand or Deck ...
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
}
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_EITHER,LOCATION_HAND|LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_EITHER as u8, (LOCATION_HAND | LOCATION_DECK) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.spfilter(c,e,sp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, sp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK) and c:IsRace(RACE_SPELLCASTER) and c:IsCanBeSpecialSummoned(e,0,sp,false,false,POS_FACEUP,sp)
    card::is_attribute(d, c, ATTRIBUTE_DARK)
        && card::is_race(d, c, RACE_SPELLCASTER)
        && card::is_can_be_special_summoned(d, c, e, 0, sp, false, false, Some(POS_FACEUP as u8), Some(sp), None)
}
/// `s.spresolution(e,sp)`
fn spresolution(d: &mut Duel, e: EffectId, sp: u8) -> bool {
    // local name_chk=false
    let mut name_chk = false;
    // if Duel.GetLocationCount(sp,LOCATION_MZONE,sp)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,sp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,sp)
    //     and Duel.SelectYesNo(sp,aux.Stringid(id,1)) then
    if duel::get_location_count(d, sp, LOCATION_MZONE as u8, Some(sp), None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, sp)), sp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
        && duel::select_yes_no(d, sp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,sp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, sp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(sp,s.spfilter,sp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,sp):GetFirst()
        let g = duel::select_matching_card(d, sp, Some(&|d, c| spfilter(d, c, e, sp)), sp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc and Duel.SpecialSummonStep(sc,0,sp,sp,false,false,POS_FACEUP) then
        if let Some(sc) = sc {
            if duel::special_summon_step(d, sc, 0, sp, sp, false, false, POS_FACEUP as u8, None) {
                // name_chk=sc:IsOriginalCodeRule(CARD_DARK_MAGICIAN,CARD_DARK_MAGICIAN_GIRL)
                name_chk = card::is_original_code_rule(d, sc, &[CARD_DARK_MAGICIAN, CARD_DARK_MAGICIAN_GIRL]);
                // Neither player can activate their effects this turn
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetDescription(3302)
                effect::set_description(d, e1, 3302);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetCode(EFFECT_CANNOT_TRIGGER)
                effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
            }
        }
    }
    // return name_chk
    name_chk
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:ListsCode(CARD_DARK_MAGICIAN) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_spell_trap(d, c) && card::lists_code(d, c, &[CARD_DARK_MAGICIAN]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local turn_player=Duel.GetTurnPlayer()
    let turn_player = duel::get_turn_player(d);
    // local non_turn_player=1-turn_player
    let non_turn_player = 1 - turn_player;
    // local turn_player_name_chk=s.spresolution(e,turn_player)
    let turn_player_name_chk = spresolution(d, e, turn_player);
    // local non_turn_player_name_chk=s.spresolution(e,non_turn_player)
    let non_turn_player_name_chk = spresolution(d, e, non_turn_player);
    // if Duel.SpecialSummonComplete()==0 then return end
    if duel::special_summon_complete(d) == 0 {
        return;
    }
    // if not ((turn_player_name_chk and tp==turn_player) or (non_turn_player_name_chk and tp==non_turn_player)) then return end
    if !((turn_player_name_chk && tp == turn_player) || (non_turn_player_name_chk && tp == non_turn_player)) {
        return;
    }
    // if Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX) && duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - tp, g);
        }
    }
}
