//! Numeron Chaos Ritual
use super::prelude::*;

pub const ID: u32 = 41850466;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[79747096, CARD_NUMERON_NETWORK, 89477759], listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
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
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.GlobalCheck(s,function()
    //     local ge1=Effect.CreateEffect(c)
    //     ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    //     ge1:SetCode(EVENT_DESTROYED)
    //     ge1:SetOperation(s.checkop)
    //     Duel.RegisterEffect(ge1,0)
    // end)
    aux::global_check(d, ID, |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_DESTROYED);
        effect::set_operation(d, ge1, Some(operation(checkop)));
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEUP)
    //     and c:GetPreviousCodeOnField()==79747096
    //     and c:IsReason(REASON_EFFECT) and c:GetReasonEffect():IsMonsterEffect()
    if !(card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_previous_position(d, c, POS_FACEUP)) {
        return false;
    }
    if card::get_previous_code_on_field(d, c).first().copied() != Some(79747096) {
        return false;
    }
    if !card::is_reason(d, c, REASON_EFFECT) {
        return false;
    }
    let re = card::get_reason_effect(d, c).nil();
    effect::is_monster_effect(d, re)
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local g=eg:Filter(s.cfilter,nil)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &cfilter, NO_EX);
    // for tc in aux.Next(g) do
    for tc in group::iter(d, &g) {
        // Duel.RegisterFlagEffect(tc:GetPreviousControler(),id,RESET_PHASE|PHASE_END,0,1)
        let p = card::get_previous_controler(d, tc);
        duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetFlagEffect(tp,id)>0
    duel::get_flag_effect(d, a.tp, ID) > 0
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(89477759) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    card::is_code(d, c, &[89477759])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.matfilter(c)`
fn matfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NUMBER) and c:IsType(TYPE_XYZ)
    card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_type(d, c, TYPE_XYZ)
}
/// `s.rmgchk(Card.IsCode,CARD_NUMERON_NETWORK)`: `(c:IsLocation(LOCATION_GRAVE) or c:IsFaceup()) and c:IsCode(CARD_NUMERON_NETWORK)`
fn rmgchk_code(d: &mut Duel, c: CardId) -> bool {
    (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c)) && card::is_code(d, c, &[CARD_NUMERON_NETWORK])
}
/// `s.rmgchk(s.matfilter)`: `(c:IsLocation(LOCATION_GRAVE) or c:IsFaceup()) and s.matfilter(c)`
fn rmgchk_mat(d: &mut Duel, c: CardId) -> bool {
    (card::is_location(d, c, LOCATION_GRAVE) || card::is_faceup(d, c)) && matfilter(d, c)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return false end
    // (`chkc` is not a parameter of this function: the global `chkc` is nil, so this never returns)

    // if chk==0 then return Duel.IsExistingTarget(s.rmgchk(Card.IsCode,CARD_NUMERON_NETWORK),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,nil)
    //     and Duel.IsExistingTarget(s.rmgchk(s.matfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,4,nil)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&rmgchk_code), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&rmgchk_mat), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 4, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local atg1=Duel.SelectTarget(tp,s.rmgchk(Card.IsCode,CARD_NUMERON_NETWORK),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,1,1,nil)
    let mut atg1 = duel::select_target(d, tp, Some(&rmgchk_code), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local atg2=Duel.SelectTarget(tp,s.rmgchk(s.matfilter),tp,LOCATION_GRAVE|LOCATION_REMOVED,0,4,4,nil)
    let atg2 = duel::select_target(d, tp, Some(&rmgchk_mat), tp, LOCATION_GRAVE | LOCATION_REMOVED, 0, 4, 4, false, NO_EX).nil();
    // local sg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_EXTRA,0,nil,e,tp)
    let sg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_EXTRA, 0, NO_EX);
    // atg1:Merge(atg2)
    group::add_card(&mut atg1, &atg2);
    // local lvgg=atg1:Filter(Card.IsLocation,nil,LOCATION_GRAVE)
    let lvgg = group::filter(d, &atg1, &|d, c| card::is_location(d, c, LOCATION_GRAVE), NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,sg,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(sg), 1, tp, LOCATION_EXTRA as i32);
    // if #lvgg>0 then
    if !lvgg.is_empty() {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,lvgg,#lvgg,0,0)
        let n = lvgg.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(lvgg), n, 0, 0);
    }
    true
}
/// `s.limittg(e,c,tp)`
fn limittg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(2);
    // local sp=Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)+Duel.GetActivityCount(tp,ACTIVITY_SUMMON)
    let sp = duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON])[0] + duel::get_activity_count(d, tp, &[ACTIVITY_SUMMON])[0];
    // return sp-e:GetLabel()>=1
    sp - effect::get_label(d, e) >= 1
}
/// `s.countval(e,re,tp)`
fn countval(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let tp = a.p(2);
    // local sp=Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)+Duel.GetActivityCount(tp,ACTIVITY_SUMMON)
    let sp = duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON])[0] + duel::get_activity_count(d, tp, &[ACTIVITY_SUMMON])[0];
    // if sp-e:GetLabel()>=1 then return 0 else return 1-sp+e:GetLabel() end
    let label = effect::get_label(d, e);
    if sp - label >= 1 { 0 } else { 1 - sp + label }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_SET_ATTACK)
            effect::set_code(d, e1, EFFECT_SET_ATTACK);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetValue(10000)
            effect::set_value(d, e1, 10000);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_SET_DEFENSE)
            effect::set_code(d, e2, EFFECT_SET_DEFENSE);
            // e2:SetValue(1000)
            effect::set_value(d, e2, 1000);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
            // local tg=Duel.GetTargetCards(e):Filter(Card.IsRelateToEffect,nil,e)
            let tgs = duel::get_target_cards(d, a.e).nil();
            let tg = group::filter(d, &tgs, &|d, x| card::is_relate_to_effect(d, x, a.e), NO_EX);
            // if #tg==5 then
            if tg.len() == 5 {
                // Duel.Overlay(tc,tg)
                duel::overlay(d, tc, tg, false);
            }
        }
    }
    // local spc=Duel.GetActivityCount(tp,ACTIVITY_SPSUMMON)+Duel.GetActivityCount(tp,ACTIVITY_SUMMON)
    let spc = duel::get_activity_count(d, tp, &[ACTIVITY_SPSUMMON])[0] + duel::get_activity_count(d, tp, &[ACTIVITY_SUMMON])[0];
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.limittg)
    effect::set_target(d, e1, Some(lua_fn(limittg)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetLabel(spc)
    effect::set_label(d, e1, &[spc]);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SUMMON);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EFFECT_LEFT_SPSUMMON_COUNT)
    effect::set_code(d, e3, EFFECT_LEFT_SPSUMMON_COUNT);
    // e3:SetValue(s.countval)
    effect::set_value_fn(d, e3, lua_fn(countval));
    // Duel.RegisterEffect(e3,tp)
    duel::register_effect(d, e3, tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,1),nil)
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 1)), 0, None);
}
