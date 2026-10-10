//! Abyss Script - Romantic Terror
use super::prelude::*;

pub const ID: u32 = 41803903;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ABYSS_ACTOR, SET_ABYSS_SCRIPT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // set
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(s.setcon)
    effect::set_condition(d, e2, Some(condition(setcon)));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c,e,tp)`
fn thfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_PENDULUM) and c:IsSetCard(SET_ABYSS_ACTOR) and c:IsAbleToHand()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16])
        && card::is_able_to_hand(d, c, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,hc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, hc: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_PENDULUM) and c:IsSetCard(SET_ABYSS_ACTOR)
    //     and not c:IsOriginalCode(hc:GetOriginalCode()) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    //     and Duel.GetLocationCountFromEx(tp,tp,hc,c)>0
    let hc_code = card::get_original_code(d, hc);
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16])
        && !card::is_original_code(d, c, &[hc_code])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), hc.into(), duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| thfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_MZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
    // local hc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| thfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let hc = group::get_first(d, &g);
    // if hc and Duel.SendtoHand(hc,nil,REASON_EFFECT)~=0 and hc:IsLocation(LOCATION_HAND) then
    if let Some(hc) = hc {
        if duel::send_to_hand(d, hc, None, REASON_EFFECT, None) != 0 && card::is_location(d, hc, LOCATION_HAND) {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
            // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,hc)
            let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, hc)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
            // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP_DEFENSE) end
            if !g.is_empty() {
                duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
            }
        }
    }
}
/// `s.filter2(c)`
fn filter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ABYSS_ACTOR) and c:IsFaceup() and c:IsType(TYPE_PENDULUM)
    card::is_set_card(d, c, &[SET_ABYSS_ACTOR as u16]) && card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM)
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsReason(REASON_EFFECT) and rp==1-tp and c:IsPreviousControler(tp)
    //     and c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEDOWN)
    //     and Duel.IsExistingMatchingCard(s.filter2,tp,LOCATION_EXTRA,0,1,nil)
    card::is_reason(d, c, REASON_EFFECT)
        && a.rp as i64 == 1 - a.tp as i64
        && card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_position(d, c, POS_FACEDOWN)
        && duel::is_existing_matching_card(d, Some(&filter2), a.tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ABYSS_SCRIPT) and c:IsSpell() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_ABYSS_SCRIPT as u16]) && card::is_spell(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&setfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.setfilter,tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&setfilter), tp, LOCATION_DECK, 0, NO_EX);
    // local ct=math.min(Duel.GetLocationCount(tp,LOCATION_SZONE),#g)
    let ct = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0.min(g.len() as i32);
    // if ct<0 then return end
    if ct < 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local sg=g:Select(tp,1,ct,nil)
    let sg = group::select(d, &g, tp, 1, ct as u16, false, NO_EX).nil();
    // Duel.SSet(tp,sg)
    duel::s_set(d, tp, sg, None, true);
}
