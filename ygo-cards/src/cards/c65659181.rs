//! Vylon Polytope
//! ヴァイロン・ポリトープ
use super::prelude::*;

pub const ID: u32 = 65659181;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VYLON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon any number of "Vylon" Monster Cards you control that are Equip Cards
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_VYLON) and c:IsMonsterCard() and c:IsEquipCard() and c:IsFaceup()
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_set_card(d, c, &[SET_VYLON as u16])
        && card::is_monster_card(d, c)
        && card::is_equip_card(d, c)
        && card::is_faceup(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and chkc:IsControler(tp) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && card::is_controler(d, chkc, tp) && spfilter(d, chkc, e, tp);
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then return ft>0 and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_SZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return ft > 0 && duel::is_existing_target(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_SZONE,0,1,ft,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_SZONE, 0, 1, ft as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), n, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local tg=Duel.GetTargetCards(e):Filter(Card.IsCanBeSpecialSummoned,nil,e,0,tp,false,false,POS_FACEUP_DEFENSE)
    let targets = duel::get_target_cards(d, e).nil();
    let mut tg = group::filter(
        d,
        &targets,
        &|d, x| card::is_can_be_special_summoned(d, x, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None),
        NO_EX,
    );
    // if #tg==0 or (#tg>1 and Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)) then return end
    if tg.is_empty() || (tg.len() > 1 && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()) {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local exg=nil
    let mut exg: Option<CardSet> = None;
    // if #tg>ft then
    if tg.len() as i32 > ft {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=tg:Select(tp,ft,ft,nil)
        let sg = group::select(d, &tg, tp, ft as u16, ft as u16, false, NO_EX).nil();
        // tg,exg=tg:Split(function(c) return sg:IsContains(c) end,nil)
        let (matched, rest) = group::split(d, &tg, &|_d, x| sg.iter().any(|y| y == x), NO_EX);
        tg = matched;
        exg = Some(rest);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // for tc in tg:Iter() do
    for tc in group::iter(d, &tg) {
        // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE) then
        if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) {
            // Banish it when it leaves the field
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3300)
            effect::set_description(d, e1, 3300);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
            effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
            // e1:SetValue(LOCATION_REMOVED)
            effect::set_value(d, e1, LOCATION_REMOVED as i64);
            // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc, e1, true);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // if exg then
    if let Some(exg) = exg {
        // Duel.SendtoGrave(exg,REASON_RULE,PLAYER_NONE,PLAYER_NONE)
        duel::send_to_grave(d, exg, REASON_RULE, Some(PLAYER_NONE as u8), Some(PLAYER_NONE as u8));
    }
}
