//! Supreme King Z-ARC - Synchro Universe
use super::prelude::*;

pub const ID: u32 = 48654267;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_SUPREME_KING_GATE, SET_SUPREME_KING_DRAGON],
    listed_names: &[CARD_ZARC],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum Summon procedure
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner DARK Pendulum Monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(s.matfilter),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, x, sc, st, tp| matfilter(d, x, sc, st, tp))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // Special Summon this card from the Pendulum Zone
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.selfspcost)
    effect::set_cost(d, e1, Some(cost_target(selfspcost)));
    // e1:SetTarget(s.selfsptg)
    effect::set_target(d, e1, Some(cost_target(selfsptg)));
    // e1:SetOperation(s.selfspop)
    effect::set_operation(d, e1, Some(operation(selfspop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Becomes "Supreme King Z-ARC" while on field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e2, EFFECT_CHANGE_CODE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(CARD_ZARC)
    effect::set_value(d, e2, CARD_ZARC as i64);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon up to 2 "Supreme King Dragon" monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLED)
    effect::set_code(d, e3, EVENT_BATTLED);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Place this card in Pendulum Zone
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetCondition(s.pencon)
    effect::set_condition(d, e4, Some(condition(pencon)));
    // e4:SetTarget(s.pentg)
    effect::set_target(d, e4, Some(cost_target(pentg)));
    // e4:SetOperation(s.penop)
    effect::set_operation(d, e4, Some(operation(penop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.matfilter(c,val,sc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, sc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsAttribute(ATTRIBUTE_DARK,sc,sumtype,tp) and c:IsType(TYPE_PENDULUM,sc,sumtype,tp)
    card::is_attribute_ctx(d, c, ATTRIBUTE_DARK, sc, sumtype, Some(tp)) && card::is_type_ctx(d, c, TYPE_PENDULUM, sc, sumtype, Some(tp))
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard({SET_SUPREME_KING_GATE,SET_SUPREME_KING_DRAGON}) and c:IsType(TYPE_PENDULUM)
    //   and Duel.GetMZoneCount(tp,c)>0
    card::is_set_card(d, c, &[SET_SUPREME_KING_GATE as u16, SET_SUPREME_KING_DRAGON as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `s.selfspcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfspcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckReleaseGroupCost(tp,s.cfilter,1,false,nil,nil,tp) end
    if a.chk == Some(0) {
        return duel::check_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // local g=Duel.SelectReleaseGroupCost(tp,s.cfilter,1,1,false,nil,nil,tp)
    let g = duel::select_release_group_cost(d, a.tp, Some(&|d, c| cfilter(d, c, a.tp)), 1, 1, false, None, NO_EX);
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    true
}
/// `s.selfsptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn selfsptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
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
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // local ex,_,damp=Duel.CheckEvent(EVENT_BATTLE_DAMAGE,true)
    let pe = duel::check_event(d, EVENT_BATTLE_DAMAGE);
    let ex = pe.is_some();
    let damp = pe.map_or(0, |e| e.event_player);
    // return (bc and bc:IsStatus(STATUS_BATTLE_DESTROYED) and bc:IsControler(1-tp)) or (ex and damp==1-tp)
    if let Some(bc) = bc {
        if card::is_status(d, bc, STATUS_BATTLE_DESTROYED) && card::is_controler(d, bc, 1 - a.tp) {
            return true;
        }
    }
    ex && damp == 1 - a.tp
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsSetCard(SET_SUPREME_KING_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)) then return false end
    if !(card::is_set_card(d, c, &[SET_SUPREME_KING_DRAGON as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None))
    {
        return false;
    }
    // if c:IsLocation(LOCATION_EXTRA) then
    if card::is_location(d, c, LOCATION_EXTRA) {
        // return Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    } else {
        // return Duel.GetMZoneCount(tp)>0
        duel::get_m_zone_count(d, tp, NO_EX, None, None, None).0 > 0
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(
            d,
            Some(&|d, c| spfilter(d, c, a.e, a.tp)),
            a.tp,
            LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_EXTRA | LOCATION_GRAVE) as i32);
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
/// `s.rescon(ft1,ft2,ft3,ft4,ft)`: `function(sg,e,tp,mg) ... return res,not res end`
fn rescon(ft1: i64, _ft2: i64, ft3: i64, ft4: i64) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| {
        // local exnpct=sg:FilterCount(s.exfilter1,nil,LOCATION_EXTRA)
        let exnpct = group::filter_count(d, sg, &exfilter1, NO_EX) as i64;
        // local expct=sg:FilterCount(s.exfilter2,nil,LOCATION_EXTRA)
        let expct = group::filter_count(d, sg, &exfilter2, NO_EX) as i64;
        // local mct=sg:FilterCount(aux.NOT(Card.IsLocation),nil,LOCATION_EXTRA)
        let mct = group::filter_count(d, sg, &|d, c| !card::is_location(d, c, LOCATION_EXTRA), NO_EX) as i64;
        // local exct=sg:FilterCount(Card.IsLocation,nil,LOCATION_EXTRA)
        let _exct = group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_EXTRA), NO_EX);
        // local res=ft3>=exnpct and ft4>=expct and ft1>=mct
        let res = ft3 >= exnpct && ft4 >= expct && ft1 >= mct;
        // return res,not res
        (res, !res)
    }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ft1=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft1 = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    // local ft2=Duel.GetLocationCountFromEx(tp)
    let mut ft2 = duel::get_location_count_from_ex(d, a.tp, None, NO_EX, duel::ExtraCard::None, None).0 as i64;
    // local ft3=Duel.GetLocationCountFromEx(tp,tp,nil,TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ)
    let mut ft3 = duel::get_location_count_from_ex(d, a.tp, Some(a.tp), NO_EX, duel::ExtraCard::Type((TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ) as u32), None).0 as i64;
    // local ft4=Duel.GetLocationCountFromEx(tp,tp,nil,TYPE_PENDULUM+TYPE_LINK)
    let mut ft4 = duel::get_location_count_from_ex(d, a.tp, Some(a.tp), NO_EX, duel::ExtraCard::Type((TYPE_PENDULUM + TYPE_LINK) as u32), None).0 as i64;
    // local ft=math.min(Duel.GetUsableMZoneCount(tp),2)
    let mut ft = duel::get_usable_m_zone_count(d, a.tp, None).min(2) as i64;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
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
    // if ect then
    if let Some(ect) = aux::check_summon_gate_limit(d, a.tp) {
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
    // if ft1>0 then loc=loc|LOCATION_DECK|LOCATION_GRAVE end
    if ft1 > 0 {
        loc |= LOCATION_DECK | LOCATION_GRAVE;
    }
    // if ft2>0 or ft3>0 or ft4>0 then loc=loc|LOCATION_EXTRA end
    if ft2 > 0 || ft3 > 0 || ft4 > 0 {
        loc |= LOCATION_EXTRA;
    }
    // if loc==0 then return end
    if loc == 0 {
        return;
    }
    // local sg=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.spfilter),tp,loc,0,nil,e,tp)
    let sg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp))), a.tp, loc, 0, NO_EX);
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local rg=aux.SelectUnselectGroup(sg,e,tp,1,ft,s.rescon(ft1,ft2,ft3,ft4,ft),1,tp,HINTMSG_SPSUMMON)
    let rc = rescon(ft1, ft2, ft3, ft4);
    let rg = aux::select_unselect_group(d, &sg, Some(a.e), a.tp, Some(1), Some(ft as usize), Some(&rc), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // Duel.SpecialSummon(rg,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
    duel::special_summon(d, rg, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
}
/// `s.pencon(e,tp,eg,ep,ev,re,r,rp)`
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsFaceup()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)
}
/// `s.pentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    true
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.CheckPendulumZones(tp) then return end
    if !duel::check_pendulum_zones(d, a.tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
