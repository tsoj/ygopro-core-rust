//! Sun God Domination
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 101403072;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RA], listed_series: &[SET_THE_WINGED_DRAGON_OF_RA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // If a "The Winged Dragon of Ra" monster is on the field: Apply these effects in sequence
    // ● Special Summon 1 monster from each GY to your field
    // ● If "The Winged Dragon of Ra" is on the field, take control of all monsters your opponent controls
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_CONTROL)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_CONTROL);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(condition_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish this card from your GY and Tribute 1 "The Winged Dragon of Ra" monster; gain LP equal to the ATK it had on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_RECOVER)
    effect::set_category(d, e2, CATEGORY_RECOVER);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.AND(Cost.SelfBanish,s.lpgaincost))
    effect::set_cost(d, e2, Some(cost::and(vec![cost::self_banish(), cost_target(lpgaincost)])));
    // e2:SetTarget(s.lpgaintg)
    effect::set_target(d, e2, Some(cost_target(lpgaintg)));
    // e2:SetOperation(s.lpgainop)
    effect::set_operation(d, e2, Some(operation(lpgainop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn condition_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_THE_WINGED_DRAGON_OF_RA),tp,LOCATION_MZONE,LOCATION_MZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_THE_WINGED_DRAGON_OF_RA as u16])), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if chk==0 then
    if a.chk == Some(0) {
        // local sp_eff_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>=2
        //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp)
        //     and Duel.IsExistingMatchingCard(s.spfilter,tp,0,LOCATION_GRAVE,1,nil,e,tp)
        let sp_eff_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, 0, LOCATION_GRAVE, 1, NO_EX);
        // local ctrl_eff_chk=#g>0 and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_RA),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil)
        //     and Duel.GetLocationCount(tp,LOCATION_MZONE,tp,LOCATION_REASON_CONTROL)>=#g
        let ctrl_eff_chk = !g.is_empty()
            && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_RA])), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX)
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, Some(a.tp), Some(LOCATION_REASON_CONTROL), None).0 >= g.len() as i32;
        // return sp_eff_chk or ctrl_eff_chk
        return sp_eff_chk || ctrl_eff_chk;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,PLAYER_ALL,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, PLAYER_ALL as u8, LOCATION_GRAVE as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_CONTROL,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_possible_operation_info(d, 0, CATEGORY_CONTROL, Some(g), n, a.tp, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local break_chk=false
    let mut break_chk = false;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>=2 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,nil,e,tp)
    //     and Duel.IsExistingMatchingCard(aux.NecroValleyFilter(s.spfilter),tp,0,LOCATION_GRAVE,1,nil,e,tp) then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
        && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, 0, LOCATION_GRAVE, 1, NO_EX)
    {
        // local gyg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,e,tp)
        let gyg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
        // local sg=aux.SelectUnselectGroup(gyg,e,tp,2,2,aux.dpcheck(Card.GetControler),1,tp,HINTMSG_SPSUMMON)
        let sg = aux::select_unselect_group(
            d,
            &gyg,
            Some(a.e),
            a.tp,
            Some(2),
            Some(2),
            Some(&aux::dpcheck(|d: &mut Duel, c: CardId| card::get_controler(d, c) as i64)),
            1,
            a.tp,
            Some(HINTMSG_SPSUMMON as u64),
            None,
            None,
            false,
        )
        .unwrap_or_default();
        // if #sg==2 then
        if sg.len() == 2 {
            // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
        // break_chk=true
        break_chk = true;
    }
    // local ctrlg=Duel.GetMatchingGroup(Card.IsControlerCanBeChanged,tp,0,LOCATION_MZONE,nil)
    let ctrlg = duel::get_matching_group(d, Some(&|d, c| card::is_controler_can_be_changed(d, c, false, None)), a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #ctrlg>0 and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_RA),tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) then
    if !ctrlg.is_empty() && duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_RA])), a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX) {
        // if break_chk then Duel.BreakEffect() end
        if break_chk {
            duel::break_effect(d);
        }
        // Duel.GetControl(ctrlg,tp)
        duel::get_control(d, ctrlg, a.tp, 0, 0, None, None);
    }
}
/// `s.lpgaincostfilter(c)`
fn lpgaincostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_THE_WINGED_DRAGON_OF_RA) and c:GetAttack()>0
    card::is_set_card(d, c, &[SET_THE_WINGED_DRAGON_OF_RA as u16]) && card::get_attack(d, c) > 0
}
/// `s.lpgaincost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lpgaincost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(-1)
    effect::set_label(d, a.e, &[-1]);
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.lpgaincostfilter,1,false,nil,nil) end
    if a.chk == Some(0) {
        return aux::check_release_group_cost(d, a.tp, Some(&lpgaincostfilter), 1, 1, false, None, NO_EX);
    }
    // local sc=Duel.SelectReleaseGroupCost(tp,s.lpgaincostfilter,1,1,false,nil,nil):GetFirst()
    let g = aux::select_release_group_cost(d, a.tp, Some(&lpgaincostfilter), 1, 1, false, None, NO_EX);
    let sc = group::get_first(d, &g).nil();
    // e:GetChainData().atk=sc:GetAttack()
    let atk = card::get_attack(d, sc);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(atk as i64));
    // Duel.Release(sc,REASON_COST)
    duel::release(d, sc, REASON_COST, None);
    true
}
/// `s.lpgaintg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn lpgaintg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local cost_chk=e:GetLabel()==-1
        let cost_chk = effect::get_label(d, a.e) == -1;
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return cost_chk
        return cost_chk;
    }
    // local lp=e:GetChainData().atk
    let lp = chain::effect_data(d, a.e, None, chain::field::CARD).int();
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(lp)
    duel::set_target_param(d, lp as u32);
    // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,1,tp,lp)
    duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 1, a.tp, lp as i32);
    true
}
/// `s.lpgainop(e,tp,eg,ep,ev,re,r,rp)`
fn lpgainop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Recover(p,d,REASON_EFFECT)
    duel::recover(d, info[0].int() as u8, info[1].int(), REASON_EFFECT, false, None);
}
