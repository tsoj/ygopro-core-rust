//! Bellcat Fighter
use super::prelude::*;

pub const ID: u32 = 22953211;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[22953212], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // link summon
    // Link.AddProcedure(c,nil,3,3,s.lcheck)
    link::add_procedure(d, c, None, 3, Some(3), Some(group_check(lcheck)), None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // token
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYING);
    // e1:SetCondition(aux.bdocon)
    effect::set_condition(d, e1, Some(aux::bdocon()));
    // e1:SetTarget(s.tktg)
    effect::set_target(d, e1, Some(cost_target(tktg)));
    // e1:SetOperation(s.tkop)
    effect::set_operation(d, e1, Some(operation(tkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
fn lcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsType,1,nil,TYPE_TOKEN,lc,sumtype,tp)
    group::is_exists(d, g, &|d, x| card::is_type_ctx(d, x, TYPE_TOKEN, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
fn tg_md() -> duel::MonsterData {
    // Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,2000,2000,4,RACE_MACHINE,ATTRIBUTE_WIND)
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(2000),
        defense: Some(2000),
        level: Some(4),
        race: Some(RACE_MACHINE),
        attribute: Some(ATTRIBUTE_WIND),
    }
}
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsPlayerCanSpecialSummonMonster(...) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID + 1, tg_md(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, 0);
    true
}
fn tkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //   or not Duel.IsPlayerCanSpecialSummonMonster(...) then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !duel::is_player_can_special_summon_monster(d, tp, ID + 1, tg_md(), None, None, 0)
    {
        return;
    }
    // local token=Duel.CreateToken(tp,id+1)
    let token = duel::create_token(d, tp, ID + 1).nil();
    // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP_DEFENSE)
    duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None);
}
