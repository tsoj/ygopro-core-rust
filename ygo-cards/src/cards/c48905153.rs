//! Zoodiac Drident
use super::prelude::*;

pub const ID: u32 = 48905153;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ZOODIAC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz.AddProcedure(c,nil,4,4,s.ovfilter,aux.Stringid(id,0),4,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(4),
        4,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        Some(xyz::MaxCt::N(4)),
        Some(lua_fn(xyzop)),
        false,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
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
    effect::set_value_fn(d, e1, effect_value(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // e2:SetValue(s.defval)
    effect::set_value_fn(d, e2, effect_value(defval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e3:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ovfilter(c,tp,lc)`
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_ZOODIAC,lc,SUMMON_TYPE_XYZ,tp) and not c:IsSummonCode(lc,SUMMON_TYPE_XYZ,tp,id)
    card::is_faceup(d, c)
        && card::is_set_card_ctx(d, c, &[SET_ZOODIAC as u16], Some(lc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && !card::is_summon_code(d, c, Some(lc), SUMMON_TYPE_XYZ as u64, tp, &[ID])
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
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZOODIAC) and c:GetAttack()>=0
    card::is_set_card(d, c, &[SET_ZOODIAC as u16]) && card::get_attack(d, c) >= 0
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=e:GetHandler():GetOverlayGroup():Filter(s.atkfilter,nil)
    let h = effect::get_handler(d, e).nil();
    let og = card::get_overlay_group(d, h);
    let g = group::filter(d, &og, &atkfilter, NO_EX);
    // return g:GetSum(Card.GetAttack)
    group::get_sum(d, &g, &|d, x| card::get_attack(d, x) as i64)
}
/// `s.deffilter(c)`
fn deffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZOODIAC) and c:GetDefense()>=0
    card::is_set_card(d, c, &[SET_ZOODIAC as u16]) && card::get_defense(d, c) >= 0
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, e: EffectId) -> i64 {
    // local g=e:GetHandler():GetOverlayGroup():Filter(s.deffilter,nil)
    let h = effect::get_handler(d, e).nil();
    let og = card::get_overlay_group(d, h);
    let g = group::filter(d, &og, &deffilter, NO_EX);
    // return g:GetSum(Card.GetDefense)
    group::get_sum(d, &g, &|d, x| card::get_defense(d, x) as i64)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c)), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
