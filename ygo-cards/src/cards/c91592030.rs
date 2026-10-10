//! Intimidating Ore - Summonite
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 91592030;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon monster(s)
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
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.sptgfilter(c,e)`
fn sptgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsMonster() and c:IsCanBeEffectTarget(e)
    card::is_monster(d, c) && card::is_can_be_effect_target(d, c, Some(e))
}
/// `s.sprescon(g,e,tp)`
fn sprescon(d: &mut Duel, g: &CardSet, e: EffectId, tp: u8) -> bool {
    // return g:IsExists(Card.IsCanBeSpecialSummoned,1,nil,e,0,tp,false,false)
    group::is_exists(d, g, &|d, c| card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None), 1, NO_EX)
}
/// `s.sprescon` as the `rescon` of `aux.SelectUnselectGroup`: `(res, not stop)`
fn sprescon_fn(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    (sprescon(d, sg, e.nil(), tp), false)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g=Duel.GetMatchingGroup(s.sptgfilter,tp,LOCATION_GRAVE,0,nil,e)
    let g = duel::get_matching_group(d, Some(&|d, c| sptgfilter(d, c, a.e)), a.tp, LOCATION_GRAVE, 0, NO_EX);
    let dn = &sprescon_fn;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and aux.SelectUnselectGroup(g,e,tp,3,3,s.sprescon,0) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && matches!(aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&dn), 0, a.tp, None, None, None, false), Err(true));
    }
    // local g=aux.SelectUnselectGroup(g,e,tp,3,3,s.sprescon,1,tp,HINTMSG_SPSUMMON)
    let g = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(3), Some(3), Some(&dn), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft==0 then return end
    if ft == 0 {
        return;
    }
    // local g=Duel.GetTargetCards(e)
    let mut g = duel::get_target_cards(d, a.e).nil();
    // if #g==0 or not s.sprescon(g,e,tp) then return end
    if g.is_empty() || !sprescon(d, &g, a.e, a.tp) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SELECT as u64);
    // local tc=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    // if not tc then return end
    let Some(tc) = group::get_first(d, &sel) else {
        return;
    };
    // Duel.HintSelection(tc,true)
    duel::hint_selection(d, tc, true);
    // g:RemoveCard(tc)
    group::remove_card(&mut g, &CardSet::from_card(tc));
    // local b1=tc:IsCanBeSpecialSummoned(e,0,tp,false,false)
    let b1 = card::is_can_be_special_summoned(d, tc, a.e, 0, a.tp, false, false, None, None, None);
    // local b2=#g:Match(Card.IsCanBeSpecialSummoned,nil,e,0,tp,false,false)>0
    // (Group.Match filters `g` in place)
    group::match_(d, &mut g, &|d, c| card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None), NO_EX);
    let b2 = !g.is_empty();
    // local op=Duel.SelectEffect(1-tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, 1 - a.tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    // if op==1 then
    if op == Some(1) {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    // elseif op==2 then
    } else if op == Some(2) {
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // if ft<#g then
        if ft < g.len() as i32 {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
            // g=g:Select(tp,ft,ft,nil)
            g = group::select(d, &g, a.tp, ft as u16, ft as u16, false, NO_EX).nil();
        }
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
