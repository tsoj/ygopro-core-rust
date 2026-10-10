//! Tellarknight Ptolemaeus
//! 星守の騎士 プトレマイオス
use super::prelude::*;

pub const ID: u32 = 18326736;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER, SET_STELLARKNIGHT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, Some(xyz::MaxCt::Infinite), None, false, None);
    // Special Summon from your Extra Deck, 1 Xyz Monster that is 1 Rank higher than this card, except a "Number" monster, by using this face-up card you control as material
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
    // e1:SetCountLimit(1,0,EFFECT_COUNT_CODE_CHAIN)
    effect::set_count_limit(d, e1, 1, 0, 0, EFFECT_COUNT_CODE_CHAIN as u8);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(Cost.AND(Cost.DetachFromSelf(3),Cost.HintSelectedEffect))
    effect::set_cost(d, e1, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(3), None, None), cost::hint_selected_effect()])));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Skip your opponent's next turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(Cost.AND(Cost.DetachFromSelf(7),Cost.HintSelectedEffect))
    effect::set_cost(d, e2, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(7), None, None), cost::hint_selected_effect()])));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return not Duel.IsPlayerAffectedByEffect(1-tp,EFFECT_SKIP_TURN) end end)
    effect::set_target(d, e2, Some(cost_target(skip_target)));
    // e2:SetOperation(s.skipop)
    effect::set_operation(d, e2, Some(operation(skipop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Attach 1 "Stellarknight" card from your Extra Deck to this card as material
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.attachtg)
    effect::set_target(d, e3, Some(cost_target(attachtg)));
    // e3:SetOperation(s.attachop)
    effect::set_operation(d, e3, Some(operation(attachop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.spfilter(c,e,tp,mc,rk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mc: CardId, rk: u32) -> bool {
    // return c:IsRank(rk+1) and not c:IsSetCard(SET_NUMBER) and mc:IsCanBeXyzMaterial(c,tp)
    //     and Duel.GetLocationCountFromEx(tp,tp,mc,c)>0
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_XYZ,tp,false,false)
    card::is_rank(d, c, &[rk + 1])
        && !card::is_set_card(d, c, &[SET_NUMBER as u16])
        && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), mc.into(), duel::ExtraCard::Card(c), None).0 > 0
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_XYZ, tp, false, false, None, None, None)
}
/// `local pg=aux.GetMustBeMaterialGroup(tp,Group.FromCards(c),tp,nil,nil,REASON_XYZ)` followed by the test
/// `(#pg<=0 or (#pg==1 and pg:IsContains(c)))`
fn must_be_ok(d: &mut Duel, tp: u8, c: CardId) -> bool {
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::from_card(c)), tp, None, None, REASON_XYZ);
    pg.is_empty() || (pg.len() == 1 && group::is_contains(&pg, c))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then
    if a.chk == Some(0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // return (#pg<=0 or (#pg==1 and pg:IsContains(c))) and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,c,c:GetRank())
        let rk = card::get_rank(d, c) as u32;
        return must_be_ok(d, tp, c) && duel::is_existing_matching_card(d, Some(&|d, cc| spfilter(d, cc, a.e, tp, c, rk)), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_EXTRA as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not (c:IsRelateToEffect(e) and c:IsFaceup() and c:IsControler(tp)) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::is_controler(d, c, tp)) {
        return;
    }
    // if not (#pg<=0 or (#pg==1 and pg:IsContains(c))) then return end
    if !must_be_ok(d, tp, c) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,c,c:GetRank()):GetFirst()
    let rk = card::get_rank(d, c) as u32;
    let sg = duel::select_matching_card(d, tp, Some(&|d, cc| spfilter(d, cc, a.e, tp, c, rk)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sg);
    // if sc then
    if let Some(sc) = sc {
        // sc:SetMaterial(c)
        card::set_material(d, sc, CardSet::from_card(c));
        // Duel.Overlay(sc,c)
        duel::overlay(d, sc, c, false);
        // if Duel.SpecialSummon(sc,SUMMON_TYPE_XYZ,tp,tp,false,false,POS_FACEUP)==0 then return end
        if duel::special_summon(d, sc, SUMMON_TYPE_XYZ, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
            return;
        }
        // sc:CompleteProcedure()
        card::complete_procedure(d, sc);
    }
}
/// `function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return not Duel.IsPlayerAffectedByEffect(1-tp,EFFECT_SKIP_TURN) end end`
fn skip_target(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        return duel::is_player_affected_by_effect(d, 1 - a.tp, EFFECT_SKIP_TURN).is_none();
    }
    true
}
/// `s.skipop(e,tp,eg,ep,ev,re,r,rp)`
fn skipop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_SKIP_TURN)
    effect::set_code(d, e1, EFFECT_SKIP_TURN);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetCondition(function(e) return Duel.GetTurnPlayer()~=e:GetHandlerPlayer() end)
    effect::set_condition(d, e1, Some(condition(skip_condition)));
    // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `function(e) return Duel.GetTurnPlayer()~=e:GetHandlerPlayer() end`
fn skip_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = duel::get_turn_player(d);
    tp != effect::get_handler_player(d, a.e)
}
/// `s.attachfilter(c,xyz,tp)`: the Lua body reads the global `xyzc`, which is nil, so the
/// `IsCanBeXyzMaterial` call gets no card; `tp` is the player argument as passed.
fn attachfilter(d: &mut Duel, c: CardId, tp: Option<u8>) -> bool {
    // return c:IsSetCard(SET_STELLARKNIGHT) and c:IsCanBeXyzMaterial(xyzc,tp,REASON_EFFECT)
    card::is_set_card(d, c, &[SET_STELLARKNIGHT as u16]) && card::is_can_be_xyz_material(d, c, None, tp, Some(REASON_EFFECT))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.attachfilter,tp,LOCATION_EXTRA,0,1,nil,e:GetHandler(),tp) end
    // (the handler is the extra argument: the filter ignores it, so it is not passed on)
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| attachfilter(d, c, Some(tp))), tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
    // local g=Duel.SelectMatchingCard(tp,s.attachfilter,tp,LOCATION_EXTRA,0,1,1,nil)
    // (no extra arguments: the filter gets tp = nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, cc| attachfilter(d, cc, None)), tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Overlay(c,g)
        duel::overlay(d, c, g, false);
    }
}
