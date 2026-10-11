//! K9-66X "Jacks"
//! Ｋ９－６６６号 “Ｊａｃｋｓ”
use super::prelude::*;

pub const ID: u32 = 67515699;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_K9], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 5 monsters
    // Xyz.AddProcedure(c,nil,5,2)
    xyz::add_procedure(d, c, None, Some(5), 2, None, None, None, None, false, None);
    // Destroy 1 monster on the field
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e1a, CATEGORY_DESTROY);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SPSUMMON_SUCCESS);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e1a, Some(lua_fn(cond_xyz_summoned)));
    // e1a:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1a, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1a:SetTarget(s.destg)
    effect::set_target(d, e1a, Some(cost_target(destg)));
    // e1a:SetOperation(s.desop)
    effect::set_operation(d, e1a, Some(operation(desop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1b:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1b, EVENT_CHAINING);
    // e1b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1b, LOCATION_MZONE);
    // e1b:SetCondition(s.descon)
    effect::set_condition(d, e1b, Some(condition(descon)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // A "K9" Xyz Monster that has this card as material gains this effect: ...
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_XMATERIAL)
    effect::set_type(d, e2, EFFECT_TYPE_XMATERIAL);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.doubledamcon)
    effect::set_condition(d, e2, Some(lua_fn(doubledamcon)));
    // e2:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e2, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Keep track of monster effects activated in the hand or GY
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,s.chainfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(chainfilter));
}
/// `function(e) return e:GetHandler():IsXyzSummoned() end`
fn cond_xyz_summoned(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_xyz_summoned(d, h)
}
/// `s.chainfilter(re,tp,cid)`
fn chainfilter(d: &mut Duel, a: Args) -> bool {
    // return not (re:IsMonsterEffect() and re:GetActivateLocation()&(LOCATION_HAND|LOCATION_GRAVE)>0)
    let re = a.e(0);
    !(effect::is_monster_effect(d, re) && (effect::get_activate_location(d, re) & (LOCATION_HAND | LOCATION_GRAVE)) > 0)
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local trig_loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let trig_loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // return ep==1-tp and re:IsMonsterEffect() and trig_loc&(LOCATION_HAND|LOCATION_GRAVE)>0
    a.ep == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && (trig_loc & (LOCATION_HAND | LOCATION_GRAVE) as i64) > 0
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsMonster() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_monster(d, tc) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
/// `s.doubledamcon(e)`
fn doubledamcon(d: &mut Duel, a: Args) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e(0)).nil();
    // return c:IsSetCard(SET_K9) and c:IsXyzMonster() and Duel.GetCustomActivityCount(id,1-e:GetHandlerPlayer(),ACTIVITY_CHAIN)>0
    let p = 1 - effect::get_handler_player(d, a.e(0));
    card::is_set_card(d, c, &[SET_K9 as u16]) && card::is_xyz_monster(d, c) && duel::get_custom_activity_count(d, ID, p, ACTIVITY_CHAIN) > 0
}
