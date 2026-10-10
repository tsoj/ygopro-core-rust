//! Link Devotee
//! scripted by Larry126
use super::prelude::*;

pub const ID: u32 = 48068378;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_LINK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(crate::procs::mat_filter(|d, c, lc, sumtype, tp| matfilter(d, c, lc, sumtype, tp))), 1, Some(1), None, None, None);
    // splimit
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetOperation(s.ls)
    effect::set_operation(d, e1, Some(operation(ls)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_RELEASE)
    effect::set_code(d, e2, EVENT_RELEASE);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EVENT_LEAVE_FIELD_P)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD_P);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // e3:SetOperation(s.chk)
    effect::set_operation(d, e3, Some(operation(chk)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.chk(e,tp,eg,ep,ev,re,r,rp)`
fn chk(d: &mut Duel, a: &EffArgs) {
    // e:GetLabelObject():SetLabel(e:GetHandler():GetMutualLinkedGroupCount())
    let obj = effect::get_label_object(d, a.e).as_effect().nil();
    let h = effect::get_handler(d, a.e).nil();
    let n = card::get_mutual_linked_group_count(d, h) as i64;
    effect::set_label(d, obj, &[n]);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:IsLevelBelow(4) and c:IsRace(RACE_CYBERSE,lc,sumtype,tp)
    card::is_level_below(d, c, 4) && card::is_race_ctx(d, c, RACE_CYBERSE, lc, sumtype, Some(tp))
}
/// `s.ls(e,tp,eg,ep,ev,re,r,rp,c)`
fn ls(d: &mut Duel, a: &EffArgs) {
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.splimit(e,c,tp,sumtp,sumpos)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // return c:IsLinkMonster() and c:GetLink()>=3 and (sumtp&SUMMON_TYPE_LINK)==SUMMON_TYPE_LINK
    card::is_link_monster(d, c) && card::get_link(d, c) >= 3 && (a.int(3) as u64 & SUMMON_TYPE_LINK as u64) == SUMMON_TYPE_LINK as u64
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetLabel()>0
    effect::get_label(d, a.e) > 0
}
/// The Token Link's data: `TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_LIGHT`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_CYBERSE),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    if a.chk == Some(0) {
        // return not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
        //     and Duel.GetLocationCount(tp,LOCATION_MZONE)>1
        //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_LINK,0,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_LIGHT)
        return duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 1
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_LINK, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) or Duel.GetLocationCount(tp,LOCATION_MZONE)<2
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_LINK,0,TYPES_TOKEN,0,0,1,RACE_CYBERSE,ATTRIBUTE_LIGHT) then return end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some()
        || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 < 2
        || !duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_LINK, token_data(), None, None, 0)
    {
        return;
    }
    // for i=1,2 do
    for _ in 0..2 {
        // local token=Duel.CreateToken(tp,TOKEN_LINK)
        let token = duel::create_token(d, a.tp, TOKEN_LINK).nil();
        // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
