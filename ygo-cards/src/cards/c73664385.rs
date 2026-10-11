//! Verre Magic - Lacrima of Light
use super::prelude::*;

pub const ID: u32 = 73664385;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGISTUS, SET_WITCHCRAFTER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e1:SetCost(s.effcost)
    effect::set_cost(d, e1, Some(cost_target(effcost)));
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSpell() or c:IsRace(RACE_SPELLCASTER)) and c:IsAbleToGrave()
    (card::is_spell(d, c) || card::is_race(d, c, RACE_SPELLCASTER)) && card::is_able_to_grave(d, c, None)
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard({SET_MAGISTUS,SET_WITCHCRAFTER}) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_MAGISTUS as u16, SET_WITCHCRAFTER as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `aux.FaceupFilter(Card.IsSetCard,{SET_MAGISTUS,SET_WITCHCRAFTER})`
fn faceup_setcard(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_MAGISTUS as u16, SET_WITCHCRAFTER as u16])
}
/// `Duel.CheckEvent(EVENT_CHAINING,true)`: the chaining flag and the event player
fn chaining_info(d: &mut Duel) -> (bool, Option<u8>) {
    match duel::check_event(d, EVENT_CHAINING) {
        Some(ev) => (true, Some(ev.event_player)),
        None => (false, None),
    }
}
/// `s.effcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn effcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // e:SetLabel(-100)
    effect::set_label(d, a.e, &[-100]);
    // local b1=not Duel.HasFlagEffect(tp,id)
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,{SET_MAGISTUS,SET_WITCHCRAFTER}),tp,LOCATION_MZONE,0,1,nil)
    //   and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = !duel::has_flag_effect(d, tp, ID, None)
        && duel::is_existing_matching_card(d, Some(&faceup_setcard), tp, LOCATION_MZONE, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local event_chaining,_,event_player=Duel.CheckEvent(EVENT_CHAINING,true)
    let (event_chaining, event_player) = chaining_info(d);
    // local b2=not Duel.HasFlagEffect(tp,id+1) and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp)
    //   and event_chaining and event_player==1-tp
    let b2 = !duel::has_flag_effect(d, tp, ID + 1, None)
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX)
        && event_chaining
        && event_player == Some(1 - tp);
    // if chk==0 then return b1 or b2 end
    if a.chk == Some(0) {
        return b1 || b2;
    }
    true
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local cost_skip=e:GetLabel()~=-100
    let cost_skip = effect::get_label(d, a.e) != -100;
    // local b1=(cost_skip or (not Duel.HasFlagEffect(tp,id)
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,{SET_MAGISTUS,SET_WITCHCRAFTER}),tp,LOCATION_MZONE,0,1,nil)))
    //   and Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil)
    let b1 = (cost_skip
        || (!duel::has_flag_effect(d, tp, ID, None)
            && duel::is_existing_matching_card(d, Some(&faceup_setcard), tp, LOCATION_MZONE, 0, 1, NO_EX)))
        && duel::is_existing_matching_card(d, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    // local event_chaining,_,event_player=Duel.CheckEvent(EVENT_CHAINING,true)
    let (event_chaining, event_player) = chaining_info(d);
    // local b2=(cost_skip or (not Duel.HasFlagEffect(tp,id+1)
    //   and event_chaining and event_player==1-tp))
    //   and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,nil,e,tp)
    let b2 = (cost_skip
        || (!duel::has_flag_effect(d, tp, ID + 1, None) && event_chaining && event_player == Some(1 - tp)))
        && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, NO_EX);
    // if chk==0 then e:SetLabel(0) return b1 or b2 end
    if a.chk == Some(0) {
        effect::set_label(d, a.e, &[0]);
        return b1 || b2;
    }
    // local cd=e:GetChainData()
    // cd.choice=Duel.SelectEffect(tp, {b1,aux.Stringid(id,1)}, {b2,aux.Stringid(id,2)})
    let choice = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 1)), (b2, aux::stringid(ID, 2))]);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(choice.map_or(0, |x| x as i64)));
    if choice == Some(1) {
        // e:SetCategory(CATEGORY_TOGRAVE)
        effect::set_category(d, a.e, CATEGORY_TOGRAVE);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, LOCATION_DECK as i32);
    } else if choice == Some(2) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // if not cost_skip then Duel.RegisterFlagEffect(tp,id+1,RESET_PHASE|PHASE_END,0,1) end
        if !cost_skip {
            duel::register_flag_effect(d, tp, ID + 1, RESET_PHASE | PHASE_END, 0, 1, 0);
        }
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND|LOCATION_DECK)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_HAND | LOCATION_DECK) as i32);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local cd=e:GetChainData()
    let choice = chain::effect_data(d, a.e, None, chain::field::CARD).int();
    if choice == 1 {
        // Send 1 Spellcaster monster or 1 Spell from your Deck to the GY
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, tp, Some(&tgfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then Duel.SendtoGrave(g,REASON_EFFECT) end
        if !g.is_empty() {
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    } else if choice == 2 {
        // Special Summon 1 "Magistus" or "Witchcrafter" monster from your hand or Deck
        // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
        if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
            return;
        }
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
        // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND|LOCATION_DECK,0,1,1,nil,e,tp)
        let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_HAND | LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP) end
        if !g.is_empty() {
            duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        }
    }
}
