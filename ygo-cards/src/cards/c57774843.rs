//! Judgment Dragon
use super::prelude::*;

pub const ID: u32 = 57774843;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_LIGHTSWORN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(Cost.PayLP(1000))
    effect::set_cost(d, e2, Some(cost::pay_lp(cost::PayLp::Amount(1000))));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // discard deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCategory(CATEGORY_DECKDES)
    effect::set_category(d, e3, CATEGORY_DECKDES);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.condition2)
    effect::set_condition(d, e3, Some(condition(condition2)));
    // e3:SetTarget(s.target2)
    effect::set_target(d, e3, Some(cost_target(target2)));
    // e3:SetOperation(s.operation2)
    effect::set_operation(d, e3, Some(operation(operation2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // splimit
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE);
    // e4:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e4, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e4:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e4, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_LIGHTSWORN) and c:IsMonster()
    card::is_set_card(d, c, &[SET_LIGHTSWORN as u16]) && card::is_monster(d, c)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let tp = card::get_controler(d, c);
    // if Duel.GetLocationCount(c:GetControler(),LOCATION_MZONE)<=0 then return false end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,c:GetControler(),LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&spfilter), tp, LOCATION_GRAVE, 0, NO_EX);
    // local ct=g:GetClassCount(Card.GetCode)
    let ct = group::get_class_count(d, &g, &|d, x| card::get_code(d, x)[0] as i64);
    // return ct>3
    ct > 3
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, c.into());
    }
    // local sg=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,c)
    let sg = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,sg,#sg,0,0)
    let n = sg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(sg), n, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local sg=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let sg = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // Duel.Destroy(sg,REASON_EFFECT)
    duel::destroy(d, sg, REASON_EFFECT, None, None);
}
/// `s.condition2(e,tp,eg,ep,ev,re,r,rp)`
fn condition2(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp==Duel.GetTurnPlayer()
    a.tp == duel::get_turn_player(d)
}
/// `s.target2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target2(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,nil,0,tp,4)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, a.tp, 4);
    true
}
/// `s.operation2(e,tp,eg,ep,ev,re,r,rp)`
fn operation2(d: &mut Duel, a: &EffArgs) {
    // Duel.DiscardDeck(tp,4,REASON_EFFECT)
    duel::discard_deck(d, a.tp, 4, REASON_EFFECT);
}
