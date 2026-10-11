//! Epurrely Plump
use super::prelude::*;

pub const ID: u32 = 24434049;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[55584558],
    listed_series: &[SET_PURRELY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 2 monsters
    // Xyz.AddProcedure(c,nil,2,2)
    xyz::add_procedure(d, c, None, Some(2), 2, None, None, None, None, false, None);
    // Attach 2 Spell/Trap cards
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e1, CATEGORY_LEAVE_GRAVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e1:SetCondition(s.gyovcon)
    effect::set_condition(d, e1, Some(condition(gyovcon)));
    // e1:SetTarget(s.gyovtg)
    effect::set_target(d, e1, Some(cost_target(gyovtg)));
    // e1:SetOperation(s.gyovop)
    effect::set_operation(d, e1, Some(operation(gyovop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_END_PHASE));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Attach "Purrely" Quick-Play Spell
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(3)
    effect::set_count_limit(d, e3, 3, 0, 0, 0);
    // e3:SetCondition(s.qpovcon)
    effect::set_condition(d, e3, Some(condition(qpovcon)));
    // e3:SetTarget(s.qpovtg)
    effect::set_target(d, e3, Some(cost_target(qpovtg)));
    // e3:SetOperation(s.qpovop)
    effect::set_operation(d, e3, Some(operation(qpovop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.gyovcon(e,tp,eg,ep,ev,re,r,rp)`
fn gyovcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:IsHasType(EFFECT_TYPE_QUICK_O)==e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,55584558)
    let quick = effect::is_has_type(d, a.e, EFFECT_TYPE_QUICK_O);
    let h = effect::get_handler(d, a.e).nil();
    let ov = card::get_overlay_group(d, h);
    let exists = group::is_exists(d, &ov, &|d, x| card::is_code(d, x, &[55584558]), 1, NO_EX);
    quick == exists
}
/// `s.gyovfilter(c,xc,tp)`
fn gyovfilter(d: &mut Duel, c: CardId, xc: CardId, tp: u8) -> bool {
    // return c:IsSpellTrap() and c:IsCanBeXyzMaterial(xc,tp,REASON_EFFECT)
    card::is_spell_trap(d, c) && card::is_can_be_xyz_material(d, c, Some(xc), Some(tp), Some(REASON_EFFECT))
}
/// `s.gyovtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn gyovtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and s.gyovfilter(chkc,c,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && gyovfilter(d, chkc, c, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.gyovfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,c,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| gyovfilter(d, x, c, tp)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local g=Duel.SelectTarget(tp,s.gyovfilter,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,2,nil,c,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| gyovfilter(d, x, c, tp)), tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 2, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), n, 0, 0);
    true
}
/// `s.gyovop(e,tp,eg,ep,ev,re,r,rp)`
fn gyovop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsImmuneToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_immune_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetTargetCards(e):Filter(s.gyovfilter,nil,c,tp):Remove(Card.IsImmuneToEffect,nil,e)
    let targets = duel::get_target_cards(d, a.e).nil();
    let mut g = group::filter(d, &targets, &|d, x| gyovfilter(d, x, c, tp), NO_EX);
    group::remove(d, &mut g, &|d, x| card::is_immune_to_effect(d, x, a.e), NO_EX);
    // if #g>0 then
    if g.len() > 0 {
        // Duel.Overlay(c,g)
        duel::overlay(d, c, g, false);
    }
}
/// `s.qpovcon(e,tp,eg,ep,ev,re,r,rp)`
fn qpovcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if rp==1-tp or not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return false end
    if a.rp == 1 - a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // return rc:IsSetCard(SET_PURRELY) and rc:IsQuickPlaySpell()
    //     and rc:IsOnField() and rc:IsCanBeXyzMaterial(e:GetHandler(),tp,REASON_EFFECT)
    card::is_set_card(d, rc, &[SET_PURRELY as u16])
        && card::is_quick_play_spell(d, rc)
        && card::is_on_field(d, rc)
        && card::is_can_be_xyz_material(d, rc, Some(effect::get_handler(d, a.e).nil()), Some(a.tp), Some(REASON_EFFECT))
}
/// `s.qpovtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn qpovtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_REMOVE,nil,1,PLAYER_ALL,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_REMOVE, None, 1, PLAYER_ALL as u8, LOCATION_MZONE as i32);
    true
}
/// `s.qpovop(e,tp,eg,ep,ev,re,r,rp)`
fn qpovop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if c:IsRelateToEffect(e) and rc:IsRelateToEffect(re)
    //     and not c:IsImmuneToEffect(e) and not rc:IsImmuneToEffect(e)
    //     and rc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT) then
    if card::is_relate_to_effect(d, c, a.e)
        && card::is_relate_to_effect(d, rc, re)
        && !card::is_immune_to_effect(d, c, a.e)
        && !card::is_immune_to_effect(d, rc, a.e)
        && card::is_can_be_xyz_material(d, rc, Some(c), Some(tp), Some(REASON_EFFECT))
    {
        // Duel.Overlay(c,rc)
        duel::overlay(d, c, CardSet::from_card(rc), false);
        // if not c:GetOverlayGroup():IsContains(rc) then return end
        let ov = card::get_overlay_group(d, c);
        if !group::is_contains(&ov, rc) {
            return;
        }
        // rc:CancelToGrave()
        card::cancel_to_grave(d, rc, true);
        // if Duel.IsExistingMatchingCard(Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
            // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemove,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
            let g = duel::select_matching_card(d, tp, Some(&|d, x| card::is_able_to_remove(d, x, None, None, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
            // if #g==0 then return end
            if g.len() == 0 {
                return;
            }
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // aux.RemoveUntil(g,nil,REASON_EFFECT,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp)
            aux::remove_until(
                d,
                g,
                None,
                REASON_EFFECT,
                PHASE_END,
                ID,
                a.e,
                tp,
                Some(aux::default_field_return()),
                None,
                None,
                None,
                None,
                None,
            );
        }
    }
}
