//! Concours de Cuisine (Culinary Confrontation)
use super::prelude::*;

pub const ID: u32 = 14283055;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NOUVELLES, SET_PATISSCIEL, SET_RECIPE], ..ScriptInfo::DEFAULT };

// local locs=LOCATION_HAND|LOCATION_DECK|LOCATION_EXTRA
const LOCS: u32 = LOCATION_HAND | LOCATION_DECK | LOCATION_EXTRA;

// Sumtypes for aux.cannotmatfilter
const SUMTYPES: [u32; 4] = [SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK];

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 "Nouvelles" Pendulum Monster and 1 "Patissciel" Pendulum Monster from your hand, Deck, and/or Extra Deck
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
    // e1:SetHintTiming(0,TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase the ATK of a monster by 200 x the number of "Recipe" cards in the GYs
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e2, CATEGORY_ATKCHANGE);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c,archetyp)
fn cfilter(d: &mut Duel, c: CardId, archetyp: u32) -> bool {
    // return c:IsSetCard(archetyp) and c:IsType(TYPE_PENDULUM)
    card::is_set_card(d, c, &[archetyp as u16]) && card::is_type(d, c, TYPE_PENDULUM)
}
// function s.firstsummon(c,e,tp,sg)
fn firstsummon(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sg: &CardSet) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and ((not c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
    //     or Duel.GetLocationCountFromEx(tp,tp,nil,c)>0)
    //     and sg:IsExists(s.secondsummon,1,c,e,tp) --exclude 'c'
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && ((!card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0)
            || duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0)
        && group::is_exists(d, sg, &|d: &mut Duel, x: CardId| secondsummon(d, x, e, tp), 1, c.into())
}
// function s.secondsummon(c,e,tp)
fn secondsummon(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
    //     and ((not c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0)
    //     or Duel.GetLocationCountFromEx(1-tp,tp,nil,c)>0)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None)
        && ((!card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, Some(tp), None, None).0 > 0)
            || duel::get_location_count_from_ex(d, 1 - tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0)
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    // return sg:FilterCount(Card.IsSetCard,nil,SET_PATISSCIEL)==1
    //     and sg:FilterCount(Card.IsSetCard,nil,SET_NOUVELLES)==1
    //     and sg:IsExists(s.firstsummon,1,nil,e,tp,sg)
    let res = group::filter_count(d, sg, &|d: &mut Duel, c: CardId| card::is_set_card(d, c, &[SET_PATISSCIEL as u16]), NO_EX) == 1
        && group::filter_count(d, sg, &|d: &mut Duel, c: CardId| card::is_set_card(d, c, &[SET_NOUVELLES as u16]), NO_EX) == 1
        && group::is_exists(d, sg, &|d: &mut Duel, c: CardId| firstsummon(d, c, e, tp, sg), 1, NO_EX);
    (res, false)
}
// function s.target(e,tp,eg,ep,ev,re,r,rp,chk)
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local g1=Duel.GetMatchingGroup(s.cfilter,tp,locs,0,nil,SET_NOUVELLES)
    let g1 = duel::get_matching_group(d, Some(&|d: &mut Duel, c: CardId| cfilter(d, c, SET_NOUVELLES as u32)), a.tp, LOCS, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.cfilter,tp,locs,0,nil,SET_PATISSCIEL)
    let g2 = duel::get_matching_group(d, Some(&|d: &mut Duel, c: CardId| cfilter(d, c, SET_PATISSCIEL as u32)), a.tp, LOCS, 0, NO_EX);
    // local g=g1+g2
    let mut g = g1.clone();
    group::add_card(&mut g, &g2);
    // if chk==0 then return #g1>0 and #g2>0
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0) end
    if a.chk == Some(0) {
        return g1.len() > 0
            && g2.len() > 0
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT as u32).is_none()
            && matches!(
                aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
                Err(true)
            );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,locs)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCS as i32);
    true
}
// function s.sumlimit(e,sumc,sumtype,sumplayer)
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // if not sumc then return false end
    let Some(sumc) = a.card(1) else {
        return false;
    };
    // local tp=e:GetHandlerPlayer()
    let e = a.e(0);
    let tp = effect::get_handler_player(d, e);
    // return sumplayer==tp and sumc:IsControler(tp)
    //     and aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK)(e,sumc,sumtype,sumplayer)
    a.int(3) == tp as i64
        && card::is_controler(d, sumc, tp)
        && aux::cannotmatfilter(&SUMTYPES)
            .call(d, &[Value::Effect(e), Value::Card(sumc), Value::Int(a.int(2)), Value::Int(a.int(3))])
            .truthy()
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT as u32).is_some() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // --You cannot use monsters as material for a Fusion, Synchro, Xyz, or Link Summon, except "Nouvelles" and "Patissciel" monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_MATERIAL);
    // e1:SetTargetRange(LOCATION_ALL,LOCATION_ALL)
    effect::set_target_range(d, e1, LOCATION_ALL, LOCATION_ALL);
    // e1:SetTarget(function(e,c) return not c:IsSetCard({SET_NOUVELLES,SET_PATISSCIEL}) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_NOUVELLES as u16, SET_PATISSCIEL as u16]))));
    // e1:SetValue(s.sumlimit)
    effect::set_value_fn(d, e1, lua_fn(sumlimit));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(c,nil,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // local g1=Duel.GetMatchingGroup(s.cfilter,tp,locs,0,nil,SET_NOUVELLES)
    let g1 = duel::get_matching_group(d, Some(&|d: &mut Duel, c: CardId| cfilter(d, c, SET_NOUVELLES as u32)), a.tp, LOCS, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.cfilter,tp,locs,0,nil,SET_PATISSCIEL)
    let g2 = duel::get_matching_group(d, Some(&|d: &mut Duel, c: CardId| cfilter(d, c, SET_PATISSCIEL as u32)), a.tp, LOCS, 0, NO_EX);
    // if #g1==0 or #g2==0 then return end
    if g1.len() == 0 || g2.len() == 0 {
        return;
    }
    // local g=g1+g2
    let mut g = g1.clone();
    group::add_card(&mut g, &g2);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_SPSUMMON)
    let mut sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false)
        .unwrap_or_default();
    // if #sg~=2 then return end
    if sg.len() != 2 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 3));
    // local sc1=sg:FilterSelect(tp,s.firstsummon,1,1,nil,e,tp,sg):GetFirst()
    let sel = group::filter_select(d, &sg, a.tp, &|d: &mut Duel, c: CardId| firstsummon(d, c, a.e, a.tp, &sg), 1, 1, false, NO_EX).nil();
    let sc1 = group::get_first(d, &sel).nil();
    // local sc2=sg:RemoveCard(sc1):GetFirst()
    group::remove_card(&mut sg, &CardSet::from_card(sc1));
    let sc2 = group::get_first(d, &sg).nil();
    // Duel.SpecialSummonStep(sc1,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon_step(d, sc1, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    // Duel.SpecialSummonStep(sc2,0,tp,1-tp,false,false,POS_FACEUP)
    duel::special_summon_step(d, sc2, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP as u8, None);
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local ct=Duel.GetMatchingGroupCount(Card.IsSetCard,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,SET_RECIPE)
    let ct = duel::get_matching_group_count(d, Some(&|d: &mut Duel, c: CardId| card::is_set_card(d, c, &[SET_RECIPE as u16])), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_faceup(d, chkc);
    }
    // if chk==0 then return ct>0 and Duel.IsExistingTarget(Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return ct > 0 && duel::is_existing_target(d, Some(&|d: &mut Duel, c: CardId| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,Card.IsFaceup,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    duel::select_target(d, a.tp, Some(&|d: &mut Duel, c: CardId| card::is_faceup(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    true
}
// function s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.GetMatchingGroupCount(Card.IsSetCard,tp,LOCATION_GRAVE,LOCATION_GRAVE,nil,SET_RECIPE)
    let ct = duel::get_matching_group_count(d, Some(&|d: &mut Duel, c: CardId| card::is_set_card(d, c, &[SET_RECIPE as u16])), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, NO_EX);
    // if ct==0 then return end
    if ct == 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc.nil(), a.e) && card::is_faceup(d, tc.nil()) {
        // tc:UpdateAttack(ct*200,RESET_EVENT|RESETS_STANDARD,e:GetHandler())
        let handler = effect::get_handler(d, a.e).nil();
        card::update_attack(d, tc.nil(), ct as i64 * 200, Some(RESET_EVENT | RESETS_STANDARD), Some(handler), None);
    }
}
