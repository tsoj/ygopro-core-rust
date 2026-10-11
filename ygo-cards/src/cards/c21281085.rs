//! Fabled Levilazebul
use super::prelude::*;

pub const ID: u32 = 21281085;

// s.listed_series={SET_FABLED}
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FABLED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Tribute any number of "Fabled" monsters and take control of that many face-up monsters your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_RELEASE+CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_RELEASE + CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(ctcon)));
    // e1:SetTarget(s.cttg)
    effect::set_target(d, e1, Some(cost_target(cttg)));
    // e1:SetOperation(s.ctop)
    effect::set_operation(d, e1, Some(operation(ctop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return this card to the Extra Deck and add 1 other "Fabled" card from your GY to your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOEXTRA+CATEGORY_TOHAND)
    effect::set_category(d, e2, CATEGORY_TOEXTRA + CATEGORY_TOHAND);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rlfilter(c,tp,chk)`
fn rlfilter(d: &mut Duel, c: CardId, tp: u8, chk: i64) -> bool {
    // return c:IsReleasableByEffect() and c:IsSetCard(SET_FABLED)
    //     and (chk==1 or Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_CONTROL)>0)
    card::is_releasable_by_effect(d, c, None, None)
        && card::is_set_card(d, c, &[SET_FABLED as u16])
        && (chk == 1 || duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0)
}
/// `function() return Duel.IsMainPhase() end`
fn ctcon(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `aux.FaceupFilter(Card.IsAbleToChangeControler)`
fn faceup_ctrl(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_able_to_change_controler(d, c)
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rlfilter,tp,LOCATION_MZONE,0,1,nil,tp,chk)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAbleToChangeControler),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        // chk is 0 in this branch
        return duel::is_existing_matching_card(d, Some(&|d, c| rlfilter(d, c, tp, 0)), tp, LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&faceup_ctrl), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_RELEASE,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_RELEASE, None, 1, tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,1,1-tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, 1 - tp, LOCATION_MZONE as i32);
    true
}
/// `s.rlrescon(sg,e,tp)`
fn rlrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return Duel.GetMZoneCount(tp,sg,tp,LOCATION_REASON_CONTROL)>=#sg
    let res = duel::get_m_zone_count(d, tp, sg.into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 >= sg.len() as i32;
    (res, false)
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local cg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAbleToChangeControler),tp,0,LOCATION_MZONE,nil)
    let cg = duel::get_matching_group(d, Some(&faceup_ctrl), tp, 0, LOCATION_MZONE, NO_EX);
    // if #cg==0 then return end
    if cg.is_empty() {
        return;
    }
    // local rg=Duel.GetMatchingGroup(s.rlfilter,tp,LOCATION_MZONE,0,nil,tp,1)
    let rg = duel::get_matching_group(d, Some(&|d, c| rlfilter(d, c, tp, 1)), tp, LOCATION_MZONE, 0, NO_EX);
    // if #rg==0 then return end
    if rg.is_empty() {
        return;
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,#cg,s.rlrescon,1,tp,HINTMSG_RELEASE)
    let maxc = cg.len();
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(maxc), Some(&rlrescon), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, false)
        .unwrap_or_default();
    // if #g==0 or Duel.Release(g,REASON_EFFECT)==0 then return end
    if g.is_empty() || duel::release(d, &g, REASON_EFFECT, None) == 0 {
        return;
    }
    // local og=Duel.GetOperatedGroup()
    let og = duel::get_operated_group(d);
    // local ct=#og
    let ct = og.len();
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONTROL as u64);
    // local sg=(cg-og):Select(tp,ct,ct,nil)
    let mut diff = cg.clone();
    group::remove_card(&mut diff, &og);
    let sg = group::select(d, &diff, tp, ct as u16, ct as u16, false, NO_EX).nil();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // Duel.HintSelection(sg)
    duel::hint_selection(d, &sg, true);
    // if Duel.GetControl(sg,tp)>0 then
    if duel::get_control(d, &sg, tp, 0, 0, None, None) > 0 {
        // Duel.GetOperatedGroup():ForEach(Card.NegateEffects,c)
        let operated = duel::get_operated_group(d);
        for tc in group::iter(d, &operated) {
            card::negate_effects(d, tc, c, None, false, None);
        }
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FABLED) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_FABLED as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.thfilter(chkc) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && thfilter(d, chkc) && chkc != c;
    }
    // if chk==0 then return c:IsAbleToExtra() and Duel.IsExistingTarget(s.thfilter,tp,LOCATION_GRAVE,0,1,c) end
    if a.chk == Some(0) {
        return card::is_able_to_extra(d, c) && duel::is_existing_target(d, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectTarget(tp,s.thfilter,tp,LOCATION_GRAVE,0,1,1,c)
    let g = duel::select_target(d, tp, Some(&thfilter), tp, LOCATION_GRAVE, 0, 1, 1, false, c.into());
    // Duel.SetOperationInfo(0,CATEGORY_TOEXTRA,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOEXTRA, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g, 1, tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and Duel.SendtoDeck(c,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)>0
    //     and c:IsLocation(LOCATION_EXTRA) and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e)
        && duel::send_to_deck(d, c, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None) > 0
        && card::is_location(d, c, LOCATION_EXTRA)
        && card::is_relate_to_effect(d, tc.nil(), e)
    {
        // Duel.SendtoHand(tc,nil,REASON_EFFECT)
        duel::send_to_hand(d, tc.nil(), None, REASON_EFFECT, None);
    }
}
