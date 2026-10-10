//! Punishment Dragon
use super::prelude::*;

pub const ID: u32 = 19959563;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_LIGHTSWORN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER_E as u32));
    // e2:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Send the top 4 cards to the GY
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e4, CATEGORY_DECKDES);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCondition(s.ddcon)
    effect::set_condition(d, e4, Some(condition(ddcon)));
    // e4:SetTarget(s.ddtg)
    effect::set_target(d, e4, Some(cost_target(ddtg)));
    // e4:SetOperation(s.ddop)
    effect::set_operation(d, e4, Some(operation(ddop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Special Summon limit
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e5, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e5:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e5, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
// function s.spfilter(c)
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_LIGHTSWORN) and c:IsMonster()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]) && card::is_monster(d, c)
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let p = card::get_controler(d, c);
    // if Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)<=0 then return false end
    if duel::get_location_count(d, p, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,c:GetControler(),LOCATION_REMOVED,0,nil)
    let g = duel::get_matching_group(d, Some(&spfilter), p, LOCATION_REMOVED, 0, NO_EX);
    // local ct=g:GetClassCount(Card.GetCode)
    let ct = group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64);
    // return ct>3
    ct > 3
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsFaceup() or c:IsLocation(LOCATION_GRAVE)) and not (c:IsSetCard(SET_LIGHTSWORN) and c:IsMonster()) and c:IsAbleToDeck()
    (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_GRAVE))
        && !(card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]) && card::is_monster(d, c))
        && card::is_able_to_deck(d, c)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE|LOCATION_REMOVED,LOCATION_GRAVE|LOCATION_REMOVED,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, LOCATION_GRAVE | LOCATION_REMOVED, 1, Exclude::Card(h));
    }
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE|LOCATION_REMOVED,LOCATION_GRAVE|LOCATION_REMOVED,e:GetHandler())
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, LOCATION_GRAVE | LOCATION_REMOVED, Exclude::Card(h));
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(g), n, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.filter,tp,LOCATION_GRAVE|LOCATION_REMOVED,LOCATION_GRAVE|LOCATION_REMOVED,nil)
    let g = duel::get_matching_group(d, Some(&filter), a.tp, LOCATION_GRAVE | LOCATION_REMOVED, LOCATION_GRAVE | LOCATION_REMOVED, NO_EX);
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, g, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
// function s.ddcon(e,tp,eg,ep,ev,re,r,rp)
fn ddcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // return re:IsMonsterEffect() and rc~=c
    //     and rc:IsSetCard(SET_LIGHTSWORN) and rc:IsControler(tp)
    effect::is_monster_effect(d, re) && rc != c && card::is_set_card(d, rc, &[SET_LIGHTSWORN as u16]) && card::is_controler(d, rc, a.tp)
}
// function s.ddtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn ddtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,4)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 4);
    true
}
// function s.ddop(e,tp,eg,ep,ev,re,r,rp)
fn ddop(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardDeck(tp,4,REASON_EFFECT)
    duel::discard_deck(d, a.tp, 4, REASON_EFFECT);
}
