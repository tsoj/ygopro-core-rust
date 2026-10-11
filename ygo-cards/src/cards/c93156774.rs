//! Vanquish Soul Hollie Sue
//! ＶＳ ホーリー・スー
use super::prelude::*;

pub const ID: u32 = 93156774;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VANQUISH_SOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
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
    // e1:SetCost(Cost.AND(Cost.HardOncePerChain(id),Cost.Reveal(function(c) return c:IsSetCard(SET_VANQUISH_SOUL) and c:IsMonster() end,true)))
    effect::set_cost(
        d,
        e1,
        Some(cost::and(vec![
            cost::hard_once_per_chain(ID, None),
            cost::reveal(
                Some(std::sync::Arc::new(|d: &mut Duel, c: CardId, _e: EffectId, _tp: u8| {
                    card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16]) && card::is_monster(d, c)
                })),
                true,
                None,
                None,
                None,
                None,
            ),
        ])),
    );
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Activate 1 of these effects by revealing monster(s) in your hand with the listed Attribute(s)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.HardOncePerChain(id))
    effect::set_cost(d, e2, Some(cost::hard_once_per_chain(ID, None)));
    // e2:SetTarget(s.vstg)
    effect::set_target(d, e2, Some(cost_target(vstg)));
    // e2:SetOperation(s.vsop)
    effect::set_operation(d, e2, Some(operation(vsop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.vscostfilter(c,att) return c:IsAttribute(att) and not c:IsPublic() end
fn vscostfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    card::is_attribute(d, c, att) && !card::is_public(d, c)
}
// function s.vsrescon(sg) return sg:GetBinClassCount(Card.GetAttribute)==2 end
fn vsrescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let r = group::get_bin_class_count(d, sg, &|d, c| card::get_attribute(d, c) as i64) == 2;
    (r, false)
}
// function s.deckspfilter(c,e,tp)
//     return c:IsSetCard(SET_VANQUISH_SOUL) and not c:IsRace(RACE_PSYCHIC) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
// end
fn deckspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_VANQUISH_SOUL as u16])
        && !card::is_race(d, c, RACE_PSYCHIC)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.vstg(e,tp,eg,ep,ev,re,r,rp,chk)
fn vstg(d: &mut Duel, a: &EffArgs) -> bool {
    let rs: aux::Rescon = &vsrescon;
    // local cg1=Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_EARTH|ATTRIBUTE_DARK)
    let cg1 = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_EARTH | ATTRIBUTE_DARK)), a.tp, LOCATION_HAND, 0, NO_EX);
    // local b1=aux.SelectUnselectGroup(cg1,e,tp,2,2,s.vsrescon,0)
    //     and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,1,nil)
    let b1 = matches!(
        aux::select_unselect_group(d, &cg1, Some(a.e), a.tp, Some(2), Some(2), Some(rs), 0, a.tp, None, None, None, false),
        Err(true)
    ) && duel::is_existing_matching_card(
        d,
        Some(&aux::faceup_filter(|d, c| card::is_controler_can_be_changed(d, c, false, None))),
        a.tp,
        0,
        LOCATION_MZONE,
        1,
        NO_EX,
    );
    // local cg2=Duel.GetMatchingGroup(s.vscostfilter,tp,LOCATION_HAND,0,nil,ATTRIBUTE_FIRE|ATTRIBUTE_DARK)
    let cg2 = duel::get_matching_group(d, Some(&|d, c| vscostfilter(d, c, ATTRIBUTE_FIRE | ATTRIBUTE_DARK)), a.tp, LOCATION_HAND, 0, NO_EX);
    // local b2=aux.SelectUnselectGroup(cg2,e,tp,2,2,s.vsrescon,0)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.deckspfilter,tp,LOCATION_DECK,0,1,nil,e,tp)
    let b2 = matches!(
        aux::select_unselect_group(d, &cg2, Some(a.e), a.tp, Some(2), Some(2), Some(rs), 0, a.tp, None, None, None, false),
        Err(true)
    ) && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| deckspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
    let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, a.e, &[op]);
    if op == 1 {
        // EARTH & DARK
        // local g=aux.SelectUnselectGroup(cg1,e,tp,2,2,s.vsrescon,1,tp,HINTMSG_CONFIRM)
        let g = aux::select_unselect_group(d, &cg1, Some(a.e), a.tp, Some(2), Some(2), Some(rs), 1, a.tp, Some(HINTMSG_CONFIRM as u64), None, None, false)
            .unwrap_or_default();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // e:SetCategory(CATEGORY_CONTROL)
        effect::set_category(d, a.e, CATEGORY_CONTROL);
        // Duel.SetOperationInfo(0,CATEGORY_CONTROL,nil,1,1-tp,LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_CONTROL, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    } else if op == 2 {
        // FIRE & DARK
        // local g=aux.SelectUnselectGroup(cg2,e,tp,2,2,s.vsrescon,1,tp,HINTMSG_CONFIRM)
        let g = aux::select_unselect_group(d, &cg2, Some(a.e), a.tp, Some(2), Some(2), Some(rs), 1, a.tp, Some(HINTMSG_CONFIRM as u64), None, None, false)
            .unwrap_or_default();
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
        // Duel.ShuffleHand(tp)
        duel::shuffle_hand(d, a.tp);
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_DECK as i32);
    }
    true
}
// function s.vsop(e,tp,eg,ep,ev,re,r,rp)
fn vsop(d: &mut Duel, a: &EffArgs) {
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    if op == 1 {
        // Take control of 1 monster your opponent controls with the lowest ATK (your choice, if tied) until the End Phase
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsControlerCanBeChanged),tp,0,LOCATION_MZONE,nil)
        let g = duel::get_matching_group(
            d,
            Some(&aux::faceup_filter(|d, c| card::is_controler_can_be_changed(d, c, false, None))),
            a.tp,
            0,
            LOCATION_MZONE,
            NO_EX,
        );
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // local ming=g:GetMinGroup(Card.GetAttack)
        let ming = group::get_min_group(d, &g, &|d, c| card::get_attack(d, c) as i64)
            .map(|x| x.0)
            .unwrap_or_default();
        // local sc=ming:GetFirst()
        let mut sc: Option<CardSet> = group::get_first(d, &ming).map(CardSet::from_card);
        // if #ming>1 then
        if ming.len() > 1 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONTROL)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONTROL as u64);
            // sc=ming:Select(tp,1,1,nil)
            sc = Some(group::select(d, &ming, a.tp, 1, 1, false, NO_EX).nil());
        }
        // if sc then
        if let Some(sc) = sc {
            // Duel.HintSelection(sc)
            duel::hint_selection(d, sc.clone(), true);
            // Duel.GetControl(sc,tp,PHASE_END,1)
            duel::get_control(d, sc, a.tp, PHASE_END as u16, 1, None, None);
        }
    } else if op == 2 {
        // Special Summon 1 non-Psychic "Vanquish Soul" monster from your Deck
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.deckspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, c| deckspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
