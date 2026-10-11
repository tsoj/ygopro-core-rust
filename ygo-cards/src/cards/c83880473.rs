//! War Rock Spirit
use super::prelude::*;

pub const ID: u32 = 83880473;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WAR_ROCK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase() and aux.StatChangeDamageStepCondition()
    duel::is_battle_phase(d, None) && aux::stat_change_damage_step_condition(d)
}
/// `s.filter(c,e,tp,pos)` (`pos` is nil when the core asks about a chain target)
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, pos: Option<u32>) -> bool {
    // return c:IsSetCard(SET_WAR_ROCK) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,pos)
    card::is_set_card(d, c, &[SET_WAR_ROCK as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, pos.map(|p| p as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && filter(d, chkc, a.e, a.tp, None);
    }
    // local atk=Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp,POS_FACEUP_ATTACK)
    let atk = duel::is_existing_target(d, Some(&|d: &mut Duel, c| filter(d, c, a.e, a.tp, Some(POS_FACEUP_ATTACK))), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // local def=Duel.IsExistingTarget(s.filter,tp,LOCATION_GRAVE,0,1,nil,e,tp,POS_FACEUP_DEFENSE)
    let def = duel::is_existing_target(d, Some(&|d: &mut Duel, c| filter(d, c, a.e, a.tp, Some(POS_FACEUP_DEFENSE))), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and (atk or def) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && (atk || def);
    }
    // local choice=-1
    let mut choice: i64 = -1;
    // local pos=0
    let mut pos: u32 = 0;
    if atk && def {
        // choice=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
        choice = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)]) as i64;
    } else if atk {
        // choice=Duel.SelectOption(tp,aux.Stringid(id,0))
        choice = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0)]) as i64;
    } else if def {
        // choice=Duel.SelectOption(tp,aux.Stringid(id,1))+1
        choice = duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]) as i64 + 1;
    }
    // if choice==0 then pos=POS_FACEUP_ATTACK elseif choice==1 then pos=POS_FACEUP_DEFENSE end
    if choice == 0 {
        pos = POS_FACEUP_ATTACK;
    } else if choice == 1 {
        pos = POS_FACEUP_DEFENSE;
    }
    // e:SetLabel(choice)
    effect::set_label(d, a.e, &[choice]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,pos)
    let g = duel::select_target(d, a.tp, Some(&|d: &mut Duel, c| filter(d, c, a.e, a.tp, Some(pos))), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c,tc=e:GetHandler(),Duel.GetFirstTarget()
    let c = effect::get_handler(d, a.e).nil();
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local choice=e:GetLabel()
        let choice = effect::get_label(d, a.e);
        // if choice==0 then
        if choice == 0 {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_ATTACK)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_ATTACK as u8, None);
            // Negate its effects
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e1, EFFECT_DISABLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetCode(EFFECT_DISABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
            // Cannot attack directly
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
            // e3:SetCode(EFFECT_CANNOT_DIRECT_ATTACK)
            effect::set_code(d, e3, EFFECT_CANNOT_DIRECT_ATTACK);
            // e3:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e3, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc, e3, false);
        } else if choice == 1 {
            // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
            duel::special_summon(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP_DEFENSE as u8, None);
            // local e4=Effect.CreateEffect(c)
            let e4 = effect::create_effect(d, c);
            // e4:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e4, EFFECT_TYPE_FIELD);
            // e4:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
            effect::set_code(d, e4, EFFECT_INDESTRUCTABLE_COUNT);
            // e4:SetTargetRange(LOCATION_MZONE,0)
            effect::set_target_range(d, e4, LOCATION_MZONE, 0);
            // e4:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_WAR_ROCK))
            effect::set_target(d, e4, Some(effect_card_fn(|d: &mut Duel, _e, c| card::is_set_card(d, c, &[SET_WAR_ROCK as u16]))));
            // e4:SetValue(s.indct)
            effect::set_value_fn(d, e4, lua_fn(indct));
            // e4:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e4,tp)
            duel::register_effect(d, e4, a.tp);
        }
    }
}
/// `s.indct(e,re,r)`
fn indct(_d: &mut Duel, a: Args) -> i64 {
    // if (r&REASON_BATTLE)>0 then return 1 else return 0 end
    if ((a.int(2) as u32) & REASON_BATTLE) > 0 { 1 } else { 0 }
}
