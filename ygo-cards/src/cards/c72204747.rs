//! Over Destiny
use super::prelude::*;

pub const ID: u32 = 72204747;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DESTINY_HERO], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter1(c,e,sp)`
fn filter1(d: &mut Duel, c: CardId, e: EffectId, sp: u8) -> bool {
    // local lv=math.floor(c:GetLevel()/2)
    let lv = card::get_level(d, c) as i64 / 2;
    // return lv>0 and c:IsSetCard(SET_DESTINY_HERO)
    //     and Duel.IsExistingMatchingCard(s.filter2,sp,LOCATION_DECK,0,1,nil,lv,e,sp)
    lv > 0
        && card::is_set_card(d, c, &[SET_DESTINY_HERO as u16])
        && duel::is_existing_matching_card(d, Some(&|d, x| filter2(d, x, lv, e, sp)), sp, LOCATION_DECK, 0, 1, NO_EX)
}
/// `s.filter2(c,lv,e,sp)`
fn filter2(d: &mut Duel, c: CardId, lv: i64, e: EffectId, sp: u8) -> bool {
    // return c:IsLevelBelow(lv) and c:IsSetCard(SET_DESTINY_HERO) and c:IsCanBeSpecialSummoned(e,0,sp,false,false)
    card::is_level_below(d, c, lv as u32)
        && card::is_set_card(d, c, &[SET_DESTINY_HERO as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, sp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.filter1(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && filter1(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter1,tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter1(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,0))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 0));
    // Duel.SelectTarget(tp,s.filter1,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let _ = duel::select_target(d, a.tp, Some(&|d, c| filter1(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    // if not tc or not tc:IsRelateToEffect(e) then return end
    let Some(tc) = duel::first_target(d) else {
        return;
    };
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local cg=Duel.SelectMatchingCard(tp,s.filter2,tp,LOCATION_DECK,0,1,1,nil,math.floor(tc:GetLevel()/2),e,tp)
    let lv = card::get_level(d, tc) as i64 / 2;
    let cg = duel::select_matching_card(d, a.tp, Some(&|d, x| filter2(d, x, lv, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #cg==0 then return end
    if cg.is_empty() {
        return;
    }
    // local sc=cg:GetFirst()
    let sc = group::get_first(d, &cg).nil();
    // Duel.SpecialSummon(cg,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, cg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    // --destroy
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetOperation(s.des)
    effect::set_operation(d, e1, Some(operation(des)));
    // sc:RegisterEffect(e1)
    card::register_effect(d, sc, e1, false);
}
/// `s.des(e,tp,eg,ep,ev,re,r,rp)`
fn des(d: &mut Duel, a: &EffArgs) {
    // Duel.Destroy(e:GetHandler(),REASON_EFFECT)
    let h = effect::get_handler(d, a.e).nil();
    duel::destroy(d, h, REASON_EFFECT, None, None);
}
