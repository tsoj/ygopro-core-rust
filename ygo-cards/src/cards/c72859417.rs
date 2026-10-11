//! Lyrilusc - Bird Sanctuary
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 72859417;

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
    // Attach
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.xyztg)
    effect::set_target(d, e2, Some(cost_target(xyztg)));
    // e2:SetOperation(s.xyzop)
    effect::set_operation(d, e2, Some(operation(xyzop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // draw
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.drcon)
    effect::set_condition(d, e3, Some(condition(drcon)));
    // e3:SetTarget(s.drtg)
    effect::set_target(d, e3, Some(cost_target(drtg)));
    // e3:SetOperation(s.drop)
    effect::set_operation(d, e3, Some(operation(drop_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.xyzfilter(c)
fn xyzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsRace(RACE_WINGEDBEAST)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_race(d, c, RACE_WINGEDBEAST)
}
// function s.xyztg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn xyztg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,2,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&xyzfilter), a.tp, LOCATION_MZONE, 0, 2, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 2));
    // local g1=Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil)
    let g1 = duel::select_target(d, a.tp, Some(&xyzfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g1:GetFirst())
    let first = group::get_first(d, &g1);
    effect::set_label_object(d, a.e, first.map_or(Value::Nil, Value::Card));
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 3));
    // local g2=Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,g1:GetFirst())
    let _g2 = duel::select_target(
        d,
        a.tp,
        Some(&xyzfilter),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        1,
        false,
        first.map_or(NO_EX, |x| x.into()),
    );
    true
}
// function s.xyzop(e,tp,eg,ep,ev,re,r,rp)
fn xyzop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).unwrap_or_default();
    // if #g~=2 then return end
    if g.len() != 2 {
        return;
    }
    // local tc=g:GetFirst()
    // local xc=g:GetNext()
    let order = group::iter(d, &g);
    let mut tc = order[0];
    let mut xc = order[1];
    // if tc:IsImmuneToEffect(e) or xc:IsImmuneToEffect(e) then return end
    if card::is_immune_to_effect(d, tc, a.e) || card::is_immune_to_effect(d, xc, a.e) {
        return;
    }
    // if xc==e:GetLabelObject() then tc,xc=xc,tc end
    if effect::get_label_object(d, a.e).as_card() == Some(xc) {
        std::mem::swap(&mut tc, &mut xc);
    }
    // Duel.Overlay(tc,xc)
    duel::overlay(d, tc, CardSet::from_card(xc), false);
}
// function s.drcfilter(c)
fn drcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:GetOverlayCount()>=3
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::get_overlay_count(d, c) >= 3
}
// function s.drcon(e,tp,eg,ep,ev,re,r,rp)
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.drcfilter,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&drcfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
// function s.drop(e,tp,eg,ep,ev,re,r,rp)
fn drop_op(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
