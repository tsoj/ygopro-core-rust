//! Mimicat
use super::prelude::*;

pub const ID: u32 = 88032456;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_TOON_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.cfilter1(c)`
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCode(15259703)
    card::is_faceup(d, c) && card::is_code(d, c, &[CARD_TOON_WORLD])
}
/// `s.cfilter2(c)`
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_TOON)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_TOON)
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter1,tp,LOCATION_ONFIELD,0,1,nil) and Duel.IsExistingMatchingCard(s.cfilter2,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter1), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&cfilter2), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:IsMonster() then
    if card::is_monster(d, c) {
        // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        return card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // local ct=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ct = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:IsHasType(EFFECT_TYPE_ACTIVATE) and not e:GetHandler():IsLocation(LOCATION_SZONE) then ct=ct-1 end
    if effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        let h = effect::get_handler(d, e).nil();
        if !card::is_location(d, h, LOCATION_SZONE) {
            ct -= 1;
        }
    }
    // return c:IsSSetable(true) and (c:IsType(TYPE_FIELD) or ct>0)
    card::is_s_setable(d, c, true, None) && (card::is_type(d, c, TYPE_FIELD) || ct > 0)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(1-tp) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, 1 - tp) && filter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,0,LOCATION_GRAVE,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| filter(d, c, e, tp)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,0,LOCATION_GRAVE,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // if g:GetFirst():IsMonster() then
    let tc = group::get_first(d, &g).nil();
    if card::is_monster(d, tc) {
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), 1, 0, 0);
    }
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // if tc:IsMonster() then
    if card::is_monster(d, tc) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    } else if card::is_type(d, tc, TYPE_FIELD) || duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0 {
        // Duel.SSet(tp,tc)
        duel::s_set(d, a.tp, tc, None, true);
    }
}
