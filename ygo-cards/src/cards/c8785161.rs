//! Raidraptor - Wild Vulture
use super::prelude::*;

pub const ID: u32 = 8785161;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetCost(Cost.SelfTribute)
    effect::set_cost(d, e1, Some(cost::self_tribute()));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function() ... end)
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_SUMMON_SUCCESS)
        effect::set_code(d, ge1, EVENT_SUMMON_SUCCESS);
        // ge1:SetLabel(id)
        effect::set_label(d, ge1, &[ID as i64]);
        // ge1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, ge1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // ge1:SetOperation(aux.sumreg)
        effect::set_operation(d, ge1, Some(aux::sumreg()));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
        // local ge2=ge1:Clone()
        let ge2 = effect::clone(d, ge1);
        // ge2:SetCode(EVENT_SPSUMMON_SUCCESS)
        effect::set_code(d, ge2, EVENT_SPSUMMON_SUCCESS);
        // ge2:SetLabel(id)
        effect::set_label(d, ge2, &[ID as i64]);
        // Duel.RegisterEffect(ge2,0)
        duel::register_effect(d, ge2, 0);
    });
}
// s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFlagEffect(id)>0
    let h = effect::get_handler(d, a.e).nil();
    card::get_flag_effect(d, h, ID) > 0
}
// s.spfilter1(c,e,tp)
fn spfilter1(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c) as i64;
    // return lv>0 and lv<6 and c:IsSetCard(SET_RAIDRAPTOR) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //   and Duel.IsExistingMatchingCard(s.spfilter2,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,c,e,tp,6-lv)
    lv > 0
        && lv < 6
        && card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, cc| spfilter2(d, cc, e, tp, 6 - lv)),
            tp,
            LOCATION_HAND | LOCATION_GRAVE,
            0,
            1,
            c.into(),
        )
}
// s.spfilter2(c,e,tp,lv)
fn spfilter2(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    // return c:IsSetCard(SET_RAIDRAPTOR) and c:IsLevel(lv) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_RAIDRAPTOR as u16])
        && card::is_level(d, c, &[lv as u32])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if e:GetHandler():GetSequence()<5 then ft=ft+1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::get_sequence(d, h) < 5 {
        ft += 1;
    }
    // if chk==0 then return ft>1 and Duel.IsExistingMatchingCard(s.spfilter1,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return ft > 1
            && duel::is_existing_matching_card(
                d,
                Some(&|d, c| spfilter1(d, c, a.e, a.tp)),
                a.tp,
                LOCATION_HAND | LOCATION_GRAVE,
                0,
                1,
                NO_EX,
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_GRAVE) as i32);
    true
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g1=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter1),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let nv1 = aux::necro_valley_filter(|d, c| spfilter1(d, c, a.e, tp));
    let mut g1 = duel::select_matching_card(d, tp, Some(&nv1), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local tc1=g1:GetFirst()
    let tc1 = group::get_first(d, &g1);
    // if not tc1 then return end
    let Some(tc1) = tc1 else {
        return;
    };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g2=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter2),tp,LOCATION_HAND|LOCATION_GRAVE,0,1,1,tc1,e,tp,6-tc1:GetLevel())
    let lv = 6 - card::get_level(d, tc1) as i64;
    let nv2 = aux::necro_valley_filter(|d, c| spfilter2(d, c, a.e, tp, lv));
    let g2 = duel::select_matching_card(d, tp, Some(&nv2), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, tc1.into()).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SpecialSummon(g1,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, g1, 0, tp, tp, false, false, POS_FACEUP as u8, None);
}
