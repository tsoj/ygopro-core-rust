//! Champion's Pulse
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 46001505;
const CARD_HOT_RED_DRAGON_ARCHFIEND: u32 = 39765958;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_RED_DRAGON_ARCHFIEND, CARD_HOT_RED_DRAGON_ARCHFIEND],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetHintTiming(0,TIMING_BATTLE_START)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_BATTLE_START));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spfilter(c,e,tp,mmz_chk)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // if not (c:IsCode(CARD_RED_DRAGON_ARCHFIEND,CARD_HOT_RED_DRAGON_ARCHFIEND) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    if !(card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND, CARD_HOT_RED_DRAGON_ARCHFIEND])
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
    {
        return false;
    }
    // if c:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, c, LOCATION_GRAVE) {
        // return mmz_chk
        mmz_chk
    } else {
        // return Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    }
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local b1=Duel.IsBattlePhase(1-tp)
    let b1 = duel::is_battle_phase(d, Some(1 - tp));
    // local event_chaining,_,event_player,event_value,event_reason_effect=Duel.CheckEvent(EVENT_CHAINING,true)
    let ev = duel::check_event(d, EVENT_CHAINING);
    let event_chaining = ev.is_some();
    let event_player = ev.as_ref().map(|x| x.event_player);
    let event_value = ev.as_ref().map(|x| x.event_value);
    let event_reason_effect = ev.as_ref().and_then(|x| x.reason_effect);
    // local b2=Duel.GetFieldGroupCount(tp,LOCATION_ONFIELD,0)<Duel.GetFieldGroupCount(tp,0,LOCATION_ONFIELD)
    //     and event_chaining and event_player==1-tp and event_reason_effect:IsMonsterEffect() and Duel.IsChainDisablable(event_value)
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA|LOCATION_GRAVE,0,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
    let b2 = duel::get_field_group_count(d, tp, LOCATION_ONFIELD, 0) < duel::get_field_group_count(d, tp, 0, LOCATION_ONFIELD)
        && event_chaining
        && event_player == Some(1 - tp)
        && effect::is_monster_effect(d, event_reason_effect.nil())
        && duel::is_chain_disablable(d, event_value.nil() as u8)
        && {
            let mzone = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
            duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, mzone)), tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, 1, NO_EX)
        };
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).nil() as i64;
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_RECOVER)
        effect::set_category(d, e, CATEGORY_RECOVER);
        // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,1-tp,1000)
        duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, 1 - tp, 1000);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DISABLE+CATEGORY_DESTROY)
        effect::set_category(d, e, CATEGORY_SPECIAL_SUMMON + CATEGORY_DISABLE + CATEGORY_DESTROY);
        // e:SetLabel(op,event_value)
        effect::set_label(d, e, &[op, event_value.nil() as i64]);
        // local rc=event_reason_effect:GetHandler()
        let rc = effect::get_handler(d, event_reason_effect.nil()).nil();
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_EXTRA | LOCATION_GRAVE) as i32);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,rc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, Some(CardSet::from_card(rc)), 1, tp, 0);
        // if rc:IsDestructable() and rc:IsRelateToEffect(event_reason_effect) then
        if card::is_destructable(d, rc, None) && card::is_relate_to_effect(d, rc, event_reason_effect.nil()) {
            // Duel.SetOperationInfo(0,CATEGORY_DESTROY,rc,1,tp,0)
            duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(rc)), 1, tp, 0);
        }
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local op,event_value=e:GetLabel()
    let labels = effect::get_labels(d, e);
    let op = labels.first().copied().unwrap_or(0);
    let event_value = labels.get(1).copied();
    // if op==1 then
    if op == 1 {
        // --Your opponent gains 1000 LP, then end the Battle Phase
        // if Duel.Recover(1-tp,1000,REASON_EFFECT)>0 then
        if duel::recover(d, 1 - tp, 1000, REASON_EFFECT, false, None) > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.SkipPhase(1-tp,PHASE_BATTLE,RESET_PHASE|PHASE_BATTLE_STEP,1)
            duel::skip_phase(d, 1 - tp, PHASE_BATTLE, RESET_PHASE | PHASE_BATTLE_STEP, 1, 0);
        }
    // elseif op==2 then
    } else if op == 2 {
        // local trig_eff=Duel.GetChainInfo(event_value,CHAININFO_TRIGGERING_EFFECT)
        let trig_eff = duel::get_chain_info(d, event_value.nil() as u8, &[CHAININFO_TRIGGERING_EFFECT])[0].as_effect().nil();
        // local rc=trig_eff:GetHandler()
        let rc = effect::get_handler(d, trig_eff).nil();
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_EXTRA|LOCATION_GRAVE,0,1,1,nil,e,tp,Duel.GetLocationCount(tp,LOCATION_MZONE)>0)
        let mzone = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(move |d, c| spfilter(d, c, e, tp, mzone))),
            tp,
            LOCATION_EXTRA | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            NO_EX,
        )
        .nil();
        // if #g>0 and Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)>0 and Duel.NegateEffect(event_value)
        //     and rc:IsRelateToEffect(trig_eff) then
        if !g.is_empty()
            && duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0
            && duel::negate_effect(d, event_value.nil() as u8)
            && card::is_relate_to_effect(d, rc, trig_eff)
        {
            // Duel.Destroy(rc,REASON_EFFECT)
            duel::destroy(d, CardSet::from_card(rc), REASON_EFFECT, None, None);
        }
    }
}
