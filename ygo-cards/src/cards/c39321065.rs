//! Silent Swordsman Zero
use super::prelude::*;

pub const ID: u32 = 39321065;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SHINING_SARCOPHAGUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Increase this card's Level by 1
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():HasLevel() end)
    effect::set_condition(d, e1, Some(condition(lv_cond)));
    // e1:SetOperation(s.lvop)
    effect::set_operation(d, e1, Some(operation(lvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Gains ATK equal to the difference between its current Level and its original Level x 500
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Negate an opponent's effect that targets "Shining Sarcophagus" or a monster(s) that mentions it
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_NEGATE+CATEGORY_LVCHANGE)
    effect::set_category(d, e3, CATEGORY_NEGATE + CATEGORY_LVCHANGE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.negcon)
    effect::set_condition(d, e3, Some(condition(negcon)));
    // e3:SetTarget(s.negtg)
    effect::set_target(d, e3, Some(cost_target(negtg)));
    // e3:SetOperation(s.negop)
    effect::set_operation(d, e3, Some(operation(negop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():HasLevel() end`
fn lv_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::has_level(d, c)
}
/// Level +1 effect of `s.lvop` and `s.negop`
fn add_level(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_LEVEL)
    effect::set_code(d, e1, EFFECT_UPDATE_LEVEL);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.lvop(e,tp,eg,ep,ev,re,r,rp)`
fn lvop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:IsFaceup() and c:HasLevel() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::has_level(d, c) {
        add_level(d, c);
    }
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // local ct=c:GetLevel()-c:GetOriginalLevel()
    let ct = card::get_level(d, c) as i64 - card::get_original_level(d, c) as i64;
    // return ct>0 and ct*500
    if ct > 0 {
        ct * 500
    } else {
        0
    }
}
/// `s.negconfilter(c,tp)`
fn negconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return (c:IsCode(CARD_SHINING_SARCOPHAGUS) or (c:ListsCode(CARD_SHINING_SARCOPHAGUS) and c:IsMonster()))
    //     and c:IsFaceup() and c:IsControler(tp) and c:IsOnField()
    let sarco = card::is_code(d, c, &[CARD_SHINING_SARCOPHAGUS]) || (card::lists_code(d, c, &[CARD_SHINING_SARCOPHAGUS]) && card::is_monster(d, c));
    sarco && card::is_faceup(d, c) && card::is_controler(d, c, tp) && card::is_on_field(d, c)
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let h = effect::get_handler(d, e).nil();
    let re = a.re.nil();
    // if rp==tp or e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) or not re:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return false end
    if a.rp == tp || card::is_status(d, h, STATUS_BATTLE_DESTROYED) || !effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0) {
        return false;
    }
    // local tg=Duel.GetChainInfo(ev,CHAININFO_TARGET_CARDS)
    let tg = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned());
    // return tg and tg:IsExists(s.negconfilter,1,nil,tp) and Duel.IsChainNegatable(ev)
    match tg {
        Some(tg) => {
            group::is_exists(d, &tg, &|d, c| negconfilter(d, c, tp), 1, NO_EX) && duel::is_chain_negatable(d, a.ev as u8)
        }
        None => false,
    }
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():HasLevel() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::has_level(d, h);
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.NegateActivation(ev) and c:IsRelateToEffect(e) and c:IsFaceup() and c:HasLevel() then
    if duel::negate_activation(d, a.ev as u8) && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::has_level(d, c) {
        add_level(d, c);
    }
}
