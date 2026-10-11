//! Ninjitsu Art Notebook of Mystery
use super::prelude::*;

pub const ID: u32 = 95545183;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NINJA, SET_NINJITSU_ART], listed_names: &[ID], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Set 1 "Ninjitsu Art" Spell/Trap and/or 1 "Ninja" monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e1:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>0 end)
    effect::set_condition(d, e1, Some(lua_fn(e1_cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(operation_fn)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Change 1 face-up monster to face-down Defense Position
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_POSITION + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.poscond)
    effect::set_condition(d, e2, Some(condition(poscond)));
    // e2:SetTarget(s.postg)
    effect::set_target(d, e2, Some(cost_target(postg)));
    // e2:SetOperation(s.posop)
    effect::set_operation(d, e2, Some(operation(posop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.ninjitsu(c,zone_chk)`
fn ninjitsu(d: &mut Duel, c: CardId, zone_chk: bool) -> bool {
    // return c:IsSetCard(SET_NINJITSU_ART) and c:IsSpellTrap() and c:IsSSetable() and not c:IsCode(id) and (zone_chk or c:IsType(TYPE_FIELD))
    card::is_set_card(d, c, &[SET_NINJITSU_ART as u16])
        && card::is_spell_trap(d, c)
        && card::is_s_setable(d, c, false, None)
        && !card::is_code(d, c, &[ID])
        && (zone_chk || card::is_type(d, c, TYPE_FIELD))
}
/// `s.ninja(c,e,tp)`
fn ninja(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NINJA) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEDOWN_DEFENSE) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_NINJA as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEDOWN_DEFENSE as u8), None, None)
        && !card::is_code(d, c, &[ID])
}
/// `function(e,tp) return Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)>0 end`
fn e1_cond(d: &mut Duel, a: Args) -> bool {
    let tp = a.p(1);
    duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD) > 0
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local mzones=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mzones = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // local stzones=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut stzones = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then stzones=stzones-1 end
    let h = effect::get_handler(d, a.e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        stzones -= 1;
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.ninjitsu,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,stzones>0)
    //     or (mzones>0 and Duel.IsExistingMatchingCard(s.ninja,tp,LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp)) end
    if a.chk == Some(0) {
        let sz = stzones > 0;
        let (e, tp) = (a.e, a.tp);
        return duel::is_existing_matching_card(d, Some(&|d, c| ninjitsu(d, c, sz)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX)
            || (mzones > 0 && duel::is_existing_matching_card(d, Some(&|d, c| ninja(d, c, e, tp)), tp, LOCATION_DECK | LOCATION_GRAVE, 0, 1, NO_EX));
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.rescon(sg,e,tp,mg)`
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetLocation)==#sg
    //     and (sg:FilterCount(s.ninjitsu,nil,true)==1 or sg:FilterCount(s.ninja,nil,e,tp)==1)
    let e = e.nil();
    let class_count = group::get_class_count(d, sg, &|d, c| card::get_location(d, c) as i64);
    let res = class_count == group::get_count(sg)
        && (group::filter_count(d, sg, &|d, c| ninjitsu(d, c, true), NO_EX) == 1
            || group::filter_count(d, sg, &|d, c| ninja(d, c, e, tp), NO_EX) == 1);
    (res, false)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn operation_fn(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local mzones=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mzones = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local g1=Group.CreateGroup()
    let mut g1 = CardSet::new();
    // if mzones>0 then
    if mzones > 0 {
        // g1=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.ninja),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil,e,tp)
        let nv = aux::necro_valley_filter(|d, c| ninja(d, c, e, tp));
        g1 = duel::get_matching_group(d, Some(&nv), tp, LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    }
    // local g2=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.ninjitsu),tp,LOCATION_DECK|LOCATION_GRAVE,0,nil,true)
    let nv2 = aux::necro_valley_filter(|d, c| ninjitsu(d, c, true));
    let g2 = duel::get_matching_group(d, Some(&nv2), tp, LOCATION_DECK | LOCATION_GRAVE, 0, NO_EX);
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // if #g1==0 then return end
    if g1.is_empty() {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g1,e,tp,1,2,s.rescon,1,tp,HINTMSG_TOFIELD)
    let sg = aux::select_unselect_group(d, &g1, Some(e), tp, Some(1), Some(2), Some(&rescon), 1, tp, Some(HINTMSG_TOFIELD as u64), None, None, false).unwrap_or_default();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // for tc in sg:Iter() do
    for tc in group::iter(d, &sg) {
        // if tc:IsSpellTrap() then
        if card::is_spell_trap(d, tc) {
            // Duel.SSet(tp,tc,tp,false)
            duel::s_set(d, tp, tc, Some(tp), false);
        } else {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEDOWN_DEFENSE)
            duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEDOWN_DEFENSE as u8, None);
        }
    }
    // Duel.ConfirmCards(1-tp,sg)
    duel::confirm_cards(d, 1 - tp, sg);
}
/// `s.poscond(e,tp,eg,ep,ev,re,r,rp)`
fn poscond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousPosition(POS_FACEDOWN)
    card::is_previous_location(d, c, LOCATION_ONFIELD) && card::is_previous_position(d, c, POS_FACEDOWN)
}
/// `s.postg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn postg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsCanTurnSet() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_can_turn_set(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_can_turn_set(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // local g=Duel.SelectTarget(tp,Card.IsCanTurnSet,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_can_turn_set(d, c)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,1,0,POS_FACEDOWN_DEFENSE)
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), 1, 0, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.posop(e,tp,eg,ep,ev,re,r,rp)`
fn posop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // Duel.ChangePosition(tc,POS_FACEDOWN_DEFENSE)
        duel::change_position(d, tc, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
    }
}
