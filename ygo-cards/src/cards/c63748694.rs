//! Gunkan Suship Shari Red
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 63748694;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_SUSHIP_SHARI], listed_series: &[SET_GUNKAN], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Change its name to "Gunkan Suship Shari"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE);
    // e1:SetValue(CARD_SUSHIP_SHARI)
    effect::set_value(d, e1, CARD_SUSHIP_SHARI as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Special Summon itself from the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsCode(CARD_SUSHIP_SHARI) and not c:IsPublic()
    card::is_code(d, c, &[CARD_SUSHIP_SHARI]) && !card::is_public(d, c)
}
// function s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), tp, LOCATION_HAND, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,c)
    let g = duel::select_matching_card(d, tp, Some(&cfilter), tp, LOCATION_HAND, 0, 1, 1, false, c.into()).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, &g);
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, tp);
    true
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `Group.FromCards(a, b)`
fn from_cards(a: CardId, b: CardId) -> CardSet {
    let mut g = CardSet::from_card(a);
    group::add_card(&mut g, &CardSet::from_card(b));
    g
}
// function s.spfilter(c,e,tp,mc)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId) -> bool {
    // return c:IsSetCard(SET_GUNKAN) and not c:IsCode(CARD_SUSHIP_SHARI) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.xyzfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,mc,c)
    card::is_set_card(d, c, &[SET_GUNKAN as u16])
        && !card::is_code(d, c, &[CARD_SUSHIP_SHARI])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(d, Some(&|d, x| xyzfilter(d, x, e, tp, mc, c)), tp, LOCATION_EXTRA, 0, 1, NO_EX)
}
// function s.xyzfilter(c,e,tp,mc1,mc2)
fn xyzfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc1: CardId, mc2: CardId) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(mc1,mc2),tp,nil,nil,REASON_XYZ)
    let both = from_cards(mc1, mc2);
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&both), tp, None, None, REASON_XYZ);
    // return (#pg<=0 or (#pg==2 and pg:IsContains(mc1) and pg:IsContains(mc2)))
    //     and c:IsType(TYPE_XYZ,c,SUMMON_TYPE_XYZ,tp) and c:ListsCode(mc2:GetCode())
    //     and mc1:IsCanBeXyzMaterial(c,tp) and mc2:IsCanBeXyzMaterial(c,tp)
    //     and Duel.GetLocationCountFromEx(tp,tp,Group.FromCards(mc1,mc2),c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    let pg_ok = pg.is_empty() || (pg.len() == 2 && group::is_contains(&pg, mc1) && group::is_contains(&pg, mc2));
    let lists = {
        let codes = card::get_code(d, mc2);
        card::lists_code(d, c, &codes)
    };
    pg_ok
        && card::is_type_ctx(d, c, TYPE_XYZ, Some(c), SUMMON_TYPE_XYZ as u64, Some(tp))
        && lists
        && card::is_can_be_xyz_material(d, mc1, Some(c), Some(tp), None)
        && card::is_can_be_xyz_material(d, mc2, Some(c), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), (&both).into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) and Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0
    //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_relate_to_effect(d, c, e)
        && duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
    {
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp,c)
        let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, tp, c)), tp, LOCATION_DECK, 0, NO_EX);
        // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,1)) then return end
        if g.is_empty() || !duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local sc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        // if not sc then return end
        let Some(sc) = group::get_first(d, &sel) else {
            return;
        };
        // if Duel.SpecialSummonStep(sc,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // --Negate its effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e1)
            card::register_effect(d, sc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetValue(RESET_TURN_SET)
            effect::set_value(d, e2, RESET_TURN_SET as i64);
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // sc:RegisterEffect(e2)
            card::register_effect(d, sc, e2, false);
        }
        // if Duel.SpecialSummonComplete()==0 then return end
        if duel::special_summon_complete(d) == 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local xyzc=Duel.SelectMatchingCard(tp,s.xyzfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,c,sc):GetFirst()
        let sel2 = duel::select_matching_card(d, tp, Some(&|d, x| xyzfilter(d, x, e, tp, c, sc)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if not xyzc then return end
        let Some(xyzc) = group::get_first(d, &sel2) else {
            return;
        };
        // local mg=Group.FromCards(c,sc)
        let mg = from_cards(c, sc);
        // xyzc:SetMaterial(mg)
        card::set_material(d, xyzc, mg.clone());
        // Duel.Overlay(xyzc,mg)
        duel::overlay(d, xyzc, mg, false);
        // if Duel.SpecialSummon(xyzc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)>0 then
        if duel::special_summon(d, xyzc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
            // xyzc:CompleteProcedure()
            card::complete_procedure(d, xyzc);
        }
    }
}
