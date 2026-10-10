//! Delta of Invitation
use super::prelude::*;

pub const ID: u32 = 3129133;
/// `local TOKEN_DELTA=id+1`
const TOKEN_DELTA: u32 = ID + 1;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_DELTA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon 1 "Delta Token"
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_ZOMBIE),0,LOCATION_MZONE,LOCATION_MZONE,1,nil) end)
    effect::set_condition(d, e2, Some(condition(e2_condition)));
    // e2:SetTarget(s.tktg)
    effect::set_target(d, e2, Some(cost_target(tktg)));
    // e2:SetOperation(s.tkop)
    effect::set_operation(d, e2, Some(operation(tkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Add this card to your hand
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e3, CATEGORY_TOHAND);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e3:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e3, EVENT_TO_HAND);
    // e3:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e3, LOCATION_GRAVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.thcon)
    effect::set_condition(d, e3, Some(condition(thcon)));
    // e3:SetTarget(s.thtg)
    effect::set_target(d, e3, Some(cost_target(thtg)));
    // e3:SetOperation(s.thop)
    effect::set_operation(d, e3, Some(operation(thop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function() return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,RACE_ZOMBIE),0,LOCATION_MZONE,LOCATION_MZONE,1,nil) end`
fn e2_condition(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_faceup(d, c) && card::is_race(d, c, RACE_ZOMBIE)), 0, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLevelAbove(5) and c:IsRace(RACE_ZOMBIE) and c:IsAbleToGrave()
    card::is_level_above(d, c, 5) && card::is_race(d, c, RACE_ZOMBIE) && card::is_able_to_grave(d, c, None)
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // if Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,3)) then
    if duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 3)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
        // local g=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil)
        let g = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #g>0 then
        if !g.is_empty() {
            // Duel.SendtoGrave(g,REASON_EFFECT)
            duel::send_to_grave(d, g, REASON_EFFECT, None, None);
        }
    }
}
/// The Delta token data: `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_DELTA,0,TYPES_TOKEN,0,0,5,RACE_ZOMBIE,ATTRIBUTE_DARK)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(0),
        defense: Some(0),
        level: Some(5),
        race: Some(RACE_ZOMBIE),
        attribute: Some(ATTRIBUTE_DARK),
    }
}
/// `s.tktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_DELTA,0,TYPES_TOKEN,0,0,5,RACE_ZOMBIE,ATTRIBUTE_DARK) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_DELTA, token_data(), None, None, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `s.tkop(e,tp,eg,ep,ev,re,r,rp)`
fn tkop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_DELTA,0,TYPES_TOKEN,0,0,5,RACE_ZOMBIE,ATTRIBUTE_DARK) then
    if duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_DELTA, token_data(), None, None, 0)
    {
        // local token=Duel.CreateToken(tp,TOKEN_DELTA)
        let token = duel::create_token(d, a.tp, TOKEN_DELTA).nil();
        // Duel.SpecialSummon(token,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, token, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.thconfilter(c)`
fn thconfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsPreviousLocation(LOCATION_GRAVE) and c:IsMonster()
    card::is_previous_location(d, c, LOCATION_GRAVE) && card::is_monster(d, c)
}
/// `s.thcon(e,tp,eg,ep,ev,re,r,rp)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.thconfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &thconfilter, 1, NO_EX)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToHand() end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, c, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SendtoHand(c,nil,REASON_EFFECT)
        duel::send_to_hand(d, c, None, REASON_EFFECT, None);
    }
}
