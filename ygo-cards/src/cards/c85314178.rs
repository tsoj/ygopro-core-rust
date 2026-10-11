//! Cosmo Queen the Queen of Prayers
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 85314178;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand in Defense Position
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return re:IsActiveType(TYPE_FIELD) and re:IsHasType(EFFECT_TYPE_ACTIVATE) end)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Target 1 Field Spell, then activate this effect based on who controls it
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.fieldefftg)
    effect::set_target(d, e2, Some(cost_target(fieldefftg)));
    // e2:SetOperation(s.fieldeffop)
    effect::set_operation(d, e2, Some(operation(fieldeffop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e,tp,eg,ep,ev,re,r,rp) return re:IsActiveType(TYPE_FIELD) and re:IsHasType(EFFECT_TYPE_ACTIVATE) end
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    effect::is_active_type(d, a.re.nil(), TYPE_FIELD) && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
// function s.tgfieldfilter(c,tp)
fn tgfieldfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // if not c:IsFaceup() then return false end
    if !card::is_faceup(d, c) {
        return false;
    }
    // if c:IsControler(tp) then
    if card::is_controler(d, c, tp) {
        // return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil,c:GetCode())
        let code = card::get_code(d, c)[0];
        duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, Some(code))), tp, LOCATION_DECK, 0, 1, NO_EX)
    // else
    } else {
        // return c:IsNegatableSpellTrap() and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil)
        card::is_negatable_spell_trap(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| thfilter(d, x, None)), tp, LOCATION_DECK, 0, 1, NO_EX)
    }
}
// function s.thfilter(c,code)
fn thfilter(d: &mut Duel, c: CardId, code: Option<u32>) -> bool {
    // return c:IsFieldSpell() and c:IsAbleToHand() and not (code and c:IsCode(code))
    card::is_field_spell(d, c) && card::is_able_to_hand(d, c, None) && !code.is_some_and(|code| card::is_code(d, c, &[code]))
}
// function s.fieldefftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn fieldefftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_FZONE) and s.tgfieldfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_FZONE) && tgfieldfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.tgfieldfilter,tp,LOCATION_FZONE,LOCATION_FZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| tgfieldfilter(d, c, a.tp)), a.tp, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.tgfieldfilter,tp,LOCATION_FZONE,LOCATION_FZONE,1,1,nil,tp):GetFirst()
    let sel = duel::select_target(d, a.tp, Some(&|d, c| tgfieldfilter(d, c, a.tp)), a.tp, LOCATION_FZONE, LOCATION_FZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel).nil();
    // if tc:IsControler(tp) then
    if card::is_controler(d, tc, a.tp) {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
        // e:SetCategory(CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    } else {
        // e:SetLabel(2)
        effect::set_label(d, a.e, &[2]);
        // e:SetCategory(CATEGORY_DISABLE+CATEGORY_TOHAND+CATEGORY_SEARCH)
        effect::set_category(d, a.e, CATEGORY_DISABLE + CATEGORY_TOHAND + CATEGORY_SEARCH);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(tc)), 1, a.tp, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.fieldeffop(e,tp,eg,ep,ev,re,r,rp)
fn fieldeffop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // local code=tc:GetCode()
        let code = card::get_code(d, tc)[0];
        // if Duel.Destroy(tc,REASON_EFFECT)==0 then return end
        if duel::destroy(d, tc, REASON_EFFECT, None, None) == 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil,code)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, Some(code))), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,tp,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), Some(a.tp), REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    // elseif op==2 then
    } else if op == 2 {
        // if not (tc:IsNegatableSpellTrap() and tc:IsCanBeDisabledByEffect(e)) then return end
        if !(card::is_negatable_spell_trap(d, tc) && card::is_can_be_disabled_by_effect(d, tc, a.e, true)) {
            return;
        }
        // tc:NegateEffects(e:GetHandler(),RESET_PHASE|PHASE_END,true)
        let h = effect::get_handler(d, a.e).nil();
        card::negate_effects(d, tc, h, Some(RESET_PHASE | PHASE_END), true, None);
        // Duel.AdjustInstantly(tc)
        duel::adjust_instantly(d, Some(tc));
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| thfilter(d, c, None)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoHand(g,tp,REASON_EFFECT)
            duel::send_to_hand(d, g.clone(), Some(a.tp), REASON_EFFECT, None);
            // Duel.ConfirmCards(1-tp,g)
            duel::confirm_cards(d, 1 - a.tp, g);
        }
    }
}
