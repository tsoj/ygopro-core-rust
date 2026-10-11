//! Forbidden Trapezohedron
use super::prelude::*;

pub const ID: u32 = 49033797;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_OUTER_ENTITY, SET_ELDER_ENTITY, SET_OLD_ENTITY],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.outerfilter(c,e,tp)`
fn outerfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_OUTER_ENTITY) and c:IsXyzMonster() and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_OUTER_ENTITY as u16])
        && card::is_xyz_monster(d, c)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.elderfilter(c,e,tp)`
fn elderfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_ELDER_ENTITY) and c:IsFusionMonster() and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_ELDER_ENTITY as u16])
        && card::is_fusion_monster(d, c)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.oldfilter(c,e,tp)`
fn oldfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_OLD_ENTITY) and c:IsSynchroMonster() and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_OLD_ENTITY as u16])
        && card::is_synchro_monster(d, c)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `Duel.GetMatchingGroup(Card.IsFaceup,tp,LOCATION_MZONE,0,nil):GetBitwiseOr(function(c) return c:GetType()&(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ) end)`
fn face_up_types(d: &mut Duel, tp: u8) -> u64 {
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_faceup(d, x)), tp, LOCATION_MZONE, 0, NO_EX);
    group::get_bitwise_or(d, &g, &|d, x| (card::get_type(d, x) & (TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ)) as i64) as u64
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local flag=...
        let flag = face_up_types(d, a.tp);
        // if flag==(TYPE_FUSION|TYPE_SYNCHRO) then
        if flag == (TYPE_FUSION | TYPE_SYNCHRO) as u64 {
            // return e:IsHasType(EFFECT_TYPE_ACTIVATE) and Duel.IsExistingMatchingCard(s.outerfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
            return effect::is_has_type(d, a.e, EFFECT_TYPE_ACTIVATE)
                && duel::is_existing_matching_card(d, Some(&|d, x| outerfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
        // elseif flag==(TYPE_SYNCHRO|TYPE_XYZ) then
        } else if flag == (TYPE_SYNCHRO | TYPE_XYZ) as u64 {
            // return Duel.IsExistingMatchingCard(s.elderfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
            return duel::is_existing_matching_card(d, Some(&|d, x| elderfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
        // elseif flag==(TYPE_XYZ|TYPE_FUSION) then
        } else if flag == (TYPE_XYZ | TYPE_FUSION) as u64 {
            // return Duel.IsExistingMatchingCard(s.oldfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp)
            return duel::is_existing_matching_card(d, Some(&|d, x| oldfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
        // else
        } else {
            // return false
            return false;
        }
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local flag=...
    let flag = face_up_types(d, a.tp);
    // if flag==(TYPE_FUSION|TYPE_SYNCHRO) then
    if flag == (TYPE_FUSION | TYPE_SYNCHRO) as u64 {
        // --● Fusion and Synchro: Special Summon 1 "Outer Entity" Xyz Monster from your Extra Deck, and if you do, attach this card to it as material
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.outerfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| outerfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        let sc = group::get_first(d, &g);
        // if sc and Duel.SpecialSummon(sc,0,tp,tp,false,false,POS_FACEUP)>0 then
        if let Some(sc) = sc {
            if duel::special_summon(d, sc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None) > 0 {
                // local c=e:GetHandler()
                let c = effect::get_handler(d, a.e).nil();
                // if c:IsRelateToEffect(e) then
                if card::is_relate_to_effect(d, c, a.e) {
                    // c:CancelToGrave()
                    card::cancel_to_grave(d, c, true);
                    // Duel.Overlay(sc,c)
                    duel::overlay(d, sc, c, false);
                }
            }
        }
    // elseif flag==(TYPE_SYNCHRO|TYPE_XYZ) then
    } else if flag == (TYPE_SYNCHRO | TYPE_XYZ) as u64 {
        // --● Synchro and Xyz: Special Summon 1 "Elder Entity" Fusion Monster from your Extra Deck
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.elderfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| elderfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if group::get_count(&g) > 0 {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    // elseif flag==(TYPE_XYZ|TYPE_FUSION) then
    } else if flag == (TYPE_XYZ | TYPE_FUSION) as u64 {
        // --● Xyz and Fusion: Special Summon 1 "Old Entity" Synchro Monster from your Extra Deck
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.oldfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, a.tp, Some(&|d, x| oldfilter(d, x, a.e, a.tp)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if group::get_count(&g) > 0 {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
