//! Junk Signal
use super::prelude::*;

pub const ID: u32 = 26387390;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_JUNK_WARRIOR, CARD_STARDUST_DRAGON], ..ScriptInfo::DEFAULT };

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
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some((TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E) as u32));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.spcostfilter(c,e,tp)
fn spcostfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return Duel.GetMZoneCount(tp,c)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,nil,e,tp)
    duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
        && duel::is_existing_matching_card(
            d,
            Some(&|d, x| spfilter(d, x, e, tp)),
            tp,
            LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            NO_EX,
        )
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return (c:IsCode(CARD_JUNK_WARRIOR,CARD_STARDUST_DRAGON) or c:ListsCode(CARD_JUNK_WARRIOR,CARD_STARDUST_DRAGON))
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    (card::is_code(d, c, &[CARD_JUNK_WARRIOR, CARD_STARDUST_DRAGON]) || card::lists_code(d, c, &[CARD_JUNK_WARRIOR, CARD_STARDUST_DRAGON]))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    // local b1=Duel.CheckReleaseGroupCost(tp,s.spcostfilter,1,false,nil,nil,e,tp)
    let b1 = duel::check_release_group_cost(d, tp, Some(&|d, c| spcostfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
    // local ex,_,event_player,event_value,event_reff=Duel.CheckEvent(EVENT_CHAINING,true)
    let ev = duel::check_event(d, EVENT_CHAINING);
    let ex = ev.is_some();
    let event_player = ev.as_ref().map_or(0, |x| x.event_player);
    let event_value = ev.as_ref().map_or(0, |x| x.event_value);
    // local trig_player,trig_type=nil
    let mut trig_player: Value = Value::Nil;
    let mut trig_type: Value = Value::Nil;
    // if ex then
    if ex {
        // trig_player,trig_type=Duel.GetChainInfo(event_value-1,CHAININFO_TRIGGERING_PLAYER,CHAININFO_TRIGGERING_TYPE)
        let info = duel::get_chain_info(d, (event_value - 1) as u8, &[CHAININFO_TRIGGERING_PLAYER, CHAININFO_TRIGGERING_TYPE]);
        trig_player = info[0].clone();
        trig_type = info[1].clone();
    }
    // local b2=ex and event_player==1-tp and Duel.IsChainDisablable(event_value) and trig_player==tp and trig_type&TYPE_SYNCHRO>0
    let b2 = ex
        && event_player == 1 - tp
        && duel::is_chain_disablable(d, event_value as u8)
        && !trig_player.is_nil()
        && trig_player.int() == tp as i64
        && (!trig_type.is_nil() && (trig_type.int() & TYPE_SYNCHRO as i64) > 0);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,1)},
    //     {b2,aux.Stringid(id,2)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]).unwrap_or(0) as i64;
    // e:SetLabel(op)
    effect::set_label(d, e, &[op]);
    // if op==1 then
    if op == 1 {
        // local sc=Duel.SelectReleaseGroupCost(tp,s.spcostfilter,1,1,false,nil,nil,e,tp):GetFirst()
        let sg = duel::select_release_group_cost(d, tp, Some(&|d, c| spcostfilter(d, c, e, tp)), 1, 1, false, None, NO_EX);
        let sc = group::get_first(d, &sg).nil();
        // Duel.Release(sc,REASON_COST)
        duel::release(d, sc, REASON_COST, None);
        // e:SetLabelObject(sc)
        effect::set_label_object(d, e, Value::Card(sc));
        // sc:CreateEffectRelation(e)
        card::create_effect_relation(d, sc, e);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetLabel(op,event_value)
        effect::set_label(d, e, &[op, event_value as i64]);
    }
    true
}
// function s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    // elseif op==2 then
    } else if op == 2 {
        // e:SetCategory(CATEGORY_DISABLE)
        effect::set_category(d, a.e, CATEGORY_DISABLE);
        // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, tp, 0);
    }
    true
}
// function s.effop(e,tp,eg,ep,ev,re,r,rp)
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op,event_value=e:GetLabel()
    let labels = effect::get_labels(d, a.e);
    let op = labels.first().copied().unwrap_or(0);
    let event_value = labels.get(1).copied().unwrap_or(0);
    // if op==1 then
    if op == 1 {
        // Tribute 1 monster; Special Summon 1 "Junk Warrior", 1 "Stardust Dragon", or 1 monster that mentions either of them from your hand, Deck, or GY, except the Tributed monster
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // local sc=e:GetLabelObject()
        let sc = effect::get_label_object(d, a.e).as_card().nil();
        // local exc=sc:IsRelateToEffect(e) and sc or nil
        let exc = if card::is_relate_to_effect(d, sc, a.e) { Some(sc) } else { None };
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,1,1,exc,e,tp)
        let g = duel::select_matching_card(
            d,
            tp,
            Some(&aux::necro_valley_filter(|d, x| spfilter(d, x, a.e, tp))),
            tp,
            LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE,
            0,
            1,
            1,
            false,
            exc.into(),
        )
        .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    // elseif op==2 then
    } else if op == 2 {
        // When your opponent activates a card or effect in response to the activation of your Synchro Monster's effect: Negate that opponent's effect
        // Duel.NegateEffect(event_value)
        duel::negate_effect(d, event_value as u8);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // You cannot Special Summon from the Extra Deck for the rest of this turn, except Synchro Monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(e,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsSynchroMonster() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_synchro_monster(d, c))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // "Clock Lizard" check
    // aux.addTempLizardCheck(c,tp,function(e,c) return not c:IsOriginalType(TYPE_SYNCHRO) end)
    aux::add_temp_lizard_check(d, c, tp, Some(effect_card_fn(|d, _e, c| !card::is_original_type(d, c, TYPE_SYNCHRO))), None, None, None, None);
}
