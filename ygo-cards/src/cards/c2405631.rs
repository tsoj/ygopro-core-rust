//! Guoglim, Spear of the Ghoti
use super::prelude::*;

pub const ID: u32 = 2405631;

// local REASON_SYNCHRO_MATERIAL=REASON_MATERIAL+REASON_SYNCHRO
const REASON_SYNCHRO_MATERIAL: u32 = REASON_MATERIAL + REASON_SYNCHRO;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_FISH),1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_FISH, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner(None)),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.rmvtg)
    effect::set_target(d, e1, Some(cost_target(rmvtg)));
    // e1:SetOperation(s.rmvop)
    effect::set_operation(d, e1, Some(operation(rmvop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e2, EVENT_PHASE | PHASE_STANDBY);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(_,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(_,tp) return Duel.IsTurnPlayer(1-tp) end
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, 1 - a.tp)
}
// function s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetHandler():GetBattleTarget()
    let c = effect::get_handler(d, a.e).nil();
    let tc = card::get_battle_target(d, c);
    // if chk==0 then return tc and tc:IsControler(1-tp) and tc:IsAbleToRemove() end
    if a.chk == Some(0) {
        return match tc {
            Some(tc) => card::is_controler(d, tc, 1 - a.tp) && card::is_able_to_remove(d, tc, None, None, None),
            None => false,
        };
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
// function s.rmvop(e,tp,eg,ep,ev,re,r,rp)
fn rmvop(d: &mut Duel, a: &EffArgs) {
    // local tc=e:GetHandler():GetBattleTarget()
    let c = effect::get_handler(d, a.e).nil();
    // if tc and tc:IsRelateToBattle() and tc:IsControler(1-tp) then
    if let Some(tc) = card::get_battle_target(d, c) {
        if card::is_relate_to_battle(d, tc) && card::is_controler(d, tc, 1 - a.tp) {
            // Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)
            duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None);
        }
    }
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToRemove() end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, c, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(CardSet::from_card(c)), 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
// function s.mgfilter(c,e,tp,sync)
fn mgfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, sync: CardId) -> bool {
    // return c:IsControler(tp) and c:IsLocation(LOCATION_GRAVE)
    //   and (c:GetReason()&(REASON_SYNCHRO_MATERIAL))==(REASON_SYNCHRO_MATERIAL)
    //   and c:GetReasonCard()==sync and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_controler(d, c, tp)
        && card::is_location(d, c, LOCATION_GRAVE)
        && (card::get_reason(d, c) & REASON_SYNCHRO_MATERIAL) == REASON_SYNCHRO_MATERIAL
        && card::get_reason_card(d, c) == Some(sync)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,chk)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local mg=c:GetMaterial()
    let mg = card::get_material(d, c);
    // local ct=#mg
    let ct = mg.len();
    // local sumtype=c:GetSummonType()
    let sumtype = card::get_summon_type(d, c);
    // if Duel.Remove(c,POS_FACEUP,REASON_EFFECT)>0 and sumtype==SUMMON_TYPE_SYNCHRO
    //   and ct>0 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //   and ct<=Duel.GetLocationCount(tp,LOCATION_MZONE)
    //   and mg:FilterCount(aux.NecroValleyFilter(s.mgfilter),nil,e,tp,c)==ct
    //   and Duel.SelectYesNo(tp,aux.Stringid(id,2)) then
    if duel::remove(d, c, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0
        && sumtype == SUMMON_TYPE_SYNCHRO
        && ct > 0
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && (ct as i32) <= duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0
        && group::filter_count(d, &mg, &aux::necro_valley_filter(|d: &mut Duel, x: CardId| mgfilter(d, x, a.e, tp, c)), NO_EX) == ct
        && duel::select_yes_no(d, tp, aux::stringid(ID, 2))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.SpecialSummon(mg,0,tp,tp,false,false,POS_FACEUP)~=ct then return end
        if duel::special_summon(d, mg.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) != ct as i32 {
            return;
        }
        // for tc in mg:Iter() do
        for tc in group::iter(d, &mg) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3300)
            effect::set_description(d, e1, 3300);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
            effect::set_code(d, e1, EFFECT_LEAVE_FIELD_REDIRECT);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
            // e1:SetValue(LOCATION_REMOVED)
            effect::set_value(d, e1, LOCATION_REMOVED as i64);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, tc, e1, true);
        }
    }
}
