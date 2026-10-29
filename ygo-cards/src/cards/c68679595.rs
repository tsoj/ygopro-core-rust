//! Ultimate Leo Utopia Ray
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 68679595;

pub const INFO: ScriptInfo = ScriptInfo { loads: &[56840427],
    listed_names: &[56840427],
    listed_series: &[SET_ZW],
    xyz_number: Some(39),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz Summon
    // Xyz.AddProcedure(c,nil,5,3)
    xyz::add_procedure(d, c, None, Some(5), 3, None, None, None, None, false, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_INACTIVATE+EFFECT_FLAG_CANNOT_NEGATE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_INACTIVATE + EFFECT_FLAG_CANNOT_NEGATE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_DISABLE + CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c,tc,tp)`
fn filter(d: &mut Duel, c: CardId, tc: CardId, tp: u8) -> bool {
    // if not (c:IsSetCard(SET_ZW) and not c:IsForbidden()) then return false end
    if !(card::is_set_card(d, c, &[SET_ZW as u16]) && !card::is_forbidden(d, c)) {
        return false;
    }
    // local effs={c:GetCardEffect(75402014)}
    let effs = card::get_card_effect(d, c, 75402014);
    // for _,te in ipairs(effs) do
    for te in effs {
        // if te:GetValue()(tc,c,tp) then return true end
        let v = effect::get_value(d, te);
        if let Some(f) = v.as_fn() {
            if f.call(d, &[Value::Card(tc), Value::Card(c), Value::Int(tp as i64)]).truthy() {
                return true;
            }
        }
    }
    // return false
    false
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e:GetHandler(),tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, h, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 or c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 || card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,c,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| filter(d, cc, c, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // local eff=tc:GetCardEffect(75402014)
        let eff = card::get_card_effect(d, tc, 75402014).first().copied().nil();
        // eff:GetOperation()(tc,eff:GetLabelObject(),tp,c)
        let op = effect::get_operation(d, eff).nil();
        let lo = effect::get_label_object(d, eff);
        op.call(d, &[Value::Card(tc), lo, Value::Int(tp as i64), Value::Card(c)]);
    }
}
/// `s.discfilter(c)`
fn discfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZW) and c:GetOriginalType() & TYPE_MONSTER ~= 0
    card::is_set_card(d, c, &[SET_ZW as u16]) && (card::get_original_type(d, c) & TYPE_MONSTER) != 0
}
/// `s.discon(e)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetEquipGroup():IsExists(s.discfilter,1,nil)
    let h = effect::get_handler(d, a.e).nil();
    let eqg = card::get_equip_group(d, h);
    group::is_exists(d, &eqg, &discfilter, 1, NO_EX)
}
/// `s.disfilter(c)`
fn disfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EFFECT) and not c:IsDisabled()
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT) && !card::is_disabled(d, c)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and s.disfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && disfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.disfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&disfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_NEGATE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_NEGATE as u64);
    // local g=Duel.SelectTarget(tp,s.disfilter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&disfilter), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, g, 1, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) and not tc:IsDisabled() then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && !card::is_disabled(d, tc) {
            // Negate its effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // if tc:IsImmuneToEffect(e1) or tc:IsImmuneToEffect(e2) then return end
            if card::is_immune_to_effect(d, tc, e1) || card::is_immune_to_effect(d, tc, e2) {
                return;
            }
            // Duel.AdjustInstantly(tc)
            duel::adjust_instantly(d, Some(tc));
            // local atk=tc:GetAttack()/2
            let atk = card::get_attack(d, tc) as f64 / 2.0;
            // Halve its ATK
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_SET_ATTACK_FINAL)
            effect::set_code(d, e3, EFFECT_SET_ATTACK_FINAL);
            // e3:SetValue(atk)
            effect::set_value(d, e3, lua_int(atk));
            // e3:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc, e3, false);
        }
    }
}
