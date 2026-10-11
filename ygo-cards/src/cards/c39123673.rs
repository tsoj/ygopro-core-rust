//! Spell Power Attainment
//! 魔力到達
use super::prelude::*;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_ENDYMION], counter_list: &[COUNTER_SPELL], ..ScriptInfo::DEFAULT };

pub const ID: u32 = 39123673;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 card from your Deck or GY to your hand that mentions "Spell Counter" in its text, except "Spell Power Attainment", then if you control a Level 7 or higher "Endymion" Monster Card, you can remove any number of Spell Counters from your field, and if you do, negate the effects of that many face-up cards your opponent controls, and if you do that, destroy them
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DISABLE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:ListsCounter(COUNTER_SPELL) and c:IsAbleToHand() and not c:IsCode(id)
    card::lists_counter(d, c, COUNTER_SPELL as u32) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DISABLE,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.endymionfilter(c)`
fn endymionfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(7) and c:IsSetCard(SET_ENDYMION) and c:IsMonsterCard() and c:IsFaceup()
    card::is_level_above(d, c, 7) && card::is_set_card(d, c, &[SET_ENDYMION as u16]) && card::is_monster_card(d, c) && card::is_faceup(d, c)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(thfilter)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if not sc then return end
    let Some(sc) = sc else {
        return;
    };
    // if sc:IsLocation(LOCATION_GRAVE) then Duel.HintSelection(sc) end
    if card::is_location(d, sc, LOCATION_GRAVE) {
        duel::hint_selection(d, CardSet::from_card(sc), true);
    }
    // if Duel.SendtoHand(sc,nil,REASON_EFFECT)==0 or not sc:IsLocation(LOCATION_HAND) then return end
    if duel::send_to_hand(d, CardSet::from_card(sc), None, REASON_EFFECT, None) == 0 || !card::is_location(d, sc, LOCATION_HAND) {
        return;
    }
    // if sc:IsPreviousLocation(LOCATION_DECK) then Duel.ConfirmCards(1-tp,sc) end
    if card::is_previous_location(d, sc, LOCATION_DECK) {
        duel::confirm_cards(d, 1 - tp, CardSet::from_card(sc));
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // if not Duel.IsExistingMatchingCard(s.endymionfilter,tp,LOCATION_ONFIELD,0,1,nil) then return end
    if !duel::is_existing_matching_card(d, Some(&endymionfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX) {
        return;
    }
    // if not Duel.IsCanRemoveCounter(tp,1,0,COUNTER_SPELL,1,REASON_EFFECT) then return end
    if !duel::is_can_remove_counter(d, tp, 1, 0, COUNTER_SPELL as u16, 1, REASON_EFFECT) {
        return;
    }
    // local ng=Duel.GetMatchingGroup(Card.IsNegatable,tp,0,LOCATION_ONFIELD,nil)
    let ng = duel::get_matching_group(d, Some(&|d, c| card::is_negatable(d, c)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #ng==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
    if ng.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        return;
    }
    // needs to loop because Duel.GetCounter does not include checks for counters that can be removed
    // local max_ct=1
    let mut max_ct: usize = 1;
    // for i=2,#ng do
    for i in 2..=ng.len() {
        // if not Duel.IsCanRemoveCounter(tp,1,0,COUNTER_SPELL,i,REASON_EFFECT) then break end
        if !duel::is_can_remove_counter(d, tp, 1, 0, COUNTER_SPELL as u16, i as u32, REASON_EFFECT) {
            break;
        }
        // max_ct=i
        max_ct = i;
    }
    // local ct=max_ct==1 and 1 or Duel.AnnounceNumberRange(tp,1,max_ct)
    let ct: i64 = if max_ct == 1 {
        1
    } else {
        duel::announce_number_range(d, tp, Some(1), Some(max_ct as u32), &[]).nil().0 as i64
    };
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if not Duel.RemoveCounter(tp,1,0,COUNTER_SPELL,ct,REASON_EFFECT) then return end
    if !duel::remove_counter(d, tp, 1, 0, COUNTER_SPELL as u16, ct as u32, REASON_EFFECT) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local sng=ng:Select(tp,ct,ct,nil)
    let mut sng = group::select(d, &ng, tp, ct as u16, ct as u16, false, NO_EX).nil();
    // if #sng==ct then
    if sng.len() as i64 == ct {
        // Duel.HintSelection(sng)
        duel::hint_selection(d, sng.clone(), true);
        // sng:Match(Card.IsCanBeDisabledByEffect,nil,e)
        group::match_(d, &mut sng, &|d, c| card::is_can_be_disabled_by_effect(d, c, e, true), NO_EX);
        // Negate their effects
        // sng:ForEach(Card.NegateEffects,e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        for tc in group::iter(d, &sng) {
            card::negate_effects(d, tc, h, None, false, None);
        }
        // Duel.AdjustInstantly()
        duel::adjust_instantly(d, None);
        // Duel.Destroy(sng,REASON_EFFECT)
        duel::destroy(d, sng, REASON_EFFECT, None, None);
    }
}
