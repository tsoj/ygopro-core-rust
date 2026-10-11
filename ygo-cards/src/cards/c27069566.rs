//! Virtual World Phoenix - Fanfan
//! Scripted by Naim
use super::prelude::*;

pub const ID: u32 = 27069566;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VIRTUAL_WORLD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,6,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(6), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Banish 2 targets
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCost(Cost.DetachFromSelf(2))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(2), None, None)));
    // e1:SetTarget(s.rmvtg)
    effect::set_target(d, e1, Some(cost_target(rmvtg)));
    // e1:SetOperation(s.rmvop)
    effect::set_operation(d, e1, Some(operation(rmvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 2 "Datascape"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsAbleToRemove),tp,0,LOCATION_ONFIELD,1,nil)
    //     and Duel.IsExistingTarget(Card.IsAbleToRemove,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&faceup_removable), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g1=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsAbleToRemove),tp,0,LOCATION_ONFIELD,1,1,nil)
    let mut g1 = duel::select_target(d, a.tp, Some(&faceup_removable), a.tp, 0, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g2=Duel.SelectTarget(tp,Card.IsAbleToRemove,tp,LOCATION_GRAVE,LOCATION_GRAVE,1,1,nil)
    let g2 = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_remove(d, c, None, None, None)), a.tp, LOCATION_GRAVE, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // g1:Merge(g2)
    group::add_card(&mut g1, &g2);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g1,2,0,LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g1), 2, 0, (LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
fn faceup_removable(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
fn rmvop(d: &mut Duel, a: &EffArgs) {
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e);
    // if tg and #tg>0 then Duel.Remove(tg,POS_FACEUP,REASON_EFFECT) end
    if let Some(tg) = tg {
        if !tg.is_empty() {
            duel::remove(d, &tg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousControler(tp) and c:IsPreviousLocation(LOCATION_MZONE)
    //     and (c:IsReason(REASON_EFFECT) or (c:IsReason(REASON_BATTLE) and Duel.GetAttacker():IsControler(1-tp)))
    //     and c:IsXyzSummoned() and rp==1-tp
    card::is_previous_controler(d, c, a.tp)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && (card::is_reason(d, c, REASON_EFFECT) || (card::is_reason(d, c, REASON_BATTLE) && card::is_controler(d, duel::get_attacker(d).nil(), 1 - a.tp)))
        && card::is_xyz_summoned(d, c)
        && a.rp == 1 - a.tp
}
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    card::is_set_card(d, c, &[SET_VIRTUAL_WORLD as u16])
        && card::is_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP as u8), None, None)
}
fn rescon(d: &mut Duel, sg: &CardSet, _e: Option<EffectId>, _tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    // return sg:GetClassCount(Card.GetRace)==1 and sg:GetClassCount(Card.GetAttribute)==1
    let res = group::get_class_count(d, sg, &|d, c| card::get_race(d, c) as i64) == 1
        && group::get_class_count(d, sg, &|d, c| card::get_attribute(d, c) as i64) == 1;
    (res, false)
}
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return false end
        if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
            return false;
        }
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return false end
        if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2 {
            return false;
        }
        // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
        let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
        // return aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,0)
        return matches!(
            aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 0, a.tp, None, None, None, false),
            Err(true)
        );
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, LOCATION_DECK as i32);
    true
}
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2
    {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_DECK,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_DECK, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,2,s.rescon,1,tp,HINTMSG_SPSUMMON)
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(2), Some(2), Some(&rescon), 1, a.tp, Some(HINTMSG_SPSUMMON as u64), None, None, false).unwrap_or_default();
    // if sg and #sg==2 then Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP) end
    if sg.len() == 2 {
        duel::special_summon(d, &sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
