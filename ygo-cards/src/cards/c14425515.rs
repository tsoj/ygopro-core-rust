//! Infernity Doom Slinger
//! インフェルニティ・デス・ガンマン
use super::prelude::*;

pub const ID: u32 = 14425515;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // During your Main Phase: You can Special Summon 1 Fiend monster from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // If you have no cards in your hand (Quick Effect): You can banish this card from your GY; your opponent chooses 1 of these effects for you to apply
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0 end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetCost(Cost.SelfBanish)
    effect::set_cost(d, e2, Some(cost::self_banish()));
    // e2:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk) if chk==0 then return true end Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,4000) end)
    effect::set_target(d, e2, Some(cost_target(e2_target)));
    // e2:SetOperation(s.effop)
    effect::set_operation(d, e2, Some(operation(effop)));
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_FIEND) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_race(d, c, RACE_FIEND) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_HAND,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_HAND as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_HAND,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `function(e,tp) return Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0 end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_field_group_count(d, a.tp, LOCATION_HAND, 0) == 0
}
/// The target of the second effect: `if chk==0 then return true end Duel.SetPossibleOperationInfo(...)`
fn e2_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,4000)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, a.tp, 4000);
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local top_card=Duel.GetDecktopGroup(tp,1):GetFirst()
    let decktop = duel::get_decktop_group(d, tp, 1);
    let top_card = group::get_first(d, &decktop);
    // local b1=top_card
    let b1 = top_card;
    // local b2=true
    let b2 = true;
    // local op=Duel.SelectEffect(1-tp, {b1,aux.Stringid(id,2)}, {b2,aux.Stringid(id,3)})
    let op = duel::select_effect(
        d,
        1 - tp,
        &[(b1.is_some(), aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))],
    );
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if op==1 then
    if op == Some(1) {
        // Reveal the top card of your Deck, and if it is a monster, your opponent takes any effect damage you would have taken this turn instead. Otherwise, you take 4000 damage
        // Duel.ConfirmDecktop(tp,1)
        duel::confirm_decktop(d, tp, 1);
        // if top_card:IsMonster() then
        let top = top_card.nil();
        if card::is_monster(d, top) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(aux.Stringid(id,4))
            effect::set_description(d, e1, aux::stringid(ID, 4));
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_REFLECT_DAMAGE)
            effect::set_code(d, e1, EFFECT_REFLECT_DAMAGE);
            // e1:SetTargetRange(1,0)
            effect::set_target_range(d, e1, 1, 0);
            // e1:SetValue(function(e,_,_,r) return (r&REASON_EFFECT)==REASON_EFFECT end)
            effect::set_value_fn(d, e1, lua_fn(|_d, a| (a.int(3) as u32 & REASON_EFFECT) == REASON_EFFECT));
            // e1:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e1,tp)
            duel::register_effect(d, e1, tp);
        } else {
            // Duel.Damage(tp,4000,REASON_EFFECT)
            duel::damage(d, tp, 4000, REASON_EFFECT, false, None);
        }
    } else if op == Some(2) {
        // You take no effect damage this turn
        // local e2a=Effect.CreateEffect(c)
        let e2a = effect::create_effect(d, c);
        // e2a:SetDescription(aux.Stringid(id,5))
        effect::set_description(d, e2a, aux::stringid(ID, 5));
        // e2a:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2a, EFFECT_TYPE_FIELD);
        // e2a:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e2a, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e2a:SetCode(EFFECT_CHANGE_DAMAGE)
        effect::set_code(d, e2a, EFFECT_CHANGE_DAMAGE);
        // e2a:SetTargetRange(1,0)
        effect::set_target_range(d, e2a, 1, 0);
        // e2a:SetValue(s.damval)
        effect::set_value_fn(d, e2a, lua_fn(damval));
        // e2a:SetReset(RESET_PHASE|PHASE_END)
        effect::set_reset(d, e2a, RESET_PHASE | PHASE_END, 1);
        // Duel.RegisterEffect(e2a,tp)
        duel::register_effect(d, e2a, tp);
        // local e2b=e2a:Clone()
        let e2b = effect::clone(d, e2a);
        // e2b:SetCode(EFFECT_NO_EFFECT_DAMAGE)
        effect::set_code(d, e2b, EFFECT_NO_EFFECT_DAMAGE);
        // Duel.RegisterEffect(e2b,tp)
        duel::register_effect(d, e2b, tp);
    }
}
/// `s.damval(e,re,val,r,rp,rc)`
fn damval(_d: &mut Duel, a: Args) -> Value {
    // if (r&REASON_EFFECT)>0 then return 0 else return val end
    let r = a.int(3) as u32;
    if (r & REASON_EFFECT) > 0 {
        Value::Int(0)
    } else {
        Value::Int(a.int(2))
    }
}
