//! Rindbrumm the Striking Dragon
use super::prelude::*;

pub const ID: u32 = 51409648;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ALBAZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials
    // Fusion.AddProcMix(c,true,true,CARD_ALBAZ,aux.FilterBoolFunctionEx(Card.IsRace,RACES_BEAST_BWARRIOR_WINGB))
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::Code(CARD_ALBAZ),
            fusion::Mat::F(fusion::fus_mat(|d, x, fc, st, tp| card::is_race_ctx(d, x, RACES_BEAST_BWARRIOR_WINGB as u64, Some(fc), st, Some(tp)))),
        ],
    );
    // Negate the effect of a Fusion/Synchro/Xyz/Link monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DISABLE+CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_DISABLE + CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.negcon)
    effect::set_condition(d, e1, Some(condition(negcon)));
    // e1:SetTarget(s.negtg)
    effect::set_target(d, e1, Some(cost_target(negtg)));
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon/banish itself and/or "Fallen of Albaz"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_REMOVE)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_REMOVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some((TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END) as u32));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e2:SetTarget(s.sprmtg)
    effect::set_target(d, e2, Some(cost_target(sprmtg)));
    // e2:SetOperation(s.sprmop)
    effect::set_operation(d, e2, Some(operation(sprmop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsMonsterEffect() and re:IsActiveType(TYPE_EXTRA)
    let re = a.re.nil();
    effect::is_monster_effect(d, re) && effect::is_active_type(d, re, TYPE_EXTRA)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,PLAYER_EITHER,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.NegateEffect(ev) and Duel.IsExistingMatchingCard(Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::negate_effect(d, a.ev as u8)
        && duel::is_existing_matching_card(d, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RTOHAND as u64);
        // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToHand,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.HintSelection(g,true)
            duel::hint_selection(d, g.clone(), true);
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SendtoHand(g,nil,REASON_EFFECT)
            duel::send_to_hand(d, g, None, REASON_EFFECT, None);
        }
    }
}
/// `s.tgfilter(c,e,tp,rmv_chk,sp_chk)`
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rmv_chk: bool, sp_chk: bool) -> bool {
    // return c:IsCode(CARD_ALBAZ) and ((sp_chk and c:IsAbleToRemove()) or (rmv_chk and c:IsCanBeSpecialSummoned(e,0,tp,false,false)))
    card::is_code(d, c, &[CARD_ALBAZ])
        && ((sp_chk && card::is_able_to_remove(d, c, None, None, None))
            || (rmv_chk && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)))
}
/// `s.sprmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sprmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local rmv_chk=c:IsAbleToRemove()
    let rmv_chk = card::is_able_to_remove(d, c, None, None, None);
    // local sp_chk=c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let sp_chk = card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.tgfilter(chkc,e,tp,rmv_chk,sp_chk) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc, e, tp, rmv_chk, sp_chk);
    }
    // if chk==0 then return (rmv_chk or sp_chk)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,rmv_chk,sp_chk) end
    if a.chk == Some(0) {
        return (rmv_chk || sp_chk)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, cc| tgfilter(d, cc, e, tp, rmv_chk, sp_chk)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,rmv_chk,sp_chk)
    let mut g = duel::select_target(d, tp, Some(&|d, cc| tgfilter(d, cc, e, tp, rmv_chk, sp_chk)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // g:AddCard(c)
    group::add_card(&mut g, &CardSet::from_card(c));
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g.clone()), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, 0, 0);
    true
}
/// `s.spfilter(c,e,tp,g)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, g: &CardSet) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false) and g:IsExists(Card.IsAbleToRemove,1,c)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && group::is_exists(d, g, &|d, x| card::is_able_to_remove(d, x, None, None, None), 1, c.into())
}
/// `s.sprmop(e,tp,eg,ep,ev,re,r,rp)`
fn sprmop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not (c:IsRelateToEffect(e) and tc:IsRelateToEffect(e)) then return end
    if !(card::is_relate_to_effect(d, c, e) && card::is_relate_to_effect(d, tc.nil(), e)) {
        return;
    }
    // local g=Group.FromCards(c,tc)
    let tc = tc.nil();
    let mut g = CardSet::from_card(c);
    group::add_card(&mut g, &CardSet::from_card(tc));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sg=g:FilterSelect(tp,s.spfilter,1,1,nil,e,tp,g)
    let sg = group::filter_select(d, &g, tp, &|d, x| spfilter(d, x, e, tp, &g), 1, 1, false, NO_EX).nil();
    // if #sg>0 and Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)>0 then
    if !sg.is_empty() && duel::special_summon(d, sg.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Duel.Remove(g:Sub(sg),POS_FACEUP,REASON_EFFECT)
        let mut rest = g.clone();
        group::remove_card(&mut rest, &sg);
        duel::remove(d, rest, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
