//! Terrors of the Overroot
use super::prelude::*;

pub const ID: u32 = 63086455;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 card to the GY and Set 1 card from the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_LEAVE_GRAVE+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_LEAVE_GRAVE + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E|TIMING_MAIN_END)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E | TIMING_MAIN_END));
    // e1:SetTarget(s.tgtg)
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    // e1:SetOperation(s.tgop)
    effect::set_operation(d, e1, Some(operation(tgop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// s.tgfilter(c,e)
fn tgfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsAbleToGrave() and c:IsCanBeEffectTarget(e)
    card::is_able_to_grave(d, c, None) && card::is_can_be_effect_target(d, c, Some(e))
}
// s.setfilter(c,e,tp)
fn setfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeEffectTarget(e) and ((c:IsSpellTrap() and c:IsSSetable(true,1-tp))
    //     or c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE,1-tp))
    card::is_can_be_effect_target(d, c, Some(e))
        && ((card::is_spell_trap(d, c) && card::is_s_setable(d, c, true, Some(1 - tp)))
            || card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), Some(1 - tp), None))
}
// s.rescon(sg,e,tp)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local g1,g2=sg:Split(Card.IsOnField,nil)
    let (g1, g2) = group::split(d, sg, &|d, c| card::is_on_field(d, c), NO_EX);
    // if #g1~=1 or #g2~=1 then return end
    if g1.len() != 1 || g2.len() != 1 {
        return (false, false);
    }
    // local c1,c2=g1:GetFirst(),g2:GetFirst()
    let c1 = group::get_first(d, &g1).nil();
    let c2 = group::get_first(d, &g2).nil();
    // if c2:IsMonster() then return Duel.GetMZoneCount(1-tp,c1)>0 end
    if card::is_monster(d, c2) {
        return (duel::get_m_zone_count(d, 1 - tp, c1.into(), None, None, None).0 > 0, false);
    }
    // return (c2:IsSpell() and c2:IsType(TYPE_FIELD))
    //     or Duel.GetLocationCount(1-tp,LOCATION_SZONE)>0
    //     or (c1:IsLocation(LOCATION_SZONE) and c1:GetSequence()<5 --[[and check if exc's zone is not disabled]])
    let res = (card::is_spell(d, c2) && card::is_type(d, c2, TYPE_FIELD))
        || duel::get_location_count(d, 1 - tp, LOCATION_SZONE as u8, None, None, None).0 > 0
        || (card::is_location(d, c1, LOCATION_SZONE) && card::get_sequence(d, c1) < 5);
    (res, false)
}
// s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local g1=Duel.GetMatchingGroup(s.tgfilter,tp,0,LOCATION_ONFIELD,nil,e)
    let mut g1 = duel::get_matching_group(d, Some(&|d, c| tgfilter(d, c, a.e)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.setfilter,tp,0,LOCATION_GRAVE,nil,e,tp)
    let g2 = duel::get_matching_group(d, Some(&|d, c| setfilter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_GRAVE, NO_EX);
    // if chk==0 then return #g1>0 and #g2>0 and aux.SelectUnselectGroup(g1+g2,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        let mut gg = g1.clone();
        group::add_card(&mut gg, &g2);
        return !g1.is_empty()
            && !g2.is_empty()
            && matches!(aux::select_unselect_group(d, &gg, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false), Err(true));
    }
    // local tg=aux.SelectUnselectGroup(g1+g2,e,tp,2,2,s.rescon,1,tp,HINTMSG_TARGET)
    let mut gg = g1.clone();
    group::add_card(&mut gg, &g2);
    let tg = aux::select_unselect_group(d, &gg, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_TARGET as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, &tg);
    // g1,g2=tg:Split(Card.IsOnField,nil)
    let (ng1, ng2) = group::split(d, &tg, &|d, c| card::is_on_field(d, c), NO_EX);
    g1 = ng1;
    let g2 = ng2;
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g1,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g1.clone()), 1, 0, 0);
    // local cat=e:GetCategory()
    let cat = effect::get_category(d, a.e);
    // if g2:GetFirst():IsMonster() then
    if card::is_monster(d, group::get_first(d, &g2).nil()) {
        // e:SetCategory(cat|CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, cat | CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g2,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(g2.clone()), 1, 0, 0);
    } else {
        // e:SetCategory(cat&~CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, cat & !CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g2,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g2.clone()), 1, 0, 0);
    }
    true
}
// s.tgop(e,tp,eg,ep,ev,re,r,rp)
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // local g1,g2=tg:Split(Card.IsOnField,nil)
    let (g1, g2) = group::split(d, &tg, &|d, c| card::is_on_field(d, c), NO_EX);
    // if #g1>0 and Duel.SendtoGrave(g1,REASON_EFFECT)>0 and g1:GetFirst():IsLocation(LOCATION_GRAVE) then
    if !g1.is_empty() && duel::send_to_grave(d, g1.clone(), REASON_EFFECT, None, None) > 0 && card::is_location(d, group::get_first(d, &g1).nil(), LOCATION_GRAVE) {
        // local sc=g2:GetFirst()
        // if not sc then return end
        let Some(sc) = group::get_first(d, &g2) else {
            return;
        };
        // if sc:IsMonster() and Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0
        //     and Duel.SpecialSummon(sc,0,tp,1-tp,false,false,POS_FACEDOWN_DEFENSE)>0 then
        if card::is_monster(d, sc)
            && duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::special_summon(d, sc, 0, a.tp, 1 - a.tp, false, false, POS_FACEDOWN_DEFENSE as u8, None) > 0
        {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - a.tp, sc);
        // elseif sc:IsSpellTrap() and sc:IsSSetable(false,1-tp) then
        } else if card::is_spell_trap(d, sc) && card::is_s_setable(d, sc, false, Some(1 - a.tp)) {
            // Duel.SSet(tp,sc,1-tp)
            duel::s_set(d, a.tp, sc, Some(1 - a.tp), true);
        }
    }
}
