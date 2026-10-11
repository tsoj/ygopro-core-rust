//! Shinobird Crow
use super::prelude::*;

pub const ID: u32 = 39817919;

pub const INFO: ScriptInfo = ScriptInfo { listed_card_types: &[TYPE_SPIRIT], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Spirit.AddProcedure(c,EVENT_SUMMON_SUCCESS,EVENT_FLIP)
    spirit::add_procedure(d, c, &[EVENT_SUMMON_SUCCESS, EVENT_FLIP]);
    // Cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase this card's ATK/DEF
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e2, TIMING_DAMAGE_STEP, None);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(s.atkcost)
    effect::set_target(d, e2, Some(cost_target(atkcost)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // if Duel.GetCurrentPhase()~=PHASE_DAMAGE or Duel.IsDamageCalculated() then return false end
    if duel::get_current_phase(d) != PHASE_DAMAGE || duel::is_damage_calculated(d) {
        return false;
    }
    // local a,b=Duel.GetBattleMonster(tp)
    let (x, b) = duel::get_battle_monster(d, a.tp);
    // return a==e:GetHandler() and b and b:IsControler(1-tp)
    let h = effect::get_handler(d, a.e).nil();
    if x != Some(h) {
        return false;
    }
    match b {
        Some(b) => card::is_controler(d, b, 1 - a.tp),
        None => false,
    }
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_SPIRIT) and (c:IsAttackAbove(1) or c:IsDefenseAbove(1)) and c:IsDiscardable()
    card::is_type(d, c, TYPE_SPIRIT)
        && (card::is_attack_above(d, c, 1) || card::is_defense_above(d, c, 1))
        && card::is_discardable(d, c, Some(REASON_COST), None)
}
/// `s.atkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DISCARD)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DISCARD as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&cfilter), a.tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // e:SetLabelObject(g:GetFirst())
    let first = group::get_first(d, &g);
    let label = match first {
        Some(f) => Value::Card(f),
        None => Value::Nil,
    };
    effect::set_label_object(d, a.e, label);
    // Duel.SendtoGrave(g,REASON_COST|REASON_DISCARD)
    duel::send_to_grave(d, g, REASON_COST | REASON_DISCARD, None, None);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToBattle() or not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_battle(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local tc=e:GetLabelObject()
    // if not tc then return end
    let Some(tc) = effect::get_label_object(d, a.e).as_card() else {
        return;
    };
    // local atk=math.max(tc:GetAttack(),0)
    let atk = card::get_attack(d, tc).max(0);
    // local def=math.max(tc:GetDefense(),0)
    let def = card::get_defense(d, tc).max(0);
    // c:UpdateAttack(atk,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    card::update_attack(d, c, atk as i64, Some(RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE), None, None);
    // c:UpdateDefense(def,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE)
    card::update_defense(d, c, def as i64, Some(RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE), None, None);
}
