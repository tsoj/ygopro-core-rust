//! Possessed Partnerships
use super::prelude::*;

pub const ID: u32 = 65046521;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_POSSESSED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Spellcaster with 1500 DEF from your hand or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 1 "Possessed" Continuous Spell/Trap face-up on your field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.tftg)
    effect::set_target(d, e2, Some(cost_target(tftg)));
    // e2:SetOperation(s.tfop)
    effect::set_operation(d, e2, Some(operation(tfop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_SPELLCASTER) and c:IsDefense(1500)
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE)
    card::is_race(d, c, RACE_SPELLCASTER) && card::is_defense(d, c, &[1500]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some((POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8), None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
/// `Card.IsFaceup` as a filter
fn faceup(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_ATTACK|POS_FACEDOWN_DEFENSE)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, (POS_FACEUP_ATTACK | POS_FACEDOWN_DEFENSE) as u8, None) > 0 {
            // if tc:IsFacedown() then Duel.ConfirmCards(1-tp,tc) end
            if card::is_facedown(d, tc) {
                duel::confirm_cards(d, 1 - a.tp, tc);
            }
            // local cg=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil)
            let cg = duel::get_matching_group(d, Some(&faceup), a.tp, LOCATION_MZONE, 0, NO_EX);
            // local dg=Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,nil)
            let dg = duel::get_matching_group(d, Some(&faceup), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, NO_EX);
            // if #cg>=1 and cg:GetBinClassCount(Card.GetAttribute)>=2 and #dg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
            let bin = if cg.len() >= 1 { group::get_bin_class_count(d, &cg, &|d, c| card::get_attribute(d, c) as i64) } else { 0 };
            if cg.len() >= 1 && bin >= 2 && !dg.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
                // Duel.BreakEffect()
                duel::break_effect(d);
                // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
                duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
                // local tc=dg:Select(tp,1,1,nil)
                let sel = group::select(d, &dg, a.tp, 1, 1, false, NO_EX).nil();
                // Duel.HintSelection(tc)
                duel::hint_selection(d, &sel, true);
                // Duel.Destroy(tc,REASON_EFFECT)
                duel::destroy(d, &sel, REASON_EFFECT, None, None);
            }
        }
    }
}
/// `s.tffilter(c,tp)`
fn tffilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsContinuousSpellTrap() and c:IsSetCard(SET_POSSESSED) and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_continuous_spell_trap(d, c) && card::is_set_card(d, c, &[SET_POSSESSED as u16]) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
/// `s.tftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tffilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && tffilter(d, chkc, a.tp);
    }
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and Duel.IsExistingTarget(s.tffilter,tp,LOCATION_GRAVE,0,1,e:GetHandler(),tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| tffilter(d, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
    // local g=Duel.SelectTarget(tp,s.tffilter,tp,LOCATION_GRAVE,0,1,1,e:GetHandler(),tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| tffilter(d, c, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, h.into()).nil();
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), 1, 0, 0);
    true
}
/// `s.tfop(e,tp,eg,ep,ev,re,r,rp)`
fn tfop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFirstTarget()
    let g = duel::first_target(d);
    // local a,b=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let avail = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if a<1 or not g or not g:IsRelateToEffect(e) then return end
    if avail < 1 {
        return;
    }
    let g = match g {
        Some(g) => g,
        None => return,
    };
    if !card::is_relate_to_effect(d, g, a.e) {
        return;
    }
    // Duel.MoveToField(g,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
    duel::move_to_field(d, g, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
}
