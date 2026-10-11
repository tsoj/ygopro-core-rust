//! 魔救の輝跡
use super::prelude::*;

pub const ID: u32 = 41458361;
const TOKEN_GLIMMER: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_GLIMMER], listed_series: &[SET_ADAMANCIPATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2 Effect Monsters, including a Synchro Monster
    // Link.AddProcedure(c,nil,2,2,s.linkmatcheck)
    link::add_procedure(d, c, None, 2, Some(2), Some(group_check(|d, g, lc, st, tp| linkmatcheck(d, g, lc, st, tp))), None, None);
    // If this card is Link Summoned, or a monster(s) is Special Summoned to a zone(s) this card points to: You can target 1 monster in your field or GY that has a Level; Special Summon 1 "Glimmer Token" (Rock/LIGHT/ATK 0/DEF 0) with the same Level
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1a, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SPSUMMON_SUCCESS);
    // e1a:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCondition(function(e)
    //     return e:GetHandler():IsLinkSummoned()
    // end)
    effect::set_condition(d, e1a, Some(condition(link_summoned_cond)));
    // e1a:SetTarget(s.tkntg)
    effect::set_target(d, e1a, Some(cost_target(tkntg)));
    // e1a:SetOperation(s.tknop)
    effect::set_operation(d, e1a, Some(operation(tknop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=e1a:Clone()
    let e1b = effect::clone(d, e1a);
    // e1b:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1b, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1b:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1b, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1b:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1b, LOCATION_MZONE);
    // e1b:SetCondition(aux.zptcon(nil))
    effect::set_condition(d, e1b, Some(aux::zptcon(None)));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // When another monster on the field activates its effect (Quick Effect): You can place it on top of the Deck
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TODECK)
    effect::set_category(d, e2, CATEGORY_TODECK);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.tdcon)
    effect::set_condition(d, e2, Some(condition(tdcon)));
    // e2:SetTarget(s.tdtg)
    effect::set_target(d, e2, Some(cost_target(tdtg)));
    // e2:SetOperation(s.tdop)
    effect::set_operation(d, e2, Some(operation(tdop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
fn linkmatcheck(d: &mut Duel, g: &CardSet, lc: CardId, sumtype: u64, tp: u8) -> bool {
    // return g:IsExists(Card.IsType,1,nil,TYPE_SYNCHRO,lc,sumtype,tp)
    group::is_exists(d, g, &|d, c| card::is_type_ctx(d, c, TYPE_SYNCHRO, Some(lc), sumtype, Some(tp)), 1, NO_EX)
}
fn link_summoned_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsLinkSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_link_summoned(d, h)
}
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // c:IsFaceup() and c:HasLevel()
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_GLIMMER,0,TYPES_TOKEN,0,0,c:GetLevel(),RACE_ROCK,ATTRIBUTE_LIGHT)
    if !(card::is_faceup(d, c) && card::has_level(d, c)) {
        return false;
    }
    let lv = card::get_level(d, c);
    duel::is_player_can_special_summon_monster(d, tp, TOKEN_GLIMMER, glimmer_data(lv), None, None, 0)
}
fn glimmer_data(level: u32) -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(level),
        race: Some(RACE_ROCK),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
fn tkntg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE|LOCATION_GRAVE) and chkc:IsControler(tp) and s.tgfilter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE | LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && tgfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,1,1,nil,e,tp)
    let _g = duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
fn tknop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_GLIMMER,0,TYPES_TOKEN,0,0,tc:GetLevel(),RACE_ROCK,ATTRIBUTE_LIGHT) then
    let tc_lv = card::get_level(d, tc);
    if card::is_relate_to_effect(d, tc, a.e)
        && card::is_faceup(d, tc)
        && duel::is_player_can_special_summon_monster(d, tp, TOKEN_GLIMMER, glimmer_data(tc_lv), None, None, 0)
    {
        // local token=Duel.CreateToken(tp,TOKEN_GLIMMER)
        let token = duel::create_token(d, tp, TOKEN_GLIMMER).nil();
        // token:Level(tc:GetLevel())
        let lv = card::get_level(d, tc);
        card::info_level(d, token, Some(lv));
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, CardSet::from_card(token), 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
fn tdcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsMonsterEffect() and re:GetHandler()~=e:GetHandler() and Chain.IsTriggeringLocation(ev,LOCATION_MZONE)
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    let h = effect::get_handler(d, a.e).nil();
    effect::is_monster_effect(d, re)
        && rc != h
        && chain::is_triggering(d, a.ev as u8, chain::Prop::Location, &[LOCATION_MZONE as i64]).truthy()
}
fn tdtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if chk==0 then return rc:IsRelateToEffect(re) and rc:IsAbleToDeck() end
    if a.chk == Some(0) {
        return card::is_relate_to_effect(d, rc, re) && card::is_able_to_deck(d, rc);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TODECK,rc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TODECK, Some(CardSet::from_card(rc)), 1, a.tp, 0);
    true
}
fn tdop(d: &mut Duel, a: &EffArgs) {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // if rc:IsRelateToEffect(re) then
    if card::is_relate_to_effect(d, rc, re) {
        // Duel.SendtoDeck(rc,nil,SEQ_DECKTOP,REASON_EFFECT)
        duel::send_to_deck(d, rc, None, SEQ_DECKTOP as i32, REASON_EFFECT, None);
    }
}
