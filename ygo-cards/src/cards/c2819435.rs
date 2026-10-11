//! Pacifis, the Phantasm City
use super::prelude::*;

pub const ID: u32 = 2819435;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[2819436], listed_series: &[SET_PHANTASM_SPIRAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Search 1 "Phantasm Spiral" card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,0,EFFECT_COUNT_CODE_SINGLE)
    effect::set_count_limit(d, e2, 1, 0, 0, EFFECT_COUNT_CODE_SINGLE as u8);
    // e2:SetCondition(s.thcon)
    effect::set_condition(d, e2, Some(condition(thcon)));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Special Summon 1 "Phantasm Spiral Token"
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e6, aux::stringid(ID, 1));
    // e6:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e6, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e6:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e6, EFFECT_FLAG_DELAY, 0);
    // e6:SetCode(EVENT_CHAINING)
    effect::set_code(d, e6, EVENT_CHAINING);
    // e6:SetRange(LOCATION_FZONE)
    effect::set_range(d, e6, LOCATION_FZONE);
    // e6:SetCondition(s.spcon)
    effect::set_condition(d, e6, Some(condition(spcon)));
    // e6:SetCost(s.cost)
    effect::set_cost(d, e6, Some(cost_target(cost_fn)));
    // e6:SetTarget(s.sptg)
    effect::set_target(d, e6, Some(cost_target(sptg)));
    // e6:SetOperation(s.spop)
    effect::set_operation(d, e6, Some(operation(spop)));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SUMMON, lua_fn(counterfilter));
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(counterfilter));
}
// s.listed_names: the Phantasm Spiral Token
// function s.counterfilter(c)
fn counterfilter(d: &mut Duel, a: Args) -> bool {
    // return not c:IsType(TYPE_EFFECT)
    !card::is_type(d, a.c(0), TYPE_EFFECT)
}
// function s.cost(e,tp,eg,ep,ev,re,r,rp,chk)
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SUMMON)==0
    if a.chk == Some(0) {
        // and Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SUMMON) == 0 && duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // Cannot Normal or Special Summon Effect Monsters
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| splimit(d, c))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    true
}
// function s.splimit(e,c,sump,sumtype,sumpos,targetp,se)
fn splimit(d: &mut Duel, c: CardId) -> bool {
    // if c:IsMonster() then
    if card::is_monster(d, c) {
        // return c:IsType(TYPE_EFFECT)
        card::is_type(d, c, TYPE_EFFECT)
    } else {
        // return c:IsOriginalType(TYPE_EFFECT)
        card::is_original_type(d, c, TYPE_EFFECT)
    }
}
// function s.thcon(e,tp,eg,ep,ev,re,r,rp)
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=eg:GetFirst()
    let eg = a.eg.clone().nil();
    // return #eg==1 and tc:IsSummonPlayer(tp) and tc:IsFaceup() and tc:IsType(TYPE_NORMAL)
    eg.len() == 1 && {
        let tc = group::get_first(d, &eg).nil();
        card::is_summon_player(d, tc, a.tp) && card::is_faceup(d, tc) && card::is_type(d, tc, TYPE_NORMAL)
    }
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_PHANTASM_SPIRAL) and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_PHANTASM_SPIRAL as u16]) && card::is_able_to_hand(d, c, None)
}
// function s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    let desc = effect::get_description(d, a.e);
    duel::hint(d, HINT_OPSELECTED as u8, 1 - a.tp, desc);
    true
}
// function s.thop(e,tp,eg,ep,ev,re,r,rp)
fn thop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&thfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - a.tp, g);
    }
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp
    a.rp == 1 - a.tp
}
// function s.tknfilter(c)
fn tknfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsType(TYPE_TOKEN) or c:IsOriginalType(TYPE_TOKEN)
    card::is_type(d, c, TYPE_TOKEN) || card::is_original_type(d, c, TYPE_TOKEN)
}
// The Phantasm Spiral Token: (tp, id+1, SET_PHANTASM_SPIRAL, TYPES_TOKEN, 2000, 2000, 6, RACE_WYRM, ATTRIBUTE_WATER)
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_PHANTASM_SPIRAL as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(2000),
        defense: Some(2000),
        level: Some(6),
        race: Some(RACE_WYRM as u64),
        attribute: Some(ATTRIBUTE_WATER),
    }
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            // and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_PHANTASM_SPIRAL,TYPES_TOKEN,2000,2000,6,RACE_WYRM,ATTRIBUTE_WATER)
            && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0)
            // and e:GetHandler():GetFlagEffect(2819435)==0
            && card::get_flag_effect(d, h, ID) == 0
            // and not Duel.IsExistingMatchingCard(s.tknfilter,tp,LOCATION_ONFIELD,0,1,nil) end
            && !duel::is_existing_matching_card(d, Some(&tknfilter), a.tp, LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // e:GetHandler():RegisterFlagEffect(2819435,RESET_EVENT|RESETS_STANDARD|RESET_CHAIN,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_CHAIN, 0, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        // and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,SET_PHANTASM_SPIRAL,TYPES_TOKEN,2000,2000,6,RACE_WYRM,ATTRIBUTE_WATER) then
        && duel::is_player_can_special_summon_monster(d, a.tp, ID + 1, token_data(), None, None, 0)
    {
        // local token=Duel.CreateToken(tp,id+1)
        let token = duel::create_token(d, a.tp, ID + 1).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
