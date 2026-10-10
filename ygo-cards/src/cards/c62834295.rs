//! オルフェゴール・アインザッツ (Orcustrated Einsatz)
use super::prelude::*;

pub const ID: u32 = 62834295;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WORLD_LEGACY, SET_ORCUST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send to GY or banish 1 "Orcust" or "World Legacy" monster from your hand/Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.tgcon)
    effect::set_condition(d, e2, Some(condition(tgcon)));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.tgcon(e,tp,eg,ep,ev,re,r,rp)
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    let eg = a.eg.clone().nil();
    let p = 1 - a.tp;
    group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, p), 1, NO_EX)
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard({SET_WORLD_LEGACY,SET_ORCUST}) and c:IsMonster() and (c:IsAbleToGrave() or c:IsAbleToRemove())
    card::is_set_card(d, c, &[SET_WORLD_LEGACY as u16, SET_ORCUST as u16])
        && card::is_monster(d, c)
        && (card::is_able_to_grave(d, c, None) || card::is_able_to_remove(d, c, None, None, None))
}
// function s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK|LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK|LOCATION_HAND)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
// function s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK|LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // local tc=g:GetFirst()
        let tc = group::get_first(d, &g).nil();
        // if tc and tc:IsAbleToGrave() and (not tc:IsAbleToRemove() or Duel.SelectYesNo(tp,aux.Stringid(id,2))) then
        if card::is_able_to_grave(d, tc, None)
            && (!card::is_able_to_remove(d, tc, None, None, None) || duel::select_yes_no(d, tp, aux::stringid(ID, 2)))
        {
            // Duel.SendtoGrave(tc,REASON_EFFECT)
            duel::send_to_grave(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
        } else {
            // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, CardSet::from_card(tc), POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
