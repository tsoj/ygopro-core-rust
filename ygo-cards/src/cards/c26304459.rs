//! Ancient Flamvell Deity
//! --エンシェント・ゴッド・フレムベル
use super::prelude::*;

pub const ID: u32 = 26304459;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // synchro summon
    // Synchro.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsAttribute,ATTRIBUTE_FIRE),1,1,Synchro.NonTunerEx(Card.IsRace,RACE_PYRO),1,99)
    synchro::add_procedure(
        d,
        c,
        Some(mat_filter(|d, c, sc, st, tp| card::is_attribute_ctx(d, c, ATTRIBUTE_FIRE, sc, st, Some(tp)))),
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_PYRO, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // remove
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(s.remcon)
    effect::set_condition(d, e1, Some(condition(remcon)));
    // e1:SetTarget(s.remtg)
    effect::set_target(d, e1, Some(cost_target(remtg)));
    // e1:SetOperation(s.remop)
    effect::set_operation(d, e1, Some(operation(remop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn remcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned()
    let c = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, c)
}
fn remtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // if Duel.IsPlayerAffectedByEffect(1-tp,CARD_SPIRIT_ELIMINATION) then
    if duel::is_player_affected_by_effect(d, 1 - a.tp, CARD_SPIRIT_ELIMINATION).is_some() {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_MZONE|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, (LOCATION_MZONE | LOCATION_GRAVE) as i32);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,1-tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, 1 - a.tp, LOCATION_GRAVE as i32);
    }
    true
}
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and aux.SpElimFilter(c)
    card::is_able_to_remove(d, c, None, None, None) && aux::sp_elim_filter(d, c, false, false)
}
fn remop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ht=Duel.GetFieldGroupCount(tp,0,LOCATION_HAND)
    let ht = duel::get_field_group_count(d, tp, 0, LOCATION_HAND);
    // if ht==0 then return end
    if ht == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local rg=Duel.SelectMatchingCard(tp,s.rmfilter,tp,0,LOCATION_MZONE|LOCATION_GRAVE,1,ht,nil)
    let rg = duel::select_matching_card(
        d,
        tp,
        Some(&rmfilter),
        tp,
        0,
        LOCATION_MZONE | LOCATION_GRAVE,
        1,
        ht as u16,
        false,
        NO_EX,
    )
    .nil();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if #rg>0 then
    if !rg.is_empty() {
        let n = rg.len();
        // Duel.Remove(rg,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, rg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
        if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
        // e1:SetValue(#rg*200)
        effect::set_value(d, e1, (n * 200) as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD_DISABLE, 1);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
