//! Bujin Regalia - The Jewel
use super::prelude::*;

pub const ID: u32 = 63049052;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BUJIN], ..ScriptInfo::DEFAULT };

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
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(stat_change_damage_step_condition)));
    // e1:SetCost(aux.RemainFieldCost)
    effect::set_cost(d, e1, Some(cost::remain_field_cost()));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// aux.StatChangeDamageStepCondition
fn stat_change_damage_step_condition(d: &mut Duel, _a: &EffArgs) -> bool {
    aux::stat_change_damage_step_condition(d)
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRank(4)
    card::is_faceup(d, c) && card::is_rank(d, c, &[4])
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc);
    }
    // if chk==0 then return e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
            && duel::is_existing_target(d, Some(&filter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&filter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsLocation(LOCATION_SZONE) or not c:IsRelateToEffect(e) or c:IsStatus(STATUS_LEAVE_CONFIRMED) then return end
    if !card::is_location(d, c, LOCATION_SZONE) || !card::is_relate_to_effect(d, c, a.e) || card::is_status(d, c, STATUS_LEAVE_CONFIRMED) {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if tc and tc:IsRelateToEffect(e) and tc:IsFaceup() and tc:IsControler(tp) then
    if let Some(tc) = duel::first_target(d) {
        if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && card::is_controler(d, tc, tp) {
            // Duel.Equip(tp,c,tc)
            duel::equip(d, tp, c, tc, true, false);
            // Atk up
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_EQUIP)
            effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(s.atkval)
            effect::set_value_fn(d, e1, effect_card_value(atkval));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // material
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetDescription(aux.Stringid(id,0))
            effect::set_description(d, e2, aux::stringid(ID, 0));
            // e2:SetType(EFFECT_TYPE_QUICK_O)
            effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
            // e2:SetCode(EVENT_FREE_CHAIN)
            effect::set_code(d, e2, EVENT_FREE_CHAIN);
            // e2:SetRange(LOCATION_SZONE)
            effect::set_range(d, e2, LOCATION_SZONE);
            // e2:SetCountLimit(1)
            effect::set_count_limit(d, e2, 1, 0, 0, 0);
            // e2:SetTarget(s.mattg)
            effect::set_target(d, e2, Some(cost_target(mattg)));
            // e2:SetOperation(s.matop)
            effect::set_operation(d, e2, Some(operation(matop)));
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // c:RegisterEffect(e2)
            card::register_effect(d, c, e2, false);
            // Equip limit
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
    // else c:CancelToGrave(false) end
    card::cancel_to_grave(d, c, false);
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetOverlayCount()*300
    card::get_overlay_count(d, c) as i64 * 300
}
// function s.eqlimit(e,c)
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:GetControler()==e:GetOwnerPlayer() and c:IsType(TYPE_XYZ)
    card::get_controler(d, c) == effect::get_owner_player(d, e) && card::is_type(d, c, TYPE_XYZ)
}
// function s.mfilter(c)
fn mfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BUJIN) and c:IsMonster()
    card::is_set_card(d, c, &[SET_BUJIN as u16]) && card::is_monster(d, c)
}
// function s.mattg(e,tp,eg,ep,ev,re,r,rp,chk)
fn mattg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.mfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&mfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    true
}
// function s.matop(e,tp,eg,ep,ev,re,r,rp)
fn matop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local ec=e:GetHandler():GetEquipTarget()
    // if not ec then return end
    let Some(ec) = card::get_equip_target(d, h) else {
        return;
    };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local g=Duel.SelectMatchingCard(tp,s.mfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&mfilter), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then Duel.Overlay(ec,g) end
    if !g.is_empty() {
        duel::overlay(d, ec, g, false);
    }
}
