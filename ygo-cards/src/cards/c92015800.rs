//! Number 76: Harmonizer Gradielle
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 92015800;
pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(76), ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon Procedure
    // Xyz.AddProcedure(c,nil,7,2)
    xyz::add_procedure(d, c, None, Some(7), 2, None, None, None, None, false, None);
    // Attribute change
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_ADD_ATTRIBUTE)
    effect::set_code(d, e1, EFFECT_ADD_ATTRIBUTE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return e:GetHandler():GetOverlayGroup():GetBitwiseOr(Card.GetAttribute) end)
    effect::set_value_fn(d, e1, effect_card_value(|d, e, _c| {
        let handler = effect::get_handler(d, e).nil();
        let g = card::get_overlay_group(d, handler);
        group::get_bitwise_or(d, &g, &|d, x| card::get_attribute(d, x) as i64) as i64
    }));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be destroyed
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_BATTLE);
    // e2:SetValue(function(e,c) return c:IsAttribute(e:GetHandler():GetAttribute()) end)
    effect::set_value_fn(d, e2, lua_fn(|d, a| {
        let c = a.c(1);
        let handler = effect::get_handler(d, a.e(0)).nil();
        let attr = card::get_attribute(d, handler);
        card::is_attribute(d, c, attr)
    }));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_EFFECT);
    // e3:SetValue(function(e,re,rp) return re:IsActivated() and re:IsMonsterEffect() and rp==1-e:GetHandlerPlayer() and re:IsCardAttribute(e:GetHandler():GetAttribute()) end)
    effect::set_value_fn(d, e3, lua_fn(|d, a| {
        let (e, re) = (a.e(0), a.e(1));
        let rp = a.int(2);
        effect::is_activated(d, re)
            && effect::is_monster_effect(d, re)
            && rp == 1 - effect::get_handler_player(d, e) as i64
            && {
                let handler = effect::get_handler(d, e).nil();
                let attr = card::get_attribute(d, handler);
                effect::is_card_attribute(d, re, attr)
            }
    }));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Attach material
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_CARD_TARGET, 0);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e4, 0, Some(TIMING_END_PHASE));
    // e4:SetCountLimit(1,id)
    effect::set_count_limit(d, e4, 1, ID, 0, 0);
    // e4:SetTarget(s.mttg)
    effect::set_target(d, e4, Some(cost_target(mttg)));
    // e4:SetOperation(s.mtop)
    effect::set_operation(d, e4, Some(operation(mtop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
fn mttg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(1-tp) and chkc:IsMonster() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - tp) && card::is_monster(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsMonster,tp,0,LOCATION_GRAVE,1,nil)
    //     and e:GetHandler():CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end
    if a.chk == Some(0) {
        let handler = effect::get_handler(d, e).nil();
        return duel::is_existing_target(d, Some(&card::is_monster), tp, 0, LOCATION_GRAVE, 1, NO_EX)
            && card::check_remove_overlay_card(d, handler, tp, 1, REASON_EFFECT);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
    // local g=Duel.SelectTarget(tp,Card.IsMonster,tp,0,LOCATION_GRAVE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&card::is_monster), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
fn mtop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    // if c:IsRelateToEffect(e) and c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)>0 and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) && card::remove_overlay_card(d, c, tp, 1, 1, REASON_EFFECT) > 0 {
        let tc = duel::first_target(d).nil();
        if card::is_relate_to_effect(d, tc, e) {
            // Duel.Overlay(c,tc)
            duel::overlay(d, c, tc, false);
        }
    }
}
