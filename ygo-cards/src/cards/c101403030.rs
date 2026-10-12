//! The Sun God of Darkness - The Winged Dragon of Ra
//! Scripted by Eerie Code
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 101403030;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_RA],
    listed_series: &[SET_SUN_GOD],
    material_setcode: &[SET_THE_WINGED_DRAGON_OF_RA, SET_SUN_GOD],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: "The Winged Dragon of Ra" + 3 "Sun God" monsters
    // Fusion.AddProcMixN(c,true,true,CARD_RA,1,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_SUN_GOD),3)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![
            (fusion::Mat::Code(CARD_RA), 1),
            (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_SUN_GOD as u16], Some(fc), st, Some(tp)))), 3),
        ],
    );
    // Unaffected by other cards' effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,te) return te:GetOwner()~=e:GetOwner() end)
    effect::set_value_fn(
        d,
        e1,
        lua_fn(|d: &mut Duel, a: Args| effect::get_owner(d, a.e(1)) != effect::get_owner(d, a.e(0))),
    );
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Once per turn, during the Main Phase (Quick Effect): You can discard any number of "Sun God" cards, then choose that many of your opponent's occupied Main Monster Zones; your opponent must send the monsters in those zones to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e2:SetCost(Cost.Discard(function(c) return c:IsSetCard(SET_SUN_GOD) end,nil,1,function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_MMZONE) end))
    effect::set_cost(
        d,
        e2,
        Some(cost::discard(
            Some(Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, _tp: u8| card::is_set_card(d, c, &[SET_SUN_GOD as u16]))),
            false,
            Some(cost::Count::N(1)),
            Some(cost::Count::F(Arc::new(|d: &mut Duel, _e: EffectId, tp: u8| {
                duel::get_field_group_count(d, tp, 0, LOCATION_MMZONE) as i64
            }))),
            None,
        )),
    );
    // e2:SetTarget(s.gytg)
    effect::set_target(d, e2, Some(cost_target(gytg)));
    // e2:SetOperation(s.gyop)
    effect::set_operation(d, e2, Some(operation(gyop)));
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e2, 0, Some((TIMING_MAIN_END | TIMINGS_CHECK_MONSTER) as u32));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // When an attack is declared involving this card: You can pay LP so that you only have 1 left; this card gains ATK/DEF equal to the amount of LP paid and the total ATK of all "Sun God" monsters currently in your GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_ATKCHANGE+CATEGORY_DEFCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE + CATEGORY_DEFCHANGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return e:GetHandler():IsRelateToBattle() end)
    effect::set_condition(
        d,
        e3,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_relate_to_battle(d, h)
        })),
    );
    // e3:SetCost(Cost.PayLP(1,true))
    effect::set_cost(d, e3, Some(cost::pay_lp(cost::PayLp::Until(1))));
    // e3:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e3, Some(operation(op_e3)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local mg=Duel.GetFieldGroup(tp,0,LOCATION_MMZONE)
    let mg = duel::get_field_group(d, tp, 0, LOCATION_MMZONE);
    // local filter=0
    let mut filter: u32 = 0;
    // for mc in mg:Iter() do
    for mc in group::iter(d, &mg) {
        // filter=filter|(1<<(mc:GetSequence()+16))
        filter |= 1u32 << (card::get_sequence(d, mc) + 16);
    }
    // local cd=e:GetChainData()
    // local cost_discarded_cards_count=#cd.cost_discarded_cards
    let cost_discarded_cards_count = chain::effect_data(d, e, None, chain::field::COST_DISCARDED_CARDS).as_group().cloned().nil().len();
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local chosen_zones=Duel.SelectFieldZone(tp,cost_discarded_cards_count,0,LOCATION_MZONE,~filter)
    let chosen_zones = duel::select_field_zone(d, tp, cost_discarded_cards_count as u8, 0, LOCATION_MZONE, Some(!filter));
    // Duel.Hint(HINT_ZONE,tp,chosen_zones)
    duel::hint(d, HINT_ZONE as u8, tp, chosen_zones as u64);
    // cd.chosen_zones=chosen_zones
    chain::set_effect_data(d, e, None, chain::field::CARD, Value::Int(chosen_zones as i64));
    true
}
// function s.gyop(e,tp,eg,ep,ev,re,r,rp)
fn gyop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local chosen_zones=e:GetChainData().chosen_zones
    let chosen_zones = chain::effect_data(d, e, None, chain::field::CARD).int() as u32;
    // local g=Duel.GetMatchingGroup(aux.IsZone,tp,0,LOCATION_MMZONE,nil,chosen_zones,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| aux::is_zone(d, c, chosen_zones, tp)), tp, 0, LOCATION_MMZONE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_RULE,nil,1-tp)
        duel::send_to_grave(d, g, REASON_RULE, None, Some(1 - tp));
    }
}
// function(e,tp,eg,ep,ev,re,r,rp) (the operation of e3)
fn op_e3(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, e) && card::is_faceup(d, c) {
        // local cost_lp_paid=e:GetChainData().cost_lp_paid
        let cost_lp_paid = chain::effect_data(d, e, None, chain::field::COST_LP_PAID).int();
        // local total_gy_atk=Duel.GetMatchingGroup(Card.IsSetCard,tp,LOCATION_GRAVE,0,nil,SET_SUN_GOD):GetSum(Card.GetAttack)
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_set_card(d, x, &[SET_SUN_GOD as u16])), tp, LOCATION_GRAVE, 0, NO_EX);
        let total_gy_atk = group::get_sum(d, &g, &|d, x| card::get_attack(d, x) as i64);
        // local val=cost_lp_paid+total_gy_atk
        let val = cost_lp_paid + total_gy_atk;
        // c:UpdateAttack(val)
        card::update_attack(d, c, val, None, None, None);
        // c:UpdateDefense(val)
        card::update_defense(d, c, val, None, None, None);
    }
}
