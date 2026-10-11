//! BF－南風のアウステル
//! Blackwing - Auster the South Wind
use super::prelude::*;

pub const ID: u32 = 17465972;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[0x1002, COUNTER_FEATHER as u32],
    listed_series: &[SET_BLACKWING],
    listed_names: &[CARD_BLACK_WINGED_DRAGON],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // spsummon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // place counters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_COUNTER)
    effect::set_category(d, e3, CATEGORY_COUNTER);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e3, Some(cost::self_banish()));
    // e3:SetTarget(s.cttg)
    effect::set_target(d, e3, Some(cost_target(cttg)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsLevelBelow(4)
    //   and c:IsSetCard(SET_BLACKWING) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_faceup(d, c)
        && card::is_level_below(d, c, 4)
        && card::is_set_card(d, c, &[SET_BLACKWING as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_REMOVED) and s.spfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_REMOVED) && spfilter(d, chkc, a.e, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_REMOVED,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_REMOVED,0,1,1,nil,e,tp)
    let g = duel::select_target(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_REMOVED, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
        duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
    }
}
/// `s.wcfilter(c)`
fn wcfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:GetCounter(0x1002)==0
    card::is_faceup(d, c) && card::get_counter(d, c, 0x1002) == 0
}
/// `s.cttg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local b1=Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_BLACK_WINGED_DRAGON),tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>0
    let bwd = aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_BLACK_WINGED_DRAGON]));
    let b1 = duel::is_existing_matching_card(d, Some(&bwd), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD) > 0;
    // local b2=Duel.IsExistingMatchingCard(s.wcfilter,tp,0,LOCATION_MZONE,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&wcfilter), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=0
    let op = if b1 && b2 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,2),aux.Stringid(id,3))
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2), aux::stringid(ID, 3)])
    } else if b1 {
        // op=Duel.SelectOption(tp,aux.Stringid(id,2))
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 2)])
    } else {
        // op=Duel.SelectOption(tp,aux.Stringid(id,3))+1
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 3)]) + 1
    };
    // if op==0 then
    if op == 0 {
        // e:SetOperation(s.bwop)
        effect::set_operation(d, a.e, Some(operation(bwop)));
    } else {
        // e:SetOperation(s.wcop)
        effect::set_operation(d, a.e, Some(operation(wcop)));
    }
    true
}
/// `s.bwop(e,tp,eg,ep,ev,re,r,rp)`
fn bwop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.FaceupFilter(Card.IsCode,CARD_BLACK_WINGED_DRAGON),tp,LOCATION_MZONE,0,1,1,nil):GetFirst()
    let bwd = aux::faceup_filter(|d, c| card::is_code(d, c, &[CARD_BLACK_WINGED_DRAGON]));
    let sel = duel::select_matching_card(d, a.tp, Some(&bwd), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // local ct=Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    let ct = duel::get_field_group_count(d, a.tp, 0, LOCATION_ONFIELD);
    // if tc and ct>0 then
    if let Some(tc) = tc {
        if ct > 0 {
            // tc:AddCounter(COUNTER_FEATHER,ct)
            card::add_counter(d, tc, COUNTER_FEATHER as u16, ct as u16, false);
        }
    }
}
/// `s.wcop(e,tp,eg,ep,ev,re,r,rp)`
fn wcop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.wcfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&wcfilter), a.tp, 0, LOCATION_MZONE, NO_EX);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // tc:AddCounter(0x1002,1)
        card::add_counter(d, tc, 0x1002, 1, false);
    }
}
