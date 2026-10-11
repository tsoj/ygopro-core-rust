//! Conductor of Nephthys
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 8454126;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[23459650, ID],
    listed_series: &[SET_NEPHTHYS],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon 1 "Nephthys" Ritual from your hand/Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsRitualSummoned() end)
    effect::set_condition(d, e1, Some(condition(econd1)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Register if Tributed by a "Nephthys" card's effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2, EVENT_RELEASE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.regcond)
    effect::set_condition(d, e2, Some(condition(regcond)));
    // e2:SetTarget(s.regtg)
    effect::set_target(d, e2, Some(cost_target(regtg)));
    // e2:SetOperation(s.regop)
    effect::set_operation(d, e2, Some(operation(regop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Register if destroyed by a "Nephthys" card's effect
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// e1:SetCondition(function(e) return e:GetHandler():IsRitualSummoned() end)
fn econd1(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_ritual_summoned(d, h)
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // local pg=aux.GetMustBeMaterialGroup(tp,Group.CreateGroup(),tp,c,nil,REASON_RITUAL)
    let pg = aux::get_must_be_material_group_opt(d, tp, Some(&CardSet::new()), tp, Some(c), None, REASON_RITUAL);
    // return #pg<=0 and c:IsSetCard(SET_NEPHTHYS) and c:IsRitualMonster() and not c:IsCode(id)
    //     and c:IsCanBeSpecialSummoned(e,SUMMON_TYPE_RITUAL,tp,true,false,POS_FACEUP)
    pg.is_empty()
        && card::is_set_card(d, c, &[SET_NEPHTHYS as u16])
        && card::is_ritual_monster(d, c)
        && !card::is_code(d, c, &[ID])
        && card::is_can_be_special_summoned(d, c, e, SUMMON_TYPE_RITUAL, tp, true, false, Some(POS_FACEUP as u8), None, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummon(tc,SUMMON_TYPE_RITUAL,tp,tp,true,false,POS_FACEUP)>0 then
    if let Some(tc) = tc {
        if duel::special_summon(d, CardSet::from_card(tc), SUMMON_TYPE_RITUAL, a.tp, a.tp, true, false, POS_FACEUP as u8, None) > 0 {
            // tc:CompleteProcedure()
            card::complete_procedure(d, tc);
        }
    }
}
// function s.regcond(e,tp,eg,ep,ev,re,r,rp)
fn regcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return re and re:GetHandler():IsSetCard(SET_NEPHTHYS) and r&REASON_EFFECT>0
    match a.re {
        Some(re) => {
            let h = effect::get_handler(d, re).nil();
            card::is_set_card(d, h, &[SET_NEPHTHYS as u16]) && (a.r & REASON_EFFECT) > 0
        }
        None => false,
    }
}
// function s.regtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn regtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, a.tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD) as i32);
    true
}
// function s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // local ct=Duel.IsPhase(PHASE_STANDBY) and 2 or 1
    let ct: i64 = if duel::is_phase(d, PHASE_STANDBY) { 2 } else { 1 };
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.descond)
    effect::set_condition(d, e1, Some(condition(descond)));
    // e1:SetOperation(s.desop)
    effect::set_operation(d, e1, Some(operation(desop)));
    // e1:SetLabel(ct,Duel.GetTurnCount())
    let turn = duel::get_turn_count(d, None) as i64;
    effect::set_label(d, e1, &[ct, turn]);
    // e1:SetReset(RESET_PHASE|PHASE_STANDBY,ct)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_STANDBY, ct as u16);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
// function s.desfilter(c,e)
fn desfilter(d: &mut Duel, c: CardId, e: EffectId) -> bool {
    // return c:IsSetCard(SET_NEPHTHYS) and c:IsDestructable(e) and not c:IsRitualMonster() and (c:IsFaceup() or not c:IsLocation(LOCATION_ONFIELD))
    card::is_set_card(d, c, &[SET_NEPHTHYS as u16])
        && card::is_destructable(d, c, Some(e))
        && !card::is_ritual_monster(d, c)
        && (card::is_faceup(d, c) || !card::is_location(d, c, LOCATION_ONFIELD))
}
// function s.descond(e,tp,eg,ep,ev,re,r,rp)
fn descond(d: &mut Duel, a: &EffArgs) -> bool {
    // local sp_label,turn=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let sp_label = labels.first().copied().unwrap_or(0);
    let turn = labels.get(1).copied().unwrap_or(0);
    // return (sp_label==1 or turn~=Duel.GetTurnCount()) and Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_ONFIELD,0,1,nil,e)
    let first = sp_label == 1 || turn != duel::get_turn_count(d, None) as i64;
    first
        && duel::is_existing_matching_card(d, Some(&|d, c| desfilter(d, c, a.e)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD, 0, 1, NO_EX)
}
// function s.descheck(sg,e,tp,mg)
fn descheck(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // local res=sg:GetClassCount(Card.GetLocation)==#sg
    let res = group::get_class_count(d, sg, &|d, c| card::get_location(d, c) as i64) == sg.len();
    // return res,not res
    (res, !res)
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // local sg=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_ONFIELD,0,nil,e)
    let sg = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, a.e)), a.tp, LOCATION_HAND | LOCATION_DECK | LOCATION_ONFIELD, 0, NO_EX);
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local rg=aux.SelectUnselectGroup(sg,e,tp,1,3,s.descheck,1,tp,HINTMSG_DESTROY)
    let rs: aux::Rescon = &descheck;
    let rg = aux::select_unselect_group(d, &sg, Some(a.e), a.tp, Some(1), Some(3), Some(rs), 1, a.tp, Some(HINTMSG_DESTROY as u64), None, None, false)
        .unwrap_or_default();
    // Duel.Destroy(rg,REASON_EFFECT)
    duel::destroy(d, rg, REASON_EFFECT, None, None);
    // e:Reset()
    effect::reset(d, a.e);
}
