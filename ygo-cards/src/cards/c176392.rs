//! Koa'ki Meiru Prototype
//! コアキメイル・テストベッド
use super::prelude::*;

pub const ID: u32 = 176392;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[176393], listed_series: &[SET_KOAKI_MEIRU], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // destroy replace
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e1, EFFECT_DESTROY_REPLACE);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.descon)
    effect::set_condition(d, e1, Some(condition(descon)));
    // e1:SetTarget(s.destg)
    effect::set_target(d, e1, Some(cost_target(destg)));
    // e1:SetValue(s.repval)
    effect::set_value_fn(d, e1, effect_card_fn(repval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // token
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(condition(spcon)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_END)
    duel::is_phase(d, PHASE_END)
}
/// `s.rfilter(c)`
fn rfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLocation(LOCATION_MZONE) and c:IsSetCard(SET_KOAKI_MEIRU) and not c:IsReason(REASON_REPLACE)
    card::is_faceup(d, c) && card::is_location(d, c, LOCATION_MZONE) && card::is_set_card(d, c, &[SET_KOAKI_MEIRU as u16]) && !card::is_reason(d, c, REASON_REPLACE)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsDestructable(e) and eg:IsExists(s.rfilter,1,e:GetHandler()) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_destructable(d, c, Some(a.e)) && group::is_exists(d, &eg, &rfilter, 1, c.into());
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        // Duel.Destroy(c,REASON_EFFECT|REASON_REPLACE)
        duel::destroy(d, CardSet::from_card(c), REASON_EFFECT | REASON_REPLACE, None, None);
        // return true
        true
    } else {
        // else return false end
        false
    }
}
/// `s.repval(e,c)`
fn repval(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_KOAKI_MEIRU) and c~=e:GetHandler()
    let h = effect::get_handler(d, e).nil();
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_KOAKI_MEIRU as u16]) && c != h
}
/// `s.spfilter(c)`
fn spfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousSetCard(SET_KOAKI_MEIRU)
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_previous_position(d, c, POS_FACEUP) && card::is_previous_set_card(d, c, &[SET_KOAKI_MEIRU as u16])
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsPhase(PHASE_END) and eg:IsExists(s.spfilter,1,nil)
    let eg = a.eg.clone().nil();
    duel::is_phase(d, PHASE_END) && group::is_exists(d, &eg, &spfilter, 1, NO_EX)
}
/// The token data of `Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_KOAKI_MEIRU,TYPES_TOKEN,1800,1800,4,RACE_ROCK,ATTRIBUTE_EARTH)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_KOAKI_MEIRU as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(1800),
        defense: Some(1800),
        level: Some(4),
        race: Some(RACE_ROCK),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_KOAKI_MEIRU,TYPES_TOKEN,1800,1800,4,RACE_ROCK,ATTRIBUTE_EARTH) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP as u8), Some(tp), 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_KOAKI_MEIRU,TYPES_TOKEN,1800,1800,4,RACE_ROCK,ATTRIBUTE_EARTH) then
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), Some(POS_FACEUP as u8), Some(tp), 0)
    {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(token), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
