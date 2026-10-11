//! FA－ブラック・レイ・ランサー
//! Full Armored Black Ray Lancer
use super::prelude::*;

pub const ID: u32 = 25853045;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // xyz summon
    // Xyz.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_WATER),4,3,s.ovfilter,aux.Stringid(id,0))
    xyz::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, sc, st, Some(tp)))),
        Some(4),
        3,
        Some(xyz::alter_filter(ovfilter)),
        Some(aux::stringid(ID, 0)),
        None,
        None,
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
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(s.atkval)
    effect::set_value_fn(d, e1, lua_fn(atkval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // destroy replace
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.reptg)
    effect::set_target(d, e2, Some(cost_target(reptg)));
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
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e3, EVENT_BATTLE_DESTROYING);
    // e3:SetCondition(aux.bdocon)
    effect::set_condition(d, e3, Some(aux::bdocon()));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.ovfilter(c,tp,xyzc)
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, xyzc: CardId) -> bool {
    // return c:IsFaceup() and c:GetRank()==3 and c:IsAttribute(ATTRIBUTE_WATER,xyzc,SUMMON_TYPE_XYZ,tp) and c:GetOverlayCount()==0
    card::is_faceup(d, c)
        && card::get_rank(d, c) == 3
        && card::is_attribute_ctx(d, c, ATTRIBUTE_WATER, Some(xyzc), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::get_overlay_count(d, c) == 0
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, a: Args) -> i64 {
    let c = a.c(1);
    // return c:GetOverlayCount()*200
    card::get_overlay_count(d, c) as i64 * 200
}
// function s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && card::check_remove_overlay_card(d, c, tp, 1, REASON_EFFECT);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, tp, c, Some(96)) {
        // local g=c:GetOverlayGroup()
        let g = card::get_overlay_group(d, c);
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        // return true
        true
    // else return false end
    } else {
        false
    }
}
// function s.filter(c)
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap()
    card::is_spell_trap(d, c)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, Some(&filter), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
