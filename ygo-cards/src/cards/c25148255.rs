//! Junk Anchor
use super::prelude::*;

pub const ID: u32 = 25148255;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_JUNK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(s.sccost)
    effect::set_cost(d, e1, Some(cost_target(sccost)));
    // e1:SetTarget(s.sctg)
    effect::set_target(d, e1, Some(cost_target(sctg)));
    // e1:SetOperation(s.scop)
    effect::set_operation(d, e1, Some(operation(scop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetCode(20932152)
    effect::set_code(d, e2, 20932152);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.sccost(e,tp,eg,ep,ev,re,r,rp,chk)
fn sccost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsDiscardable,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_discardable(d, c, None, None)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.DiscardHand(tp,Card.IsDiscardable,1,1,REASON_COST)
    duel::discard_hand(d, a.tp, Some(&|d, c| card::is_discardable(d, c, None, None)), 1, 1, REASON_COST, NO_EX);
    true
}
// function s.mfilter(c,e,tp,mc)
fn mfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // local mg=Group.FromCards(c,mc)
    let mut mg = CardSet::from_card(c);
    group::add_card(&mut mg, &CardSet::from_card(mc));
    // return c:IsSetCard(SET_JUNK) and not c:IsType(TYPE_TUNER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.scfilter,tp,LOCATION_EXTRA,0,1,nil,mg,tp)
    card::is_set_card(d, c, &[SET_JUNK as u16])
        && !card::is_type(d, c, TYPE_TUNER)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, sc| scfilter(d, sc, &mg, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.scfilter(c,mg,tp)
fn scfilter(d: &mut Duel, c: CardId, mg: &CardSet, tp: u8) -> bool {
    // return c:ListsArchetypeAsMaterial(SET_SYNCHRON) and Duel.GetLocationCountFromEx(tp,tp,mg,c)>0 and c:IsSynchroSummonable(nil,mg)
    card::lists_archetype_as_material(d, c, &[SET_SYNCHRON as u32])
        && duel::get_location_count_from_ex(d, tp, Some(tp), mg.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_synchro_summonable(d, c, None, Some(mg.clone()), 0, 0)
}
// function s.sctg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sctg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.mfilter(chkc,e,tp,c) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && mfilter(d, chkc, a.e, tp, c);
    }
    // if chk==0 then return Duel.IsPlayerCanSpecialSummonCount(tp,2)
    if a.chk == Some(0) {
        // and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
        // and Duel.IsExistingTarget(s.mfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,c) end
        return duel::is_player_can_special_summon_count(d, tp, 2)
            && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, cc| mfilter(d, cc, a.e, tp, c)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SMATERIAL as u64);
    // Duel.SelectTarget(tp,s.mfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,c)
    duel::select_target(d, tp, Some(&|d, cc| mfilter(d, cc, a.e, tp, c)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
// function s.scop(e,tp,eg,ep,ev,re,r,rp)
fn scop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if !card::is_relate_to_effect(d, tc, a.e) || duel::special_summon(d, CardSet::from_card(tc), 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local mg=Group.FromCards(c,tc)
    let mut mg = CardSet::from_card(c);
    group::add_card(&mut mg, &CardSet::from_card(tc));
    // local g=Duel.GetMatchingGroup(s.scfilter,tp,LOCATION_EXTRA,0,nil,mg,tp)
    let g = duel::get_matching_group(d, Some(&|d, sc| scfilter(d, sc, &mg, tp)), tp, LOCATION_EXTRA, 0, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
        effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
        // e1:SetValue(LOCATION_REMOVED)
        effect::set_value(d, e1, LOCATION_REMOVED as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // tc:RegisterEffect(e2,true)
        card::register_effect(d, tc, e2, true);
        // Duel.SynchroSummon(tp,sg:GetFirst(),nil,mg)
        let sc = group::get_first(d, &sg).nil();
        duel::synchro_summon(d, tp, sc, None, Some(mg), 0, 0);
    }
}
