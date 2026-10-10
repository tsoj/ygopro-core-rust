//! Tri-Gate Wizard
use super::prelude::*;

pub const ID: u32 = 32617464;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ monsters, except Tokens
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(aux.NOT(Card.IsType),TYPE_TOKEN),2)
    link::add_procedure(d, c, Some(mat_filter(|d, cc, sc, st, tp| !card::is_type_ctx(d, cc, TYPE_TOKEN, sc, st, Some(tp)))), 2, None, None, None, None);
    // If a monster co-linked to this card battles your opponent's monster, any battle damage it inflicts to your opponent is doubled
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e1, LOCATION_MZONE, LOCATION_MZONE);
    // e1:SetTarget(s.doubledamtg)
    effect::set_target(d, e1, Some(effect_card_fn(doubledamtg)));
    // e1:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
    effect::set_value_fn(d, e1, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish 1 card on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():GetMutualLinkedGroupCount()>=2 end)
    effect::set_condition(d, e2, Some(condition(rm_cond)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Negate the activation of a card or effect, and if you do, banish that card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_NEGATE+CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_NEGATE + CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.negcon)
    effect::set_condition(d, e3, Some(condition(negcon)));
    // e3:SetTarget(s.negtg)
    effect::set_target(d, e3, Some(cost_target(negtg)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.doubledamtg(e,c)`
fn doubledamtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c);
    // local lg=e:GetHandler():GetMutualLinkedGroup()
    let h = effect::get_handler(d, e).nil();
    let lg = card::get_mutual_linked_group(d, h);
    // return lg:IsContains(c) and bc and bc:IsControler(1-e:GetHandlerPlayer())
    if !lg.contains(c) {
        return false;
    }
    match bc {
        None => false,
        Some(bc) => {
            let p = effect::get_handler_player(d, e);
            card::is_controler(d, bc, 1 - p)
        }
    }
}
/// `s.rmfilter`: `Card.IsAbleToRemove` as a filter (default player, position and reason)
fn is_able_to_remove_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_remove(d, c, None, None, None)
}
/// `function(e) return e:GetHandler():GetMutualLinkedGroupCount()>=2 end`
fn rm_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_mutual_linked_group_count(d, h) >= 2
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&is_able_to_remove_filter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&is_able_to_remove_filter), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, g, 1, a.tp, 0);
    true
}
/// `s.rmop(e,...)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then Duel.Remove(tc,POS_FACEUP,REASON_EFFECT) end
    if card::is_relate_to_effect(d, tc, a.e) {
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
/// `s.negcon(e,...)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:GetMutualLinkedGroupCount()>=3 and not c:IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)
    card::get_mutual_linked_group_count(d, c) >= 3 && !card::is_status(d, c, STATUS_BATTLE_DESTROYED) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // local relation=rc:IsRelateToEffect(re)
    let relation = card::is_relate_to_effect(d, rc, re);
    // if chk==0 then return rc:IsAbleToRemove(tp) or (not relation and Duel.IsPlayerCanRemove(tp)) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, rc, Some(a.tp), None, None) || (!relation && duel::is_player_can_remove(d, a.tp, None));
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    if relation {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, a.tp, 0);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,0,rc:GetPreviousLocation())
        let prev_loc = card::get_previous_location(d, rc);
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, 0, prev_loc as i32);
    }
    true
}
/// `s.negop(e,...)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.Remove(eg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, a.eg(), POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
