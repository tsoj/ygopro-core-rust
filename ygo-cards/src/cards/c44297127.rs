//! Miracle Rupture
use super::prelude::*;

pub const ID: u32 = 44297127;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_FOSSIL_FUSION], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsRace(RACE_ROCK) and c:IsLevelBelow(4) and c:IsAbleToGrave()
    card::is_monster(d, c) && card::is_race(d, c, RACE_ROCK) && card::is_level_below(d, c, 4) && card::is_able_to_grave(d, c, None)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil)
    //     and (not Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_FOSSIL_FUSION) or Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>0) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX)
            && (!duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_FOSSIL_FUSION])), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
                || duel::get_field_group_count(d, a.tp, LOCATION_DECK, 0) > 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoGrave(g,REASON_EFFECT) and g:GetFirst():IsLocation(LOCATION_GRAVE) and Duel.IsPlayerCanDraw(tp)
    //     and Duel.IsExistingMatchingCard(Card.IsCode,tp,LOCATION_GRAVE,0,1,nil,CARD_FOSSIL_FUSION) then
    if !g.is_empty() {
        // Duel.SendtoGrave returns a number, which is true in Lua even when it is 0
        duel::send_to_grave(d, g.clone(), REASON_EFFECT, None, None);
        let f = group::get_first(d, &g).nil();
        if card::is_location(d, f, LOCATION_GRAVE)
            && duel::is_player_can_draw(d, a.tp, 0)
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_code(d, c, &[CARD_FOSSIL_FUSION])), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
        {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, a.tp, 1, REASON_EFFECT);
        }
    }
}
