//! Starving Venom Fusion Dragon
use super::prelude::*;

pub const ID: u32 = 41209827;

pub const INFO: ScriptInfo = ScriptInfo { material_location: Some(LOCATION_ONFIELD), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMixN(c,true,true,s.ffilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp))), 2)]);
    // atk up
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.atkcon)
    effect::set_condition(d, e1, Some(condition(atkcon)));
    // e1:SetTarget(s.atktg)
    effect::set_target(d, e1, Some(cost_target(atktg)));
    // e1:SetOperation(s.atkop)
    effect::set_operation(d, e1, Some(operation(atkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // copy
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
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCost(s.copycost)
    effect::set_cost(d, e2, Some(cost_target(copycost)));
    // e2:SetTarget(s.copytg)
    effect::set_target(d, e2, Some(cost_target(copytg)));
    // e2:SetOperation(s.copyop)
    effect::set_operation(d, e2, Some(operation(copyop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ffilter(c,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK,fc,sumtype,tp) and c:IsOnField() and not c:IsType(TYPE_TOKEN,fc,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, Some(fc), sumtype, Some(tp))
        && card::is_on_field(d, c)
        && !card::is_type_ctx(d, c, TYPE_TOKEN, Some(fc), sumtype, Some(tp))
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpecialSummoned() and c:IsFaceup()
    card::is_special_summoned(d, c) && card::is_faceup(d, c)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.atkfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| atkfilter(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectMatchingCard(tp,s.atkfilter,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| atkfilter(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc and c:IsRelateToEffect(e) and c:IsFaceup() then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
            // local atk=tc:GetAttack()
            let atk = card::get_attack(d, tc);
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetValue(atk)
            effect::set_value(d, e1, atk as i64);
            // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
        }
    }
}
/// `s.copycost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn copycost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():GetFlagEffect(id)==0 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::get_flag_effect(d, h, ID) == 0;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    true
}
/// `s.copyfilter(c)`
fn copyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(5)
    card::is_faceup(d, c) && card::is_level_above(d, c, 5)
}
/// `s.copytg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn copytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_MZONE) and s.copyfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && copyfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.copyfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| copyfilter(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.copyfilter,tp,0,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d, c| copyfilter(d, c)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
/// `s.copyop(e,tp,eg,ep,ev,re,r,rp)`
fn copyop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and c:IsRelateToEffect(e) and c:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsType(TYPE_TOKEN) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, c, a.e)
            && card::is_faceup(d, c)
            && card::is_relate_to_effect(d, tc, a.e)
            && card::is_faceup(d, tc)
            && !card::is_type(d, tc, TYPE_TOKEN)
        {
            // local code=tc:GetOriginalCodeRule()
            let code = card::get_original_code_rule(d, tc).first().copied().nil();
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_CODE)
            effect::set_code(d, e1, EFFECT_CHANGE_CODE);
            // e1:SetValue(code)
            effect::set_value(d, e1, code as i64);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // c:RegisterEffect(e1)
            card::register_effect(d, c, e1, false);
            // if not tc:IsType(TYPE_TRAPMONSTER) then
            if !card::is_type(d, tc, TYPE_TRAPMONSTER) {
                // c:CopyEffect(code,RESETS_STANDARD_PHASE_END,1)
                card::copy_effect(d, c, code, RESETS_STANDARD_PHASE_END, 1);
            }
        }
    }
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsFusionSummoned()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_fusion_summoned(d, c)
}
/// `s.desfilter(c)`
fn desfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpecialSummoned()
    card::is_special_summoned(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
