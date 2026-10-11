//! Gem-Knight Master Diamond Dispersion
//! ジェムナイトマスター・ダイヤ－ディスパージョン
use super::prelude::*;

pub const ID: u32 = 25342956;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_GEM, SET_GEM_KNIGHT], material_setcode: &[SET_GEM], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Materials: 3 "Gem-" monsters
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_GEM),3)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_GEM as u16], Some(fc), st, Some(tp)))), 3)],
    );
    // Special Summon up to 3 non-Rock "Gem-" monsters with different names from your Extra Deck and/or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER));
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1, Some(cost::self_to_grave()));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon itself from the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYED);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.selfspcon)
    effect::set_condition(d, e2, Some(condition(selfspcon)));
    // e2:SetTarget(s.selfsptg)
    effect::set_target(d, e2, Some(cost_target(selfsptg)));
    // e2:SetOperation(s.selfspop)
    effect::set_operation(d, e2, Some(operation(selfspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp,hc)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, hc: Option<CardId>) -> bool {
    // if not (c:IsSetCard(SET_GEM) and not c:IsRace(RACE_ROCK) and c:IsMonster() and c:IsCanBeSpecialSummoned(e,0,tp,true,false)) then return false end
    if !(card::is_set_card(d, c, &[SET_GEM as u16]) && !card::is_race(d, c, RACE_ROCK as u64) && card::is_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, true, false, None, None, None)) {
        return false;
    }
    // if c:IsLocation(LOCATION_EXTRA) then
    if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,hc,c)>0
        let ex = match hc {
            Some(h) => h.into(),
            None => NO_EX,
        };
        duel::get_location_count_from_ex(d, tp, Some(tp), ex, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        // return Duel.GetMZoneCount(tp,hc)>0
        let ex = match hc {
            Some(h) => h.into(),
            None => NO_EX,
        };
        duel::get_m_zone_count(d, tp, ex, None, None, None).0 > 0
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE|LOCATION_EXTRA,0,1,nil,e,tp,e:GetHandler()) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, Some(h))), a.tp, LOCATION_GRAVE | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_EXTRA) as i32);
    true
}
/// `s.exfilter1(c)`
fn exfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_EXTRA) and c:IsFacedown() and c:IsType(TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ)
    card::is_location(d, c, LOCATION_EXTRA) && card::is_facedown(d, c) && card::is_type(d, c, TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ)
}
/// `s.exfilter2(c)`
fn exfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_EXTRA) and (c:IsType(TYPE_LINK) or (c:IsFaceup() and c:IsType(TYPE_PENDULUM)))
    card::is_location(d, c, LOCATION_EXTRA) && (card::is_type(d, c, TYPE_LINK) || (card::is_faceup(d, c) && card::is_type(d, c, TYPE_PENDULUM)))
}
/// `s.rescon(ft1,ft2,ft3,ft4,ft)`: returns `function(sg,e,tp,mg)` (the Lua never reads `ft2`)
fn rescon(ft1: i64, _ft2: i64, ft3: i64, ft4: i64, ft: i64) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| {
        // local exnpct=sg:FilterCount(s.exfilter1,nil,LOCATION_EXTRA)
        let exnpct = group::filter_count(d, sg, &exfilter1, NO_EX) as i64;
        // local expct=sg:FilterCount(s.exfilter2,nil,LOCATION_EXTRA)
        let expct = group::filter_count(d, sg, &exfilter2, NO_EX) as i64;
        // local mct=sg:FilterCount(aux.NOT(Card.IsLocation),nil,LOCATION_EXTRA)
        let mct = group::filter_count(d, sg, &|d, cc| !card::is_location(d, cc, LOCATION_EXTRA), NO_EX) as i64;
        // local exct=sg:FilterCount(Card.IsLocation,nil,LOCATION_EXTRA)
        let _exct = group::filter_count(d, sg, &|d, cc| card::is_location(d, cc, LOCATION_EXTRA), NO_EX);
        // local groupcount=#sg
        let groupcount = sg.len() as i64;
        // local classcount=sg:GetClassCount(Card.GetCode)
        let classcount = group::get_class_count(d, sg, &|d, cc| card::get_code(d, cc)[0] as i64) as i64;
        // local res=ft3>=exnpct and ft4>=expct and ft1>=mct and ft>=groupcount and classcount==groupcount
        let res = ft3 >= exnpct && ft4 >= expct && ft1 >= mct && ft >= groupcount && classcount == groupcount;
        // return res,not res
        (res, !res)
    }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    // local ft2=Duel.GetLocationCountFromEx(tp)
    let mut ft2 = duel::get_location_count_from_ex(d, tp, None, NO_EX, duel::ExtraCard::None, None).0 as i64;
    // local ft3=Duel.GetLocationCountFromEx(tp,tp,nil,TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ)
    let mut ft3 = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Type(TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ), None).0 as i64;
    // local ft4=Duel.GetLocationCountFromEx(tp,tp,nil,TYPE_PENDULUM|TYPE_LINK)
    let mut ft4 = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Type(TYPE_PENDULUM | TYPE_LINK), None).0 as i64;
    // local ft=math.min(Duel.GetUsableMZoneCount(tp),3)
    let mut ft = (duel::get_usable_m_zone_count(d, tp, None) as i64).min(3);
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        // if ft1>0 then ft1=1 end
        if ft1 > 0 {
            ft1 = 1;
        }
        // if ft2>0 then ft2=1 end
        if ft2 > 0 {
            ft2 = 1;
        }
        // if ft3>0 then ft3=1 end
        if ft3 > 0 {
            ft3 = 1;
        }
        // if ft4>0 then ft4=1 end
        if ft4 > 0 {
            ft4 = 1;
        }
        // ft=1
        ft = 1;
    }
    // local ect=aux.CheckSummonGate(tp)
    let ect = aux::check_summon_gate_limit(d, tp);
    // if ect then
    if let Some(ect) = ect {
        // ft1=math.min(ect,ft1)
        ft1 = ect.min(ft1);
        // ft2=math.min(ect,ft2)
        ft2 = ect.min(ft2);
        // ft3=math.min(ect,ft3)
        ft3 = ect.min(ft3);
        // ft4=math.min(ect,ft4)
        ft4 = ect.min(ft4);
    }
    // local loc=0
    let mut loc: u32 = 0;
    // if ft1>0 then loc=LOCATION_GRAVE end
    if ft1 > 0 {
        loc = LOCATION_GRAVE;
    }
    // if ft2>0 or ft3>0 or ft4>0 then loc=loc|LOCATION_EXTRA end
    if ft2 > 0 || ft3 > 0 || ft4 > 0 {
        loc |= LOCATION_EXTRA;
    }
    // if loc>0 then
    if loc > 0 {
        // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,loc,0,nil,e,tp)
        let e = a.e;
        let sg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(move |d, cc| spfilter(d, cc, e, tp, None))), tp, loc, 0, NO_EX);
        // if #sg>0 then
        if !sg.is_empty() {
            // local rg=aux.SelectUnselectGroup(sg,e,tp,1,ft,s.rescon(ft1,ft2,ft3,ft4,ft),1,tp,HINTMSG_SPSUMMON)
            let rc = rescon(ft1, ft2, ft3, ft4, ft);
            let rg = aux::select_unselect_group(d, &sg, Some(e), tp, Some(1), Some(ft.max(0) as usize), Some(&rc), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
            // Duel.SpecialSummon(rg,0,tp,tp,true,false,POS_FACEUP)
            duel::special_summon(d, rg, 0, tp, tp, true, false, POS_FACEUP as u8, None);
        }
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Cannot Special Summon from the Extra Deck for the rest of this turn, except Fusion Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_FUSION) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, cc| card::is_location(d, cc, LOCATION_EXTRA) && !card::is_type(d, cc, TYPE_FUSION))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // Clock Lizard check
    // aux.addTempLizardCheck(c,tp,function(e,c) return not c:IsOriginalType(TYPE_FUSION) end)
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, cc| !card::is_original_type(d, cc, TYPE_FUSION))), None, None, None, None);
}
/// `s.selfspconfilter(c,tp)`
fn selfspconfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousSetCard(SET_GEM_KNIGHT) and c:IsPreviousTypeOnField(TYPE_FUSION) and c:IsPreviousControler(tp)
    card::is_previous_set_card(d, c, &[SET_GEM_KNIGHT as u16]) && card::is_previous_type_on_field(d, c, TYPE_FUSION) && card::is_previous_controler(d, c, tp)
}
/// `s.selfspcon(e,tp,eg,ep,ev,re,r,rp)`
fn selfspcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not eg:IsContains(e:GetHandler()) and eg:IsExists(s.selfspconfilter,1,nil,tp)
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    !group::is_contains(&eg, h) && group::is_exists(d, &eg, &|d, cc| selfspconfilter(d, cc, a.tp), 1, NO_EX)
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.selfspop(e,tp,eg,ep,ev,re,r,rp)`
fn selfspop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
