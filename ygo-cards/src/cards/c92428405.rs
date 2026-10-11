//! Soul of the Supreme King
//! (覇王龍の魂)
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 92428405;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_ZARC], listed_series: &[SET_PENDULUM_DRAGON, SET_XYZ_DRAGON, SET_SYNCHRO_DRAGON, SET_FUSION_DRAGON], ..ScriptInfo::DEFAULT };

/// `local LOCATION_HDEG=LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA|LOCATION_GRAVE`
const LOCATION_HDEG: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon "Supreme King Z-Arc"
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
    // e1:SetHintTiming(0,TIMING_END_PHASE)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_END_PHASE));
    // e1:SetCost(s.cost)
    effect::set_cost(d, e1, Some(cost_target(cost_fn)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon up 4 for monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.PayLPCost(tp,math.floor(Duel.GetLP(tp)/2))
    let half = duel::get_lp(d, a.tp) / 2;
    duel::pay_lp_cost(d, a.tp, half as u32);
    true
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_ZARC) and c:IsCanBeSpecialSummoned(e,0,tp,true,false) and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    card::is_code(d, c, &[CARD_ZARC])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_EXTRA,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d, c| filter(d, c, e, tp)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummonStep(tc,0,tp,tp,true,false,POS_FACEUP) then
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, tp, tp, true, false, POS_FACEUP as u8, None) {
            // local c=e:GetHandler()
            let c = effect::get_handler(d, e).nil();
            // Negate its effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
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
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // Return it to the Extra Deck
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e3:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
            effect::set_property(d, e3, EFFECT_FLAG_IGNORE_IMMUNE, 0);
            // e3:SetCode(EVENT_PHASE+PHASE_END)
            effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
            // e3:SetCountLimit(1)
            effect::set_count_limit(d, e3, 1, 0, 0, 0);
            // e3:SetLabel(Duel.GetTurnCount()+1)
            effect::set_label(d, e3, &[duel::get_turn_count(d, None) as i64 + 1]);
            // e3:SetLabelObject(tc)
            effect::set_label_object(d, e3, Value::Card(tc));
            // e3:SetCondition(s.tdcon)
            effect::set_condition(d, e3, Some(condition(tdcon)));
            // e3:SetOperation(s.tdop)
            effect::set_operation(d, e3, Some(operation(tdop)));
            // Duel.RegisterEffect(e3,tp)
            duel::register_effect(d, e3, tp);
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,2))
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, 0, aux::stringid(ID, 2));
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.tdcon(e,tp,eg,ep,ev,re,r,rp)`
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // if tc:HasFlagEffect(id) then
    if card::has_flag_effect(d, tc, ID, None) {
        // return Duel.GetTurnCount()==e:GetLabel()
        duel::get_turn_count(d, None) as i64 == effect::get_label(d, a.e)
    } else {
        // e:Reset()
        effect::reset(d, a.e);
        // return false
        false
    }
}
/// `s.tdop(e,tp,eg,ep,ev,re,r,rp)`
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    // Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT)
    duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT, None);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsSpellEffect()
    let re = a.re.nil();
    a.rp == 1 - a.tp && effect::is_spell_effect(d, re)
}
/// `s.spcfilter(c,e,tp)`
fn spcfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsCode(CARD_ZARC) and c:IsAbleToRemoveAsCost()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HDEG,0,1,nil,e,tp,c)
    card::is_faceup(d, c)
        && card::is_code(d, c, &[CARD_ZARC])
        && card::is_able_to_remove_as_cost(d, c, None)
        && duel::is_existing_matching_card(d, Some(&|d, c2| spfilter(d, c2, e, tp, Some(c))), tp, LOCATION_HDEG, 0, 1, NO_EX)
}
/// `s.spfilter(c,e,tp,rmc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rmc: Option<CardId>) -> bool {
    // local loc_extr=c:IsLocation(LOCATION_EXTRA)
    let loc_extr = card::is_location(d, c, LOCATION_EXTRA);
    // return c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and c:IsSetCard({SET_PENDULUM_DRAGON,SET_XYZ_DRAGON,SET_SYNCHRO_DRAGON,SET_FUSION_DRAGON})
    //     and ((not loc_extr and Duel.GetMZoneCount(tp,rmc)>0) or (loc_extr and Duel.GetLocationCountFromEx(tp,tp,rmc,c)>0))
    if !(card::is_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)) {
        return false;
    }
    if !card::is_set_card(d, c, &[SET_PENDULUM_DRAGON as u16, SET_XYZ_DRAGON as u16, SET_SYNCHRO_DRAGON as u16, SET_FUSION_DRAGON as u16]) {
        return false;
    }
    if !loc_extr {
        let mzone = match rmc {
            Some(rc) => duel::get_m_zone_count(d, tp, rc.into(), None, None, None).0,
            None => duel::get_m_zone_count(d, tp, NO_EX, None, None, None).0,
        };
        mzone > 0
    } else {
        let ex = match rmc {
            Some(rc) => duel::get_location_count_from_ex(d, tp, Some(tp), rc.into(), duel::ExtraCard::Card(c), None).0,
            None => duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0,
        };
        ex > 0
    }
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // if chk==0 then return aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)
    //     and Duel.IsExistingMatchingCard(s.spcfilter,tp,LOCATION_MZONE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return bfgcost(d, a, 0) && duel::is_existing_matching_card(d, Some(&|d, c| spcfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)
    bfgcost(d, a, a.chk.unwrap_or(1));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcfilter,tp,LOCATION_MZONE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spcfilter(d, c, e, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// `aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)` called from a cost with the same arguments
fn bfgcost(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    aux::bfgcost()
        .call(
            d,
            &[
                Value::Effect(a.e),
                Value::Int(a.tp as i64),
                a.eg.clone().map_or(Value::Nil, Value::Group),
                Value::Int(a.ep as i64),
                Value::Int(a.ev as i64),
                Value::effect(a.re),
                Value::Int(a.r as i64),
                Value::Int(a.rp as i64),
                Value::Int(chk),
            ],
        )
        .truthy()
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HDEG)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_HDEG as i32);
    true
}
/// `s.resfilter(c)`
fn resfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_EXTRA) and c:IsType(TYPE_PENDULUM) and c:IsFaceup()
    card::is_location(d, c, LOCATION_EXTRA) && card::is_type(d, c, TYPE_PENDULUM) && card::is_faceup(d, c)
}
/// `aux.PropertyTableFilter(Card.GetSetCard,SET_PENDULUM_DRAGON,SET_XYZ_DRAGON,SET_SYNCHRO_DRAGON,SET_FUSION_DRAGON)`
fn checkfunc() -> impl Fn(&mut Duel, CardId) -> Vec<i64> {
    aux::property_table_filter(
        |d, c| card::get_set_card(d, c).into_iter().map(i64::from).collect(),
        &[SET_PENDULUM_DRAGON as i64, SET_XYZ_DRAGON as i64, SET_SYNCHRO_DRAGON as i64, SET_FUSION_DRAGON as i64],
    )
}
/// `s.rescon(checkfunc)`: `function(sg,e,tp,mg) return true, ... end`
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let checkfunc = checkfunc();
    // Duel.GetLocationCountFromEx(tp,tp,nil,TYPE_PENDULUM)<sg:FilterCount(s.resfilter,nil) or not sg:CheckDifferentProperty(checkfunc)
    let count = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Type(TYPE_PENDULUM), None).0;
    let filtered = group::filter_count(d, sg, &resfilter, NO_EX) as i32;
    (true, count < filtered || !group::check_different_property(d, sg, &checkfunc))
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HDEG,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp, None))), tp, LOCATION_HDEG, 0, NO_EX);
    // local ft=math.min(Duel.GetLocationCount(tp,LOCATION_MZONE)+Duel.GetLocationCountFromEx(tp,tp),4)
    let ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    let ft2 = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::None, None).0;
    let mut ft = (ft1 + ft2).min(4);
    // if #g>0 and ft>0 then
    if !g.is_empty() && ft > 0 {
        // local checkfunc=aux.PropertyTableFilter(...)
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
        if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
            ft = 1;
        }
        // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,s.rescon(checkfunc),1,tp,HINTMSG_SPSUMMON,s.rescon(checkfunc))
        let sg = aux::select_unselect_group(
            d,
            &g,
            Some(e),
            tp,
            Some(1),
            Some(ft as usize),
            Some(&rescon),
            1,
            tp,
            Some(HINTMSG_SPSUMMON as u64),
            Some(&rescon),
            None,
            false,
        )
        .unwrap_or_default();
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // local exg=sg:Filter(Card.IsLocation,nil,LOCATION_EXTRA)
        let exg = group::filter(d, &sg, &|d, c| card::is_location(d, c, LOCATION_EXTRA), NO_EX);
        // local nexg=sg-exg
        let mut nexg = sg.clone();
        group::remove_card(&mut nexg, &exg);
        // for tc in nexg:Iter() do
        for tc in group::iter(d, &nexg) {
            // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
        // for tc in exg:Iter() do
        for tc in group::iter(d, &exg) {
            // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
