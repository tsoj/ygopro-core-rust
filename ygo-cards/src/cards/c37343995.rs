//! Exosister Martha
use super::prelude::*;

pub const ID: u32 = 37343995;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[16474916], listed_series: &[SET_EXOSISTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon self
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.hspcon)
    effect::set_condition(d, e1, Some(condition(hspcon)));
    // e1:SetCost(s.hspcost)
    effect::set_cost(d, e1, Some(cost_target(hspcost)));
    // e1:SetTarget(s.hsptg)
    effect::set_target(d, e1, Some(cost_target(hsptg)));
    // e1:SetOperation(s.hspop)
    effect::set_operation(d, e1, Some(operation(hspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,function(c) return c:IsSetCard(SET_EXOSISTER) end)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
    // Special Summon Xyz
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD | EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_LEAVE_GRAVE)
    effect::set_code(d, e2, EVENT_LEAVE_GRAVE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.xsptg)
    effect::set_target(d, e2, Some(cost_target(xsptg)));
    // e2:SetOperation(s.xspop)
    effect::set_operation(d, e2, Some(operation(xspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(c) return c:IsSetCard(SET_EXOSISTER) end
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    card::is_set_card(d, a.c(0), &[SET_EXOSISTER as u16])
}
// function s.hspcon(e,tp,eg,ep,ev,re,r,rp)
fn hspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(aux.NOT(aux.FaceupFilter(Card.IsType,TYPE_XYZ)),tp,LOCATION_MZONE,0,1,nil)
    !duel::is_existing_matching_card(
        d,
        Some(&|d, c| !(card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ))),
        a.tp,
        LOCATION_MZONE,
        0,
        1,
        NO_EX,
    )
}
// function s.hspcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn hspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // --Cannot Special Summon monsters, except "Exosister" monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_OATH | EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return not c:IsSetCard(SET_EXOSISTER) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_EXOSISTER as u16]))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    true
}
// function s.dspfilter(c,e,tp)
fn dspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(16474916) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_code(d, c, &[16474916]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.hsptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn hsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            // and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            // and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None)
            // and Duel.IsExistingMatchingCard(s.dspfilter,tp,LOCATION_DECK,0,1,nil,e,tp) end
            && duel::is_existing_matching_card(d, Some(&|d, x| dspfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,2,tp,LOCATION_DECK|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 2, a.tp, (LOCATION_DECK | LOCATION_HAND) as i32);
    true
}
// function s.hspop(e,tp,eg,ep,ev,re,r,rp)
fn hspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // if Duel.SpecialSummonStep(c,0,tp,tp,false,false,POS_FACEUP) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if duel::special_summon_step(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None)
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.dspfilter,tp,LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| dspfilter(d, x, a.e, a.tp)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc then
        if let Some(sc) = sc {
            // Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.xspfilter(c,e,tp,mc)
fn xspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return c:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp) and c:IsSetCard(SET_EXOSISTER) and mc:IsCanBeXyzMaterial(c,tp)
    card::is_type_ctx(d, c, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && card::is_set_card(d, c, &[SET_EXOSISTER as u16])
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        // and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        // and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
// function s.xsptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn xsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
        let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::from_card(c)), a.tp, None, None, REASON_XYZ);
        let n = group::get_count(&pg);
        // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and Duel.IsExistingMatchingCard(s.xspfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c)
        return (n == 0 || (n == 1 && group::is_contains(&pg, c)))
            && duel::is_existing_matching_card(d, Some(&|d, x| xspfilter(d, x, a.e, a.tp, c)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
// function s.xspop(e,tp,eg,ep,ev,re,r,rp)
fn xspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) or c:IsControler(1-tp) or c:IsImmuneToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) || card::is_controler(d, c, 1 - a.tp) || card::is_immune_to_effect(d, c, a.e) {
        return;
    }
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)
    let pg = aux::get_must_be_material_group_opt(d, a.tp, Some(&CardSet::from_card(c)), a.tp, None, None, REASON_XYZ);
    let n = group::get_count(&pg);
    // if #pg>1 or (#pg==1 and not pg:IsContains(c)) then return end
    if n > 1 || (n == 1 && !group::is_contains(&pg, c)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.xspfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,c):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| xspfilter(d, x, a.e, a.tp, c)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &g);
    // if sc then
    if let Some(sc) = sc {
        // local mg=Group.FromCards(c)
        let mg = CardSet::from_card(c);
        // sc:SetMaterial(mg)
        card::set_material(d, sc, mg.clone());
        // Duel.Overlay(sc,mg)
        duel::overlay(d, sc, mg, false);
        // if Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
        if duel::special_summon(d, sc, SUMMON_TYPE_XYZ, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
            // sc:CompleteProcedure()
            card::complete_procedure(d, sc);
        }
    }
}
