//! Myutant Beast
use super::prelude::*;

pub const ID: u32 = 34695290;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MYUTANT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be Special Summoned, except with the effect of a "Myutant" card
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(function(e,se,sp,st) return se:IsCardSetcode(SET_MYUTANT) and se:IsHasType(EFFECT_TYPE_ACTIONS) end)
    effect::set_value_fn(d, e0, lua_fn(spsummon_value));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Your opponent cannot target this card with monster effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,re,rp) return re:GetHandlerPlayer()==1-e:GetHandlerPlayer() and re:IsMonsterEffect() end)
    effect::set_value_fn(d, e1, lua_fn(target_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // When your opponent activates a Spell Card or effect (Quick Effect): You can banish 1 card from your hand or field;
    // negate the activation, and if you do, banish that card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_condition(d, e2, Some(condition(activate_cond)));
    // e2:SetCost(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_cost(d, e2, Some(cost_target(activate_cost)));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) ... end)
    effect::set_target(d, e2, Some(cost_target(activate_target)));
    // e2:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_operation(d, e2, Some(operation(activate_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card in its owner's possession is destroyed by an opponent's card: You can target 1 of your banished "Myutant" Traps; add it to your hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and e:GetHandler():IsPreviousControler(tp) end)
    effect::set_condition(d, e3, Some(condition(destroyed_cond)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,se,sp,st) return se:IsCardSetcode(SET_MYUTANT) and se:IsHasType(EFFECT_TYPE_ACTIONS) end`
fn spsummon_value(d: &mut Duel, a: Args) -> bool {
    let se = a.v(1).as_effect().nil();
    effect::is_card_setcode(d, se, &[SET_MYUTANT as u16]) && effect::is_has_type(d, se, EFFECT_TYPE_ACTIONS)
}
/// `function(e,re,rp) return re:GetHandlerPlayer()==1-e:GetHandlerPlayer() and re:IsMonsterEffect() end`
fn target_value(d: &mut Duel, a: Args) -> bool {
    let re = a.v(1).as_effect().nil();
    let me = effect::get_handler_player(d, a.e(0)) as i64;
    let rp = effect::get_handler_player(d, re) as i64;
    rp == 1 - me && effect::is_monster_effect(d, re)
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and re:IsSpellEffect() and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev) end`
fn activate_cond(d: &mut Duel, a: &EffArgs) -> bool {
    if a.rp != 1 - a.tp {
        return false;
    }
    if !effect::is_spell_effect(d, a.re.nil()) {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) ... end` (cost)
fn activate_cost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemoveAsCost,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), a.tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, &g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) ... end` (target)
fn activate_target(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // local relation=rc:IsRelateToEffect(re)
    let relation = card::is_relate_to_effect(d, rc, a.re.nil());
    // if chk==0 then return rc:IsAbleToRemove(tp) or (not relation and Duel.IsPlayerCanRemove(tp)) end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, rc, Some(a.tp), None, None) || (!relation && duel::is_player_can_remove(d, a.tp, None));
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, a.tp, 0);
    if relation {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,rc,1,rc:GetControler(),rc:GetLocation())
        let p = card::get_controler(d, rc);
        let loc = card::get_location(d, rc) as i32;
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(rc)), 1, p, loc);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,0,0,rc:GetPreviousLocation())
        let loc = card::get_previous_location(d, rc) as i32;
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 0, 0, loc);
    }
    true
}
/// `function(e,tp,eg,ep,ev,re,r,rp) ... end` (operation)
fn activate_op(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let rc = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rc, a.re.nil()) {
            // Duel.Remove(eg,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, a.eg(), POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return rp==1-tp and e:GetHandler():IsPreviousControler(tp) end`
fn destroyed_cond(d: &mut Duel, a: &EffArgs) -> bool {
    if a.rp != 1 - a.tp {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    card::is_previous_controler(d, h, a.tp)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MYUTANT) and c:IsTrap() and c:IsFaceup() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_MYUTANT as u16]) && card::is_trap(d, c) && card::is_faceup(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_REMOVED) and s.thfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_REMOVED) && thfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.thfilter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_REMOVED,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&thfilter), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
    }
}
