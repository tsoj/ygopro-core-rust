//! Lady's Dragonmaid
//! scripted by pyrQ
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 48658295;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRAGONMAID as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 2 "Dragonmaid" monsters with the same Attribute but different Levels
    // Fusion.AddProcMixN(c,true,true,s.matfilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(|d, x, a| matfilter(d, x, a))), 2)]);
    // Fusion.AddContactProc(c,s.contactfil,function(g) Duel.Remove(g,POS_FACEUP,REASON_COST|REASON_MATERIAL) end,
    //     function(e) return not e:GetHandler():IsLocation(LOCATION_EXTRA) end)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(Arc::new(contactfil)),
            material_operation: Some(Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(sumcon))),
            ..Default::default()
        },
    );
    // Special Summon 1 Level 4 or lower "Dragonmaid" monster from your Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Fusion Summon 1 Dragon Fusion Monster from your Extra Deck, by shuffling its materials from your field and/or banishment into the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_FUSION_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_FUSION_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(Fusion.SummonEffTG(params))
    let tg = fusion::summon_eff_tg(d, summon_params());
    effect::set_target(d, e2, Some(tg));
    // e2:SetOperation(Fusion.SummonEffOP(params))
    let op = fusion::summon_eff_op(d, summon_params());
    effect::set_operation(d, e2, Some(op));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `params` of the Fusion Summon effect: `{fusfilter=aux.FilterBoolFunction(Card.IsRace,RACE_DRAGON),
/// matfilter=aux.FALSE, extrafil=s.fextra, extraop=Fusion.ShuffleMaterial, extratg=s.extratg}`
fn summon_params() -> fusion::SummonEff {
    fusion::SummonEff {
        fusfilter: Some(fusion::fus_filter(|d, c, _tp| card::is_race(d, c, RACE_DRAGON))),
        matfilter: Some(fusion::mat_filter_fn(|_d, _c, _e, _tp, _v| false)),
        extrafil: Some(Arc::new(fextra)),
        extraop: Some(fusion::shuffle_material()),
        extratg: Some(cost_target(extratg)),
        ..Default::default()
    }
}
/// `s.matfilter(c,fc,sumtype,sp,sub,mg,sg)`
fn matfilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    let (fc, sumtype, sp) = (a.fc, a.sumtype, a.tp);
    // return c:IsSetCard(SET_DRAGONMAID,fc,sumtype,sp) and (not sg or sg:FilterCount(aux.TRUE,c)==0
    //     or (sg:IsExists(Card.IsAttribute,1,c,c:GetAttribute(),fc,sumtype,sp) and not sg:IsExists(Card.IsLevel,1,c,c:GetLevel())
    //     and not sg:IsExists(Card.IsLocation,1,c,c:GetLocation())))
    if !card::is_set_card_ctx(d, c, &[SET_DRAGONMAID as u16], Some(fc), sumtype, Some(sp)) {
        return false;
    }
    let Some(sg) = a.sg else { return true };
    if group::filter_count(d, sg, &|_d, _c| true, c.into()) == 0 {
        return true;
    }
    let att = card::get_attribute(d, c);
    if !group::is_exists(d, sg, &|d, x| card::is_attribute_ctx(d, x, att, Some(fc), sumtype, Some(sp)), 1, c.into()) {
        return false;
    }
    let lv = card::get_level(d, c);
    if group::is_exists(d, sg, &|d, x| card::is_level(d, x, &[lv]), 1, c.into()) {
        return false;
    }
    let loc = card::get_location(d, c);
    !group::is_exists(d, sg, &|d, x| card::is_location(d, x, loc), 1, c.into())
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then return false end
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        return CardSet::new();
    }
    // return Duel.GetMatchingGroup(Card.IsAbleToRemoveAsCost,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX)
}
/// `function(g) Duel.Remove(g,POS_FACEUP,REASON_COST|REASON_MATERIAL) end`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST | REASON_MATERIAL, None, None);
}
/// `function(e) return not e:GetHandler():IsLocation(LOCATION_EXTRA) end`
fn sumcon(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    !card::is_location(d, h, LOCATION_EXTRA)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsSetCard(SET_DRAGONMAID) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_level_below(d, c, 4) && card::is_set_card(d, c, &[SET_DRAGONMAID as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.fextrafil(c)`
fn fextrafil(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToDeck() and (c:IsOnField() or c:IsFaceup())
    card::is_able_to_deck(d, c) && (card::is_on_field(d, c) || card::is_faceup(d, c))
}
/// `s.fextra(e,tp,mg)`: `return Duel.GetMatchingGroup(Fusion.IsMonsterFilter(s.fextrafil),tp,LOCATION_ONFIELD|LOCATION_REMOVED,0,nil)`
fn fextra(d: &mut Duel, _e: EffectId, tp: u8, _mg: &CardSet) -> (Option<CardSet>, Option<LuaFn>) {
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c) && fextrafil(d, c)), tp, LOCATION_ONFIELD | LOCATION_REMOVED, 0, NO_EX);
    (Some(g), None)
}
/// `s.extratg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extratg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,0,tp,LOCATION_ONFIELD|LOCATION_REMOVED)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 0, a.tp, (LOCATION_ONFIELD | LOCATION_REMOVED) as i32);
    true
}
