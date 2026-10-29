//! Elemental Grace Doriado
use super::prelude::*;

pub const ID: u32 = 32965616;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot special summon
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
    // spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(|d, a: Args| spcon(d, a.e(0), a.card(1)))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atk/def
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // disable special summon
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e5, aux::stringid(ID, 0));
    // e5:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e5, CATEGORY_DISABLE_SUMMON + CATEGORY_DESTROY);
    // e5:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e5, EFFECT_TYPE_QUICK_O);
    // e5:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e5, EVENT_SPSUMMON);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetCondition(s.discon)
    effect::set_condition(d, e5, Some(condition(discon)));
    // e5:SetCost(s.discost)
    effect::set_cost(d, e5, Some(cost_target(discost)));
    // e5:SetTarget(s.distg)
    effect::set_target(d, e5, Some(cost_target(distg)));
    // e5:SetOperation(s.disop)
    effect::set_operation(d, e5, Some(operation(disop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, _e: EffectId, c: Option<CardId>) -> bool {
    // if c==nil then return true end
    let Some(c) = c else { return true };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return false;
    }
    // local g=Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c)), tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // return g:GetClassCount(Card.GetAttribute)>=6
    group::get_class_count(d, &g, &|d, c| card::get_attribute(d, c) as i64) >= 6
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=Duel.GetMatchingGroup(Card.IsMonster,e:GetHandlerPlayer(),LOCATION_GRAVE,LOCATION_GRAVE,nil)
    let p = effect::get_handler_player(d, e);
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c)), p, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // return g:GetClassCount(Card.GetAttribute)*500
    group::get_class_count(d, &g, &|d, c| card::get_attribute(d, c) as i64) as i64 * 500
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return tp~=ep and Duel.GetCurrentChain()==0
    a.tp != a.ep && duel::get_current_chain(d, false) == 0
}
/// `s.discfilter(c)`
fn discfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsAbleToRemoveAsCost()
    card::is_monster(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.discfilter,tp,LOCATION_GRAVE,0,3,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&discfilter), a.tp, LOCATION_GRAVE, 0, 3, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.discfilter,tp,LOCATION_GRAVE,0,3,3,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&discfilter), a.tp, LOCATION_GRAVE, 0, 3, 3, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,0,0)
    let eg = a.eg.clone().nil();
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), eg.len() as u8, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,#eg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(eg.clone()), eg.len() as u8, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, a.eg());
    // Duel.Destroy(eg,REASON_EFFECT)
    duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
}
