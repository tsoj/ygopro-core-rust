//! GMX - VELOX
use super::prelude::*;

pub const ID: u32 = 82706696;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GMX],
    material_setcode: &[SET_GMX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 1 "GMX" monster + 1 Dinosaur monster
    // Fusion.AddProcMix(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GMX),aux.FilterBoolFunctionEx(Card.IsRace,RACE_DINOSAUR))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| {
                card::is_set_card_ctx(d, x, &[SET_GMX as u16], Some(fc), st, Some(tp))
            })),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| {
                card::is_race_ctx(d, x, RACE_DINOSAUR, Some(fc), st, Some(tp))
            })),
        ],
    );
    // Each time your opponent Normal or Special Summons a monster(s), gain 200 LP
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1a, LOCATION_MZONE);
    // e1a:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end)
    effect::set_condition(d, e1a, Some(condition(summon_cond)));
    // e1a:SetOperation(s.lpop)
    effect::set_operation(d, e1a, Some(operation(lpop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // During your opponent's turn (Quick Effect): You can target 1 card your opponent controls; excavate the top cards
    // of your Deck until you excavate a "GMX" monster or a Dinosaur monster, lose 400 LP for each excavated card, add
    // that "GMX" monster or Dinosaur monster to your hand or Special Summon it, and shuffle the rest into the Deck.
    // Also, after that, destroy the targeted card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.excavtg)
    effect::set_target(d, e2, Some(cost_target(excavtg)));
    // e2:SetOperation(s.excavop)
    effect::set_operation(d, e2, Some(operation(excavop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,tp,eg,ep,ev,re,r,rp) return eg:IsExists(Card.IsSummonPlayer,1,nil,1-tp) end`
fn summon_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, x| card::is_summon_player(d, x, 1 - a.tp), 1, NO_EX)
}
/// `function(e,tp) return Duel.IsTurnPlayer(1-tp) end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
/// `s.lpop(e,tp,eg,ep,ev,re,r,rp)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // Duel.Recover(tp,200,REASON_EFFECT)
        duel::recover(d, a.tp, 200, REASON_EFFECT, false, None);
    } else {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // Gain 200 LP at the end of the Chain Link
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e1, EVENT_CHAIN_SOLVED);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetOperation(function(e,tp,eg,ep,ev,re,r,rp) return Duel.Recover(tp,200,REASON_EFFECT) end)
        effect::set_operation(
            d,
            e1,
            Some(operation(|d, a| {
                duel::recover(d, a.tp, 200, REASON_EFFECT, false, None);
            })),
        );
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_CHAIN)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
        // Reset "e1" at the end of the Chain Link
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e2:SetCode(EVENT_CHAIN_SOLVED)
        effect::set_code(d, e2, EVENT_CHAIN_SOLVED);
        // e2:SetOperation(function() e1:Reset() end)
        effect::set_operation(
            d,
            e2,
            Some(operation(move |d, _a| {
                effect::reset(d, e1);
            })),
        );
        // e2:SetReset(RESET_CHAIN)
        effect::set_reset(d, e2, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
}
/// `s.thspfilter(c,e,tp,mmz_chk)`
fn thspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // return (c:IsSetCard(SET_GMX) or c:IsRace(RACE_DINOSAUR)) and c:IsMonster() and (c:IsAbleToHand()
    //     or (mmz_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    (card::is_set_card(d, c, &[SET_GMX as u16]) || card::is_race(d, c, RACE_DINOSAUR as u64))
        && card::is_monster(d, c)
        && (card::is_able_to_hand(d, c, None)
            || (mmz_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.excavtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn excavtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_on_field(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,0,LOCATION_ONFIELD,1,nil)
    //     and Duel.IsExistingMatchingCard(s.thspfilter,tp,LOCATION_DECK,0,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0) end
    if a.chk == Some(0) {
        let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        return duel::is_existing_target(d, None, tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| thspfilter(d, c, a.e, tp, mmz_chk)),
                tp,
                LOCATION_DECK,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,0,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, 1, tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.excavop(e,tp,eg,ep,ev,re,r,rp)`
fn excavop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local deck_count=Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)
    let deck_count = duel::get_field_group_count(d, tp, LOCATION_DECK, 0);
    // if deck_count>0 then
    if deck_count > 0 {
        // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        let mmz_chk = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        // local g=Duel.GetMatchingGroup(s.thspfilter,tp,LOCATION_DECK,0,nil,e,tp,mmz_chk)
        let g = duel::get_matching_group(d, Some(&|d, c| thspfilter(d, c, a.e, tp, mmz_chk)), tp, LOCATION_DECK, 0, NO_EX);
        // if #g==0 then
        if g.is_empty() {
            // Duel.ConfirmDecktop(tp,deck_count)
            duel::confirm_decktop(d, tp, deck_count as u32);
            // local excav_g=Duel.GetDecktopGroup(tp,deck_count)
            let excav_g = duel::get_decktop_group(d, tp, deck_count as u32);
            // Duel.RaiseEvent(excav_g,EVENT_CUSTOM+1595137,e,REASON_EFFECT,tp,tp,deck_count)
            duel::raise_event(d, excav_g, EVENT_CUSTOM + 1595137, Some(a.e), REASON_EFFECT, tp, tp, deck_count as u32);
            // Duel.SetLP(tp,Duel.GetLP(tp)-deck_count*400)
            let lp = duel::get_lp(d, tp) - deck_count * 400;
            duel::set_lp(d, tp, lp);
        } else {
            // local sc=g:GetMaxGroup(Card.GetSequence):GetFirst()
            let max = group::get_max_group(d, &g, &|d, c| card::get_sequence(d, c) as i64).nil().0;
            let sc = group::get_first(d, &max).nil();
            // local sc_seq=sc:GetSequence()
            let sc_seq = card::get_sequence(d, sc) as i32;
            // local excav_count=deck_count-sc_seq
            let excav_count = deck_count - sc_seq;
            // Duel.ConfirmDecktop(tp,excav_count)
            duel::confirm_decktop(d, tp, excav_count as u32);
            // local excav_g=Duel.GetDecktopGroup(tp,excav_count)
            let excav_g = duel::get_decktop_group(d, tp, excav_count as u32);
            // Duel.RaiseEvent(excav_g,EVENT_CUSTOM+1595137,e,REASON_EFFECT,tp,tp,excav_count)
            duel::raise_event(d, excav_g, EVENT_CUSTOM + 1595137, Some(a.e), REASON_EFFECT, tp, tp, excav_count as u32);
            // Duel.SetLP(tp,Duel.GetLP(tp)-excav_count*400)
            let lp = duel::get_lp(d, tp) - excav_count * 400;
            duel::set_lp(d, tp, lp);
            // aux.ToHandOrElse(sc,tp,
            //     function() return mmz_chk and sc:IsCanBeSpecialSummoned(e,0,tp,false,false) end,
            //     function() Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP) end,
            //     aux.Stringid(id,1))
            aux::to_hand_or_else(
                d,
                sc,
                tp,
                Some(&|d: &mut Duel, _c: CardId| {
                    mmz_chk && card::is_can_be_special_summoned(d, sc, a.e, 0, tp, false, false, None, None, None)
                }),
                Some(&|d: &mut Duel, _g: &CardSet| -> i32 {
                    duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
                    0
                }),
                Some(aux::stringid(ID, 1)),
            );
        }
    }
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None);
    }
}
