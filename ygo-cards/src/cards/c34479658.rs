//! Dark Alligator
//! Scripted by Naim
use super::prelude::*;
use std::sync::Arc;

pub const ID: u32 = 34479658;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[34479659],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Summon with 1 tribute
    // local ns1=aux.AddNormalSummonProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _ns1 = aux::add_normal_summon_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE as u32),
        Some(aux::stringid(ID, 0)),
        Some(Arc::new(otfilter)),
        None,
    );
    // local ns2=aux.AddNormalSetProcedure(c,true,true,1,1,SUMMON_TYPE_TRIBUTE,aux.Stringid(id,0),s.otfilter)
    let _ns2 = aux::add_normal_set_procedure(
        d,
        c,
        true,
        true,
        Some(1),
        Some(1),
        Some(SUMMON_TYPE_TRIBUTE as u32),
        Some(aux::stringid(ID, 0)),
        Some(Arc::new(otfilter)),
        None,
    );
    // Special Summon tokens
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetCondition(s.spcond)
    effect::set_condition(d, e1, Some(condition(spcond)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Tribute check
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e2, EFFECT_MATERIAL_CHECK);
    // e2:SetValue(s.valcheck)
    effect::set_value_fn(d, e2, lua_fn(valcheck));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // Add to hand
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_SEARCH+CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_SEARCH + CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.thcond)
    effect::set_condition(d, e3, Some(condition(thcond)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.otfilter(c)
fn otfilter(d: &mut Duel, c: CardId, _tp: u8) -> bool {
    // return c:IsRace(RACE_REPTILE)
    card::is_race(d, c, RACE_REPTILE)
}
// function s.valcheck(e,c)
fn valcheck(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(1);
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // local ct=g:FilterCount(Card.IsRace,nil,RACE_REPTILE)
    let ct = group::filter_count(d, &g, &|d, x| card::is_race(d, x, RACE_REPTILE), NO_EX) as i64;
    // e:SetLabel(ct)
    effect::set_label(d, e, &[ct]);
    // e:GetLabelObject():SetLabel(ct)
    let lo = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, lo, &[ct]);
}
// function s.spcond(e,tp,eg,ep,ev,re,r,rp)
fn spcond(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsTributeSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_tribute_summoned(d, h)
}
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(2000),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_REPTILE),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and e:GetLabel()>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,2000,0,1,RACE_REPTILE,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && effect::get_label(d, a.e) > 0
            && duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local ct,zones=e:GetLabel(),Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ct = effect::get_label(d, e);
    let zones = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 as i64;
    // if ct<1 or zones<1 or not Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,2000,0,1,RACE_REPTILE,ATTRIBUTE_DARK) then return end
    if ct < 1 || zones < 1 || !duel::is_player_can_special_summon_monster(d, tp, ID + 1, token_data(), None, None, 0) {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) and ct>=2 then ct=1 end
    if duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() && ct >= 2 {
        ct = 1;
    }
    // if ct>zones then ct=zones end
    if ct > zones {
        ct = zones;
    }
    // for i=1,ct do
    for _ in 0..ct {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, tp, ID + 1).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
// function s.thcond(e,tp,eg,ep,ev,re,r,rp)
fn thcond(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsTributeSummoned() and (c:IsReason(REASON_BATTLE) or rp==1-tp)
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_tribute_summoned(d, c)
        && (card::is_reason(d, c, REASON_BATTLE) || a.rp == 1 - a.tp)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_REPTILE) and c:IsAbleToHand() and not c:IsCode(id)
    card::is_race(d, c, RACE_REPTILE) && card::is_able_to_hand(d, c, None) && !card::is_code(d, c, &[ID])
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, &g, None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, &g);
    }
}
