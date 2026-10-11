//! Epurrely Happiness
use super::prelude::*;

pub const ID: u32 = 52645235;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[82105704], listed_series: &[SET_PURRELY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2 Level 2 monsters
    // Xyz.AddProcedure(c,nil,2,2)
    xyz::add_procedure(d, c, None, Some(2), 2, None, None, None, None, false, None);
    // Search 1 "Purrely" card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_DAMAGE_STEP_END)
    effect::set_code(d, e1, EVENT_DAMAGE_STEP_END);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Track if this card "battled"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetOperation(function(e) e:GetHandler():RegisterFlagEffect(id,RESET_PHASE|PHASE_DAMAGE,0,1) end)
    effect::set_operation(d, e2, Some(operation(e2op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Attach "Purrely" Quick-Play Spell
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
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
/// `function(e) e:GetHandler():RegisterFlagEffect(id,RESET_PHASE|PHASE_DAMAGE,0,1) end`
fn e2op(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    let _ = card::register_flag_effect(d, h, ID, RESET_PHASE | PHASE_DAMAGE, 0, 1, 0, 0);
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:HasFlagEffect(id) and (c:IsRelateToBattle() or c:IsReason(REASON_BATTLE))
    card::has_flag_effect(d, c, ID, None) && (card::is_relate_to_battle(d, c) || card::is_reason(d, c, REASON_BATTLE))
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PURRELY) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_PURRELY as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and c:GetOverlayGroup():IsExists(Card.IsCode,1,nil,82105704)
    //     and Duel.IsExistingMatchingCard(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    let overlay = card::get_overlay_group(d, c);
    if card::is_relate_to_effect(d, c, a.e)
        && group::is_exists(d, &overlay, &|d, x| card::is_code(d, x, &[82105704]), 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
        // local tc=Duel.SelectMatchingCard(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil):GetFirst()
        let sel = duel::select_matching_card(d, tp, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel);
        // if not tc then return end
        let Some(tc) = tc else { return };
        // Duel.HintSelection(tc,true)
        duel::hint_selection(d, tc, true);
        // Halve ATK
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_ATTACK_FINAL)
        effect::set_code(d, e1, EFFECT_SET_ATTACK_FINAL);
        // e1:SetValue(tc:GetAttack()//2)
        let atk = card::get_attack(d, tc);
        effect::set_value(d, e1, atk.div_euclid(2) as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
/// `s.qpovcon(e,tp,eg,ep,ev,re,r,rp)`
fn qpovcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if rp==1-tp or not re:IsHasType(EFFECT_TYPE_ACTIVATE) then return false end
    let re = a.re.nil();
    if a.rp == 1 - a.tp || !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    let c = effect::get_handler(d, a.e).nil();
    // return rc:IsSetCard(SET_PURRELY) and rc:IsQuickPlaySpell() and rc:IsOnField()
    //     and rc:IsCanBeXyzMaterial(e:GetHandler(),tp,REASON_EFFECT)
    card::is_set_card(d, rc, &[SET_PURRELY as u16])
        && card::is_quick_play_spell(d, rc)
        && card::is_on_field(d, rc)
        && card::is_can_be_xyz_material(d, rc, Some(c), Some(a.tp), Some(REASON_EFFECT))
}
/// `s.qpovtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn qpovtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.rthfilter(c)`
fn rthfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSpellTrap() and c:IsAbleToHand()
    card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.qpovop(e,tp,eg,ep,ev,re,r,rp)`
fn qpovop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let re = a.re.nil();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rc=re:GetHandler()
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
        duel::overlay(d, c, rc, false);
        // if not c:GetOverlayGroup():IsContains(rc) then return end
        let overlay = card::get_overlay_group(d, c);
        if !group::is_contains(&overlay, rc) {
            return;
        }
        // rc:CancelToGrave()
        card::cancel_to_grave(d, rc, true);
        // if Duel.IsExistingMatchingCard(s.rthfilter,tp,0,LOCATION_ONFIELD,1,nil)
        //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
        if duel::is_existing_matching_card(d, Some(&rthfilter), tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::select_yes_no(d, tp, aux::stringid(ID, 3))
        {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
            // local g=Duel.SelectMatchingCard(tp,s.rthfilter,tp,0,LOCATION_ONFIELD,1,1,nil)
            let g = duel::select_matching_card(d, tp, Some(&rthfilter), tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
            // if #g==0 then return end
            if g.is_empty() {
                return;
            }
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g, None, REASON_EFFECT, None);
        }
    }
}
