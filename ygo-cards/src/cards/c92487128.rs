//! Sosei Ryu-Ge Mistva
use super::prelude::*;

pub const ID: u32 = 92487128;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_RYU_GE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // c:SetSPSummonOnce(id)
    card::set_spsummon_once(d, c, ID);
    // Add 1 "Ryu-Ge" card from your Deck to your hand, except a Pendulum Monster, then destroy this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_DESTROY)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_DESTROY);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Tribute 1 Level 10 "Ryu-Ge" monster, and if you do, Special Summon this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_RELEASE+CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_RELEASE + CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e2, LOCATION_EXTRA);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RYU_GE) and not c:IsType(TYPE_PENDULUM) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_RYU_GE as u16]) && !card::is_type(d, c, TYPE_PENDULUM) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 and Duel.SendtoHand(g,nil,REASON_EFFECT)>0 then
    if !g.is_empty() && duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if not c:IsRelateToEffect(e) then return end
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(c,REASON_EFFECT)
        duel::destroy(d, c, REASON_EFFECT, None, None);
    }
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsReason(REASON_BATTLE|REASON_EFFECT)
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.spconfilter,1,nil)
    let h = effect::get_handler(d, a.e).nil();
    let eg = a.eg();
    !group::is_contains(&eg, h) && group::is_exists(d, &eg, &spconfilter, 1, NO_EX)
}
/// `s.tributefilter(c,e,tp,hc)`
fn tributefilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, hc: CardId) -> bool {
    // return c:IsLevel(10) and c:IsSetCard(SET_RYU_GE) and c:IsReleasableByEffect()
    //     and Duel.GetLocationCountFromEx(tp,tp,c,hc)>0
    let _ = e;
    card::is_level(d, c, &[10])
        && card::is_set_card(d, c, &[SET_RYU_GE as u16])
        && card::is_releasable_by_effect(d, c, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), c.into(), duel::ExtraCard::Card(hc), None).0 > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tributefilter,tp,LOCATION_MZONE,0,1,nil,e,tp,c)
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_RITUAL,tp,true,true) end
    if a.chk == Some(0) {
        let e = a.e;
        let tp = a.tp;
        return duel::is_existing_matching_card(d, Some(&|d, x| tributefilter(d, x, e, tp, c)), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
            && card::is_can_be_special_summoned(d, c, a.e, SUMMON_TYPE_RITUAL, a.tp, true, true, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.plfilter(c)`
fn plfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_RYU_GE) and c:IsContinuousSpell() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_RYU_GE as u16]) && card::is_continuous_spell(d, c) && !card::is_forbidden(d, c)
}
/// `s.tfcheck(sg,e,tp)`
fn tfcheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 or sg:IsExists(Card.IsLocation,1,nil,LOCATION_STZONE)
    let r = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        || group::is_exists(d, sg, &|d, c| card::is_location(d, c, LOCATION_STZONE), 1, NO_EX);
    (r, false)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // local rg=Duel.SelectMatchingCard(tp,s.tributefilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp,c)
    let e = a.e;
    let tp = a.tp;
    let rg = duel::select_matching_card(d, a.tp, Some(&|d, x| tributefilter(d, x, e, tp, c)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #rg==0 or Duel.Release(rg,REASON_EFFECT)==0 then return end
    if rg.is_empty() || duel::release(d, rg, REASON_EFFECT, None) == 0 {
        return;
    }
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,SUMMON_TYPE_RITUAL,tp,tp,true,true,POS_FACEUP)>0 then
    if card::is_relate_to_effect(d, c, a.e) && duel::special_summon(d, c, SUMMON_TYPE_RITUAL, a.tp, a.tp, true, true, POS_FACEUP as u8, None) > 0 {
        // c:CompleteProcedure()
        card::complete_procedure(d, c);
        // local fg=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,0)
        let fg = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, 0);
        // local dg=Duel.GetMatchingGroup(s.plfilter,tp,LOCATION_DECK,0,nil)
        let dg = duel::get_matching_group(d, Some(&plfilter), a.tp, LOCATION_DECK, 0, NO_EX);
        // if #fg>0 and #dg>0 and aux.SelectUnselectGroup(fg,e,tp,1,2,s.tfcheck,0) and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
        if !fg.is_empty()
            && !dg.is_empty()
            && matches!(aux::select_unselect_group(d, &fg, Some(a.e), a.tp, Some(1), Some(2), Some(&tfcheck), 0, a.tp, None, None, None, false), Err(true))
            && duel::select_yes_no(d, a.tp, aux::stringid(ID, 2))
        {
            // local desg=aux.SelectUnselectGroup(fg,e,tp,1,2,s.tfcheck,1,tp,HINTMSG_DESTROY,s.tfcheck)
            let desg = aux::select_unselect_group(d, &fg, Some(a.e), a.tp, Some(1), Some(2), Some(&tfcheck), 1, a.tp, Some(HINTMSG_DESTROY as u64), Some(&tfcheck), None, false).unwrap_or_default();
            // Duel.HintSelection(desg)
            duel::hint_selection(d, desg.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // local ct=Duel.Destroy(desg,REASON_EFFECT)
            let ct = duel::destroy(d, desg, REASON_EFFECT, None, None);
            // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
            let ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
            // ct=math.min(ct,ft)
            let ct = ct.min(ft);
            // if ct==0 then return end
            if ct == 0 {
                return;
            }
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOFIELD)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOFIELD as u64);
            // local placeg=dg:Select(tp,1,ct,nil)
            let placeg = group::select(d, &dg, a.tp, 1, ct as u16, false, NO_EX).nil();
            // for tc in placeg:Iter() do
            for tc in group::iter(d, &placeg) {
                // Duel.MoveToField(tc,tp,tp,LOCATION_SZONE,POS_FACEUP,true)
                duel::move_to_field(d, tc, a.tp, a.tp, LOCATION_SZONE as u16, POS_FACEUP as u8, true, None);
            }
        }
    }
}
