//! Advanced Dark
use super::prelude::*;

pub const ID: u32 = 12644061;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CRYSTAL_BEAST, SET_ULTIMATE_CRYSTAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change Attribute to DARK
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE|LOCATION_GRAVE,LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_target_range(d, e2, LOCATION_MZONE | LOCATION_GRAVE, LOCATION_MZONE | LOCATION_GRAVE);
    // e2:SetCode(EFFECT_CHANGE_ATTRIBUTE)
    effect::set_code(d, e2, EFFECT_CHANGE_ATTRIBUTE);
    // e2:SetTarget(s.tg)
    effect::set_target(d, e2, Some(effect_card_fn(tg)));
    // e2:SetValue(ATTRIBUTE_DARK)
    effect::set_value(d, e2, ATTRIBUTE_DARK as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Negate
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EVENT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetCondition(s.discon)
    effect::set_condition(d, e3, Some(condition(discon)));
    // e3:SetOperation(s.disop)
    effect::set_operation(d, e3, Some(operation(disop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Prevent damage
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e4, EVENT_PRE_DAMAGE_CALCULATE);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCondition(s.damcon)
    effect::set_condition(d, e4, Some(condition(damcon)));
    // e4:SetCost(s.damcost)
    effect::set_cost(d, e4, Some(cost_target(damcost)));
    // e4:SetOperation(s.damop)
    effect::set_operation(d, e4, Some(operation(damop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // id chk
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD);
    // e5:SetCode(id)
    effect::set_code(d, e5, ID);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e5, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e5:SetTargetRange(1,1)
    effect::set_target_range(d, e5, 1, 1);
    // e5:SetValue(s.val)
    effect::set_value_fn(d, e5, lua_fn(val));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.tg(e,c)`
fn tg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // if not c:IsSetCard(SET_CRYSTAL_BEAST) then return false end
    if !card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) {
        return false;
    }
    // if c:GetFlagEffect(1)==0 then
    if card::get_flag_effect(d, c, 1) == 0 {
        // c:RegisterFlagEffect(1,0,0,0)
        card::register_flag_effect(d, c, 1, 0, 0, 0, 0, 0);
        // local eff
        // if c:IsLocation(LOCATION_MZONE) then
        let eff = if card::is_location(d, c, LOCATION_MZONE) {
            // eff={Duel.GetPlayerEffect(c:GetControler(),EFFECT_NECRO_VALLEY)}
            let controler = card::get_controler(d, c);
            duel::get_player_effect(d, controler, EFFECT_NECRO_VALLEY)
        } else {
            // eff={c:GetCardEffect(EFFECT_NECRO_VALLEY)}
            card::get_card_effect(d, c, EFFECT_NECRO_VALLEY)
        };
        // c:ResetFlagEffect(1)
        card::reset_flag_effect(d, c, 1);
        // for _,te in ipairs(eff) do
        for te in eff {
            // local op=te:GetOperation()
            // if not op or op(e,c) then return false end
            match effect::get_operation(d, te) {
                None => return false,
                Some(op) => {
                    if op.call(d, &[Value::Effect(e), Value::Card(c)]).truthy() {
                        return false;
                    }
                }
            }
        }
    }
    // return true
    true
}
/// `s.val(e,c,re,chk)`
fn val(d: &mut Duel, a: Args) -> Value {
    // if chk==0 then return c:IsSetCard(SET_CRYSTAL_BEAST) end
    let c = a.c(1);
    if a.int(3) == 0 {
        return Value::Bool(card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]));
    }
    // return ATTRIBUTE_DARK
    Value::Int(ATTRIBUTE_DARK as i64)
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, _a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let a = duel::get_attacker(d);
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return at and a:IsSetCard(SET_ULTIMATE_CRYSTAL)
    at.is_some() && card::is_set_card(d, a.nil(), &[SET_ULTIMATE_CRYSTAL as u16])
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetAttackTarget()
    let tc = duel::get_attack_target(d).nil();
    // Negate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e1, EFFECT_DISABLE);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_DISABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 1);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
    // Duel.AdjustInstantly(tc)
    duel::adjust_instantly(d, Some(tc));
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local a=Duel.GetAttacker()
    let at_a = duel::get_attacker(d);
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    // return Duel.GetBattleDamage(tp)>0
    //     and ((a:IsControler(tp) and a:IsSetCard(SET_CRYSTAL_BEAST)) or (at and at:IsControler(tp) and at:IsSetCard(SET_CRYSTAL_BEAST)))
    if duel::get_battle_damage(d, tp) <= 0 {
        return false;
    }
    if card::is_controler(d, at_a.nil(), tp) && card::is_set_card(d, at_a.nil(), &[SET_CRYSTAL_BEAST as u16]) {
        return true;
    }
    match at {
        Some(at) => card::is_controler(d, at, tp) && card::is_set_card(d, at, &[SET_CRYSTAL_BEAST as u16]),
        None => false,
    }
}
/// `s.dfilter(c)`
fn dfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsMonster() and c:IsAbleToGraveAsCost()
    card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16]) && card::is_monster(d, c) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.damcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.dfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&dfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.dfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&dfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e1, EFFECT_AVOID_BATTLE_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
