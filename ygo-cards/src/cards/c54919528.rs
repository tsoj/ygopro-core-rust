//! K9－ØØ号 “Ｈｏｕｎｄ” (K9-00 "Hound")
use super::prelude::*;

pub const ID: u32 = 54919528;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 5 monsters
    // Xyz.AddProcedure(c,nil,5,2)
    xyz::add_procedure(d, c, None, Some(5), 2, None, None, None, None, false, None);
    // Cannot be destroyed by battle or your opponent's card effects the turn it is Special Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
    // e1:SetCondition(function(e) return e:GetHandler():IsStatus(STATUS_SPSUMMON_TURN) end)
    effect::set_condition(d, e1, Some(lua_fn(e1_condition)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetValue(aux.indoval)
    effect::set_value_fn(d, e2, aux::indoval());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Register when the opponent activates a monster effect
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetOperation(s.regop)
    effect::set_operation(d, e3, Some(operation(regop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // This card gains 500 ATK when that effect resolves
    // local e3b=Effect.CreateEffect(c)
    let e3b = effect::create_effect(d, c);
    // e3b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3b, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3b:SetCode(EVENT_CHAIN_SOLVED)
    effect::set_code(d, e3b, EVENT_CHAIN_SOLVED);
    // e3b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3b, LOCATION_MZONE);
    // e3b:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(d, e3b, Some(lua_fn(e3b_condition)));
    // e3b:SetOperation(s.atkop)
    effect::set_operation(d, e3b, Some(operation(atkop)));
    // c:RegisterEffect(e3b)
    card::register_effect(d, c, e3b, false);
    // Banish 1 card on the field
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e4, CATEGORY_REMOVE);
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e4, EVENT_PHASE | PHASE_STANDBY);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e4, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e4:SetTarget(s.rmvtg)
    effect::set_target(d, e4, Some(cost_target(rmvtg)));
    // e4:SetOperation(s.rmvop)
    effect::set_operation(d, e4, Some(operation(rmvop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `function(e) return e:GetHandler():IsStatus(STATUS_SPSUMMON_TURN) end`
fn e1_condition(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_status(d, h, STATUS_SPSUMMON_TURN)
}
/// `function(e) return e:GetHandler():HasFlagEffect(id) end`
fn e3b_condition(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::has_flag_effect(d, h, ID, None)
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // if not (re:IsMonsterEffect() and re:GetHandlerPlayer()==1-tp) then return end
    let re = a.re.nil();
    if !(effect::is_monster_effect(d, re) && effect::get_handler_player(d, re) == 1 - a.tp) {
        return;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESET_CHAIN|RESETS_STANDARD&~RESET_TURN_SET,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESET_CHAIN | (RESETS_STANDARD & !RESET_TURN_SET), 0, 1, 0, 0);
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // if not (re:IsMonsterEffect() and re:GetHandlerPlayer()==1-tp) then return end
    let re = a.re.nil();
    if !(effect::is_monster_effect(d, re) && effect::get_handler_player(d, re) == 1 - a.tp) {
        return;
    }
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_COPY_INHERIT)
    effect::set_property(d, e1, EFFECT_FLAG_COPY_INHERIT, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(500)
    effect::set_value(d, e1, 500);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsAbleToRemove() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_able_to_remove(d, chkc, None, None, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, tp, 0);
    true
}
/// `s.rmvop(e,tp,eg,ep,ev,re,r,rp)`
fn rmvop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
