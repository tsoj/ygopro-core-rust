//! B・F－革命のグラン・パルチザン
//! Battlewasp - Grand Partisan the Revolution
use super::prelude::*;

pub const ID: u32 = 35057188;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro Summon procedure: 1 Tuner + 1+ non-Tuner monsters
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // Cannot be destroyed by card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Insect Synchro Monsters you control gain 200 ATK for each of your banished Insect monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(function(e,c) return c:IsRace(RACE_INSECT) and c:IsType(TYPE_SYNCHRO) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_race(d, c, RACE_INSECT as u64) && card::is_type(d, c, TYPE_SYNCHRO))));
    // e2:SetValue(function(e,c) return 200*Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_INSECT),e:GetHandlerPlayer(),LOCATION_REMOVED,0,nil) end)
    effect::set_value_fn(d, e2, effect_card_value(|d, e, _c| {
        let p = effect::get_handler_player(d, e);
        200 * duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_INSECT as u64)), p, LOCATION_REMOVED, 0, NO_EX) as i64
    }));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON + CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_REMOVE)
    effect::set_code(d, e3, EVENT_REMOVE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,1-tp,LOCATION_ONFIELD)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, 1 - a.tp, LOCATION_ONFIELD as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,500)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 500);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)==0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) == 0 {
        return;
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, tp, 0, LOCATION_ONFIELD);
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsRace,RACE_INSECT),tp,LOCATION_REMOVED,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_INSECT as u64)), tp, LOCATION_REMOVED, 0, NO_EX);
    // if #g>0 and ct>0 and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !g.is_empty() && ct > 0 && duel::select_yes_no(d, tp, aux::stringid(ID, 1)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
        // local sg=g:Select(tp,1,math.min(ct,#g),nil)
        let sg = group::select(d, &g, tp, 1, ct.min(g.len()) as u16, false, NO_EX).nil();
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // Duel.HintSelection(sg)
        duel::hint_selection(d, sg.clone(), true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // local des_ct=Duel.Destroy(sg,REASON_EFFECT)
        let des_ct = duel::destroy(d, sg, REASON_EFFECT, None, None);
        // if des_ct>0 then
        if des_ct > 0 {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Damage(1-tp,des_ct*500,REASON_EFFECT)
            duel::damage(d, 1 - tp, des_ct as i64 * 500, REASON_EFFECT, false, None);
        }
    }
}
