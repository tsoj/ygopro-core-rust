//! Chaos Witch
use super::prelude::*;

pub const ID: u32 = 30327674;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID + 1, ID + 2],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // --Special Summon 2 "Black Beast Tokens"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetLabel(id+1,0,TYPES_TOKEN,1000,500,2,RACE_FIEND,ATTRIBUTE_DARK)
    effect::set_label(d, e1, &[(ID + 1) as i64, 0, TYPES_TOKEN as i64, 1000, 500, 2, RACE_FIEND as i64, ATTRIBUTE_DARK as i64]);
    // e1:SetCost(s.tkcost)
    effect::set_cost(d, e1, Some(cost_target(tkcost)));
    // e1:SetTarget(s.tktg)
    effect::set_target(d, e1, Some(cost_target(tktg)));
    // e1:SetOperation(s.tkop)
    effect::set_operation(d, e1, Some(operation(tkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Special Summon 2 "White Beast Tokens"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_REMOVE)
    effect::set_code(d, e2, EVENT_REMOVE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetLabel(id+2,0,TYPES_TOKEN+TYPE_TUNER,500,1000,2,RACE_FAIRY,ATTRIBUTE_LIGHT)
    effect::set_label(d, e2, &[(ID + 2) as i64, 0, (TYPES_TOKEN + TYPE_TUNER) as i64, 500, 1000, 2, RACE_FAIRY as i64, ATTRIBUTE_LIGHT as i64]);
    // e2:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_HAND|LOCATION_GRAVE) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_previous_location(d, h, LOCATION_HAND | LOCATION_GRAVE)
    })));
    // e2:SetCost(s.tkcost)
    effect::set_cost(d, e2, Some(cost_target(tkcost)));
    // e2:SetTarget(s.tktg)
    effect::set_target(d, e2, Some(cost_target(tktg)));
    // e2:SetOperation(s.tkop)
    effect::set_operation(d, e2, Some(operation(tkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Count Special Summoned monsters
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.ctfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(ctfilter_fn));
}
/// `function(c) ... end` of `Duel.AddCustomActivityCounter` (`s.ctfilter(c)`)
fn ctfilter_fn(d: &mut Duel, a: Args) -> bool {
    // return not c:IsSummonLocation(LOCATION_EXTRA) or s.synfilter(c)
    let c = a.c(0);
    !card::is_summon_location(d, c, LOCATION_EXTRA) || synfilter(d, c)
}
/// `s.synfilter(c)`
fn synfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_SYNCHRO) and c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK)
    card::is_type(d, c, TYPE_SYNCHRO) && card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)
}
/// `s.tkcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local blk=(e:GetLabel())==id+1
    let blk = effect::get_label(d, a.e) == (ID + 1) as i64;
    // if chk==0 then
    if a.chk == Some(0) {
        // if Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)>0 then return false end
        if duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) > 0 {
            return false;
        }
        // return not blk or c:IsReleasable()
        return !blk || card::is_releasable(d, c, None, None);
    }
    // if blk then Duel.Release(c,REASON_COST) end
    if blk {
        duel::release(d, c, REASON_COST, None);
    }
    // --Cannot Special Summon monsters from the Extra Deck, except LIGHT or DARK Synchro monsters
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(function(_,c) return c:IsLocation(LOCATION_EXTRA) and not s.synfilter(c) end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_location(d, c, LOCATION_EXTRA) && !synfilter(d, c))));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.addTempLizardCheck(c,tp,s.lizfilter)
    aux::add_temp_lizard_check(d, c, a.tp, Some(effect_card_fn(lizfilter)), None, None, None, None);
    true
}
/// `s.lizfilter(e,c)`
fn lizfilter(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsOriginalType(TYPE_SYNCHRO) or not c:IsAttribute(ATTRIBUTE_LIGHT|ATTRIBUTE_DARK)
    !card::is_original_type(d, c, TYPE_SYNCHRO) || !card::is_attribute(d, c, ATTRIBUTE_LIGHT | ATTRIBUTE_DARK)
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // return Duel.GetMZoneCount(tp,e:GetHandler())>1
        //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and Duel.IsPlayerCanSpecialSummonMonster(tp,e:GetLabel())
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_m_zone_count(d, a.tp, h.into(), None, None, None).0 > 1
            && duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_player_can_special_summon_monster(d, a.tp, effect::get_label(d, a.e) as u32, duel::MonsterData::default(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, 0);
    true
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    // if s.tktg(e,tp,eg,ep,ev,re,r,rp,0) then
    if tktg(d, &EffArgs { chk: Some(0), ..a.clone() }) {
        // local c=e:GetHandler()
        let _c = effect::get_handler(d, a.e);
        // local tk=e:GetLabel()
        let tk = effect::get_label(d, a.e);
        // for _=1,2 do
        for _ in 1..=2 {
            // local token=Duel.CreateToken(tp,tk)
            let token = duel::create_token(d, a.tp, tk as u32).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
