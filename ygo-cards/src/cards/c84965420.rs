//! Drytron Meteor Shower
//! ドライトロン流星群
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 84965420;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Negate and summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_TODECK)
    effect::set_category(d, e1, CATEGORY_DISABLE_SUMMON + CATEGORY_TODECK);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_SUMMON)
    effect::set_code(d, e1, EVENT_SUMMON);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e2, EVENT_SPSUMMON);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)==0 and eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRitualMonster),tp,LOCATION_MZONE,0,1,nil)
    let tp = a.tp;
    if duel::get_current_chain(d, true) != 0 {
        return false;
    }
    let eg = a.eg.clone().nil();
    if !group::is_exists(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), 1, NO_EX) {
        return false;
    }
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_ritual_monster(d, c)), tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    let tp = a.tp;
    // local g=eg:Filter(Card.IsControler,nil,1-tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| card::is_controler(d, c, 1 - tp), NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(g.clone()), g.len() as u8, 0, 0);
    // local g2=g:Filter(Card.IsPreviousLocation,nil,LOCATION_EXTRA)
    let g2 = group::filter(d, &g, &|d, c| card::is_previous_location(d, c, LOCATION_EXTRA), NO_EX);
    // if #g2>0 then
    if !g2.is_empty() {
        // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,g2,#g2,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, Some(g2.clone()), g2.len() as u8, 0, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=eg:Filter(Card.IsSummonPlayer,nil,1-tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| card::is_summon_player(d, c, 1 - tp), NO_EX);
    // Duel.NegateSummon(g)
    duel::negate_summon(d, g.clone());
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
