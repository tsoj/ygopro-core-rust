//! Flowering Etoile the Melodious Magnificat
use super::prelude::*;

pub const ID: u32 = 83793721;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    listed_series: &[SET_MELODIOUS, SET_MELODIOUS_MAESTRA],
    material_setcode: &[SET_MELODIOUS, SET_MELODIOUS_MAESTRA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Summon Procedure
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_MELODIOUS_MAESTRA),1,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_MELODIOUS),2)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![
            (
                fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_MELODIOUS_MAESTRA as u16], Some(fc), st, Some(tp)))),
                1,
            ),
            (fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_set_card_ctx(d, x, &[SET_MELODIOUS as u16], Some(fc), st, Some(tp)))), 2),
        ],
    );
    // Banish any number of your "Melodious" monsters, then you can return up to that many opponent's cards to hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Melodious" monster from your Deck or Extra Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rmfilter(c,e,tp)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_MELODIOUS) and c:IsAbleToRemove()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MELODIOUS as u16]) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, tp, LOCATION_MZONE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, 1 - tp, LOCATION_ONFIELD as i32);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rg=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_MZONE,0,1,99,nil)
    let rg = duel::select_matching_card(d, tp, Some(&rmfilter), tp, LOCATION_MZONE, 0, 1, 99, false, NO_EX).nil();
    // if #rg==0 then return end
    if rg.is_empty() {
        return;
    }
    // Duel.HintSelection(rg,true)
    duel::hint_selection(d, rg.clone(), true);
    // local thg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsAbleToHand),tp,0,LOCATION_ONFIELD,nil)
    let thg = duel::get_matching_group(d, Some(&aux::faceup_filter(|d, c| card::is_able_to_hand(d, c, None))), tp, 0, LOCATION_ONFIELD, NO_EX);
    // if aux.RemoveUntil(rg,nil,REASON_EFFECT,PHASE_END,id,e,tp,aux.DefaultFieldReturnOp)
    //     and #thg>0 and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if aux::remove_until(d, rg, None, REASON_EFFECT, PHASE_END, ID, e, tp, Some(aux::default_field_return()), None, None, None, None, None).is_some()
        && !thg.is_empty()
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        // local sg=thg:Select(tp,1,#Duel.GetOperatedGroup(),nil)
        let sg = group::select(d, &thg, tp, 1, duel::get_operated_group(d).len() as u16, false, NO_EX).nil();
        // Duel.HintSelection(sg,true)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg, None, REASON_EFFECT, None);
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP)
    //     and c:IsPreviousControler(tp) and c:IsFusionSummoned() and rp==1-tp
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
        && card::is_previous_controler(d, c, tp)
        && card::is_fusion_summoned(d, c)
        && a.rp == 1 - tp
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if c:IsLocation(LOCATION_DECK) and Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
    if card::is_location(d, c, LOCATION_DECK) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return false;
    }
    // if c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,nil,c)<=0 then return false end
    if card::is_location(d, c, LOCATION_EXTRA)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 <= 0
    {
        return false;
    }
    // return c:IsSetCard(SET_MELODIOUS) and not c:IsCode(id) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MELODIOUS as u16])
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
