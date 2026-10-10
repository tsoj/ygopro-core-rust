//! Demise, Supreme King of Armageddon
use super::prelude::*;

pub const ID: u32 = 59913418;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[32828635, 72426662], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Change name
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_HAND);
    // e1:SetValue(72426662)
    effect::set_value(d, e1, 72426662);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent battle destruction
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetCondition(s.indcon)
    effect::set_condition(d, e2, Some(condition(indcon)));
    // e2:SetTarget(s.indtg)
    effect::set_target(d, e2, Some(effect_card_fn(indtg)));
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Destroy cards
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(Cost.PayLP(2000))
    effect::set_cost(d, e3, Some(cost::pay_lp(cost::PayLp::Amount(2000))));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Cost Change
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_LPCOST_CHANGE)
    effect::set_code(d, e4, EFFECT_LPCOST_CHANGE);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // e4:SetCondition(s.costcon)
    effect::set_condition(d, e4, Some(condition(costcon)));
    // e4:SetValue(s.costchange)
    effect::set_value_fn(d, e4, lua_fn(costchange));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.indcon(e,tp,eg,ep,ev,re,r,rp)`
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsRitualSummoned()
    card::is_ritual_summoned(d, c)
}
/// `s.indtg(e,c)`
fn indtg(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsType(TYPE_RITUAL)
    card::is_type(d, c, TYPE_RITUAL)
}
/// `s.mfilter1(c)`
fn mfilter1(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsType(TYPE_RITUAL)
    !card::is_type(d, c, TYPE_RITUAL)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()  (used in the Lua below as e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into());
    }
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let g = duel::get_matching_group(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // local ct=g:FilterCount(Card.IsControler,nil,1-tp)
    let tp = a.tp;
    let ct = group::filter_count(d, &g, &|d, x| card::is_controler(d, x, 1 - tp), NO_EX) as i64;
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,ct*200)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, (ct * 200) as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let g = duel::get_matching_group(d, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, h.into());
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
    // local ct=Duel.GetOperatedGroup():FilterCount(function(c,tp) return c:IsPreviousControler(tp) end,nil,1-tp)
    let operated = duel::get_operated_group(d);
    let ct = group::filter_count(d, &operated, &|d, x| card::is_previous_controler(d, x, 1 - tp), NO_EX) as i64;
    // if ct>0 then
    if ct > 0 {
        // Duel.Damage(1-tp,ct*200,REASON_EFFECT)
        duel::damage(d, 1 - tp, ct * 200, REASON_EFFECT, false, None);
    }
}
/// `s.costcon(e)`
fn costcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local mg=c:GetMaterial()
    let mg = card::get_material(d, c);
    // return c:GetSummonType()==SUMMON_TYPE_RITUAL and #mg>0 and not mg:IsExists(s.mfilter1,1,nil)
    card::get_summon_type(d, c) == SUMMON_TYPE_RITUAL as u32 && !mg.is_empty() && !group::is_exists(d, &mg, &mfilter1, 1, NO_EX)
}
/// `s.costchange(e,re,rp,val)`
fn costchange(d: &mut Duel, a: Args) -> i64 {
    // if re and re:IsActivated() and re:GetHandler()==e:GetHandler() then
    let e = a.e(0);
    if let Some(re) = a.v(1).as_effect() {
        if effect::is_activated(d, re) {
            let rh = effect::get_handler(d, re);
            if rh == effect::get_handler(d, e) {
                // return 0
                return 0;
            }
        }
    }
    // else return val end
    a.int(3)
}
