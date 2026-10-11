//! Crystal Boon
use super::prelude::*;

pub const ID: u32 = 85766789;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_CRYSTAL_BEAST], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 2 "Crystal Beast" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_RECOVER)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_RECOVER);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Excavate the top card of your Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_SPECIAL_SUMMON+CATEGORY_DECKDES)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_SPECIAL_SUMMON + CATEGORY_DECKDES);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_MOVE)
    effect::set_code(d, e2, EVENT_MOVE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.exccon)
    effect::set_condition(d, e2, Some(condition(exccon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.exctg)
    effect::set_target(d, e2, Some(cost_target(exctg)));
    // e2:SetOperation(s.excop)
    effect::set_operation(d, e2, Some(operation(excop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and c:IsOriginalType(TYPE_MONSTER)
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && card::is_original_type(d, c, TYPE_MONSTER)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_SZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_SZONE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_SZONE as i32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ft=math.min(2,Duel.GetLocationCount(tp,LOCATION_MZONE))
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0.min(2);
    // if ft<1 then return end
    if ft < 1 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_SZONE,0,1,ft,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_SZONE, 0, 1, ft as u16, false, NO_EX).nil();
    // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !g.is_empty() && duel::special_summon(d, &g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Duel.Recover(tp,g:GetSum(function(c) return math.max(0,c:GetTextAttack()) end,nil),REASON_EFFECT)
        let sum = group::get_sum(d, &g, &|d, c| card::get_text_attack(d, c).max(0) as i64);
        duel::recover(d, tp, sum, REASON_EFFECT, false, None);
    }
}
/// `s.excconfilter(c,tp)`
fn excconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_CRYSTAL_BEAST) and c:IsLocation(LOCATION_SZONE) and not c:IsPreviousLocation(LOCATION_SZONE)
    //     and c:IsControler(tp) and c:GetSequence()<5
    card::is_faceup(d, c)
        && card::is_set_card(d, c, &[SET_CRYSTAL_BEAST as u16])
        && card::is_location(d, c, LOCATION_SZONE)
        && !card::is_previous_location(d, c, LOCATION_SZONE)
        && card::is_controler(d, c, tp)
        && card::get_sequence(d, c) < 5
}
/// `s.exccon(e,tp,eg,ep,ev,re,r,rp)`
fn exccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg and eg:IsExists(s.excconfilter,1,nil,tp)
    let tp = a.tp;
    match a.eg.clone() {
        Some(eg) => group::is_exists(d, &eg, &|d, c| excconfilter(d, c, tp), 1, NO_EX),
        None => false,
    }
}
/// `s.exctg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn exctg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 1);
    }
    true
}
/// `s.excop(e,tp,eg,ep,ev,re,r,rp)`
fn excop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if not Duel.IsPlayerCanDiscardDeck(tp,1) then return end
    if !duel::is_player_can_discard_deck(d, tp, 1) {
        return;
    }
    // Duel.ConfirmDecktop(tp,1)
    duel::confirm_decktop(d, tp, 1);
    // local tc=Duel.GetDecktopGroup(tp,1):GetFirst()
    let g = duel::get_decktop_group(d, tp, 1);
    let tc = group::get_first(d, &g).nil();
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // if tc:IsSetCard(SET_CRYSTAL_BEAST) then
    if card::is_set_card(d, tc, &[SET_CRYSTAL_BEAST as u16]) {
        // aux.ToHandOrElse(tc,tp,
        //     function(sc)
        //         return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and sc:IsCanBeSpecialSummoned(e,0,tp,false,false)
        //     end,
        //     function(sc)
        //         return Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)
        //     end,
        //     aux.Stringid(id,2))
        aux::to_hand_or_else(
            d,
            tc,
            tp,
            Some(&|d: &mut Duel, sc: CardId| {
                duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                    && card::is_can_be_special_summoned(d, sc, e, 0, tp, false, false, None, None, None)
            }),
            Some(&|d: &mut Duel, sc: &CardSet| duel::special_summon(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None)),
            Some(aux::stringid(ID, 2)),
        );
    } else {
        // Duel.SendtoGrave(tc,REASON_EFFECT|REASON_EXCAVATE)
        duel::send_to_grave(d, tc, REASON_EFFECT | REASON_EXCAVATE, None, None);
    }
}
