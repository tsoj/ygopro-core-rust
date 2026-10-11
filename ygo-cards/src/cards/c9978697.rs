//! Ｘ－セイバー ペリナ
//! X-Saber Pellina
//! Scripted by the Razgriz
use super::prelude::*;

pub const ID: u32 = 9978697;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_X_SABER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 2 "X-Saber" monsters from your Deck, except "X-Saber Perina", but destroy them during the End Phase
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY)
    effect::set_category(d, e1a, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SUMMON_SUCCESS);
    // e1a:SetCountLimit(1,id)
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e1a, Some(cost::self_to_grave()));
    // e1a:SetTarget(s.sptg)
    effect::set_target(d, e1a, Some(cost_target(sptg)));
    // e1a:SetOperation(s.spop)
    effect::set_operation(d, e1a, Some(operation(spop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // Apply an effect to the "X-Saber" Synchro Monster that used this card as material
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_BE_MATERIAL)
    effect::set_code(d, e2, EVENT_BE_MATERIAL);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.effcon)
    effect::set_condition(d, e2, Some(condition(effcon)));
    // e2:SetTarget(s.efftg)
    effect::set_target(d, e2, Some(cost_target(efftg)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_X_SABER) and c:IsCanBeSpecialSummoned(e,0,tp,false,false) and not c:IsCode(id)
    card::is_set_card(d, c, &[SET_X_SABER as u16])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && !card::is_code(d, c, &[ID])
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.GetMZoneCount(tp,e:GetHandler())>=2
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK,0,2,nil,e,tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_m_zone_count(d, tp, h.into(), None, None, None).0 >= 2
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 2, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,2,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 2, tp, LOCATION_MZONE as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(tp,LOCATION_MZONE)<2 then return end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 2
    {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK,0,2,2,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_DECK, 0, 2, 2, false, NO_EX).nil();
    // if #g==2 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)==2 then
    if g.len() == 2 && duel::special_summon(d, g.clone(), 0, tp, tp, false, false, POS_FACEUP as u8, None) == 2 {
        // Destroy them during the End Phase
        // aux.DelayedOperation(g,PHASE_END,id,e,tp,function(ag) Duel.Destroy(ag,REASON_EFFECT) end,nil,0,0,aux.Stringid(id,2))
        aux::delayed_operation(
            d,
            g,
            PHASE_END,
            ID,
            a.e,
            tp,
            Some(std::sync::Arc::new(|d: &mut Duel, ag: &CardSet, _a: &EffArgs| {
                duel::destroy(d, ag.clone(), REASON_EFFECT, None, None);
                true
            })),
            None,
            Some(0),
            Some(0),
            Some(aux::stringid(ID, 2)),
            None,
        );
    }
}
// function s.effcon(e,tp,eg,ep,ev,re,r,rp)
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if not r==REASON_SYNCHRO then return false end
    // (Lua: `not r` is a boolean, so `(not r)==REASON_SYNCHRO` is always false and the line never returns)
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local sync=c:GetReasonCard()
    let sync = card::get_reason_card(d, c);
    // if not (c:IsLocation(LOCATION_GRAVE) and sync:IsSetCard(SET_X_SABER)) then return false end
    if !(card::is_location(d, c, LOCATION_GRAVE) && card::is_set_card(d, sync.nil(), &[SET_X_SABER as u16])) {
        return false;
    }
    let sync = sync.nil();
    // e:SetLabelObject(sync)
    effect::set_label_object(d, a.e, Value::Card(sync));
    // sync:RegisterFlagEffect(id,RESET_EVENT|(RESETS_STANDARD&~RESET_TURN_SET),0,1)
    card::register_flag_effect(d, sync, ID, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 0, 1, 0, 0);
    // return true
    true
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    // local sync=e:GetLabelObject()
    let sync = effect::get_label_object(d, a.e).as_card().nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // return sync:HasFlagEffect(id) and not (sync:IsHasEffect(EFFECT_EXTRA_ATTACK) and sync:IsHasEffect(EFFECT_DIRECT_ATTACK))
        return card::has_flag_effect(d, sync, ID, None)
            && !(!card::is_has_effect(d, sync, EFFECT_EXTRA_ATTACK, None).is_empty()
                && !card::is_has_effect(d, sync, EFFECT_DIRECT_ATTACK, None).is_empty());
    }
    // sync:CreateEffectRelation(e)
    card::create_effect_relation(d, sync, a.e);
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local sync=e:GetLabelObject()
    let sync = effect::get_label_object(d, a.e).as_card().nil();
    // if not sync:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, sync, a.e) {
        return;
    }
    // local b1=not sync:IsHasEffect(EFFECT_EXTRA_ATTACK)
    let b1 = card::is_has_effect(d, sync, EFFECT_EXTRA_ATTACK, None).is_empty();
    // local b2=not sync:IsHasEffect(EFFECT_DIRECT_ATTACK)
    let b2 = card::is_has_effect(d, sync, EFFECT_DIRECT_ATTACK, None).is_empty();
    // if not (b1 or b2) then return end
    if !(b1 || b2) {
        return;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,3)},
    //     {b2,aux.Stringid(id,4)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
    // local desc=op==1 and 3201 or 3205
    let desc: u64 = if op == Some(1) { 3201 } else { 3205 };
    // local eff_code=op==1 and EFFECT_EXTRA_ATTACK or EFFECT_DIRECT_ATTACK
    let eff_code = if op == Some(1) { EFFECT_EXTRA_ATTACK } else { EFFECT_DIRECT_ATTACK };
    // It can make a second attack during each Battle Phase OR it can attack directly
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(desc)
    effect::set_description(d, e1, desc);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(eff_code)
    effect::set_code(d, e1, eff_code);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // sync:RegisterEffect(e1)
    card::register_effect(d, sync, e1, false);
}
