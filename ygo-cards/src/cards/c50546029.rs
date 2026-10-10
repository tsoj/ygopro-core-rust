//! G Golem Dignified Trilithon
use super::prelude::*;

pub const ID: u32 = 50546029;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Link Summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_EARTH),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_EARTH, sc, st, Some(tp)))), 2, None, None, None, None);
    // Must attack this card, if able
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e1, EFFECT_MUST_ATTACK);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    effect::set_code(d, e2, EFFECT_MUST_ATTACK_MONSTER);
    // e2:SetValue(function(e,c) return c==e:GetHandler() end)
    effect::set_value_fn(d, e2, effect_card_fn(|d, e, c| effect::get_handler(d, e) == Some(c)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Reduce ATK and negate
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DISABLE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_DISABLE + CATEGORY_ATKCHANGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_CONFIRM)
    effect::set_code(d, e3, EVENT_BATTLE_CONFIRM);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetCost(s.atkcost)
    effect::set_cost(d, e3, Some(cost_target(atkcost)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Negate and destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DISABLE+CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DISABLE + CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_CHAINING)
    effect::set_code(d, e4, EVENT_CHAINING);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetCondition(s.discon)
    effect::set_condition(d, e4, Some(condition(discon)));
    // e4:SetTarget(s.distg)
    effect::set_target(d, e4, Some(cost_target(distg)));
    // e4:SetOperation(s.disop)
    effect::set_operation(d, e4, Some(operation(disop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local _,bc=Duel.GetBattleMonster(tp)
    let (_, bc) = duel::get_battle_monster(d, a.tp);
    // if not bc then return false end
    let Some(bc) = bc else { return false };
    // e:SetLabelObject(bc)
    effect::set_label_object(d, a.e, Value::Card(bc));
    // return true
    true
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAttribute(ATTRIBUTE_EARTH) and c:IsAbleToGraveAsCost()
    card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX);
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g.nil(), REASON_COST, None, None);
    true
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local d=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card();
    // Duel.SetOperationInfo(0,CATEGORY_ATKCHANGE,d,1,1-tp,-200)
    duel::set_operation_info(d, 0, CATEGORY_ATKCHANGE, bc.map(CardSet::from_card), 1, 1 - a.tp, -200);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,d,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, bc.map(CardSet::from_card), 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local d=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card();
    // if d and d:IsRelateToBattle() and d:IsFaceup() and d:IsControler(1-tp) then
    let Some(bc) = bc else { return };
    if card::is_relate_to_battle(d, bc) && card::is_faceup(d, bc) && card::is_controler(d, bc, 1 - a.tp) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Reduce ATK
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(-200)
        effect::set_value(d, e1, -200);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // d:RegisterEffect(e1)
        card::register_effect(d, bc, e1, false);
        // Duel.NegateRelatedChain(d,RESET_TURN_SET)
        duel::negate_related_chain(d, bc, RESET_TURN_SET);
        // Negate its effects
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e2, EFFECT_DISABLE);
        // e2:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
        // d:RegisterEffect(e2)
        card::register_effect(d, bc, e2, false);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
        // e3:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e3, EFFECT_DISABLE_EFFECT);
        // e3:SetValue(RESET_TURN_SET)
        effect::set_value(d, e3, RESET_TURN_SET as i64);
        // e3:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
        // d:RegisterEffect(e3)
        card::register_effect(d, bc, e3, false);
    }
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler() (only for the status check below)
    let c = effect::get_handler(d, a.e).nil();
    // if rp==tp or e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) then return false end
    if a.rp == a.tp || card::is_status(d, c, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    // if not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    let re = a.re.nil();
    if !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]);
    // return tg and tg:IsExists(s.tfilter,1,nil,tp) and Duel.IsChainDisablable(ev)
    let Some(tg) = tg.first().and_then(|v| v.as_group()).cloned() else { return false };
    let tp = a.tp;
    group::is_exists(d, &tg, &|d, c| tfilter(d, c, tp), 1, NO_EX) && duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.tfilter(c,tp)`
fn tfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsType(TYPE_LINK) and c:IsControler(tp)
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_type(d, c, TYPE_LINK) && card::is_controler(d, c, tp)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // if chk==0 then return not rc:IsStatus(STATUS_DISABLED) end
    if a.chk == Some(0) {
        return !card::is_status(d, rc, STATUS_DISABLED);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    // if rc:IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateEffect(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_effect(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
