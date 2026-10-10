//! Sky Striker Maneuver - Scissors Cross
use super::prelude::*;

pub const ID: u32 = 46271408;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SKY_STRIKER_ACE], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SPECIAL_SUMMON+CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SPECIAL_SUMMON + CATEGORY_LEAVE_GRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()<5
    card::get_sequence(d, c) < 5
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_MZONE,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
// function s.filter(c,e,tp,spchk)
// The Lua target code passes only `spchk` as the extra argument, so the Lua `e` is that boolean and `tp`/`spchk` are nil:
// the Special Summon branch is never taken there; only the to-hand check remains.
fn filter_nospchk(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SKY_STRIKER_ACE) and c:IsMonster() and c:IsLevel(4) and (c:IsAbleToHand() or (spchk and ...))
    card::is_set_card(d, c, &[SET_SKY_STRIKER_ACE as u16]) && card::is_monster(d, c) && card::is_level(d, c, &[4]) && card::is_able_to_hand(d, c, None)
}
// function s.filter(c,e,tp,spchk) (called with the real arguments by the activation)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, spchk: bool) -> bool {
    // return c:IsSetCard(SET_SKY_STRIKER_ACE) and c:IsMonster() and c:IsLevel(4) and (c:IsAbleToHand() or (spchk and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_set_card(d, c, &[SET_SKY_STRIKER_ACE as u16])
        && card::is_monster(d, c)
        && card::is_level(d, c, &[4])
        && (card::is_able_to_hand(d, c, None)
            || (spchk && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local spchk=Duel.GetMatchingGroupCount(Card.IsSpell,tp,LOCATION_GRAVE,0,nil)>=3
    let spchk = duel::get_matching_group_count(d, Some(&card::is_spell), a.tp, LOCATION_GRAVE, 0, NO_EX) >= 3;
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_GRAVE) and s.filter(c,e,tp,spchk) end
    if let Some(chkc) = a.chkc() {
        // `c` is not a parameter of this Lua function: it is the global nil, so `s.filter(c,...)` raises on `c:IsSetCard`.
        return card::is_controler(d, chkc, 1 - a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && {
            let nil_card: Option<CardId> = None;
            filter(d, nil_card.nil(), a.e, a.tp, spchk)
        };
    }
    // if chk==0 then return Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,spchk) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&filter_nospchk), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,spchk)
    let g = duel::select_target(d, a.tp, Some(&filter_nospchk), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g.clone(), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,g,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, g.clone(), 1, a.tp, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.activate(e,tp,eg,ep,ev,re,r,rp)
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // aux.ToHandOrElse(tc,tp,
            //     function(c)
            //         return Duel.GetMatchingGroupCount(Card.IsSpell,tp,LOCATION_GRAVE,0,nil)>=3
            //             and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
            //             and tc:IsCanBeSpecialSummoned(e,0,tp,false,false)
            //     end,
            //     function(c)
            //         Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
            //     end,
            // 1152)
            aux::to_hand_or_else(
                d,
                tc,
                a.tp,
                Some(&|d, _c| {
                    duel::get_matching_group_count(d, Some(&card::is_spell), a.tp, LOCATION_GRAVE, 0, NO_EX) >= 3
                        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
                        && card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None)
                }),
                Some(&|d, _g| {
                    duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
                    0
                }),
                Some(1152),
            );
        }
    }
}
