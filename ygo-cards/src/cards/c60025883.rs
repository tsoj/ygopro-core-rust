//! Duel Link Dragon, the Duel Dragon
use super::prelude::*;

pub const ID: u32 = 60025883;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[60025884], listed_series: &[SET_POWER_TOOL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure
    // Link.AddProcedure(c,nil,2,4,s.lcheck)
    link::add_procedure(d, c, None, 2, Some(4), Some(group_check(lcheck)), None, None);
    // Special Summon 1 "Duel Dragon Token"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetLabel(0)
    effect::set_label(d, e1, &[0]);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e1, Some(condition(|d, _a| duel::is_main_phase(d, None))));
    // e1:SetCost(s.tkcost)
    effect::set_cost(d, e1, Some(cost_target(tkcost)));
    // e1:SetTarget(s.tktg)
    effect::set_target(d, e1, Some(cost_target(tktg)));
    // e1:SetOperation(s.tkop)
    effect::set_operation(d, e1, Some(operation(tkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be battle targeted while you control "Duel Dragon Token"
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_CANNOT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_BATTLE_TARGET);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.tgcon)
    effect::set_condition(d, e2, Some(condition(tgcon)));
    // e2:SetValue(aux.imval1)
    effect::set_value_fn(d, e2, aux::imval1());
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot be targeted by the opponent's effects while you control "Duel Dragon Token"
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e3, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e3:SetValue(aux.tgoval)
    effect::set_value_fn(d, e3, aux::tgoval());
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.lcheck(g,lc,sumtype,tp)
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsType,1,nil,TYPE_SYNCHRO,lc,sumtype,tp)
    group::is_exists(d, g, &|d, x| card::is_type_ctx(d, x, TYPE_SYNCHRO, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
// function s.tkcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn tkcost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(100)
    effect::set_label(d, a.e, &[100]);
    // if chk==0 then return true end
    true
}
// Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,atk,def,lv,race,attribute)
fn token_data(atk: i32, def: i32, lv: u32, race: u64, attribute: u32) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(atk),
        defense: Some(def),
        level: Some(lv),
        race: Some(race),
        attribute: Some(attribute),
    }
}
// function s.cfilter(c,e,tp,zone)
fn cfilter(d: &mut Duel, c: CardId, tp: u8, zone: u32) -> bool {
    // local lv=c:GetLevel()
    let lv = card::get_level(d, c);
    // return (c:IsSetCard(SET_POWER_TOOL) or ((c:GetLevel()==7 or c:GetLevel()==8) and c:IsRace(RACE_DRAGON)))
    //   and c:IsType(TYPE_SYNCHRO) and lv>0 and c:IsAbleToRemoveAsCost()
    //   and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,c:GetAttack(),c:GetDefense(),lv,c:GetRace(),c:GetAttribute())
    //   and Duel.GetMZoneCount(tp,nil,tp,LOCATION_REASON_TOFIELD,zone)>0
    let set_ok = card::is_set_card(d, c, &[SET_POWER_TOOL as u16]) || ((card::get_level(d, c) == 7 || card::get_level(d, c) == 8) && card::is_race(d, c, RACE_DRAGON));
    if !(set_ok && card::is_type(d, c, TYPE_SYNCHRO) && lv > 0 && card::is_able_to_remove_as_cost(d, c, None)) {
        return false;
    }
    let (atk, def, race, attr) = (card::get_attack(d, c), card::get_defense(d, c), card::get_race(d, c), card::get_attribute(d, c));
    duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(atk, def, lv, race, attr), None, None, 0)
        && duel::get_m_zone_count(d, tp, NO_EX, Some(tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 > 0
}
// function s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    // local zone=e:GetHandler():GetFreeLinkedZone()&ZONES_MMZ
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_free_linked_zone(d, h, None) & ZONES_MMZ;
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()~=100 then return false end
        if effect::get_label(d, a.e) != 100 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return zone>0 and Duel.IsExistingMatchingCard(s.cfilter,tp,LOCATION_EXTRA,0,1,nil,e,tp,zone)
        return zone > 0 && duel::is_existing_matching_card(d, Some(&|d, x| cfilter(d, x, a.tp, zone)), a.tp, LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.cfilter,tp,LOCATION_EXTRA,0,1,1,nil,e,tp,zone)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| cfilter(d, x, a.tp, zone)), a.tp, LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g).nil();
    // Duel.Remove(tc,POS_FACEUP,REASON_COST)
    duel::remove(d, tc, POS_FACEUP as u8, REASON_COST, None, None);
    // e:SetLabelObject(tc)
    effect::set_label_object(d, a.e, Value::Card(tc));
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    true
}
// function s.tkop(e,tp,eg,ep,ev,re,r,rp)
fn tkop(d: &mut Duel, a: &EffArgs) {
    // local zone=e:GetHandler():GetFreeLinkedZone()&ZONES_MMZ
    let h = effect::get_handler(d, a.e).nil();
    let zone = card::get_free_linked_zone(d, h, None) & ZONES_MMZ;
    // if zone==0 then return end
    if zone == 0 {
        return;
    }
    // local tc=e:GetLabelObject()
    let tc = effect::get_label_object(d, a.e).as_card().nil();
    let (atk, def, lv, race, attr) = (card::get_attack(d, tc), card::get_defense(d, tc), card::get_level(d, tc), card::get_race(d, tc), card::get_attribute(d, tc));
    // if Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,tc:GetAttack(),tc:GetDefense(),tc:GetLevel(),tc:GetRace(),tc:GetAttribute())
    //   and Duel.GetMZoneCount(tp,nil,tp,LOCATION_REASON_TOFIELD,zone)>0 then
    if duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(atk, def, lv, race, attr), None, None, 0)
        && duel::get_m_zone_count(d, a.tp, NO_EX, Some(a.tp), Some(LOCATION_REASON_TOFIELD), Some(zone)).0 > 0
    {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, a.tp, ID + 1).nil();
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_SET_BASE_ATTACK)
        effect::set_code(d, e1, EFFECT_SET_BASE_ATTACK);
        // e1:SetValue(tc:GetAttack())
        effect::set_value(d, e1, atk as i64);
        // e1:SetReset(RESET_EVENT|RESETS_REDIRECT)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_REDIRECT, 1);
        // token:RegisterEffect(e1)
        card::register_effect(d, token, e1, false);
        // local e2=e1:Clone()
        // e2:SetCode(EFFECT_SET_BASE_DEFENSE)
        // e2:SetValue(tc:GetDefense())
        // token:RegisterEffect(e2)
        let e2 = effect::clone(d, e1);
        effect::set_code(d, e2, EFFECT_SET_BASE_DEFENSE);
        effect::set_value(d, e2, def as i64);
        card::register_effect(d, token, e2, false);
        // local e3=e1:Clone()
        // e3:SetCode(EFFECT_CHANGE_LEVEL)
        // e3:SetValue(tc:GetLevel())
        // token:RegisterEffect(e3)
        let e3 = effect::clone(d, e1);
        effect::set_code(d, e3, EFFECT_CHANGE_LEVEL);
        effect::set_value(d, e3, lv as i64);
        card::register_effect(d, token, e3, false);
        // local e4=e1:Clone()
        // e4:SetCode(EFFECT_CHANGE_RACE)
        // e4:SetValue(tc:GetRace())
        // token:RegisterEffect(e4)
        let e4 = effect::clone(d, e1);
        effect::set_code(d, e4, EFFECT_CHANGE_RACE);
        effect::set_value(d, e4, race as i64);
        card::register_effect(d, token, e4, false);
        // local e5=e1:Clone()
        // e5:SetCode(EFFECT_CHANGE_ATTRIBUTE)
        // e5:SetValue(tc:GetAttribute())
        // token:RegisterEffect(e5)
        let e5 = effect::clone(d, e1);
        effect::set_code(d, e5, EFFECT_CHANGE_ATTRIBUTE);
        effect::set_value(d, e5, attr as i64);
        card::register_effect(d, token, e5, false);
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP,zone)
        duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
    }
}
// function s.tgcon(e)
fn tgcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,id+1),e:GetHandlerPlayer(),LOCATION_ONFIELD,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[ID + 1])), p, LOCATION_ONFIELD, 0, 1, NO_EX)
}
