//! VIP Whale
use super::prelude::*;

pub const ID: u32 = 14556954;
const COUNTER_VIP: u16 = 0x21a;
pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x21a], toss_coin: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(COUNTER_VIP)
    card::enable_counter_permit(d, c, COUNTER_VIP, None, None);
    // You can Tribute additional monsters when you Tribute Summon this card face-up
    // aux.AddNormalSummonProcedure(c,true,true,2,12,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0))
    aux::add_normal_summon_procedure(d, c, true, true, Some(2), Some(12), Some(SUMMON_TYPE_TRIBUTE as u32), Some(aux::stringid(ID, 0)), None, None);
    // If this card is Tribute Summoned: You can place VIP Counters on this card equal to the number of monsters Tributed for its Tribute Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e1, CATEGORY_COUNTER);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsTributeSummoned() end)
    effect::set_condition(d, e1, Some(condition(tribute_cond)));
    // e1:SetTarget(s.countertg)
    effect::set_target(d, e1, Some(cost_target(countertg)));
    // e1:SetOperation(s.counterop)
    effect::set_operation(d, e1, Some(operation(counterop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When your opponent activates a card or effect (Quick Effect): You can remove 1 VIP Counter from this card; toss a coin and call it. If you call it right, negate that effect. If you call it wrong, this card's original ATK becomes halved until the end of this turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_COIN+CATEGORY_DISABLE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_COIN + CATEGORY_DISABLE + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end)
    effect::set_condition(d, e2, Some(condition(opp_chain_cond)));
    // e2:SetCost(Cost.RemoveCounterFromSelf(COUNTER_VIP,1))
    effect::set_cost(d, e2, Some(cost::remove_counter_from_self(COUNTER_VIP, 1)));
    // e2:SetTarget(s.cointg)
    effect::set_target(d, e2, Some(cost_target(cointg)));
    // e2:SetOperation(s.coinop)
    effect::set_operation(d, e2, Some(operation(coinop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e) return e:GetHandler():IsTributeSummoned() end
fn tribute_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, c)
}
// function s.countertg(e,tp,eg,ep,ev,re,r,rp,chk)
fn countertg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local trib_monsters_count=c:GetMaterialCount()
    let trib = card::get_material_count(d, c) as i64;
    // if chk==0 then return trib_monsters_count>0 and c:IsCanAddCounter(COUNTER_VIP,trib_monsters_count) end
    if a.chk == Some(0) {
        return trib > 0 && card::is_can_add_counter(d, c, COUNTER_VIP, trib as u16, false, 0);
    }
    // e:SetLabel(trib_monsters_count)
    effect::set_label(d, a.e, &[trib]);
    // Duel.SetOperationInfo(0,CATEGORY_COUNTER,c,trib_monsters_count,tp,COUNTER_VIP)
    duel::set_operation_info(d, 0, CATEGORY_COUNTER, Some(CardSet::from_card(c)), trib as u8, a.tp, COUNTER_VIP as i32);
    true
}
// function s.counterop(e,tp,eg,ep,ev,re,r,rp)
fn counterop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // c:AddCounter(COUNTER_VIP,e:GetLabel())
        let label = effect::get_label(d, a.e) as u16;
        card::add_counter(d, c, COUNTER_VIP, label, false);
    }
}
// function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp end
fn opp_chain_cond(_d: &mut Duel, a: &EffArgs) -> bool {
    a.rp == 1 - a.tp
}
// function s.cointg(e,tp,eg,ep,ev,re,r,rp,chk)
fn cointg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_COIN,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_COIN, None, 0, a.tp, 1);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_ATKCHANGE,e:GetHandler(),1,tp,0)
    let c = effect::get_handler(d, a.e).nil();
    duel::set_possible_operation_info(d, 0, CATEGORY_ATKCHANGE, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.coinop(e,tp,eg,ep,ev,re,r,rp)
fn coinop(d: &mut Duel, a: &EffArgs) {
    // if Duel.CallCoin(tp) then
    if duel::call_coin(d, a.tp) {
        // If you call it right, negate that effect
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    } else {
        // If you call it wrong, this card's original ATK becomes halved until the end of this turn
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if c:IsRelateToEffect(e) and c:IsFaceup() then
        if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
            // This card's original ATK becomes halved until the end of this turn
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_BASE_ATTACK)
            effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
            // e1:SetValue(c:GetBaseAttack()/2)
            let base = card::get_base_attack(d, c);
            effect::set_value(d, e1, lua_int(base as f64 / 2.0));
            // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
