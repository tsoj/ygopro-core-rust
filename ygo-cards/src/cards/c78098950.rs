//! Tsumuha-Kutsunagi the Lord of Swords
use super::prelude::*;

pub const ID: u32 = 78098950;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Spirit.AddProcedure(c,EVENT_SUMMON_SUCCESS,EVENT_FLIP)
    spirit::add_procedure(d, c, &[EVENT_SUMMON_SUCCESS, EVENT_FLIP]);
    // Can be Tribute Summoned by Tributing 1 Normal Summoned/Set monster
    // aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),aux.FilterBoolFunction(Card.IsSummonType,SUMMON_TYPE_NORMAL))
    aux::add_normal_summon_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE as u32),
        Some(aux::stringid(ID, 0)),
        Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _tp: u8| card::is_summon_type(d, c, &[SUMMON_TYPE_NORMAL]))),
        None,
    );
    // aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),aux.FilterBoolFunction(Card.IsSummonType,SUMMON_TYPE_NORMAL))
    aux::add_normal_set_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE as u32),
        Some(aux::stringid(ID, 0)),
        Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _tp: u8| card::is_summon_type(d, c, &[SUMMON_TYPE_NORMAL]))),
        None,
    );
    // Cannot be Special Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Opponent can send cards to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_DRAW+CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_DRAW + CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetTarget(s.gytg)
    effect::set_target(d, e2, Some(cost_target(gytg)));
    // e2:SetOperation(s.gyop)
    effect::set_operation(d, e2, Some(operation(gyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_FLIP)
    effect::set_code(d, e3, EVENT_FLIP);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TODECK,nil,1,PLAYER_ALL,LOCATION_REMOVED|LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TODECK, None, 1, PLAYER_ALL as u8, (LOCATION_REMOVED | LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
/// `s.gyop(e,tp,eg,ep,ev,re,r,rp)`
fn gyop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsAbleToGrave,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_able_to_grave(d, c, None)), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 and Duel.IsPlayerCanDraw(tp,1) and Duel.IsPlayerCanDraw(1-tp,1)
    //     and Duel.SelectYesNo(1-tp,aux.Stringid(id,2)) then
    if !g.is_empty()
        && duel::is_player_can_draw(d, tp, 1)
        && duel::is_player_can_draw(d, 1 - tp, 1)
        && duel::select_yes_no(d, 1 - tp, aux::stringid(ID, 2))
    {
        // local deck1=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
        let deck1 = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
        // local deck2=Duel.GetFieldGroupCount(tp,0,LOCATION_DECK)
        let deck2 = duel::get_field_group_count(d, tp, 0, LOCATION_DECK);
        // local ct=math.min(#g,deck1,deck2)
        let ct = (g.len() as i32).min(deck1).min(deck2);
        // Duel.Hint(HINT_SELECTMSG,1-tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, 1 - tp, HINTMSG_TOGRAVE as u64);
        // local sg=g:Select(1-tp,1,ct,nil)
        let sg = group::select(d, &g, 1 - tp, 1, ct as u16, false, NO_EX).nil();
        // local oc=Duel.SendtoGrave(sg,REASON_EFFECT,PLAYER_NONE,1-tp)
        let oc = duel::send_to_grave(d, &sg, REASON_EFFECT, Some(PLAYER_NONE as u8), Some(1 - tp));
        // if oc>0 then
        if oc > 0 {
            // local turn_p=Duel.GetTurnPlayer()
            let turn_p = duel::get_turn_player(d);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(turn_p,oc,REASON_EFFECT)
            duel::draw(d, turn_p, oc as u32, REASON_EFFECT);
            // Duel.Draw(1-turn_p,oc,REASON_EFFECT)
            duel::draw(d, 1 - turn_p, oc as u32, REASON_EFFECT);
        }
    }
    // Shuffle all cards that are banished, on the field, and in the GYs into the Deck
    // local e1=Effect.CreateEffect(e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetOperation(s.tdop)
    effect::set_operation(d, e1, Some(operation(tdop)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local loc=LOCATION_ONFIELD|LOCATION_REMOVED|LOCATION_GRAVE
    let loc = LOCATION_ONFIELD | LOCATION_REMOVED | LOCATION_GRAVE;
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(Card.IsAbleToDeck),tp,loc,loc,nil)
    let f = aux::necro_valley_filter(|d, c| card::is_able_to_deck(d, c));
    let g = duel::get_matching_group(d, Some(&f), tp, loc, loc, NO_EX);
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, &g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
