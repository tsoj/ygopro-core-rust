//! Superconductive Plasma Blast
use super::prelude::*;

pub const ID: u32 = 51869363;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate this effect depending on whose turn it is;
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of a monster being destroyed by battle or card effect
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DESTROYED)
        effect::set_code(d, ge1, EVENT_DESTROYED);
        // ge1:SetOperation(s.descheckop)
        effect::set_operation(d, ge1, Some(operation(descheckop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.descheckfilter(c)`
fn descheckfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsPreviousLocation(LOCATION_MZONE) or (not c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsMonsterCard()))
    //     and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    (card::is_previous_location(d, c, LOCATION_MZONE) || (!card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_monster_card(d, c)))
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// `s.descheckop(e,tp,eg,ep,ev,re,r,rp)`
fn descheckop(d: &mut Duel, a: &EffArgs) {
    // if eg:IsExists(s.descheckfilter,1,nil) then
    let eg = a.eg.clone().nil();
    if group::is_exists(d, &eg, &|d, c| descheckfilter(d, c), 1, NO_EX) {
        // Duel.RegisterFlagEffect(0,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, 0, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_THUNDER|RACE_ROCK) and c:IsAbleToHand() and (c:IsFaceup() or c:IsLocation(LOCATION_DECK))
    card::is_race(d, c, RACE_THUNDER | RACE_ROCK) && card::is_able_to_hand(d, c, None) && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_DECK))
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local op=Duel.IsTurnPlayer(tp) and 1 or 2
    let op: i64 = if duel::is_turn_player(d, tp) { 1 } else { 2 };
    // local locations=LOCATION_MZONE|LOCATION_GRAVE|LOCATION_REMOVED
    let mut locations = LOCATION_MZONE | LOCATION_GRAVE | LOCATION_REMOVED;
    // if Duel.HasFlagEffect(0,id) then locations=locations|LOCATION_DECK end
    if duel::has_flag_effect(d, 0, ID, None) {
        locations |= LOCATION_DECK;
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // if op==1 then
        if op == 1 {
            // return Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>1
            //     and Duel.IsExistingMatchingCard(Card.IsRace,tp,LOCATION_DECK,0,1,nil,RACE_THUNDER|RACE_ROCK)
            return duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 1
                && duel::is_existing_matching_card(d, Some(&|d, c| card::is_race(d, c, RACE_THUNDER | RACE_ROCK)), tp, LOCATION_DECK, 0, 1, NO_EX);
        } else if op == 2 {
            // return Duel.IsExistingMatchingCard(s.thfilter,tp,locations,0,1,nil)
            return duel::is_existing_matching_card(d, Some(&thfilter), tp, locations, 0, 1, NO_EX);
        }
    }
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    // Duel.Hint(HINT_OPSELECTED,1-tp,aux.Stringid(id,op))
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, aux::stringid(ID, op as u32));
    if op == 1 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, e, CATEGORY_DESTROY);
        // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,PLAYER_EITHER,LOCATION_ONFIELD)
        duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, PLAYER_EITHER as u8, LOCATION_ONFIELD as i32);
    } else if op == 2 {
        // e:SetCategory(CATEGORY_TOHAND|CATEGORY_SEARCH)
        effect::set_category(d, e, CATEGORY_TOHAND | CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,locations)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, locations as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local op=e:GetLabel()
    let op = effect::get_label(d, e);
    if op == 1 {
        // Yours: Place 1 Thunder or Rock monster from your Deck on top of your Deck, then if a Thunder or Rock monster is in your field or GY, you can destroy 1 card on the field
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
        duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
        // local sc=Duel.SelectMatchingCard(tp,Card.IsRace,tp,LOCATION_DECK,0,1,1,nil,RACE_THUNDER|RACE_ROCK):GetFirst()
        let sg = duel::select_matching_card(d, tp, Some(&|d, c| card::is_race(d, c, RACE_THUNDER | RACE_ROCK)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &sg) else { return };
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.MoveSequence(sc,0)
        duel::move_sequence(d, sc, 0, None);
        // Duel.ConfirmDecktop(tp,1)
        duel::confirm_decktop(d, tp, 1);
        // local g=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)-e:GetHandler()
        let mut g = duel::get_field_group(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
        let h = effect::get_handler(d, e).nil();
        group::remove_card(&mut g, &CardSet::from_card(h));
        // if #g>0 and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_THUNDER|RACE_ROCK),tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,4)) then
        if !g.is_empty()
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_THUNDER | RACE_ROCK)),
                tp,
                LOCATION_MZONE | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            )
            && duel::select_yes_no(d, tp, aux::stringid(ID, 4))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
            // local dg=g:Select(tp,1,1,nil)
            let dg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
            // if #dg>0 then
            if !dg.is_empty() {
                // Duel.HintSelection(dg)
                duel::hint_selection(d, dg.clone(), true);
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Destroy(dg,REASON_EFFECT)
                duel::destroy(d, dg, REASON_EFFECT, None, None);
            }
        }
    } else if op == 2 {
        // Opponent's: Add 1 Thunder or Rock monster from your field, GY, or banishment to the hand, or if a monster(s) was destroyed by battle or card effect this turn, you can add from your Deck instead
        // local locations=LOCATION_MZONE|LOCATION_GRAVE|LOCATION_REMOVED
        let mut locations = LOCATION_MZONE | LOCATION_GRAVE | LOCATION_REMOVED;
        // if Duel.HasFlagEffect(0,id) then locations=locations|LOCATION_DECK end
        if duel::has_flag_effect(d, 0, ID, None) {
            locations |= LOCATION_DECK;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local hc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,locations,0,1,1,nil):GetFirst()
        let f = aux::necro_valley_filter(|d, c| thfilter(d, c));
        let sg = duel::select_matching_card(d, tp, Some(&f), tp, locations, 0, 1, 1, false, NO_EX).nil();
        // if hc then
        if let Some(hc) = group::get_first(d, &sg) {
            // if not hc:IsLocation(LOCATION_DECK) then Duel.HintSelection(hc) end
            if !card::is_location(d, hc, LOCATION_DECK) {
                duel::hint_selection(d, hc, true);
            }
            // Duel.SendtoHand(hc,nil,REASON_EFFECT)
            duel::send_to_hand(d, hc, None, REASON_EFFECT, None);
            // if hc:IsPreviousLocation(LOCATION_DECK) then Duel.ConfirmCards(1-tp,hc) end
            if card::is_previous_location(d, hc, LOCATION_DECK) {
                duel::confirm_cards(d, 1 - tp, hc);
            }
        }
    }
}
