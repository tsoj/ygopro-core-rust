//! Giant Ballgame
use super::prelude::*;

pub const ID: u32 = 60619435;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // e0:SetCost(s.actreg)
    effect::set_cost(d, e0, Some(cost_target(actreg)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon 1 Level 6 or lower Insect monster from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():HasFlagEffect(id) end)
    effect::set_condition(
        d,
        e1,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::has_flag_effect(d, h, ID, None)
        })),
    );
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Switch control of 2 targets
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e2, CATEGORY_CONTROL);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.ctrltg)
    effect::set_target(d, e2, Some(cost_target(ctrltg)));
    // e2:SetOperation(s.ctrlop)
    effect::set_operation(d, e2, Some(operation(ctrlop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.actreg(e,tp,eg,ep,ev,re,r,rp,chk)
fn actreg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,EFFECT_FLAG_OATH,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, EFFECT_FLAG_OATH, 1, 0, 0);
    true
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsLevelBelow(6) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_INSECT)
        && card::is_level_below(d, c, 6)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(0xff))
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.rvlfilter(c,tp)
fn rvlfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRace(RACE_INSECT) and c:IsAttackAbove(1) and not c:IsPublic()
    //   and Duel.IsExistingTarget(s.filter1,tp,LOCATION_MZONE,0,1,nil,c:GetAttack())
    card::is_race(d, c, RACE_INSECT)
        && card::is_attack_above(d, c, 1)
        && !card::is_public(d, c)
        && {
            let atk = card::get_attack(d, c);
            duel::is_existing_target(d, Some(&|d, cc| filter1(d, cc, atk, None)), tp, LOCATION_MZONE, 0, 1, NO_EX)
        }
}
// function s.filter1(c,atk,e)
fn filter1(d: &mut Duel, c: CardId, atk: i32, e: Option<EffectId>) -> bool {
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return c:GetAttack()<atk and c:IsFaceup() and c:IsRace(RACE_INSECT) and c:IsAbleToChangeControler()
    //   and Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_CONTROL)>0
    //   and (not e or c:IsCanBeEffectTarget(e))
    card::get_attack(d, c) < atk
        && card::is_faceup(d, c)
        && card::is_race(d, c, RACE_INSECT)
        && card::is_able_to_change_controler(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
        && (e.is_none() || card::is_can_be_effect_target(d, c, e))
}
// function s.filter2(c,e)
fn filter2(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return c:IsFaceup() and c:IsAbleToChangeControler() and Duel.GetMZoneCount(tp,c,tp,LOCATION_REASON_CONTROL)>0
    //   and (not e or c:IsCanBeEffectTarget(e))
    card::is_faceup(d, c)
        && card::is_able_to_change_controler(d, c)
        && duel::get_m_zone_count(d, tp, c.into(), Some(tp), Some(LOCATION_REASON_CONTROL), None).0 > 0
        && (e.is_none() || card::is_can_be_effect_target(d, c, e))
}
// function s.rescon(sg,e,tp,mg)
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:FilterCount(Card.IsControler,nil,tp)==1
    let res = group::filter_count(d, sg, &|d, c| card::is_controler(d, c, tp), NO_EX) == 1;
    (res, false)
}
// function s.ctrltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn ctrltg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rvlfilter,tp,LOCATION_HAND,0,1,nil,tp)
    //   and Duel.IsExistingTarget(s.filter2,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| rvlfilter(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| filter2(d, c, None)), a.tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rvlfilter,tp,LOCATION_HAND,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, c| rvlfilter(d, c, a.tp)), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local atk=g:GetFirst():GetAttack()
    let first = group::get_first(d, &g).nil();
    let atk = card::get_attack(d, first);
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - a.tp, g.clone());
    // Duel.ShuffleHand(tp)
    duel::shuffle_hand(d, a.tp);
    // local g1=Duel.GetMatchingGroup(s.filter1,tp,LOCATION_MZONE,0,nil,atk,e)
    let g1 = duel::get_matching_group(d, Some(&|d, c| filter1(d, c, atk, Some(a.e))), a.tp, LOCATION_MZONE, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.filter2,tp,0,LOCATION_MZONE,nil,e)
    let g2 = duel::get_matching_group(d, Some(&|d, c| filter2(d, c, Some(a.e))), a.tp, 0, LOCATION_MZONE, NO_EX);
    // local tg=aux.SelectUnselectGroup(g1+g2,e,tp,2,2,s.rescon,1,tp,HINTMSG_CONTROL)
    let mut union = g1.clone();
    group::add_card(&mut union, &g2);
    let rs: aux::Rescon = &rescon;
    let tg = aux::select_unselect_group(d, &union, Some(a.e), a.tp, Some(2), Some(2), Some(rs), 1, a.tp, Some(HINTMSG_CONTROL as u64), None, None, false).unwrap_or_default();
    // Duel.SetTargetCard(tg)
    duel::set_target_card(d, tg.clone());
    // e:SetLabelObject(tg:Filter(Card.IsControler,nil,1-tp):GetFirst())
    let filtered = group::filter(d, &tg, &|d, c| card::is_controler(d, c, 1 - a.tp), NO_EX);
    let label = match group::get_first(d, &filtered) {
        Some(c) => Value::Card(c),
        None => Value::Nil,
    };
    effect::set_label_object(d, a.e, label);
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,tg,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(tg), 2, 0, 0);
    true
}
// function s.ctrlop(e,tp,eg,ep,ev,re,r,rp)
fn ctrlop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetTargetCards(e)
    let g = duel::get_target_cards(d, a.e).nil();
    // if #g~=2 then return end
    if g.len() != 2 {
        return;
    }
    // local a=g:GetFirst()
    // local b=g:GetNext()
    let order = group::iter(d, &g);
    let x = order[0];
    let y = order[1];
    // if Duel.SwapControl(a,b) then
    if duel::swap_control(d, CardSet::from_card(x), CardSet::from_card(y), 0, 0) {
        // local tc=e:GetLabelObject()
        let tc = effect::get_label_object(d, a.e).as_card().nil();
        // if tc:IsControler(tp) and not tc:IsRace(RACE_INSECT) and tc:IsFaceup() then
        if card::is_controler(d, tc, a.tp) && !card::is_race(d, tc, RACE_INSECT) && card::is_faceup(d, tc) {
            // Monster becomes Insect
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_CHANGE_RACE)
            effect::set_code(d, e1, EFFECT_CHANGE_RACE);
            // e1:SetValue(RACE_INSECT)
            effect::set_value(d, e1, RACE_INSECT as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
