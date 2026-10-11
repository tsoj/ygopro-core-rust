//! D/D/D/D Dimensional King Arc Crisis
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 71398055;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_DARK_CONTRACT, SET_DOOM_KING],
    miracle_synchro_fusion: true,
    ..ScriptInfo::DEFAULT
};

/// `s.matfilter(typ)`: a Fiend monster of the given type
fn mat(typ: u32) -> fusion::Mat {
    fusion::Mat::F(fusion::fus_mat(move |d, c, fc, st, tp| {
        // return c:IsRace(RACE_FIEND,fc,sumtype,tp) and c:IsType(typ,fc,sumtype,tp)
        card::is_race_ctx(d, c, RACE_FIEND, Some(fc), st, Some(tp)) && card::is_type_ctx(d, c, typ, Some(fc), st, Some(tp))
    }))
}

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum Summon procedure
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // Fusion Materials: 4 Fiend monsters (1 Fusion, 1 Synchro, 1 Xyz, 1 Pendulum)
    // Fusion.AddProcMix(c,true,true,s.matfilter(TYPE_FUSION),s.matfilter(TYPE_SYNCHRO),s.matfilter(TYPE_XYZ),s.matfilter(TYPE_PENDULUM))
    fusion::add_proc_mix(d, c, true, true, vec![mat(TYPE_FUSION), mat(TYPE_SYNCHRO), mat(TYPE_XYZ), mat(TYPE_PENDULUM)]);
    // c:AddMustBeFusionSummoned()
    card::add_must_be_fusion_summoned(d, c);
    // Special Summon this card (from your Extra Deck) by banishing the above materials from your field and/or GY
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,false,nil,1)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: None,
            sumtype: Some(1),
            ..Default::default()
        },
    );
    // You can only Fusion Summon or Special Summon by its alternate procedure "D/D/D/D Dimensional King Arc Crisis" once per turn
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    // e0:SetCondition(s.regcon)
    effect::set_condition(d, e0, Some(condition(regcon)));
    // e0:SetOperation(s.regop)
    effect::set_operation(d, e0, Some(operation(regop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Destroy "Dark Contract" cards you control to Special Summon "Doom King" monsters from your Deck/Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the effects of all face-up monsters your opponent currently controls
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e2, CATEGORY_DISABLE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Can attack all monsters your opponent controls once each
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_ATTACK_ALL)
    effect::set_code(d, e3, EFFECT_ATTACK_ALL);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Place this card in your Pendulum Zone
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
    // e4:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e4, 1, ID, 2, 0);
    // e4:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_MZONE) end)
    effect::set_condition(d, e4, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_MZONE)
    })));
    // e4:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return Duel.CheckPendulumZones(tp) end end)
    effect::set_target(d, e4, Some(cost_target(|d, a| {
        if a.chk == Some(0) {
            return duel::check_pendulum_zones(d, a.tp);
        }
        true
    })));
    // e4:SetOperation(s.penop)
    effect::set_operation(d, e4, Some(operation(penop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.contactfil(tp)`
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // local loc=LOCATION_MZONE|LOCATION_GRAVE
    let mut loc = LOCATION_MZONE | LOCATION_GRAVE;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then loc=LOCATION_MZONE end
    if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        loc = LOCATION_MZONE;
    }
    // return Duel.GetMatchingGroup(Card.IsAbleToRemoveAsCost,tp,loc,0,nil)
    duel::get_matching_group(d, Some(&|d, c| card::is_able_to_remove_as_cost(d, c, None)), tp, loc, 0, NO_EX)
}
/// `s.contactop(g)`
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    // Duel.Remove(g,POS_FACEUP,REASON_COST|REASON_MATERIAL)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST | REASON_MATERIAL, None, None);
}
/// `s.regcon(e)`
fn regcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsFusionSummoned() or c:IsSummonType(SUMMON_TYPE_SPECIAL+1)
    card::is_fusion_summoned(d, c) || card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL + 1])
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // Prevent another Fusion Summon or Special Summon by its alternate procedure that turn
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c,sump,sumtype) return c:IsOriginalCode(id) and (sumtype&SUMMON_TYPE_FUSION==SUMMON_TYPE_FUSION or sumtype&SUMMON_TYPE_SPECIAL+1==SUMMON_TYPE_SPECIAL+1) end)
    effect::set_target(d, e1, Some(lua_fn(regop_target)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE + PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// The target of the Summon prevention effect `function(e,c,sump,sumtype)`
fn regop_target(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    let sumtype = a.int(3) as u64;
    // c:IsOriginalCode(id) and (sumtype&SUMMON_TYPE_FUSION==SUMMON_TYPE_FUSION or sumtype&SUMMON_TYPE_SPECIAL+1==SUMMON_TYPE_SPECIAL+1)
    card::is_original_code(d, c, &[ID])
        && ((sumtype & SUMMON_TYPE_FUSION as u64) == SUMMON_TYPE_FUSION as u64
            || (sumtype & (SUMMON_TYPE_SPECIAL + 1) as u64) == (SUMMON_TYPE_SPECIAL + 1) as u64)
}
/// `s.spfilter(c,e,tp,dc)`: `dc` is a global this script never assigns, so it is nil
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // if not (c:IsSetCard(SET_DOOM_KING) and c:IsType(TYPE_PENDULUM) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    if !(card::is_set_card(d, c, &[SET_DOOM_KING as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
    {
        return false;
    }
    // return (c:IsLocation(LOCATION_DECK) and Duel.GetMZoneCount(tp,dc)>0)
    //     or (c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,tp,dc,c)>0)
    (card::is_location(d, c, LOCATION_DECK) && duel::get_m_zone_count(d, tp, NO_EX, None, None, None).0 > 0)
        || (card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0)
}
/// `s.desfilter(c,e,tp)`
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DARK_CONTRACT) and c:IsFaceup()
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp,dc)
    card::is_set_card(d, c, &[SET_DARK_CONTRACT as u16])
        && card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, x| spfilter(d, x, e, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)+Duel.GetLocationCountFromEx(tp,tp)
    let _ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0
        + duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::None, None).0;
    // if chkc then return chkc:IsControler(tp) and chkc:IsOnField() and chkc:IsFaceup() and s.desfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_on_field(d, chkc) && card::is_faceup(d, chkc) && desfilter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_ONFIELD,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| desfilter(d, x, a.e, tp)), tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // local ct=Duel.GetMatchingGroupCount(s.desfilter,tp,LOCATION_ONFIELD,0,nil,e,tp)
    let ct = duel::get_matching_group_count(d, Some(&|d, x| desfilter(d, x, a.e, tp)), tp, LOCATION_ONFIELD, 0, NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_ONFIELD,0,1,ct,nil,e,tp)
    let g = duel::select_target(d, tp, Some(&|d, x| desfilter(d, x, a.e, tp)), tp, LOCATION_ONFIELD, 0, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.rescon(mmz_ct,linkmz_ct)` returns `function(sg,e,tp,mg)`
fn make_rescon(mmz_ct: i32, linkmz_ct: i32) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| {
        // return sg:FilterCount(Card.IsLocation,nil,LOCATION_DECK)<=mmz_ct
        //     and sg:FilterCount(aux.FaceupFilter(Card.IsLocation,LOCATION_EXTRA),nil)<=linkmz_ct
        let deck = group::filter_count(d, sg, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX) as i64;
        let extra = group::filter_count(d, sg, &|d, c| card::is_faceup(d, c) && card::is_location(d, c, LOCATION_EXTRA), NO_EX) as i64;
        (deck <= mmz_ct as i64 && extra <= linkmz_ct as i64, false)
    }
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e);
    // if tg==0 then return end
    // (a group is never equal to the number 0 in Lua, so this never returns)
    // local ct=Duel.Destroy(tg,REASON_EFFECT)
    let ct = duel::destroy(d, tg.nil(), REASON_EFFECT, None, None);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local mmz_ct=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mmz_ct = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local emz_ct=Duel.GetLocationCountFromEx(tp,tp,nil,nil,ZONES_EMZ)
    let emz_ct = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::None, Some(ZONES_EMZ)).0;
    // local linkmz_ct=Duel.GetLocationCountFromEx(tp,tp)
    let linkmz_ct = duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::None, None).0;
    // local ft=math.min(mmz_ct+emz_ct,ct)
    let mut ft = (mmz_ct + emz_ct).min(ct);
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, a.e, tp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,s.rescon(mmz_ct,linkmz_ct),1,tp,HINTMSG_SPSUMMON)
    let rc = make_rescon(mmz_ct, linkmz_ct);
    let sg = aux::select_unselect_group(d, &g, Some(a.e), tp, Some(1), Some(ft as usize), Some(&rc), 1, tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local fup,fdown=sg:Split(aux.FaceupFilter(Card.IsLocation,LOCATION_EXTRA),nil)
    let (fup, fdown) = group::split(d, &sg, &|d, c| card::is_faceup(d, c) && card::is_location(d, c, LOCATION_EXTRA), NO_EX);
    // local fdown_main,fdown_ex=fdown:Split(Card.IsLocation,nil,LOCATION_DECK)
    let (fdown_main, fdown_ex) = group::split(d, &fdown, &|d, c| card::is_location(d, c, LOCATION_DECK), NO_EX);
    // local priority_0,priority_1
    // if linkmz_ct<mmz_ct then
    let (priority_0, priority_1) = if linkmz_ct < mmz_ct {
        // priority_0=fup
        // priority_1=fdown_main
        (fup, fdown_main)
    } else {
        // priority_0=fdown_main
        // priority_1=fup
        (fdown_main, fup)
    };
    // for prio0_c in priority_0:Iter() do
    for prio0_c in group::iter(d, &priority_0) {
        // Duel.SpecialSummonStep(prio0_c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, prio0_c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // for prio1_c in priority_1:Iter() do
    for prio1_c in group::iter(d, &priority_1) {
        duel::special_summon_step(d, prio1_c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // for fdown_ex_c in fdown_ex:Iter() do
    for fdown_ex_c in group::iter(d, &fdown_ex) {
        duel::special_summon_step(d, fdown_ex_c, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_negatable_monster(d, c)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_negatable_monster(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(g), n, tp, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(Card.IsNegatableMonster,tp,0,LOCATION_MZONE,nil):Filter(Card.IsCanBeDisabledByEffect,nil,e)
    let g0 = duel::get_matching_group(d, Some(&|d, c| card::is_negatable_monster(d, c)), tp, 0, LOCATION_MZONE, NO_EX);
    let g = group::filter(d, &g0, &|d, c| card::is_can_be_disabled_by_effect(d, c, a.e, true), NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in g:Iter() do
    for tc in group::iter(d, &g) {
        // tc:NegateEffects(c)
        card::negate_effects(d, tc, c, None, false, None);
    }
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
