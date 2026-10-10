//! White Night Dragon
use super::prelude::*;

pub const ID: u32 = 79473793;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // negate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_QUICK_F)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_F);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e1, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // change battle target
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.cbcon)
    effect::set_condition(d, e2, Some(condition(cbcon)));
    // e2:SetCost(s.cbcost)
    effect::set_cost(d, e2, Some(cost_target(cbcost)));
    // e2:SetTarget(s.cbtg)
    effect::set_target(d, e2, Some(cost_target(cbtg)));
    // e2:SetOperation(s.cbop)
    effect::set_operation(d, e2, Some(operation(cbop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    let re = a.re.nil();
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local g=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS])
        .first()
        .and_then(|v| v.as_group().cloned());
    // if not g or not g:IsContains(c) then return false end
    match g {
        Some(g) if group::is_contains(&g, c) => {}
        _ => return false,
    }
    // return re:IsHasType(EFFECT_TYPE_ACTIVATE)
    effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    // if re:GetHandler():IsRelateToEffect(re) and re:GetHandler():IsDestructable() then
    if card::is_relate_to_effect(d, rh, re) && card::is_destructable(d, rh, None) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(re:GetHandler(),REASON_EFFECT)
            duel::destroy(d, rh, REASON_EFFECT, None, None);
        }
    }
}
/// `s.cbcon(e,tp,eg,ep,ev,re,r,rp)`
fn cbcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bt=eg:GetFirst()
    let eg = a.eg.clone().nil();
    let bt = group::get_first(d, &eg).nil();
    // return r~=REASON_REPLACE and c~=bt and bt:IsFaceup() and bt:GetControler()==c:GetControler()
    a.r != REASON_REPLACE
        && c != bt
        && card::is_faceup(d, bt)
        && card::get_controler(d, bt) == card::get_controler(d, c)
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToGraveAsCost()
    card::is_spell_trap(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.cbcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cbcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.cbtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cbtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetAttacker():GetAttackableTarget():IsContains(e:GetHandler()) end
    if a.chk == Some(0) {
        let attacker = duel::get_attacker(d).nil();
        let (targets, _) = card::get_attackable_target(d, attacker);
        let h = effect::get_handler(d, a.e).nil();
        return group::is_contains(&targets, h);
    }
    true
}
/// `s.cbop(e,tp,eg,ep,ev,re,r,rp)`
fn cbop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and not Duel.GetAttacker():IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        let attacker = duel::get_attacker(d).nil();
        if !card::is_immune_to_effect(d, attacker, a.e) {
            // Duel.ChangeAttackTarget(c)
            duel::change_attack_target(d, Some(c), false);
        }
    }
}
