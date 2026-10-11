//! Hecahands Dandalos
//! 見えざる手ダンダロス
use super::prelude::*;

pub const ID: u32 = 31411835;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_HECAHANDS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 2 "Hecahands" monsters
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_HECAHANDS),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_HECAHANDS as u16], Some(fc), st, Some(tp)))), 2)],
    );
    // You can target 1 monster your opponent controls; take control of it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.ctrltg)
    effect::set_target(d, e1, Some(cost_target(ctrltg)));
    // e1:SetOperation(s.ctrlop)
    effect::set_operation(d, e1, Some(operation(ctrlop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Your "Hecahands" Fusion Monsters and your monsters that are owned by your opponent can attack directly
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_DIRECT_ATTACK)
    effect::set_code(d, e2, EFFECT_DIRECT_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(function(e,c) return (c:IsSetCard(SET_HECAHANDS) and c:IsFusionMonster()) or c:IsOwner(1-e:GetHandlerPlayer()) end)
    effect::set_target(d, e2, Some(effect_card_fn(direct_attack_target)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If this card battles a monster, neither can be destroyed by that battle
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
    // e3:SetTarget(s.indestg)
    effect::set_target(d, e3, Some(effect_card_fn(indestg)));
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.ctrltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn ctrltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and chkc:IsControlerCanBeChanged() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && card::is_able_to_change_controler(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_able_to_change_controler(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
    // local g=Duel.SelectTarget(tp,Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_change_controler(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, g, 1, a.tp, 0);
    true
}
// function s.ctrlop(e,tp,eg,ep,ev,re,r,rp)
fn ctrlop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.GetControl(tc,tp)
        duel::get_control(d, tc, a.tp, 0, 0, None, None);
    }
}
// Your "Hecahands" Fusion Monsters and your monsters that are owned by your opponent can attack directly
// function(e,c) return (c:IsSetCard(SET_HECAHANDS) and c:IsFusionMonster()) or c:IsOwner(1-e:GetHandlerPlayer()) end
fn direct_attack_target(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    (card::is_set_card(d, c, &[SET_HECAHANDS as u16]) && card::is_fusion_monster(d, c)) || card::is_owner(d, c, 1 - effect::get_handler_player(d, e))
}
// function s.indestg(e,c)
fn indestg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local handler=e:GetHandler()
    let handler = effect::get_handler(d, e).nil();
    // return c==handler or c==handler:GetBattleTarget()
    c == handler || card::get_battle_target(d, handler) == Some(c)
}
