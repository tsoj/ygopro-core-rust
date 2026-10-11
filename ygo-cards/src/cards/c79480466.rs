//! Bazoo the Soul-Devourer
use super::prelude::*;

pub const ID: u32 = 79480466;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Duel.EnableGlobalFlag(GLOBALFLAG_DETACH_EVENT)
    duel::enable_global_flag(d, GLOBALFLAG_DETACH_EVENT);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,3)
    xyz::add_procedure(d, c, None, Some(4), 3, None, None, None, None, false, None);
    // Any Xyz Material detached from a monster is banished instead of being sent to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_RANGE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_RANGE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_TO_GRAVE_REDIRECT)
    effect::set_code(d, e1, EFFECT_TO_GRAVE_REDIRECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_OVERLAY) and not c:IsReason(REASON_LOST_TARGET) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_OVERLAY) && !card::is_reason(d, c, REASON_LOST_TARGET))));
    // e1:SetValue(LOCATION_REMOVED)
    effect::set_value(d, e1, LOCATION_REMOVED as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Detach 1 material from this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e2:SetCondition(function(e,tp,eg) return eg:IsExists(Card.IsMonsterCard,1,nil) end)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return e:GetHandler():CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end end)
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    // e2:SetOperation(s.detachop)
    effect::set_operation(d, e2, Some(operation(detachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Attach up to 3 cards in the GY(s) to this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e3, CATEGORY_LEAVE_GRAVE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetCode(EVENT_DETACH_MATERIAL)
    effect::set_code(d, e3, EVENT_DETACH_MATERIAL);
    // e3:SetCondition(function(e) return e:GetHandler():GetOverlayCount()==0 end)
    effect::set_condition(d, e3, Some(condition(e3_condition)));
    // e3:SetTarget(s.attachtg)
    effect::set_target(d, e3, Some(cost_target(attachtg)));
    // e3:SetOperation(s.attachop)
    effect::set_operation(d, e3, Some(operation(attachop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.detachop(e,tp,eg,ep,ev,re,r,rp)`
fn detachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)
        card::remove_overlay_card(d, c, a.tp, 1, 1, REASON_EFFECT);
    }
}
/// `function(e,tp,eg) return eg:IsExists(Card.IsMonsterCard,1,nil) end`
fn e2_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| card::is_monster_card(d, c), 1, NO_EX)
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return e:GetHandler():CheckRemoveOverlayCard(tp,1,REASON_EFFECT) end end`
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::check_remove_overlay_card(d, h, a.tp, 1, REASON_EFFECT);
    }
    true
}
/// `function(e) return e:GetHandler():GetOverlayCount()==0 end`
fn e3_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, h) == 0
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_can_be_xyz_material(d, chkc, Some(c), Some(a.tp), Some(REASON_EFFECT));
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsCanBeXyzMaterial,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,c,tp,REASON_EFFECT) end
    if a.chk == Some(0) {
        return duel::is_existing_target(
            d,
            Some(&|d, x| card::is_can_be_xyz_material(d, x, Some(c), Some(a.tp), Some(REASON_EFFECT))),
            a.tp,
            LOCATION_GRAVE,
            LOCATION_GRAVE,
            1,
            NO_EX,
        );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
    // local g=Duel.SelectTarget(tp,Card.IsCanBeXyzMaterial,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,3,nil,c,tp,REASON_EFFECT)
    let g = duel::select_target(
        d,
        a.tp,
        Some(&|d, x| card::is_can_be_xyz_material(d, x, Some(c), Some(a.tp), Some(REASON_EFFECT))),
        a.tp,
        LOCATION_GRAVE,
        LOCATION_GRAVE,
        1,
        3,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,#g,tp,0)
    let n = g.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, n, a.tp, 0);
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsImmuneToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_immune_to_effect(d, c, a.e) {
        return;
    }
    // local tg=Duel.GetTargetCards(e):Filter(Card.IsCanBeXyzMaterial,nil,c,tp,REASON_EFFECT):Remove(Card.IsImmuneToEffect,nil,e)
    let targets = duel::get_target_cards(d, a.e).nil();
    let mut tg = group::filter(d, &targets, &|d, x| card::is_can_be_xyz_material(d, x, Some(c), Some(a.tp), Some(REASON_EFFECT)), NO_EX);
    group::remove(d, &mut tg, &|d, x| card::is_immune_to_effect(d, x, a.e), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Overlay(c,tg)
        duel::overlay(d, c, tg, false);
    }
}
