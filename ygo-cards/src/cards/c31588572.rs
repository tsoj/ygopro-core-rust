//! Unchained Twins - Sarama
//! 破械童子サラマ
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 31588572;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_UNCHAINED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Set 1 "Unchained" card from the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.settg)
    effect::set_target(d, e1, Some(cost_target(settg)));
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Unchained" monster from the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.setfilter(c,e,tp)`
fn setfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not c:IsSetCard(SET_UNCHAINED) then return end
    if !card::is_set_card(d, c, &[SET_UNCHAINED as u16]) {
        return false;
    }
    // if c:IsMonster() and not c:IsCode(id) then
    if card::is_monster(d, c) && !card::is_code(d, c, &[ID]) {
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE)
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None);
    // elseif c:IsSpellTrap() then
    } else if card::is_spell_trap(d, c) {
        // return (c:IsFieldSpell() or Duel.GetLocationCount(tp,LOCATION_SZONE)>0) and c:IsSSetable()
        return (card::is_field_spell(d, c) || duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0)
            && card::is_s_setable(d, c, false, None);
    }
    // return false
    false
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.setfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && setfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.setfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| setfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectTarget(tp,s.setfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| setfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g.clone().nil()).nil();
    // if tc:IsMonster() then
    if card::is_monster(d, tc) {
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g.clone(), 1, tp, LOCATION_GRAVE as i32);
    // elseif tc:IsSpellTrap() then
    } else if card::is_spell_trap(d, tc) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g.clone(), 1, tp, LOCATION_GRAVE as i32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 0, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    // if not tc or not tc:IsRelateToEffect(e) then return end
    let Some(tc) = duel::first_target(d) else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local andifyoudo=false
    let mut andifyoudo = false;
    // if tc:IsMonster() then
    if card::is_monster(d, tc) {
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)>0 then
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) > 0
        {
            // andifyoudo=true
            andifyoudo = true;
        }
    // elseif tc:IsSpellTrap() then
    } else if card::is_spell_trap(d, tc) {
        // if tc:IsFieldSpell() then
        if card::is_field_spell(d, tc) {
            // local fc=Duel.GetFieldCard(tp,LOCATION_FZONE,0)
            let fc = duel::get_field_card(d, tp, LOCATION_FZONE, 0);
            // if fc then
            if let Some(fc) = fc {
                // Duel.SendtoGrave(fc,REASON_RULE)
                duel::send_to_grave(d, fc, REASON_RULE, None, None);
                // Duel.BreakEffect()
                duel::break_effect(d);
            }
        }
        // if Duel.GetLocationCount(tp,LOCATION_SZONE)>0 then
        if duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
            // Duel.SSet(tp,tc)
            duel::s_set(d, tp, tc, None, true);
            // andifyoudo=true
            andifyoudo = true;
        }
    }
    // if andifyoudo then
    if andifyoudo {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectMatchingCard(tp,nil,tp,LOCATION_ONFIELD,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, None, tp, LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Destroy(g,REASON_EFFECT)
            duel::destroy(d, g, REASON_EFFECT, None, None);
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_EFFECT+REASON_BATTLE)~=0 and e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD)
    //     and not (re and re:GetHandler():IsCode(id))
    if (a.r & (REASON_EFFECT + REASON_BATTLE)) == 0 {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_previous_location(d, h, LOCATION_ONFIELD) {
        return false;
    }
    match a.re {
        Some(re) => {
            let rh = effect::get_handler(d, re).nil();
            !card::is_code(d, rh, &[ID])
        }
        None => true,
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_UNCHAINED) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_UNCHAINED as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
        && !card::is_code(d, c, &[ID])
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
