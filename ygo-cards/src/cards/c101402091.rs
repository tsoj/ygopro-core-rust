//! Angelechy Seneschal
//! JP Name
use super::prelude::*;

pub const ID: u32 = 101402091;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ANGELECHY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_DISABLE+CATEGORY_TOEXTRA)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_DISABLE + CATEGORY_TOEXTRA);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_CONTROL)
    effect::set_category(d, e2, CATEGORY_CONTROL);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.atkcon)
    effect::set_condition(d, e2, Some(condition(atkcon)));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_EITHER,LOCATION_EXTRA)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_EITHER as u8, LOCATION_EXTRA as i32);
    true
}
// s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRace(RACE_FAIRY) and c:IsSynchroMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
    card::is_race(d, c, RACE_FAIRY)
        && card::is_synchro_monster(d, c)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
        && duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
}
// s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local turn_player=Duel.GetTurnPlayer()
    let turn_player = duel::get_turn_player(d);
    // for player=turn_player,1-turn_player,(-1)^turn_player do
    let players: [u8; 2] = if turn_player == 0 { [0, 1] } else { [1, 0] };
    for player in players {
        // local g=Duel.GetMatchingGroup(s.spfilter,player,LOCATION_EXTRA,0,nil,e,player)
        let g = duel::get_matching_group(d, Some(&|d, x| spfilter(d, x, e, player)), player, LOCATION_EXTRA, 0, NO_EX);
        // if #g>0 and Duel.SelectYesNo(player,aux.Stringid(id,2)) then
        if !g.is_empty() && duel::select_yes_no(d, player, aux::stringid(ID, 2)) {
            // Duel.Hint(HINT_SELECTMSG,player,HINTMSG_SPSUMMON)
            duel::hint(d, HINT_SELECTMSG as u8, player, HINTMSG_SPSUMMON as u64);
            // local sc=g:Select(player,1,1,nil):GetFirst()
            let sel = group::select(d, &g, player, 1, 1, false, NO_EX).nil();
            // if sc and Duel.SpecialSummonStep(sc,0,player,player,false,false,POS_FACEUP) then
            if let Some(sc) = group::get_first(d, &sel) {
                if duel::special_summon_step(d, sc, 0, player, player, false, false, POS_FACEUP as u8, None) {
                    // sc:NegateEffects(c)
                    card::negate_effects(d, sc, c, None, false, None);
                    // s.applyrestrictions(sc,c)
                    applyrestrictions(d, sc, c);
                }
            }
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
    // if c:IsRelateToEffect(e) then s.applyrestrictions(c,c) end
    if card::is_relate_to_effect(d, c, e) {
        applyrestrictions(d, c, c);
    }
}
// s.applyrestrictions(c,rc)
fn applyrestrictions(d: &mut Duel, c: CardId, rc: CardId) {
    // local e1=Effect.CreateEffect(rc)
    let e1 = effect::create_effect(d, rc);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_BE_MATERIAL)
    effect::set_code(d, e1, EFFECT_CANNOT_BE_MATERIAL);
    // e1:SetValue(aux.cannotmatfilter(SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK))
    effect::set_value_fn(d, e1, aux::cannotmatfilter(&[SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK]));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // c:RegisterEffect(e1,true)
    card::register_effect(d, c, e1, true);
    // local e2=Effect.CreateEffect(rc)
    let e2 = effect::create_effect(d, rc);
    // e2:SetDescription(aux.Stringid(id,4))
    effect::set_description(d, e2, aux::stringid(ID, 4));
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
    // e2:SetCode(EFFECT_LEAVE_FIELD_REDIRECT)
    effect::set_code(d, e2, EFFECT_LEAVE_FIELD_REDIRECT);
    // e2:SetValue(LOCATION_DECKBOT)
    effect::set_value(d, e2, LOCATION_DECKBOT as i64);
    // e2:SetReset(RESET_EVENT|RESETS_REDIRECT)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_REDIRECT, 1);
    // c:RegisterEffect(e2,true)
    card::register_effect(d, c, e2, true);
}
// s.atkcon(e,tp,eg,ep,ev,re,r,rp)
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetBattleTarget():IsSetCard(SET_ANGELECHY)
    let h = effect::get_handler(d, a.e).nil();
    let bt = card::get_battle_target(d, h).nil();
    card::is_set_card(d, bt, &[SET_ANGELECHY as u16])
}
// s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_CONTROL,e:GetHandler(),1,tp,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(h)), 1, a.tp, 0);
    true
}
// s.atkop(e,tp,eg,ep,ev,re,r,rp)
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.NegateAttack() then return end
    if !duel::negate_attack(d) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then Duel.GetControl(c,1-tp) end
    if card::is_relate_to_effect(d, c, a.e) {
        duel::get_control(d, c, 1 - tp, 0, 0, None, None);
    }
}
