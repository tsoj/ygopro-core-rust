//! The Revived Sky God
use super::prelude::*;

pub const ID: u32 = 59094601;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SLIFER, CARD_MONSTER_REBORN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local EFFECT_FLAG_CANNOT_NEGATE_ACTIV_EFF=EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CANNOT_NEGATE+EFFECT_FLAG_CANNOT_INACTIVATE
    const CANNOT_NEGATE_ACTIV_EFF: u32 = EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CANNOT_NEGATE + EFFECT_FLAG_CANNOT_INACTIVATE;
    // Special Summon 1 "Slifer the Sky Dragon" from your GY, the each player draws until they have 6 cards in their hand
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DRAW);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_property(d, e1, CANNOT_NEGATE_ACTIV_EFF, 0);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);
    // Place 1 "Monster Reborn" from your Deck or GY on top of your Deck, then, if a Divine-Beast monster is in your GY, draw 1 card
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_TODECK + CATEGORY_DRAW);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, CANNOT_NEGATE_ACTIV_EFF, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    effect::set_operation(d, e2, Some(operation(tdop)));
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_SLIFER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[CARD_SLIFER]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    // local ct1=6-Duel.GetMatchingGroupCount(nil,tp,LOCATION_HAND,0,e:GetHandler())
    let ct1 = 6 - duel::get_matching_group_count(d, None, tp, LOCATION_HAND, 0, c.into()) as i64;
    // local ct2=6-Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let ct2 = 6 - duel::get_field_group_count(d, tp, 0, LOCATION_HAND) as i64;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //   and ct1>0 and Duel.IsPlayerCanDraw(tp,ct1)
    //   and ct2>0 and Duel.IsPlayerCanDraw(1-tp,ct2) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && ct1 > 0
            && duel::is_player_can_draw(d, tp, ct1 as u32)
            && ct2 > 0
            && duel::is_player_can_draw(d, 1 - tp, ct2 as u32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,ct1+ct2)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, (ct1 + ct2) as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // local turn_p=Duel.GetTurnPlayer()
        let turn_p = duel::get_turn_player(d);
        // local ct1=6-Duel.GetFieldGroupCount(turn_p,LOCATION_HAND,0)
        let ct1 = 6 - duel::get_field_group_count(d, turn_p, LOCATION_HAND, 0) as i64;
        // local ct2=6-Duel.GetFieldGroupCount(turn_p,0,LOCATION_HAND)
        let ct2 = 6 - duel::get_field_group_count(d, turn_p, 0, LOCATION_HAND) as i64;
        // if (ct1<=0 and ct2<=0) or not (Duel.IsPlayerCanDraw(tp) or Duel.IsPlayerCanDraw(1-tp)) then return end
        if (ct1 <= 0 && ct2 <= 0) || !(duel::is_player_can_draw(d, tp, 0) || duel::is_player_can_draw(d, 1 - tp, 0)) {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if ct1>0 then Duel.Draw(turn_p,ct1,REASON_EFFECT) end
        if ct1 > 0 {
            duel::draw(d, turn_p, ct1 as u32, REASON_EFFECT);
        }
        // if ct2>0 then Duel.Draw(1-turn_p,ct2,REASON_EFFECT) end
        if ct2 > 0 {
            duel::draw(d, 1 - turn_p, ct2 as u32, REASON_EFFECT);
        }
    }
}
/// `s.tdfilter(c,deck_count)`
fn tdfilter(d: &mut Duel, c: CardId, deck_count: i32) -> bool {
    // if not c:IsCode(CARD_MONSTER_REBORN) then return false end
    if !card::is_code(d, c, &[CARD_MONSTER_REBORN]) {
        return false;
    }
    // local is_in_deck=c:IsLocation(LOCATION_DECK)
    let is_in_deck = card::is_location(d, c, LOCATION_DECK);
    // return (is_in_deck and deck_count>1) or (not is_in_deck and c:IsAbleToDeck())
    (is_in_deck && deck_count > 1) || (!is_in_deck && card::is_able_to_deck(d, c))
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local deck_count=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let deck_count = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tdfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,deck_count) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| tdfilter(d, c, deck_count)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, (LOCATION_GRAVE | LOCATION_DECK) as i32);
    // if Duel.IsExistingMatchingCard(Card.IsRace,tp,LOCATION_GRAVE,0,1,nil,RACE_DIVINE) then
    if duel::is_existing_matching_card(d, Some(&|d, c| card::is_race(d, c, RACE_DIVINE)), tp, LOCATION_GRAVE, 0, 1, NO_EX) {
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local deck_count=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let deck_count = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil,deck_count):GetFirst()
    let nv = aux::necro_valley_filter(|d, c| tdfilter(d, c, deck_count));
    let sel = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sel) else {
        return;
    };
    // if sc:IsLocation(LOCATION_DECK) then
    if card::is_location(d, sc, LOCATION_DECK) {
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, tp);
        // Duel.MoveToDeckTop(sc)
        duel::move_to_deck_top(d, sc);
    } else {
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // Duel.SendtoDeck(sc,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, sc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
    // if not ((sc:IsLocation(LOCATION_DECK) and Duel.GetDecktopGroup(tp,1):IsContains(sc)) or sc:IsLocation(LOCATION_EXTRA)) then return end
    let decktop = duel::get_decktop_group(d, tp, 1);
    if !((card::is_location(d, sc, LOCATION_DECK) && group::is_contains(&decktop, sc)) || card::is_location(d, sc, LOCATION_EXTRA)) {
        return;
    }
    // Duel.ConfirmCards(1-tp,sc)
    duel::confirm_cards(d, 1 - tp, sc);
    // if Duel.IsPlayerCanDraw(tp) and Duel.IsExistingMatchingCard(Card.IsRace,tp,LOCATION_GRAVE,0,1,nil,RACE_DIVINE) then
    if duel::is_player_can_draw(d, tp, 0) && duel::is_existing_matching_card(d, Some(&|d, c| card::is_race(d, c, RACE_DIVINE)), tp, LOCATION_GRAVE, 0, 1, NO_EX) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
