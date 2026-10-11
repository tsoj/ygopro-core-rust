//! Satellarknight Procyon
use super::prelude::*;

pub const ID: u32 = 99668578;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TELLARKNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 "tellarknight" monster from your hand to the GY, and if you do, draw 1 card
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_HANDES+CATEGORY_DRAW)
    effect::set_category(d, e1a, CATEGORY_HANDES + CATEGORY_DRAW);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetTarget(s.drtg)
    effect::set_target(d, e1a, Some(cost_target(drtg)));
    // e1a:SetOperation(s.drop)
    effect::set_operation(d, e1a, Some(operation(drop_op)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // local e1c=e1a:Clone()
    let e1c = effect::clone(d, e1a);
    // e1c:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1c, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1c)
    card::register_effect(d, c, e1c, false);
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // c:IsSetCard(SET_TELLARKNIGHT) and c:IsMonster() and c:IsAbleToGrave()
    card::is_set_card(d, c, &[SET_TELLARKNIGHT as u16]) && card::is_monster(d, c) && card::is_able_to_grave(d, c, None)
}
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // Excluding itself for a proper interaction with "Tellarknight Constellar Caduceus" [58858807]
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1)
    //   and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_HAND,0,1,nil,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_player_can_draw(d, tp, 1)
            && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_HAND, 0, 1, h.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
fn drop_op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.DiscardHand(tp,s.tgfilter,1,1,REASON_EFFECT)>0 and Duel.GetOperatedGroup():GetFirst():IsLocation(LOCATION_GRAVE) then
    if duel::discard_hand(d, tp, Some(&tgfilter), 1, 1, REASON_EFFECT, NO_EX) > 0 {
        let operated = duel::get_operated_group(d);
        let first = group::get_first(d, &operated).nil();
        if card::is_location(d, first, LOCATION_GRAVE) {
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
    }
}
