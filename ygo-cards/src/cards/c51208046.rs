//! Trickstar Live Stage
//! scripted by Logical Nonsense
//! Substitute ID
use super::prelude::*;

pub const ID: u32 = 51208046;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_TRICKSTAR], listed_series: &[SET_TRICKSTAR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special summon a "Trickstar" token if you control a "Trickstar" link, ignition effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.spcon1)
    effect::set_condition(d, e2, Some(condition(spcon1)));
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special summon a "Trickstar" if opponent controls S/T, ignition effect
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCondition(s.spcon2)
    effect::set_condition(d, e3, Some(condition(spcon2)));
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_SPSUMMON,s.counterfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_SPSUMMON, lua_fn(|d, a| counterfilter(d, a.c(0))));
}
/// `s.counterfilter(c)`: "Check for anything but "Trickstar" monsters"
fn counterfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TRICKSTAR)
    card::is_set_card(d, c, &[SET_TRICKSTAR as u16])
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thfilter(c)`: "Check for "Trickstar" monster"
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsMonster() and c:IsSetCard(SET_TRICKSTAR) and c:IsAbleToHand()
    card::is_monster(d, c) && card::is_set_card(d, c, &[SET_TRICKSTAR as u16]) && card::is_able_to_hand(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`: "Performing the recycle effect"
fn activate(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,nil)
    let g = duel::get_matching_group(d, Some(&aux::necro_valley_filter(|d, x| thfilter(d, x))), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g>0 and Duel.SelectYesNo(tp,aux.Stringid(id,0)) then
    if !g.is_empty() && duel::select_yes_no(d, a.tp, aux::stringid(ID, 0)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_ATOHAND as u64);
        // local sg=g:Select(tp,1,1,nil)
        let sg = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        // Duel.SendtoHand(sg,nil,REASON_EFFECT)
        duel::send_to_hand(d, sg.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,sg)
        duel::confirm_cards(d, 1 - a.tp, sg);
    }
}
/// `s.cfilter1(c)`: "Check for "Trickstar" link monster"
fn cfilter1(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TRICKSTAR) and c:IsLinkMonster()
    card::is_set_card(d, c, &[SET_TRICKSTAR as u16]) && card::is_link_monster(d, c)
}
/// `s.spcon1(e,tp,eg,ep,ev,re,r,rp)`
fn spcon1(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter1,tp,LOCATION_MZONE,0,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter1), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetCustomActivityCount(id,tp,ACTIVITY_SPSUMMON)==0 end
    if a.chk == Some(0) {
        return duel::get_custom_activity_count(d, ID, a.tp, ACTIVITY_SPSUMMON) == 0;
    }
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetLabelObject(e)
    effect::set_label_object(d, e1, Value::Effect(a.e));
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(|d, x| splimit(d, x))));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // aux.RegisterClientHint(e:GetHandler(),nil,tp,1,0,aux.Stringid(id,3),nil)
    aux::register_client_hint(d, h, 0, a.tp, 1, 0, Some(aux::stringid(ID, 3)), 0, None);
    true
}
/// `s.splimit(e,c,sump,sumtype,sumpos,targetp,se)`
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return not c:IsSetCard(SET_TRICKSTAR)
    !card::is_set_card(d, a.c(1), &[SET_TRICKSTAR as u16])
}
/// The "Trickstar" token (`Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_TRICKSTAR,SET_TRICKSTAR,TYPES_TOKEN,0,0,1,RACE_FAIRY,ATTRIBUTE_LIGHT)`)
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![SET_TRICKSTAR as u16]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(1),
        race: Some(RACE_FAIRY),
        attribute: Some(ATTRIBUTE_LIGHT),
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_TRICKSTAR,SET_TRICKSTAR,TYPES_TOKEN,0,0,1,RACE_FAIRY,ATTRIBUTE_LIGHT) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_TRICKSTAR, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`: "Performing the effect of special summoning token"
fn spop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e)
    //     or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0
    //     or not Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_TRICKSTAR,SET_TRICKSTAR,TYPES_TOKEN,0,0,1,RACE_FAIRY,ATTRIBUTE_LIGHT) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e)
        || duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 <= 0
        || !duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_TRICKSTAR, token_data(), None, None, 0)
    {
        return;
    }
    // local token=Duel.CreateToken(tp,TOKEN_TRICKSTAR)
    let token = duel::create_token(d, a.tp, TOKEN_TRICKSTAR).nil();
    // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
    duel::special_summon(d, CardSet::from_card(token), 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
}
/// `s.cfilter2(c)`: "Defining what zones to look in"
fn cfilter2(d: &mut Duel, c: CardId) -> bool {
    // return c:GetSequence()<5
    card::get_sequence(d, c) < 5
}
/// `s.spcon2(e,tp,eg,ep,ev,re,r,rp)`: "Checking if opponent cntrols S/T"
fn spcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.cfilter2,tp,0,LOCATION_SZONE,1,nil)
    duel::is_existing_matching_card(d, Some(&cfilter2), a.tp, 0, LOCATION_SZONE, 1, NO_EX)
}
