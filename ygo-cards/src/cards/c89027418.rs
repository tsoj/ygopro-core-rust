//! Imposter Shift
//! JP name
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 89027418;
/// `local TOKEN_IMPOSTER=id+1`
const TOKEN_IMPOSTER: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[TOKEN_IMPOSTER],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // When a monster effect that was activated on your opponent's field by targeting a card(s) on the field resolves, if any of those targets are in a different column than that opponent's monster, they can banish 1 card from their GY. If they did not, negate the activated effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_REMOVE+CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_REMOVE + CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCondition(s.discon)
    effect::set_condition(d, e1, Some(condition(discon)));
    // e1:SetOperation(s.disop)
    effect::set_operation(d, e1, Some(operation(disop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can banish 1 monster from your GY; Special Summon 1 "Imposter Token" (Psychic/EARTH/ATK 800/DEF 800) with the same Level as that monster. You can only use this effect of "Imposter Shift" once per turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(s.tokencost)
    effect::set_cost(d, e2, Some(cost_target(tokencost)));
    // e2:SetTarget(s.tokentg)
    effect::set_target(d, e2, Some(cost_target(tokentg)));
    // e2:SetOperation(s.tokenop)
    effect::set_operation(d, e2, Some(operation(tokenop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.discon(e,tp,eg,ep,ev,re,r,rp)
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local opp=1-tp
    let opp = 1 - a.tp;
    let Some(re) = a.re else {
        return false;
    };
    let ch = a.ev as u8;
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // if re:IsMonsterEffect() and rc:IsRelateToEffect(re) and rc:IsControler(opp) and re:IsHasProperty(EFFECT_FLAG_CARD_TARGET)
    //     and Chain.IsTriggeringLocation(ev,LOCATION_MZONE) and Chain.IsTriggeringControler(ev,opp) and Chain.IsDisablable(ev) then
    if effect::is_monster_effect(d, re)
        && card::is_relate_to_effect(d, rc, re)
        && card::is_controler(d, rc, opp)
        && effect::is_has_property(d, re, EFFECT_FLAG_CARD_TARGET, 0)
        && chain::is_triggering(d, ch, chain::Prop::Location, &[LOCATION_MZONE as i64]).truthy()
        && chain::is_triggering(d, ch, chain::Prop::Controler, &[opp as i64]).truthy()
        && duel::is_chain_disablable(d, ch)
    {
        // local tg=Chain.GetTargetCards(ev)
        // return tg and tg:IsExists(aux.AND(Card.IsRelateToEffect,Card.IsOnField),1,nil,re)
        let tg = chain::get_target_cards(d, ch).as_group().cloned();
        if let Some(tg) = tg {
            return group::is_exists(d, &tg, &|d: &mut Duel, c: CardId| card::is_relate_to_effect(d, c, re) && card::is_on_field(d, c), 1, NO_EX);
        }
        return false;
    }
    false
}
// function s.disconfilter(c,seq,ctrl)
fn disconfilter(d: &mut Duel, c: CardId, seq: u32, ctrl: u8) -> bool {
    // return not c:IsColumn(seq,ctrl,LOCATION_MZONE)
    !card::is_column(d, c, seq, Some(ctrl), Some(LOCATION_MZONE))
}
// function s.disop(e,tp,eg,ep,ev,re,r,rp)
fn disop(d: &mut Duel, a: &EffArgs) {
    // local opp=1-tp
    let opp = 1 - a.tp;
    let re = a.re.nil();
    let ch = a.ev as u8;
    // local tg=Chain.GetTargetCards(ev):Filter(aux.AND(Card.IsRelateToEffect,Card.IsOnField),nil,re)
    let tg = chain::get_target_cards(d, ch).as_group().cloned().nil();
    let tg = group::filter(d, &tg, &|d: &mut Duel, c: CardId| card::is_relate_to_effect(d, c, re) && card::is_on_field(d, c), NO_EX);
    // if tg:IsExists(s.disconfilter,1,nil,re:GetHandler():GetSequence(),opp) and Duel.IsExistingMatchingCard(Card.IsAbleToRemove,opp,LOCATION_GRAVE,0,1,nil,opp)
    //     and Duel.SelectYesNo(opp,aux.Stringid(id,1)) then
    let rc = effect::get_handler(d, re).nil();
    let seq = card::get_sequence(d, rc);
    if group::is_exists(d, &tg, &|d: &mut Duel, c: CardId| disconfilter(d, c, seq, opp), 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d: &mut Duel, c: CardId| card::is_able_to_remove(d, c, Some(opp), None, None)), opp, LOCATION_GRAVE, 0, 1, NO_EX)
        && duel::select_yes_no(d, opp, aux::stringid(ID, 1))
    {
        // Duel.Hint(HINT_SELECTMSG,opp,HINTMSG_REMOVE)
        duel::hint(d, HINT_SELECTMSG as u8, opp, HINTMSG_REMOVE as u64);
        // local g=Duel.SelectMatchingCard(opp,Card.IsAbleToRemove,opp,LOCATION_GRAVE,0,1,1,nil,opp)
        let g = duel::select_matching_card(d, opp, Some(&|d: &mut Duel, c: CardId| card::is_able_to_remove(d, c, Some(opp), None, None)), opp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX)
            .nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.Remove(g,POS_FACEUP,REASON_EFFECT,nil,opp)
            duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, Some(opp));
        }
    } else {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, ch);
    }
}
// function s.tokencostfilter(c,tp)
fn tokencostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:HasLevel() and c:IsAbleToRemoveAsCost() and Duel.GetMZoneCount(tp,c)>0 and aux.SpElimFilter(c,true)
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_IMPOSTER,0,TYPES_TOKEN,800,800,c:GetLevel(),RACE_PSYCHIC,ATTRIBUTE_EARTH)
    if !card::has_level(d, c) || !card::is_able_to_remove_as_cost(d, c, None) {
        return false;
    }
    if duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 <= 0 {
        return false;
    }
    if !aux::sp_elim_filter(d, c, true, false) {
        return false;
    }
    let lv = card::get_level(d, c);
    duel::is_player_can_special_summon_monster(d, tp, TOKEN_IMPOSTER, imposter_data(lv), None, None, 0)
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_IMPOSTER,0,TYPES_TOKEN,800,800,level,RACE_PSYCHIC,ATTRIBUTE_EARTH)`
fn imposter_data(level: u32) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(800),
        defense: Some(800),
        level: Some(level),
        race: Some(RACE_PSYCHIC),
        attribute: Some(ATTRIBUTE_EARTH),
    }
}
// function s.tokencost(e,tp,eg,ep,ev,re,r,rp,chk)
fn tokencost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tokencostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d: &mut Duel, c: CardId| tokencostfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_REMOVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tokencostfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&|d: &mut Duel, c: CardId| tokencostfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX)
        .nil();
    let sc = group::get_first(d, &g).nil();
    // Duel.Remove(sc,POS_FACEUP,REASON_COST)
    duel::remove(d, sc, POS_FACEUP as u8, REASON_COST, None, None);
    // e:GetChainData().token_level=sc:GetLevel()
    let level = card::get_level(d, sc);
    chain::set_effect_data(d, a.e, None, chain::field::CARD, Value::Int(level as i64));
    true
}
// function s.tokentg(e,tp,eg,ep,ev,re,r,rp,chk)
fn tokentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
// function s.tokenop(e,tp,eg,ep,ev,re,r,rp)
fn tokenop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local token_level=e:GetChainData().token_level
    let token_level = chain::effect_data(d, a.e, None, chain::field::CARD).int() as u32;
    // if Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_IMPOSTER,0,TYPES_TOKEN,800,800,token_level,RACE_PSYCHIC,ATTRIBUTE_EARTH) then
    if duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_IMPOSTER, imposter_data(token_level), None, None, 0) {
        // local token=Duel.CreateToken(tp,TOKEN_IMPOSTER)
        let token = duel::create_token(d, a.tp, TOKEN_IMPOSTER).nil();
        // token:Level(token_level)
        card::info_level(d, token, Some(token_level));
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
