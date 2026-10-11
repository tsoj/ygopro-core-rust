//! Sinful Spoils Subdual
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 26334139;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add to the hand, or Special Summon, 1 Level 5 or higher Illusion monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.thsptg)
    effect::set_target(d, e1, Some(cost_target(thsptg)));
    // e1:SetOperation(s.thspop)
    effect::set_operation(d, e1, Some(operation(thspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Return 1 face-down card to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.thspfilter(c,e,tp,ft)
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, ft: i32) -> bool {
    card::is_race(d, c, RACE_ILLUSION)
        && card::is_level_above(d, c, 5)
        && (card::is_able_to_hand(d, c, None)
            || (ft > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
// function s.thsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thsptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
        let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
        // return Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,ft)
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| thspfilter(d, c, a.e, a.tp, ft)),
            a.tp,
            LOCATION_DECK,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thspop(e,tp,eg,ep,ev,re,r,rp)
fn thspop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,2))
    duel::hint(d, HINT_SELECTMSG as u8, tp, aux::stringid(ID, 2));
    // local sc=Duel.SelectMatchingCard(tp,s.thspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp,ft):GetFirst()
    let sel = duel::select_matching_card(
        d,
        tp,
        Some(&|d, c| thspfilter(d, c, e, tp, ft)),
        tp,
        LOCATION_DECK,
        0,
        1,
        1,
        false,
        NO_EX,
    )
    .nil();
    let Some(sc) = group::get_first(d, &sel) else { return };
    // aux.ToHandOrElse(sc,tp,
    //   function() return ft>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
    //   function() ... end,
    //   aux.Stringid(id,4))
    aux::to_hand_or_else(
        d,
        sc,
        tp,
        Some(&|d, _c| ft > 0 && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None)),
        Some(&|d, _g| {
            // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
            if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
                // --Cannot activate its effects
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetDescription(aux.Stringid(id,3))
                effect::set_description(d, e1, aux::stringid(ID, 3));
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
                effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
                // e1:SetCode(EFFECT_CANNOT_TRIGGER)
                effect::set_code(d, e1, EFFECT_CANNOT_TRIGGER);
                // e1:SetCondition(function(e) return Duel.IsMainPhase() and e:GetHandler():IsControler(tp) end)
                effect::set_condition(
                    d,
                    e1,
                    Some(lua_fn(move |d, a| {
                        let h = effect::get_handler(d, a.e(0)).nil();
                        duel::is_main_phase(d, None) && card::is_controler(d, h, tp)
                    })),
                );
                // e1:SetReset(RESETS_STANDARD_PHASE_END)
                effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
                // sc:RegisterEffect(e1)
                card::register_effect(d, sc, e1, false);
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
            0
        }),
        Some(aux::stringid(ID, 4)),
    );
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsOnField() and chkc:IsControler(tp) and chkc:IsAbleToHand() and chkc:IsFacedown() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc)
            && card::is_controler(d, chkc, a.tp)
            && card::is_able_to_hand(d, chkc, None)
            && card::is_facedown(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.AND(Card.IsAbleToHand,Card.IsFacedown),tp,LOCATION_ONFIELD,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| thtg_filter(d, c)), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
    // local g=Duel.SelectTarget(tp,aux.AND(Card.IsAbleToHand,Card.IsFacedown),tp,LOCATION_ONFIELD,0,1,1,nil)
    let g = duel::select_target(
        d,
        a.tp,
        Some(&|d, c| thtg_filter(d, c)),
        a.tp,
        LOCATION_ONFIELD,
        0,
        1,
        1,
        false,
        NO_EX,
    );
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, g.map(|g| g.clone()), 1, a.tp, 0);
    true
}
// aux.AND(Card.IsAbleToHand,Card.IsFacedown)
fn thtg_filter(d: &mut Duel, c: CardId) -> bool {
    card::is_able_to_hand(d, c, None) && card::is_facedown(d, c)
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or Duel.SendtoHand(tc,nil,REASON_EFFECT)==0 then return end
    if !card::is_relate_to_effect(d, tc, a.e) || duel::send_to_hand(d, tc, None, REASON_EFFECT, None) == 0 {
        return;
    }
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    // local g=Duel.GetMatchingGroup(Card.IsSSetable,tp,LOCATION_HAND,0,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_s_setable(d, c, false, None)), tp, LOCATION_HAND, 0, NO_EX);
    // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,5)) then return end
    if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 5)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local sc=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    // if sc then
    if let Some(sc) = group::get_first(d, &sel) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SSet(tp,sc,tp,false)
        duel::s_set(d, tp, sc, Some(tp), false);
    }
}
