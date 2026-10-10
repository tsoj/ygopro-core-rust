//! Jormungandr, Generaider Boss of Eternity
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 2665273;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:SetUniqueOnField(1,0,id)
    card::set_unique_on_field(d, c, 1, 0, card::UniqueCode::Code(ID), None);
    // Xyz.AddProcedure(c,nil,9,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(9), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // This card's original ATK/DEF become 1000 x its number of materials
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_BASE_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
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
    // e2:SetCode(EFFECT_SET_BASE_DEFENSE)
    effect::set_code(d, e2, EFFECT_SET_BASE_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Each player draws 1 card and attaches 1 card to this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e3, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e3:SetTarget(s.target)
    effect::set_target(d, e3, Some(cost_target(target)));
    // e3:SetOperation(s.operation)
    effect::set_operation(d, e3, Some(operation(op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return c:GetOverlayCount()*1000
    card::get_overlay_count(d, c) as i64 * 1000
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.IsPlayerCanDraw(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,PLAYER_ALL,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, PLAYER_ALL as u8, 1);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ps={}
    let mut ps: Vec<u8> = Vec::new();
    // local pc=false
    let mut pc = false;
    // if Duel.Draw(tp,1,REASON_EFFECT)>0 then
    if duel::draw(d, a.tp, 1, REASON_EFFECT) > 0 {
        // table.insert(ps,tp)
        ps.push(a.tp);
        // pc=true
        pc = true;
    }
    // if Duel.Draw(1-tp,1,REASON_EFFECT)>0 then
    if duel::draw(d, 1 - a.tp, 1, REASON_EFFECT) > 0 {
        // table.insert(ps,1-tp)
        ps.push(1 - a.tp);
        // pc=true
        pc = true;
    }
    // if not (pc and c:IsRelateToEffect(e) and c:IsFaceup()) then return end
    if !(pc && card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c)) {
        return;
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // for _,p in pairs(ps) do
    for p in ps {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATTACH as u64);
        // local tc=Duel.SelectMatchingCard(p,Card.IsCanBeXyzMaterial,p,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,c,c,tp,REASON_EFFECT):GetFirst()
        let g = duel::select_matching_card(
            d,
            p,
            Some(&|d, tc| card::is_can_be_xyz_material(d, tc, Some(c), Some(a.tp), Some(REASON_EFFECT))),
            p,
            LOCATION_HAND | LOCATION_ONFIELD,
            0,
            1,
            1,
            false,
            c.into(),
        )
        .nil();
        // if tc then
        if let Some(tc) = group::get_first(d, &g) {
            // tc:CancelToGrave()
            card::cancel_to_grave(d, tc, true);
            // Duel.Overlay(c,tc,true)
            duel::overlay(d, c, tc, true);
        }
    }
}
