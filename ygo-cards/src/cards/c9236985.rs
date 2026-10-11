//! Gishki Photomirror
use super::prelude::*;

pub const ID: u32 = 9236985;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GISHKI], ..ScriptInfo::DEFAULT };

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
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // if not s.ritual_matching_function then
    //     s.ritual_matching_function={}
    // end
    // s.ritual_matching_function[c]=aux.FilterEqualFunction(Card.IsSetCard,SET_GISHKI)
    // (Lua: `Card.IsSetCard(target)==SET_GISHKI` compares a boolean with a number: always false)
    crate::card_var::set(d, c, crate::card_var::RITUAL_MATCHING_FUNCTION, Value::Fn(LuaFn::new(|_d, _a| Value::Bool(false))));
}
/// `s.filter(c,e,tp,lp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lp: i64) -> bool {
    // if not c:IsRitualMonster() or not c:IsSetCard(SET_GISHKI) or not c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_RITUAL,tp,true,false) then
    //     return false
    // end
    if !card::is_ritual_monster(d, c)
        || !card::is_set_card(d, c, &[SET_GISHKI as u16])
        || !card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_RITUAL, tp, true, false, None, None, None)
    {
        return false;
    }
    // return lp>c:GetLevel()*500
    lp > card::get_level(d, c) as i64 * 500
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local lp=Duel.GetLP(tp)
        let lp = duel::get_lp(d, tp) as i64;
        // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,e,tp,lp)
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, tp, lp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local lp=Duel.GetLP(tp)
    let lp = duel::get_lp(d, tp) as i64;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tg=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,e,tp,lp)
    let tg = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, a.e, tp, lp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tc=tg:GetFirst()
    let tc = group::get_first(d, &tg);
    // if tc then
    if let Some(tc) = tc {
        // mustpay=true
        set_lua_global(d, "mustpay", Value::Bool(true));
        // Duel.PayLPCost(tp,tc:GetLevel()*500)
        let cost = card::get_level(d, tc) * 500;
        duel::pay_lp_cost(d, tp, cost);
        // mustpay=false
        set_lua_global(d, "mustpay", Value::Bool(false));
        // tc:SetMaterial(nil)
        card::set_material(d, tc, CardSet::new());
        // Duel.SpecialSummon(tc,SUMMON_TYPE_RITUAL,tp,tp,true,false,POS_FACEUP)
        duel::special_summon(d, tc, SUMMON_TYPE_RITUAL, tp, tp, true, false, POS_FACEUP as u8, None);
        // tc:CompleteProcedure()
        card::complete_procedure(d, tc);
    }
}
