//! Swordsoul Assessment
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 78836195;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_SWORDSOUL], listed_series: &[SET_SWORDSOUL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Banish up to 5 cards and increase the ATK of a monster you control
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_DAMAGE_STEP)
    effect::set_hint_timing(d, e1, TIMING_DAMAGE_STEP, None);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(aux.StatChangeDamageStepCondition)
    effect::set_condition(d, e1, Some(condition(|d, _a| aux::stat_change_damage_step_condition(d))));
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Swordsoul Token"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetTarget(s.tktg)
    effect::set_target(d, e2, Some(cost_target(tktg)));
    // e2:SetOperation(s.tkop)
    effect::set_operation(d, e2, Some(operation(tkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return (c:IsSetCard(SET_SWORDSOUL) or (c:IsMonster() and c:IsRace(RACE_WYRM)))
    //     and c:IsAbleToRemove() and aux.SpElimFilter(c,true)
    (card::is_set_card(d, c, &[SET_SWORDSOUL as u16]) || (card::is_monster(d, c) && card::is_race(d, c, RACE_WYRM)))
        && card::is_able_to_remove(d, c, None, None, None)
        && aux::sp_elim_filter(d, c, true, false)
}
/// `s.atkfilter(c,g)`
fn atkfilter(d: &mut Duel, c: CardId, g: &CardSet) -> bool {
    // return c:IsFaceup() and (not g:IsContains(c) or #g>1)
    card::is_faceup(d, c) && (!group::is_contains(g, c) || g.len() > 1)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&rmfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.atkfilter(chkc,g) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && atkfilter(d, chkc, &g);
    }
    // if chk==0 then return #g>0 and Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,0,1,nil,g) end
    if a.chk == Some(0) {
        return !g.is_empty() && duel::is_existing_target(d, Some(&|d, c| atkfilter(d, c, &g)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATKDEF)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATKDEF as u64);
    // Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,0,1,1,nil,g)
    let _ = duel::select_target(d, tp, Some(&|d, c| atkfilter(d, c, &g)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,tc)
    let g = duel::get_matching_group(d, Some(&rmfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, tc.into());
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
        // local sg=g:Select(tp,1,5,tc)
        let sg = group::select(d, &g, tp, 1, 5, false, tc.into()).nil();
        // local rc=Duel.Remove(sg,POS_FACEUP,REASON_EFFECT)
        let rc = duel::remove(d, sg, POS_FACEUP as u8, REASON_EFFECT, None, None);
        // if rc>0 and tc:IsRelateToEffect(e) and tc:IsFaceup() then
        if rc > 0 && card::is_relate_to_effect(d, tc.nil(), a.e) && card::is_faceup(d, tc.nil()) {
            // Increase ATK
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_UPDATE_ATTACK)
            effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // e1:SetValue(300*rc)
            effect::set_value(d, e1, 300 * rc as i64);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc.nil(), e1, false);
        }
    }
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_SWORDSOUL,SET_SWORDSOUL,TYPES_TOKEN|TYPE_TUNER,0,0,4,RACE_WYRM,ATTRIBUTE_WATER)
    // end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, TOKEN_SWORDSOUL, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// The token of `s.tktg`: `Duel.IsPlayerCanSpecialSummonMonster(..., SET_SWORDSOUL, TYPES_TOKEN|TYPE_TUNER, 0, 0, 4, RACE_WYRM, ATTRIBUTE_WATER)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_SWORDSOUL as u16]),
        type_: Some(TYPES_TOKEN | TYPE_TUNER),
        attack: Some(0),
        defense: Some(0),
        level: Some(4),
        race: Some(RACE_WYRM),
        attribute: Some(ATTRIBUTE_WATER),
    }
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if s.tktg(e,tp,eg,ep,ev,re,r,rp,0) then
    let mut a0 = a.clone();
    a0.chk = Some(0);
    if tktg(d, &a0) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // local token=Duel.CreateToken(tp,TOKEN_SWORDSOUL)
        let token = duel::create_token(d, tp, TOKEN_SWORDSOUL).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
        // Cannot Special Summon non-Synchro monsters from Extra Deck
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetAbsoluteRange(tp,1,0)
        effect::set_absolute_range(d, e1, tp, 1, 0);
        // e1:SetTarget(function(_,c) return c:IsLocation(LOCATION_EXTRA) and not c:IsType(TYPE_SYNCHRO) end)
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !card::is_type(d, c, TYPE_SYNCHRO))));
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // token:RegisterEffect(e1,true)
        card::register_effect(d, token, e1, true);
        // Clock Lizard check
        // local e2=aux.createContinuousLizardCheck(c,LOCATION_MZONE,function(_,c) return not c:IsOriginalType(TYPE_SYNCHRO) end)
        let e2 = aux::create_continuous_lizard_check(
            d,
            c,
            LOCATION_MZONE,
            Some(effect_card_fn(|d, _e, c| !card::is_original_type(d, c, TYPE_SYNCHRO))),
            None,
            None,
        );
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // token:RegisterEffect(e2,true)
        card::register_effect(d, token, e2, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
