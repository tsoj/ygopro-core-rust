//! Zoodiac Gathering
use super::prelude::*;

pub const ID: u32 = 98918572;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ZOODIAC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If a "Zoodiac" Xyz Monster you control would detach its Xyz Material(s) you can detach from another Xyz Monster(s) you control instead
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_OVERLAY_REMOVE_REPLACE)
    effect::set_code(d, e2, EFFECT_OVERLAY_REMOVE_REPLACE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.rcon)
    effect::set_condition(d, e2, Some(condition(rcon)));
    // e2:SetOperation(s.rop)
    effect::set_operation(d, e2, Some(condition(rop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Attach 1 "Zoodiac" Xyz monster to another
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.xyztg)
    effect::set_target(d, e3, Some(cost_target(xyztg)));
    // e3:SetOperation(s.xyzop)
    effect::set_operation(d, e3, Some(operation(xyzop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.rfilter(c,tp,oc)`
fn rfilter(d: &mut Duel, c: CardId, tp: u8, oc: u32) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:CheckRemoveOverlayCard(tp,oc,REASON_COST)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::check_remove_overlay_card(d, c, tp, oc as u16, REASON_COST)
}
/// `s.rcon(e,tp,eg,ep,ev,re,r,rp)`
fn rcon(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // return (r&REASON_COST)~=0 and re:IsActivated()
    //     and re:IsActiveType(TYPE_XYZ) and ep==e:GetOwnerPlayer() and rc:IsSetCard(SET_ZOODIAC)
    //     and Duel.IsExistingMatchingCard(s.rfilter,tp,LOCATION_MZONE,0,1,rc,tp,ev)
    (a.r & REASON_COST) != 0
        && effect::is_activated(d, re)
        && effect::is_active_type(d, re, TYPE_XYZ)
        && a.ep == effect::get_owner_player(d, e)
        && card::is_set_card(d, rc, &[SET_ZOODIAC as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| rfilter(d, x, tp, a.ev)), tp, LOCATION_MZONE, 0, 1, rc.into())
}
/// `s.rop(e,tp,eg,ep,ev,re,r,rp)`: returns the count, which is a true value in Lua
fn rop(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ct=(ev&0xffff)
    let ct = (a.ev & 0xffff) as u16;
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, a.re.nil()).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DEATTACHFROM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DEATTACHFROM as u64);
    // local tg=Duel.SelectMatchingCard(tp,s.rfilter,tp,LOCATION_MZONE,0,1,1,rc,tp,ct)
    let tg = duel::select_matching_card(d, tp, Some(&|d, x| rfilter(d, x, tp, ct as u32)), tp, LOCATION_MZONE, 0, 1, 1, false, rc.into()).nil();
    // tg:GetFirst():RemoveOverlayCard(tp,ct,ct,REASON_COST)
    let first = group::get_first(d, &tg).nil();
    card::remove_overlay_card(d, first, tp, ct, ct, REASON_COST);
    // return ct
    true
}
/// `s.xyzfilter(c)`
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsSetCard(SET_ZOODIAC)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_ZOODIAC as u16])
}
/// `s.xyztg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn xyztg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&xyzfilter), tp, LOCATION_MZONE, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,1))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 1));
    // local g1=Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g1 = duel::select_target(d, tp, Some(&xyzfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g1:GetFirst())
    let first = group::get_first(d, &g1).nil();
    effect::set_label_object(d, e, Value::Card(first));
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local g2=Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,g1:GetFirst())
    let _g2 = duel::select_target(d, tp, Some(&xyzfilter), tp, LOCATION_MZONE, 0, 1, 1, false, first.into());
    true
}
/// `s.xyzop(e,tp,eg,ep,ev,re,r,rp)`
fn xyzop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, e).nil();
    // if #g~=2 then return end
    if g.len() != 2 {
        return;
    }
    // local tc=g:GetFirst()
    // local xc=g:GetNext()
    let order = group::iter(d, &g);
    let (mut tc, mut xc) = (order[0], order[1]);
    // if xc==e:GetLabelObject() then tc,xc=xc,tc end
    if Some(xc) == effect::get_label_object(d, e).as_card() {
        std::mem::swap(&mut tc, &mut xc);
    }
    // if not tc:IsImmuneToEffect(e) then
    if !card::is_immune_to_effect(d, tc, e) {
        // Duel.Overlay(tc,xc,true)
        duel::overlay(d, tc, xc, true);
    }
}
