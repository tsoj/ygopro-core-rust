//! Invoked Mechaba
use super::prelude::*;

pub const ID: u32 = 75286621;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Fusion Summon procedure
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,86120751,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_LIGHT))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(86120751),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, Some(fc), st, Some(tp)))),
        ],
    );
    // Negate and banish
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetCost(s.negcost)
    effect::set_cost(d, e1, Some(cost_target(negcost)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED)
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
        // and (re:IsMonsterEffect() or re:IsHasType(EFFECT_TYPE_ACTIVATE)) and Duel.IsChainNegatable(ev)
        && (effect::is_monster_effect(d, a.re.nil()) || effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE))
        && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.cfilter(c,rtype)`
fn cfilter(d: &mut Duel, c: CardId, rtype: u32) -> bool {
    // return c:IsType(rtype) and c:IsAbleToGraveAsCost()
    card::is_type(d, c, rtype) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.negcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local rtype=(re:GetActiveType()&(TYPE_MONSTER|TYPE_SPELL|TYPE_TRAP))
    let rtype = effect::get_active_type(d, a.re.nil()) & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil,rtype) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| cfilter(d, c, rtype)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,s.cfilter,1,1,REASON_COST,nil,rtype)
    duel::discard_hand(d, a.tp, Some(&|d, c| cfilter(d, c, rtype)), 1, 1, REASON_COST, NO_EX);
    true
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // local relation=rc:IsRelateToEffect(re)
    let relation = card::is_relate_to_effect(d, rc, a.re.nil());
    // if chk==0 then return rc:IsAbleToRemove(tp) or (not relation and Duel.IsPlayerCanRemove(tp)) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, rc, Some(tp), None, None) || (!relation && duel::is_player_can_remove(d, tp, None));
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if relation then
    if relation {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rc,1,rc:GetControler(),rc:GetLocation())
        let controler = card::get_controler(d, rc);
        let location = card::get_location(d, rc);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, controler, location as i32);
    }
    // else
    else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,0,rc:GetPreviousLocation())
        let prev = card::get_previous_location(d, rc);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, 0, prev as i32);
    }
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rc, a.re.nil()) {
            // Duel.Remove(eg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, a.eg(), POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
