//! "Raise Moon" the City that Never Sleeps
use super::prelude::*;

pub const ID: u32 = 100458025;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_RAISE_MOON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // The first time each Level 7 "Raise Moon" monster you control would be destroyed by card effect each turn, it is not destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:IsLevel(7) and c:IsSetCard(SET_RAISE_MOON) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_level(d, c, &[7]) && card::is_set_card(d, c, &[SET_RAISE_MOON as u16]))));
    // e1:SetValue(function(e,re,r,rp) return r&REASON_EFFECT==REASON_EFFECT and 1 or 0 end)
    effect::set_value_fn(d, e1, lua_fn(|_d, a| {
        let r = a.int(2) as u32;
        if r & REASON_EFFECT == REASON_EFFECT { 1i64 } else { 0i64 }
    }));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn, during the End Phase: Shuffle all "Raise Moon" cards in your GY and banishment into the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e2, EVENT_PHASE + PHASE_END);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Once per turn: You can activate 1 of these effects;
    // ● Place 1 card from your hand on the top or bottom of the Deck, then draw 1 card
    // ● If you control a Spellcaster "Raise Moon" Xyz Monster: You can draw 1 card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.efftg)
    effect::set_target(d, e3, Some(cost_target(efftg)));
    // e3:SetOperation(s.effop)
    effect::set_operation(d, e3, Some(operation(effop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.tdfilter(c)`
fn tdfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RAISE_MOON) and c:IsAbleToDeck() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_RAISE_MOON as u16]) && card::is_able_to_deck(d, c) && card::is_faceup(d, c)
}
/// `s.tdtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.tdfilter,tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&tdfilter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, a.tp, 0);
    true
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.tdfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(tdfilter)), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, NO_EX);
    // if #g>0 then Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT) end
    if !g.is_empty() {
        duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
    }
}
/// `s.drconfilter(c)`
fn drconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsSetCard(SET_RAISE_MOON) and c:IsXyzMonster() and c:IsFaceup()
    card::is_race(d, c, RACE_SPELLCASTER) && card::is_set_card(d, c, &[SET_RAISE_MOON as u16]) && card::is_xyz_monster(d, c) && card::is_faceup(d, c)
}
/// `Card.IsAbleToDeck` as a filter
fn able_to_deck(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_deck(d, c)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local option_1=Duel.IsExistingMatchingCard(Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,nil) and Duel.IsPlayerCanDraw(tp)
    let option_1 = duel::is_existing_matching_card(d, Some(&able_to_deck), tp, LOCATION_HAND, 0, 1, NO_EX) && duel::is_player_can_draw(d, tp, 0);
    // local option_2=Duel.IsExistingMatchingCard(s.drconfilter,tp,LOCATION_MZONE,0,1,nil) and Duel.IsPlayerCanDraw(tp,1)
    let option_2 = duel::is_existing_matching_card(d, Some(&drconfilter), tp, LOCATION_MZONE, 0, 1, NO_EX) && duel::is_player_can_draw(d, tp, 1);
    // if chk==0 then return option_1 or option_2 end
    if a.chk == Some(0) {
        return option_1 || option_2;
    }
    // local choice=Duel.SelectEffect(tp, {option_1,aux.Stringid(id,2)}, {option_2,aux.Stringid(id,3)})
    let choice = duel::select_effect(d, tp, &[(option_1, aux::stringid(ID, 2)), (option_2, aux::stringid(ID, 3))]);
    // e:GetChainData().choice=choice
    chain::set_effect_data(d, a.e, None, chain::field::CARD, choice_value(choice));
    // if choice==1 then
    if choice == Some(1) {
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_TODECK + CATEGORY_DRAW);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,1,LOCATION_HAND)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, tp, 1);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    // elseif choice==2 then
    } else if choice == Some(2) {
        // e:SetCategory(CATEGORY_DRAW)
        effect::set_category(d, a.e, CATEGORY_DRAW);
        // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
        duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    }
    true
}
/// Stores the chosen option in the effect's chain data (`e:GetChainData().choice`)
fn choice_value(choice: Option<usize>) -> Value {
    choice.map_or(Value::Nil, |x| Value::Int(x as i64))
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local choice=e:GetChainData().choice
    let choice = chain::effect_data(d, a.e, None, chain::field::CARD).int();
    // if choice==1 then
    if choice == 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
        // local sc=Duel.SelectMatchingCard(tp,Card.IsAbleToDeck,tp,LOCATION_HAND,0,1,1,nil):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&able_to_deck), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &g) else {
            return;
        };
        // local seq_op=0
        let mut seq_op: i32 = 0;
        // if Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0 then
        if duel::get_field_group_count(d, tp, LOCATION_DECK, 0) > 0 {
            // seq_op=Duel.SelectOption(tp,aux.Stringid(id,4),aux.Stringid(id,5))
            seq_op = duel::select_option(d, tp, true, &[aux::stringid(ID, 4), aux::stringid(ID, 5)]);
        }
        // if Duel.SendtoDeck(sc,nil,seq_op,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_DECK) and Duel.IsPlayerCanDraw(tp) then
        if duel::send_to_deck(d, CardSet::from_card(sc), None, seq_op, REASON_EFFECT, None) > 0
            && card::is_location(d, sc, LOCATION_DECK)
            && duel::is_player_can_draw(d, tp, 0)
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
    // elseif choice==2 then
    } else if choice == 2 {
        // Duel.Draw(tp,1,REASON_EFFECT)
        duel::draw(d, tp, 1, REASON_EFFECT);
    }
}
