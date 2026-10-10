//! Cyber Dragon Infinity
use super::prelude::*;

pub const ID: u32 = 10443957;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[58069384], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,s.mfilter,6,3,s.ovfilter,aux.Stringid(id,0),3,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(mfilter)),
        Some(6),
        3,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(3)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // atk
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, effect_card_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // e2:SetOperation(s.operation)
    effect::set_operation(d, e2, Some(operation(op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // negate
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.discon)
    effect::set_condition(d, e3, Some(condition(discon)));
    // e3:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(cost_target(distg)));
    // e3:SetOperation(s.disop)
    effect::set_operation(d, e3, Some(operation(disop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.mfilter(c,xyz,sumtype,tp)`
fn mfilter(d: &mut Duel, c: CardId, xyz: Option<CardId>, st: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_MACHINE,xyz,sumtype,tp) and c:IsAttribute(ATTRIBUTE_LIGHT,xyz,sumtype,tp)
    card::is_race_ctx(d, c, RACE_MACHINE, xyz, st, Some(tp)) && card::is_attribute_ctx(d, c, ATTRIBUTE_LIGHT, xyz, st, Some(tp))
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSummonCode(lc,SUMMON_TYPE_XYZ,tp,58069384)
    card::is_faceup(d, c) && card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_XYZ as u64, tp, &[58069384])
}
/// `s.xyzop(e,tp,chk)`
fn xyzop(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    // if chk==0 then return Duel.GetFlagEffect(tp,id)==0 end
    if a.int(2) == 0 {
        return duel::get_flag_effect(d, tp, ID) == 0;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // return true
    true
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetOverlayCount()*200
    (card::get_overlay_count(d, c) * 200) as i64
}
/// `s.filter(c,tp)`
fn filter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPosition(POS_FACEUP_ATTACK) and not c:IsType(TYPE_TOKEN)
    //     and (c:IsControler(tp) or c:IsAbleToChangeControler())
    card::is_position(d, c, POS_FACEUP_ATTACK) && !card::is_type(d, c, TYPE_TOKEN) && (card::is_controler(d, c, tp) || card::is_able_to_change_controler(d, c))
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.filter(chkc,tp) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && filter(d, chkc, a.tp) && Some(chkc) != effect::get_handler(d, a.e);
    }
    // if chk==0 then return e:GetHandler():IsType(TYPE_XYZ)
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler(),tp) end
    let h = effect::get_handler(d, a.e).nil();
    if a.chk == Some(0) {
        return card::is_type(d, h, TYPE_XYZ)
            && duel::is_existing_target(d, Some(&|d, c| filter(d, c, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,e:GetHandler(),tp)
    duel::select_target(d, a.tp, Some(&|d, c| filter(d, c, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, h.into());
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, c.nil(), a.e) && card::is_relate_to_effect(d, tc.nil(), a.e) && !card::is_immune_to_effect(d, tc.nil(), a.e) {
        // Duel.Overlay(c,tc,true)
        duel::overlay(d, c.nil(), tc.nil(), true);
    }
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)
    let h = effect::get_handler(d, a.e).nil();
    !card::is_status(d, h, STATUS_BATTLE_DESTROYED) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsRelateToEffect(re) and re:GetHandler():IsDestructable() then
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    if card::is_relate_to_effect(d, rh, re) && card::is_destructable(d, rh, None) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rh = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rh, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
