//! Veidos the Eruption Dragon of Extinction
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 78783557;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ASHENED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card to your opponent's field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY+CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY + CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy all monsters on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_FZONE) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_FZONE);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_FZONE,LOCATION_FZONE,1,nil)
    //     and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, tp, LOCATION_FZONE, LOCATION_FZONE, 1, NO_EX)
            && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_FZONE,LOCATION_FZONE,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_FZONE, LOCATION_FZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thsetfilter(c)`
fn thsetfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ASHENED) and c:IsContinuousTrap() and (c:IsAbleToHand() or c:IsSSetable())
    card::is_set_card(d, c, &[SET_ASHENED as u16])
        && card::is_continuous_trap(d, c)
        && (card::is_able_to_hand(d, c, None) || card::is_s_setable(d, c, false, None))
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,1-tp,false,false,POS_FACEUP)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    if duel::special_summon(d, c, 0, tp, 1 - tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0
    //     and Duel.IsExistingMatchingCard(s.thsetfilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    if duel::destroy(d, tc, REASON_EFFECT, None, None) <= 0 {
        return;
    }
    if !duel::is_existing_matching_card(d, Some(&thsetfilter), tp, LOCATION_DECK, 0, 1, NO_EX) {
        return;
    }
    if !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 3));
    // local sc=Duel.SelectMatchingCard(tp,s.thsetfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&thsetfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sel) else {
        return;
    };
    // Duel.BreakEffect()
    duel::break_effect(d);
    // aux.ToHandOrElse(sc,tp,
    //     function(sc) return sc:IsSSetable() end,
    //     function(sc) Duel.SSet(tp,sc) end,
    //     aux.Stringid(id,4))
    aux::to_hand_or_else(
        d,
        sc,
        tp,
        Some(&|d, x| card::is_s_setable(d, x, false, None)),
        Some(&|d, g| duel::s_set(d, tp, g.clone(), None, true)),
        Some(aux::stringid(ID, 4)),
    );
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousControler(1-tp)
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_previous_controler(d, c, 1 - a.tp)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return g.len() > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(nil,tp,LOCATION_MZONE,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #g>0 then Duel.Destroy(g,REASON_EFFECT) end
    if g.len() > 0 {
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
