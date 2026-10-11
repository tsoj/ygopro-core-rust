//! Sauravis, the Ancient and Ascended
use super::prelude::*;

pub const ID: u32 = 4810828;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[37626500], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Negate the activation of an opponent's effect that targets a monster you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e1, CATEGORY_NEGATE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetCost(Cost.SelfDiscard)
    effect::set_cost(d, e1, Some(cost::self_discard()));
    // e1:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0) end)
    effect::set_target(d, e1, Some(cost_target(e1_target)));
    // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateActivation(ev) end)
    effect::set_operation(d, e1, Some(operation(e1_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate an opponent's Special Summon, and if you do, banish that monster(s)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_DISABLE_SUMMON + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_SPSUMMON)
    effect::set_code(d, e2, EVENT_SPSUMMON);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.negsumcon)
    effect::set_condition(d, e2, Some(condition(negsumcon)));
    // e2:SetCost(Cost.SelfToHand)
    effect::set_cost(d, e2, Some(cost::self_to_hand()));
    // e2:SetTarget(s.negsumtg)
    effect::set_target(d, e2, Some(cost_target(negsumtg)));
    // e2:SetOperation(s.negsumop)
    effect::set_operation(d, e2, Some(operation(negsumop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.negconfilter(c,tp)`
fn negconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_MZONE)
    card::is_controler(d, c, tp) && card::is_location(d, c, LOCATION_MZONE)
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not (rp==1-tp and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) and Duel.IsChainNegatable(ev)) then return false end
    if !(a.rp == 1 - a.tp
        && effect::is_has_property(d, a.re.nil(), EFFECT_FLAG_CARD_TARGET, 0)
        && duel::is_chain_negatable(d, a.ev as u8))
    {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    // return tg and tg:IsExists(s.negconfilter,1,nil,tp)
    let Some(tg) = info[0].as_group().cloned() else { return false };
    let tp = a.tp;
    group::is_exists(d, &tg, &|d, c| negconfilter(d, c, tp), 1, NO_EX)
}
/// `s.negsumcon(e,tp,eg,ep,ev,re,r,rp)`
fn negsumcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and Duel.GetCurrentChain()==0
    a.ep == 1 - a.tp && duel::get_current_chain(d, false) == 0
}
/// `s.negsumtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negsumtg(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    // if chk==0 then return eg:IsExists(Card.IsAbleToRemove,1,nil) end
    if a.chk == Some(0) {
        return group::is_exists(d, &eg, &|d, c| card::is_able_to_remove(d, c, None, None, None), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE_SUMMON,eg,#eg,tp,0)
    let n = eg.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE_SUMMON, Some(eg.clone()), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,eg,#eg,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(eg), n, a.tp, 0);
    true
}
/// `s.negsumop(e,tp,eg,ep,ev,re,r,rp)`
fn negsumop(d: &mut Duel, a: &EffArgs) {
    let eg = a.eg.clone().nil();
    // Duel.NegateSummon(eg)
    duel::negate_summon(d, eg.clone());
    // Duel.Remove(eg,POS_FACEUP,REASON_EFFECT)
    duel::remove(d, eg, POS_FACEUP as u8, REASON_EFFECT, None, None);
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0) end`
fn e1_target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return true;
    }
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp) Duel.NegateActivation(ev) end`
fn e1_op(d: &mut Duel, a: &EffArgs) {
    duel::negate_activation(d, a.ev as u8);
}
