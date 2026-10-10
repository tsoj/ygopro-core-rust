//! The Fallen & The Virtuous
use super::prelude::*;

pub const ID: u32 = 30271097;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_ALBAZ],
    listed_series: &[SET_BRANDED, SET_DOGMATIKA, SET_ECCLESIA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetHintTiming(TIMING_END_PHASE,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, TIMING_END_PHASE, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// s.descostfilter(c)
fn descostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:ListsCode(CARD_ALBAZ) and c:IsAbleToGraveAsCost()
    card::lists_code(d, c, &[CARD_ALBAZ]) && card::is_able_to_grave_as_cost(d, c)
}
/// s.spconfilter(c)
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ECCLESIA) and c:IsMonster() and c:IsFaceup()
    card::is_set_card(d, c, &[SET_ECCLESIA as u16]) && card::is_monster(d, c) && card::is_faceup(d, c)
}
/// The two choices' conditions, `b1` and `b2`, shared by the cost and the target.
fn choices(d: &mut Duel, e: EffectId, tp: u8) -> (bool, bool) {
    // local b1=Duel.IsExistingMatchingCard(s.descostfilter,tp,LOCATION_EXTRA,0,1,nil)
    //     and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,e:GetHandler())
    let b1 = duel::is_existing_matching_card(d, Some(&descostfilter), tp, LOCATION_EXTRA, 0, 1, NO_EX) && {
        let h = effect::get_handler(d, e).nil();
        duel::is_existing_target(d, Some(&|d, c| card::is_faceup(d, c)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, h.into())
    };
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil)
    //     and Duel.IsExistingTarget(Card.IsCanBeSpecialSummoned,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil,e,0,tp,false,false)
    let b2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spconfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_target(
            d,
            Some(&|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)),
            tp,
            LOCATION_GRAVE,
            LOCATION_GRAVE,
            1,
            NO_EX,
        );
    (b1, b2)
}
/// s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let (b1, b2) = choices(d, e, tp);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local cd=e:GetChainData()
    // cd.choice=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let choice = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    set_choice(d, e, choice);
    // if cd.choice==1 then
    if choice == Some(1) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.descostfilter,tp,LOCATION_EXTRA,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&descostfilter), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX);
        // Duel.SendtoGrave(g,REASON_COST)
        if let Some(g) = g {
            duel::send_to_grave(d, g, REASON_COST, None, None);
        }
    }
    true
}
/// `cd.choice = v` (the chain data field the cost and the target share)
fn set_choice(d: &mut Duel, e: EffectId, choice: Option<usize>) {
    let v = match choice {
        Some(x) => Value::Int(x as i64),
        None => Value::Nil,
    };
    chain::set_effect_data(d, e, None, chain::field::CARD, v);
}
/// `cd.choice`
fn get_choice(d: &Duel, e: EffectId) -> Value {
    chain::effect_data(d, e, None, chain::field::CARD)
}
/// s.efftg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then
    if let Some(chkc) = a.chkc() {
        // local cd=e:GetChainData()
        let choice = get_choice(d, e);
        // if cd.choice==1 then
        if choice.int() == 1 && !choice.is_nil() {
            // return chkc:IsOnField() and chkc:IsFaceup() and chkc~=c
            return card::is_on_field(d, chkc) && card::is_faceup(d, chkc) && chkc != c;
        } else if choice.int() == 2 && !choice.is_nil() {
            // return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsCanBeSpecialSummoned(e,0,tp,false,false)
            return card::is_location(d, chkc, LOCATION_GRAVE)
                && card::is_can_be_special_summoned(d, chkc, e, 0, tp, false, false, None, None, None);
        }
    }
    // local b1, b2 as in the cost
    let (b1, b2) = choices(d, e, tp);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local cd=e:GetChainData()
    // cd.choice=cd.choice
    //     or Duel.SelectEffect(tp,
    //         {b1,aux.Stringid(id,1)},
    //         {b2,aux.Stringid(id,2)})
    if !get_choice(d, e).truthy() {
        let choice = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
        set_choice(d, e, choice);
    }
    let choice = get_choice(d, e);
    // if cd.choice==1 then
    if !choice.is_nil() && choice.int() == 1 {
        // e:SetCategory(CATEGORY_DESTROY)
        effect::set_category(d, e, CATEGORY_DESTROY);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local g=Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,c)
        let g = duel::select_target(d, tp, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, c.into());
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    } else if !choice.is_nil() && choice.int() == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectTarget(tp,Card.IsCanBeSpecialSummoned,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil,e,0,tp,false,false)
        let g = duel::select_target(
            d,
            tp,
            Some(&|d, x| card::is_can_be_special_summoned(d, x, e, 0, tp, false, false, None, None, None)),
            tp,
            LOCATION_GRAVE,
            LOCATION_GRAVE,
            1,
            1,
            false,
            NO_EX,
        );
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    }
    true
}
/// s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    // if not tc:IsRelateToEffect(e) then return end
    let tc = duel::first_target(d).nil();
    if !card::is_relate_to_effect(d, tc, e) {
        return;
    }
    // local cd=e:GetChainData()
    let choice = get_choice(d, e);
    // if cd.choice==1 then
    if !choice.is_nil() && choice.int() == 1 {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    } else if !choice.is_nil() && choice.int() == 2 {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
