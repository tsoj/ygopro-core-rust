//! Crusadia Maximus
use super::prelude::*;

pub const ID: u32 = 81524756;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CRUSADIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // You can Special Summon this card (from your hand) in Defense Position to your zone a Link Monster points to
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_SPSUM_PARAM)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_SPSUM_PARAM, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetTargetRange(POS_FACEUP_DEFENSE,0)
    effect::set_target_range(d, e1, POS_FACEUP_DEFENSE, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetValue(function(e,c) return 0,aux.GetMMZonesPointedTo(c:GetControler()) end)
    effect::set_value_fn(d, e1, lua_fn(spval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Apply a "this turn, if it battles an opponent's monster, any battle damage it inflicts to your opponent is doubled,
    // also other monsters you control cannot attack" effect on 1 "Crusadia" Link Monster you control
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsAbleToEnterBP() end)
    effect::set_condition(d, e2, Some(condition(|d, _a| duel::is_able_to_enter_bp(d))));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e,c) return 0,aux.GetMMZonesPointedTo(c:GetControler()) end
fn spval(d: &mut Duel, a: Args) -> Value {
    let c = a.c(1);
    let p = card::get_controler(d, c);
    let zones = aux::get_mm_zones_pointed_to(d, p, None, None, None, None);
    Value::Multi(vec![Value::Int(0), Value::Int(zones as i64)])
}
// function s.tgfilter(c)
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_CRUSADIA) and c:IsLinkMonster() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_CRUSADIA as u16]) && card::is_link_monster(d, c) && card::is_faceup(d, c)
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_APPLYTO)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_APPLYTO as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, a.tp, Some(&tgfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local fid=tc:GetFieldID()
        let fid = card::get_field_id(d, tc);
        // This turn, if it battles an opponent's monster, any battle damage it inflicts to your opponent is doubled
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,2))
        effect::set_description(d, e1, aux::stringid(ID, 2));
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
        effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
        // e1:SetCondition(function(e) local bc=e:GetHandler():GetBattleTarget() return bc and bc:IsControler(1-e:GetHandlerPlayer()) end)
        effect::set_condition(d, e1, Some(condition(battle_dmg_con)));
        // e1:SetValue(aux.ChangeBattleDamage(1,DOUBLE_DAMAGE))
        effect::set_value_fn(d, e1, aux::change_battle_damage(1, DOUBLE_DAMAGE as i64));
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // Also other monsters you control cannot attack
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetCode(EFFECT_CANNOT_ATTACK)
        effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
        // e2:SetTargetRange(LOCATION_MZONE,0)
        effect::set_target_range(d, e2, LOCATION_MZONE, 0);
        // e2:SetTarget(function(e,c) return fid~=c:GetFieldID() end)
        effect::set_target(d, e2, Some(effect_card_fn(move |d, _e, x| fid != card::get_field_id(d, x))));
        // e2:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
}
// function(e) local bc=e:GetHandler():GetBattleTarget() return bc and bc:IsControler(1-e:GetHandlerPlayer()) end
fn battle_dmg_con(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    match card::get_battle_target(d, h) {
        Some(bc) => card::is_controler(d, bc, 1 - effect::get_handler_player(d, a.e)),
        None => false,
    }
}
