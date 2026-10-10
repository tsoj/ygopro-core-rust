//! Hop Ear Squadron
use super::prelude::*;

pub const ID: u32 = 1980574;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMING_BATTLE_START|TIMING_BATTLE_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMING_BATTLE_START | TIMING_BATTLE_END));
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.sccon)
    effect::set_condition(d, e1, Some(condition(sccon)));
    // e1:SetTarget(s.sctg)
    effect::set_target(d, e1, Some(cost_target(sctg)));
    // e1:SetOperation(s.scop)
    effect::set_operation(d, e1, Some(operation(scop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.sccon(e,tp,eg,ep,ev,re,r,rp)`
fn sccon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not e:GetHandler():IsStatus(STATUS_CHAINING) and Duel.IsTurnPlayer(1-tp) and Duel.IsMainPhase()
    let c = effect::get_handler(d, a.e).nil();
    !card::is_status(d, c, STATUS_CHAINING) && duel::is_turn_player(d, 1 - a.tp) && duel::is_main_phase(d, None)
}
/// `s.scfilter1(c,tp,mc)`
fn scfilter1(d: &mut Duel, c: CardId, tp: u8, mc: CardId) -> bool {
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(s.scfilter2,tp,LOCATION_EXTRA,0,1,nil,tp,Group.FromCards(c,mc))
    let mg = group::create_group(&[c, mc]);
    card::is_faceup(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| scfilter2(d, x, tp, mg.clone())), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.scfilter2(c,tp,mg)`
fn scfilter2(d: &mut Duel, c: CardId, tp: u8, mg: CardSet) -> bool {
    // return Duel.GetLocationCountFromEx(tp,tp,mg,c)>0 and c:IsSynchroSummonable(nil,mg)
    duel::get_location_count_from_ex(d, tp, Some(tp), (&mg).into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_synchro_summonable(d, c, None, Some(mg.clone()), 0, 0)
}
/// `s.sctg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.scfilter1(chkc,tp,c) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && scfilter1(d, chkc, tp, c);
    }
    // if chk==0 then return Duel.IsPlayerCanSpecialSummonCount(tp,2)
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingTarget(s.scfilter1,tp,LOCATION_MZONE,0,1,nil,tp,c) end
    if a.chk == Some(0) {
        return duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
            && duel::is_existing_target(d, Some(&|d, x| scfilter1(d, x, tp, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // Duel.SelectTarget(tp,s.scfilter1,tp,LOCATION_MZONE,0,1,1,nil,tp,c)
    duel::select_target(d, tp, Some(&|d, x| scfilter1(d, x, tp, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.scop(e,tp,eg,ep,ev,re,r,rp)`
fn scop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if !card::is_relate_to_effect(d, c, e) || duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // if not tc:IsRelateToEffect(e) or tc:IsFacedown() then return end
    let tc = tc.nil();
    if !card::is_relate_to_effect(d, tc, e) || card::is_facedown(d, tc) {
        return;
    }
    // local mg=Group.FromCards(c,tc)
    let mg = group::create_group(&[c, tc]);
    // local g=Duel.GetMatchingGroup(s.scfilter2,tp,LOCATION_EXTRA,0,nil,tp,mg)
    let g = duel::get_matching_group(d, Some(&|d, x| scfilter2(d, x, tp, mg.clone())), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // Duel.SynchroSummon(tp,sg:GetFirst(),c,mg)
        let sc = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, tp, sc, Some(c.into()), Some(mg), 0, 0);
    }
}
