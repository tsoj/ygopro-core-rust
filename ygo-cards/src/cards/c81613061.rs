//! War Rock Medium
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 81613061;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_WAR_ROCK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot activate monster effects on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetCondition(s.limcon)
    effect::set_condition(d, e2, Some(condition(limcon)));
    // e2:SetValue(s.limval)
    effect::set_value_fn(d, e2, lua_fn(limval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Set 1 "War Rock" Spell/Trap from your Deck
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SET)
    effect::set_category(d, e3, CATEGORY_SET);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetTarget(s.settg)
    effect::set_target(d, e3, Some(cost_target(settg)));
    // e3:SetOperation(s.setop)
    effect::set_operation(d, e3, Some(operation(setop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.wrfilter(c)`
fn wrfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelAbove(7) and c:IsSetCard(SET_WAR_ROCK)
    card::is_faceup(d, c) && card::is_level_above(d, c, 7) && card::is_set_card(d, c, &[SET_WAR_ROCK as u16])
}
/// `s.limcon(e)`
fn limcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_MAIN1)
    //     and Duel.IsExistingMatchingCard(s.wrfilter,e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(Card.IsSummonType,e:GetHandlerPlayer(),0,LOCATION_MZONE,1,nil,SUMMON_TYPE_SPECIAL)
    if !duel::is_phase(d, PHASE_MAIN1) {
        return false;
    }
    let p = effect::get_handler_player(d, a.e);
    if !duel::is_existing_matching_card(d, Some(&|d, c| wrfilter(d, c)), p, LOCATION_MZONE, 0, 1, NO_EX) {
        return false;
    }
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_summon_type(d, c, &[SUMMON_TYPE_SPECIAL])), p, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.limval(e,re,tp)`
fn limval(d: &mut Duel, a: Args) -> bool {
    // return re:GetActivateLocation()==LOCATION_MZONE and re:IsMonsterEffect()
    let re = a.v(1).as_effect().nil();
    effect::get_activate_location(d, re) == LOCATION_MZONE && effect::is_monster_effect(d, re)
}
/// `s.setfilter(c)`
fn setfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_WAR_ROCK) and not c:IsCode(id) and c:IsSpellTrap() and c:IsSSetable()
    card::is_set_card(d, c, &[SET_WAR_ROCK as u16]) && !card::is_code(d, c, &[ID]) && card::is_spell_trap(d, c) && card::is_s_setable(d, c, false, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| setfilter(d, c)), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| setfilter(d, x)), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g:GetFirst())
        let tc = group::get_first(d, &g).nil();
        duel::s_set(d, a.tp, tc, None, true);
    }
    // --Cannot Special Summon, except Warrior monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_,c) return not c:IsRace(RACE_WARRIOR) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_race(d, c, RACE_WARRIOR))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
