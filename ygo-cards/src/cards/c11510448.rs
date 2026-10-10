//! Zoodiac Tigermortar
use super::prelude::*;

pub const ID: u32 = 11510448;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_ZOODIAC],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,nil,4,3,s.ovfilter,aux.Stringid(id,0),3,s.xyzop)
    xyz::add_procedure(
        d,
        c,
        None,
        Some(4),
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
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e2, EFFECT_UPDATE_DEFENSE);
    // e2:SetValue(s.defval)
    effect::set_value_fn(d, e2, effect_card_value(defval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // xyz material
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
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
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local g=e:GetHandler():GetOverlayGroup():Filter(s.atkfilter,nil)
    let h = effect::get_handler(d, e).nil();
    let og = card::get_overlay_group(d, h);
    let g = group::filter(d, &og, &|d, c| atkfilter(d, c), NO_EX);
    // return g:GetSum(Card.GetAttack)
    group::get_sum(d, &g, &|d, c| card::get_attack(d, c) as i64)
}
/// `s.deffilter(c)`
fn deffilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZOODIAC) and c:GetDefense()>=0
    card::is_set_card(d, c, &[SET_ZOODIAC as u16]) && card::get_defense(d, c) >= 0
}
/// `s.defval(e,c)`
fn defval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local g=e:GetHandler():GetOverlayGroup():Filter(s.deffilter,nil)
    let h = effect::get_handler(d, e).nil();
    let og = card::get_overlay_group(d, h);
    let g = group::filter(d, &og, &|d, c| deffilter(d, c), NO_EX);
    // return g:GetSum(Card.GetDefense)
    group::get_sum(d, &g, &|d, c| card::get_defense(d, c) as i64)
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ZOODIAC) and c:IsMonster()
    card::is_set_card(d, c, &[SET_ZOODIAC as u16]) && card::is_monster(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsType,TYPE_XYZ),tp,LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingTarget(s.filter2,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| filter2(d, c)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsType,TYPE_XYZ),tp,LOCATION_MZONE,0,1,1,nil)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 3));
    // local g=Duel.SelectTarget(tp,s.filter2,tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| filter2(d, c)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // local tc1=g:Filter(Card.IsLocation,nil,LOCATION_MZONE):GetFirst()
    let g_mzone = group::filter(d, &g, &|d, c| card::is_location(d, c, LOCATION_MZONE), NO_EX);
    let tc1 = group::get_first(d, &g_mzone);
    // local g2=g:Filter(Card.IsLocation,nil,LOCATION_GRAVE)
    let g2 = group::filter(d, &g, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // if tc1 and tc1:IsFaceup() and not tc1:IsImmuneToEffect(e) and #g2>0 then
    if let Some(tc1) = tc1 {
        if card::is_faceup(d, tc1) && !card::is_immune_to_effect(d, tc1, a.e) && !g2.is_empty() {
            // Duel.Overlay(tc1,g2)
            duel::overlay(d, tc1, g2, false);
        }
    }
}
