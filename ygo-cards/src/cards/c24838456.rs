//! Vanity's Call
use super::prelude::*;

pub const ID: u32 = 24838456;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetCurrentChain(true)>=3
    duel::get_current_chain(d, true) >= 3
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,math.floor(Duel.GetLP(tp)/2))
    let lp = duel::get_lp(d, a.tp);
    duel::pay_lp_cost(d, a.tp, lp.div_euclid(2) as u32);
    true
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local ng=Group.CreateGroup()
    let mut ng = CardSet::new();
    // local dg=Group.CreateGroup()
    let mut dg = CardSet::new();
    // for i=1,ev do
    for i in 1..=a.ev {
        // local te=Duel.GetChainInfo(i,CHAININFO_TRIGGERING_EFFECT)
        let te = duel::get_chain_info(d, i as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
        // local tc=te:GetHandler()
        let tc = effect::get_handler(d, te).nil();
        // if te:IsHasType(EFFECT_TYPE_ACTIVATE) or te:IsMonsterEffect() then
        if effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE) || effect::is_monster_effect(d, te) {
            // ng:AddCard(tc)
            group::add_card(&mut ng, &CardSet::from_card(tc));
            // if tc:IsRelateToEffect(te) then
            if card::is_relate_to_effect(d, tc, te) {
                // dg:AddCard(tc)
                group::add_card(&mut dg, &CardSet::from_card(tc));
            }
        }
    }
    // if chk==0 then return #ng>0 end
    if a.chk == Some(0) {
        return !ng.is_empty();
    }
    // Duel.SetTargetCard(dg)
    duel::set_target_card(d, &dg);
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,ng,#ng,0,0)
    let ng_len = ng.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, Some(ng), ng_len, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,#dg,0,0)
    let dg_len = dg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), dg_len, 0, 0);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local dg=Group.CreateGroup()
    let mut dg = CardSet::new();
    // for i=1,ev do
    for i in 1..=a.ev {
        // local te=Duel.GetChainInfo(i,CHAININFO_TRIGGERING_EFFECT)
        let te = duel::get_chain_info(d, i as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
        // local tc=te:GetHandler()
        let tc = effect::get_handler(d, te).nil();
        // if (te:IsHasType(EFFECT_TYPE_ACTIVATE) or te:IsMonsterEffect()) and Duel.NegateActivation(i)
        //     and tc:IsRelateToEffect(e) and tc:IsRelateToEffect(te) then
        if (effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE) || effect::is_monster_effect(d, te))
            && duel::negate_activation(d, i as u8)
            && card::is_relate_to_effect(d, tc, a.e)
            && card::is_relate_to_effect(d, tc, te)
        {
            // dg:AddCard(tc)
            group::add_card(&mut dg, &CardSet::from_card(tc));
        }
    }
    // Duel.Destroy(dg,REASON_EFFECT)
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
