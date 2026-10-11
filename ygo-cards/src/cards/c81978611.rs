//! Xyz Entrust
//! エクシーズ・エントラスト
use super::prelude::*;

pub const ID: u32 = 81978611;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ARMORED_XYZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Add 1 "Armored Xyz" card from the Deck/GY to the hand and change the levels of up to 2 monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH+CATEGORY_LVCHANGE)
    effect::set_category(d, e1, CATEGORY_TOHAND + CATEGORY_SEARCH + CATEGORY_LVCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 Xyz Monster treated as an Equip Card from your S/T Zone
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(aux.exccon)
    effect::set_condition(d, e2, Some(aux::exccon()));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_ARMORED_XYZ) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_ARMORED_XYZ as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_LVCHANGE,nil,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_LVCHANGE, None, 1, tp, 0);
    true
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet) -> bool {
    // return not (sg:IsExists(Card.IsLevel,1,nil,3) and sg:IsExists(Card.IsLevel,1,nil,5))
    let has3 = group::is_exists(d, sg, &|d, c| card::is_level(d, c, &[3]), 1, NO_EX);
    let has5 = group::is_exists(d, sg, &|d, c| card::is_level(d, c, &[5]), 1, NO_EX);
    !(has3 && has5)
}
/// `s.rescon` as the rescon of `aux.SelectUnselectGroup`: `(res, false)`.
fn rescon_rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (rescon(d, sg), false)
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local thg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.thfilter),tp,LOCATION_DECK|LOCATION_GRAVE,0,1,1,nil)
    let thg = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| thfilter(d, c))), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // if #thg>0 and Duel.SendtoHand(thg,nil,REASON_EFFECT)>0 then
    let Some(thg) = thg.filter(|g| !g.is_empty()) else {
        return;
    };
    if duel::send_to_hand(d, thg.clone(), None, REASON_EFFECT, None) > 0 {
        // Duel.ConfirmCards(1-tp,thg)
        duel::confirm_cards(d, 1 - tp, thg);
        // local g=Duel.GetMatchingGroup(aux.FaceupFilter(Card.HasLevel),tp,LOCATION_MZONE,0,nil)
        let g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c) && card::has_level(d, c)), tp, LOCATION_MZONE, 0, NO_EX);
        // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
        if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 2)) {
            return;
        }
        // local sg=aux.SelectUnselectGroup(g,e,tp,1,2,s.rescon,1,tp,HINTMSG_FACEUP)
        let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(2), Some(&rescon_rescon), 1, tp, Some(HINTMSG_FACEUP as u64), None, None, false).unwrap_or_default();
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // Duel.HintSelection(sg,true)
        duel::hint_selection(d, sg.clone(), true);
        // local lv=(sg:IsExists(Card.IsLevel,1,nil,3) and 5)
        //     or (sg:IsExists(Card.IsLevel,1,nil,5) and 3)
        //     or Duel.AnnounceLevel(tp,3,5,4)
        let has3 = group::is_exists(d, &sg, &|d, c| card::is_level(d, c, &[3]), 1, NO_EX);
        let has5 = group::is_exists(d, &sg, &|d, c| card::is_level(d, c, &[5]), 1, NO_EX);
        let lv: i64 = if has3 {
            5
        } else if has5 {
            3
        } else {
            duel::announce_number_range(d, tp, Some(3), Some(5), &[4]).nil().0 as i64
        };
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // Duel.BreakEffect()
        duel::break_effect(d);
        // for tc in sg:Iter() do
        for tc in group::iter(d, &sg) {
            // Its Level becomes 3 or 5
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_LEVEL_FINAL)
            effect::set_code(d, e1, EFFECT_CHANGE_LEVEL_FINAL);
            // e1:SetValue(lv)
            effect::set_value(d, e1, lv);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_EQUIP) and c:GetOriginalType()&TYPE_XYZ>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_faceup(d, c)
        && card::is_type(d, c, TYPE_EQUIP)
        && card::get_original_type(d, c) & TYPE_XYZ > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chkc then return chkc:IsLocation(LOCATION_STZONE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_STZONE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_STZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_STZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_STZONE,0,1,1,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_STZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, e) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
