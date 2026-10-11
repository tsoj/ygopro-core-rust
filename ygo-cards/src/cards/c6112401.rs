//! At One With the Sword
use super::prelude::*;

pub const ID: u32 = 6112401;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_X_SABER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetCost(aux.RemainFieldCost)
    effect::set_cost(d, e1, Some(aux::remain_field_cost()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.IsPhase(PHASE_DAMAGE) and Duel.IsDamageCalculated() then return false end
    if duel::is_phase(d, PHASE_DAMAGE) && duel::is_damage_calculated(d) {
        return false;
    }
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // e:SetLabelObject(tc)
    let label = match tc {
        Some(tc) => Value::Card(tc),
        None => Value::Nil,
    };
    effect::set_label_object(d, a.e, label);
    // return #g==1 and tc:IsFaceup() and tc:IsSetCard(SET_X_SABER)
    g.len() == 1 && card::is_faceup(d, tc.nil()) && card::is_set_card(d, tc.nil(), &[SET_X_SABER as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE) and e:GetLabelObject():IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        if !effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE) {
            return false;
        }
        let lo = effect::get_label_object(d, a.e).as_card().nil();
        return card::is_can_be_effect_target(d, lo, Some(a.e));
    }
    // Duel.SetTargetCard(e:GetLabelObject())
    let lo = effect::get_label_object(d, a.e).as_card().nil();
    duel::set_target_card(d, lo);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsLocation(LOCATION_SZONE) or not c:IsRelateToEffect(e) or c:IsStatus(STATUS_LEAVE_CONFIRMED) then return end
    if !card::is_location(d, c, LOCATION_SZONE) || !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card();
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
            // Duel.Equip(tp,c,tc)
            duel::equip(d, a.tp, c, tc, true, false);
            // --draw
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
            // e1:SetCategory(CATEGORY_DRAW)
            effect::set_category(d, e1, CATEGORY_DRAW);
            // e1:SetDescription(aux.Stringid(id,0))
            effect::set_description(d, e1, aux::stringid(ID, 0));
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
            // e1:SetCode(EVENT_BATTLE_DESTROYING)
            effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
            // e1:SetRange(LOCATION_SZONE)
            effect::set_range(d, e1, LOCATION_SZONE);
            // e1:SetCondition(s.drcon)
            effect::set_condition(d, e1, Some(condition(drcon)));
            // e1:SetTarget(s.drtg)
            effect::set_target(d, e1, Some(cost_target(drtg)));
            // e1:SetOperation(s.drop)
            effect::set_operation(d, e1, Some(operation(drop_op)));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // --Atkup
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_EQUIP)
            effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
            // e2:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // e2:SetValue(800)
            effect::set_value(d, e2, 800);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
            // --Equip limit
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_EQUIP_LIMIT)
            effect::set_code(d, e3, EFFECT_EQUIP_LIMIT);
            // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e3:SetValue(s.eqlimit)
            effect::set_value_fn(d, e3, effect_card_fn(eqlimit));
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e3)
            card::register_effect(d, c, e3, false);
            return;
        }
    }
    // c:CancelToGrave(false)
    card::cancel_to_grave(d, c, false);
}
/// `s.eqlimit(e,c)`
fn eqlimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsSetCard(SET_X_SABER)
    card::is_set_card(d, c, &[SET_X_SABER as u16])
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsContains(e:GetHandler():GetEquipTarget())
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    match card::get_equip_target(d, h) {
        Some(et) => group::is_contains(&eg, et),
        None => false,
    }
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
