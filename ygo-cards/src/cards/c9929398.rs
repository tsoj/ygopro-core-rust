//! Blackwing - Gofu the Vague Shadow
use super::prelude::*;

pub const ID: u32 = 9929398;
const TOKEN_VAGUE_SHADOW: u32 = ID + 1;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_VAGUE_SHADOW], listed_series: &[SET_BLACKWING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Must first be Special Summoned (from your hand) while you control no monsters
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND)
    effect::set_range(d, e0, LOCATION_HAND);
    // e0:SetCondition(s.selfspcon)
    effect::set_condition(d, e0, Some(lua_fn(selfspcon)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon 2 "Vague Shadow Tokens" (Winged Beast/DARK/Level 1/ATK 0/DEF 0), but they cannot be Tributed or be used as Synchro Material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsSummonLocation(LOCATION_HAND) end)
    effect::set_condition(d, e1, Some(condition(summon_from_hand_cond)));
    // e1:SetTarget(s.tokentg)
    effect::set_target(d, e1, Some(cost_target(tokentg)));
    // e1:SetOperation(s.tokenop)
    effect::set_operation(d, e1, Some(operation(tokenop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Blackwing" Synchro Monster whose Level equals the total Levels the banished monsters had on the field from your GY as a Tuner
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCost(s.gyspcost)
    effect::set_cost(d, e2, Some(cost_target(gyspcost)));
    // e2:SetTarget(s.gysptg)
    effect::set_target(d, e2, Some(cost_target(gysptg)));
    // e2:SetOperation(s.gyspop)
    effect::set_operation(d, e2, Some(operation(gyspop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `function(e,c) if c==nil then return true end ... end`
fn selfspcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0,nil)==0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    duel::get_field_group_count(d, tp, LOCATION_MZONE, 0) == 0 && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
}
fn summon_from_hand_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSummonLocation(LOCATION_HAND)
    let h = effect::get_handler(d, a.e).nil();
    card::is_summon_location(d, h, LOCATION_HAND)
}
fn token_data() -> duel::MonsterData {
    // TYPES_TOKEN, 0, 0, level 1, RACE_WINGEDBEAST, ATTRIBUTE_DARK
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_WINGEDBEAST),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
fn tokentg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>=2
    //     and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_VAGUE_SHADOW,0,TYPES_TOKEN,0,0,1,RACE_WINGEDBEAST,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && duel::is_player_can_special_summon_monster(d, tp, TOKEN_VAGUE_SHADOW, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 2, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, 0);
    true
}
fn tokenop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>=2 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and Duel.IsPlayerCanSpecialSummonMonster(...) then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 >= 2
        && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
        && duel::is_player_can_special_summon_monster(d, tp, TOKEN_VAGUE_SHADOW, token_data(), None, None, 0)
    {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // for i=1,2 do
        for _ in 0..2 {
            // local token=Duel.CreateToken(tp,TOKEN_VAGUE_SHADOW)
            let token = duel::create_token(d, tp, TOKEN_VAGUE_SHADOW).nil();
            // Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP)
            duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
            // They cannot be Tributed or be used as Synchro Material
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3303)
            effect::set_description(d, e1, 3303);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_UNRELEASABLE_SUM)
            effect::set_code(d, e1, EFFECT_UNRELEASABLE_SUM);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // token:RegisterEffect(e1)
            card::register_effect(d, token, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_UNRELEASABLE_NONSUM)
            effect::set_code(d, e2, EFFECT_UNRELEASABLE_NONSUM);
            // token:RegisterEffect(e2)
            card::register_effect(d, token, e2, false);
            // local e3=e2:Clone()
            let e3 = effect::clone(d, e2);
            // e3:SetDescription(3310)
            effect::set_description(d, e3, 3310);
            // e3:SetCode(EFFECT_CANNOT_BE_SYNCHRO_MATERIAL)
            effect::set_code(d, e3, EFFECT_CANNOT_BE_SYNCHRO_MATERIAL);
            // token:RegisterEffect(e3)
            card::register_effect(d, token, e3, false);
        }
        // Duel.SpecialSummonComplete()
        duel::special_summon_complete(d);
    }
}
fn gyspcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsType(TYPE_TUNER) and c:HasLevel() and c:IsFaceup() and c:IsAbleToRemoveAsCost()
    !card::is_type(d, c, TYPE_TUNER) && card::has_level(d, c) && card::is_faceup(d, c) && card::is_able_to_remove_as_cost(d, c, None)
}
/// `s.rescon(sg,e,tp,mg)`: `(valid, stop)`.
fn rescon(d: &mut Duel, sg: &CardSet, e: Option<EffectId>, tp: u8, _mg: &CardSet, _c: Option<CardId>) -> (bool, bool) {
    let e = e.nil();
    // return Duel.GetMZoneCount(tp,sg)>0 and Duel.IsExistingTarget(s.gyspfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp,sg:GetSum(Card.GetLevel)+e:GetHandler():GetLevel())
    let res = duel::get_m_zone_count(d, tp, sg.into(), None, None, None).0 > 0 && {
        let sum = group::get_sum(d, sg, &|d, x| card::get_level(d, x) as i64);
        let h = effect::get_handler(d, e).nil();
        let lv = sum + card::get_level(d, h) as i64;
        duel::is_existing_target(d, Some(&|d, x| gyspfilter(d, x, e, tp, lv)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
    };
    (res, false)
}
fn gyspfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, lv: i64) -> bool {
    // if not (c:IsSetCard(SET_BLACKWING) and c:IsSynchroMonster() and c:IsLevel(lv)) then return false end
    if !(card::is_set_card(d, c, &[SET_BLACKWING as u16]) && card::is_synchro_monster(d, c) && card::is_level(d, c, &[lv as u32])) {
        return false;
    }
    // c:AssumeProperty(ASSUME_TYPE,c:GetType()|TYPE_TUNER)
    let ty = card::get_type(d, c);
    card::assume_property(d, c, ASSUME_TYPE, (ty | TYPE_TUNER) as u64);
    // return c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
fn gyspcost(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local rg=Duel.GetMatchingGroup(s.gyspcostfilter,tp,LOCATION_MZONE,0,c)
    let rg = duel::get_matching_group(d, Some(&gyspcostfilter), tp, LOCATION_MZONE, 0, c.into());
    // if chk==0 then return c:IsAbleToRemoveAsCost() and c:HasLevel() and #rg>0
    //     and aux.SelectUnselectGroup(rg,e,tp,1,#rg,s.rescon,0) end
    if a.chk == Some(0) {
        return card::is_able_to_remove_as_cost(d, c, None)
            && card::has_level(d, c)
            && !rg.is_empty()
            && matches!(
                aux::select_unselect_group(d, &rg, Some(e), tp, Some(1), Some(rg.len()), Some(&rescon), 0, tp, None, None, None, false),
                Err(true)
            );
    }
    // local g=aux.SelectUnselectGroup(rg,e,tp,1,#rg,s.rescon,1,tp,HINTMSG_REMOVE,s.rescon)
    let g = aux::select_unselect_group(
        d,
        &rg,
        Some(e),
        tp,
        Some(1),
        Some(rg.len()),
        Some(&rescon),
        1,
        tp,
        Some(HINTMSG_REMOVE as u64),
        Some(&rescon),
        None,
        false,
    )
    .unwrap_or_default();
    // e:SetLabel(g:GetSum(Card.GetLevel)+c:GetLevel())
    let sum = group::get_sum(d, &g, &|d, x| card::get_level(d, x) as i64);
    let lv = sum + card::get_level(d, c) as i64;
    effect::set_label(d, e, &[lv]);
    // Duel.Remove(g+c,POS_FACEUP,REASON_COST)
    let mut gc = g.clone();
    group::add_card(&mut gc, &CardSet::from_card(c));
    duel::remove(d, gc, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
fn gysptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local label = e:GetLabel() (read where the Lua reads it)
    let label = effect::get_label(d, e);
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.gyspfilter(chkc,e,tp,e:GetLabel()) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_GRAVE) && gyspfilter(d, chkc, e, tp, label);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectTarget(tp,s.gyspfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp,e:GetLabel())
    let g = duel::select_target(d, tp, Some(&|d, x| gyspfilter(d, x, e, tp, label)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, g, 1, tp, 0);
    true
}
fn gyspop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc.nil(), e) {
        return;
    }
    let tc = tc.nil();
    // tc:AssumeProperty(ASSUME_TYPE,tc:GetType()|TYPE_TUNER)
    let ty = card::get_type(d, tc);
    card::assume_property(d, tc, ASSUME_TYPE, (ty | TYPE_TUNER) as u64);
    // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
    if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        // It is treated as a Tuner
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_ADD_TYPE)
        effect::set_code(d, e1, EFFECT_ADD_TYPE);
        // e1:SetValue(TYPE_TUNER)
        effect::set_value(d, e1, TYPE_TUNER as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
