//! Metal Illusionist
use super::prelude::*;

pub const ID: u32 = 29157292;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_MAX_METALMORPH],
    listed_series: &[SET_METALMORPH],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // This card's Level is increased by 4 during your opponent's turn only
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_LEVEL)
    effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return Duel.IsTurnPlayer(1-e:GetHandlerPlayer()) end)
    effect::set_condition(d, e1, Some(condition(cond1)));
    // e1:SetValue(4)
    effect::set_value(d, e1, 4);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 1 "Metalmorph" Trap from your Deck
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_DRAW + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Declare 1 Type to make this card that Type
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e4, EFFECT_TYPE_IGNITION);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e4, 1, ID, 1, 0);
    // e4:SetTarget(s.changetypetg)
    effect::set_target(d, e4, Some(cost_target(changetypetg)));
    // e4:SetOperation(s.changetypeop)
    effect::set_operation(d, e4, Some(operation(changetypeop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e) return Duel.IsTurnPlayer(1-e:GetHandlerPlayer()) end`
fn cond1(d: &mut Duel, a: &EffArgs) -> bool {
    let hp = effect::get_handler_player(d, a.e);
    duel::is_turn_player(d, 1 - hp)
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_METALMORPH) and c:IsTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_METALMORPH as u16]) && card::is_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_possible_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SSet(tp,g)>0 and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_MAX_METALMORPH)
    //     and Duel.IsPlayerCanDraw(tp,1) then
    if !g.is_empty()
        && duel::s_set(d, a.tp, &g, None, true) > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_MAX_METALMORPH])), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_player_can_draw(d, a.tp, 1)
    {
        // Duel.ShuffleDeck(tp)
        duel::shuffle_deck(d, a.tp);
        // if Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, a.tp, 1, REASON_EFFECT);
        }
    }
}
/// `s.changetypetg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn changetypetg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:SetLabel(e:GetHandler():AnnounceAnotherRace(tp))
    let h = effect::get_handler(d, a.e).nil();
    let race = card::announce_another_race(d, h, a.tp);
    effect::set_label(d, a.e, &[race as i64]);
    true
}
/// `s.changetypeop(e,tp,eg,ep,ev,re,r,rp)`
fn changetypeop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // This card becomes that Type until the end of your opponent's turn
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_CHANGE_RACE)
        effect::set_code(d, e1, EFFECT_CHANGE_RACE);
        // e1:SetValue(e:GetLabel())
        let label = effect::get_label(d, a.e);
        effect::set_value(d, e1, label);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END|RESET_OPPO_TURN)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END | RESET_OPPO_TURN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
