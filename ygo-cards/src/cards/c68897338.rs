//! The Fabled Behilmoth
//! 魔轟神獣ベヒルモス
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 68897338;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_FABLED], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Discard 2 or more "Fabled" monsters and Special Summon 1 "Fabled" Synchro Monster from your Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_HANDES+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_HANDES + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.extrasptg)
    effect::set_target(d, e1, Some(cost_target(extrasptg)));
    // e1:SetOperation(s.extraspop)
    effect::set_operation(d, e1, Some(operation(extraspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Fabled" monster from your hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.handspcon)
    effect::set_condition(d, e2, Some(condition(handspcon)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.handsptg)
    effect::set_target(d, e2, Some(cost_target(handsptg)));
    // e2:SetOperation(s.handspop)
    effect::set_operation(d, e2, Some(operation(handspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.discardfilter(c)`
fn discardfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FABLED) and c:IsMonster() and c:HasLevel() and c:IsDiscardable(REASON_EFFECT)
    card::is_set_card(d, c, &[SET_FABLED as u16]) && card::is_monster(d, c) && card::has_level(d, c) && card::is_discardable(d, c, Some(REASON_EFFECT), None)
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    // return Duel.IsExistingMatchingCard(s.extraspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,sg:GetSum(Card.GetOriginalLevel)+e:GetHandler():GetOriginalLevel())
    let h = effect::get_handler(d, e).nil();
    let lv = group::get_sum(d, sg, &|d, c| card::get_original_level(d, c) as i64) + card::get_original_level(d, h) as i64;
    let res = duel::is_existing_matching_card(d, Some(&|d, c| extraspfilter(d, c, e, tp, lv)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    (res, false)
}
/// `s.extraspfilter(c,e,tp,lv)`
fn extraspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    // return c:IsSetCard(SET_FABLED) and c:IsType(TYPE_SYNCHRO) and c:IsLevel(lv)
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_SYNCHRO,tp,false,false)
    card::is_set_card(d, c, &[SET_FABLED as u16])
        && card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_level(d, c, &[lv as u32])
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_SYNCHRO, tp, false, false, None, None, None)
}
/// `s.extrasptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn extrasptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,nil,nil,REASON_SYNCHRO)
        let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::new()), a.tp, None, None, REASON_SYNCHRO);
        // if #pg>0 then return false end
        if !pg.is_empty() {
            return false;
        }
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local g=Duel.GetMatchingGroup(s.discardfilter,tp,LOCATION_HAND,0,c)
        let g = duel::get_matching_group(d, Some(&discardfilter), a.tp, LOCATION_HAND, 0, c.into());
        // return s.discardfilter(c) and #g>0 and aux.SelectUnselectGroup(g,e,tp,1,#g,s.rescon,0)
        return discardfilter(d, c)
            && !g.is_empty()
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(g.len()), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_HANDES,nil,0,tp,2)
    duel::set_operation_info(d, 0, CATEGORY_HANDES, None, 0, a.tp, 2);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.extraspop(e,tp,eg,ep,ev,re,r,rp)`
fn extraspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.discardfilter,tp,LOCATION_HAND,0,c)
    let g = duel::get_matching_group(d, Some(&discardfilter), a.tp, LOCATION_HAND, 0, c.into());
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local rg=aux.SelectUnselectGroup(g,e,tp,1,#g,s.rescon,1,tp,HINTMSG_DISCARD)
    let mut rg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(g.len()), Some(&rescon), 1, a.tp, Some(HINTMSG_DISCARD as u64), None, None, false).unwrap_or_default();
    // if #rg==0 then return end
    if rg.is_empty() {
        return;
    }
    // rg:AddCard(c)
    group::add_card(&mut rg, &CardSet::from_card(c));
    // if Duel.SendtoGrave(rg,REASON_DISCARD|REASON_EFFECT)==0 then return end
    if duel::send_to_grave(d, rg.clone(), REASON_DISCARD | REASON_EFFECT, None, None) == 0 {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,nil,nil,REASON_SYNCHRO)
    let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::new()), a.tp, None, None, REASON_SYNCHRO);
    // if #pg>0 then return end
    if !pg.is_empty() {
        return;
    }
    // local lv=rg:GetSum(Card.GetOriginalLevel)
    let lv = group::get_sum(d, &rg, &|d, c| card::get_original_level(d, c) as i64);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.extraspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,lv):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| extraspfilter(d, x, a.e, a.tp, lv)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if sc and Duel.SpecialSummon(sc,SUMMON_TYPE_SYNCHRO,tp,tp,false,false,POS_FACEUP)>0 then
    if let Some(sc) = sc {
        if duel::special_summon(d, sc, SUMMON_TYPE_SYNCHRO, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
        }
    }
}
/// `s.handspconfilter(c,tp)`
fn handspconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_HAND)
    card::is_previous_controler(d, c, tp) && card::is_previous_location(d, c, LOCATION_HAND)
}
/// `s.handspcon(e,tp,eg,ep,ev,re,r,rp)`
fn handspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.handspconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    !eg.contains(h) && group::is_exists(d, &eg, &|d, c| handspconfilter(d, c, a.tp), 1, NO_EX)
}
/// `s.handspfilter(c,e,tp)`
fn handspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_FABLED) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_FABLED as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.handsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn handsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.handspfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| handspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HAND as i32);
    true
}
/// `s.handspop(e,tp,eg,ep,ev,re,r,rp)`
fn handspop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.handspfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| handspfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
