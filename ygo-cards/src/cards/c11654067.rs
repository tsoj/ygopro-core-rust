//! Fire Ejection
use super::prelude::*;

pub const ID: u32 = 11654067;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[TOKEN_BOMB], listed_series: &[SET_VOLCANIC], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send 1 Pyro monster from your Deck to the GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOGRAVE+CATEGORY_DAMAGE+CATEGORY_SPECIAL_SUMMON+CATEGORY_TOKEN)
    effect::set_category(d, e1, CATEGORY_TOGRAVE + CATEGORY_DAMAGE + CATEGORY_SPECIAL_SUMMON + CATEGORY_TOKEN);
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
}
/// `s.tgfilter(c)`
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_PYRO) and c:IsAbleToGrave()
    card::is_race(d, c, RACE_PYRO) && card::is_able_to_grave(d, c, None)
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.tgfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_TOKEN,nil,1,0,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_TOKEN, None, 1, 0, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, 0);
    true
}
/// `Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_BOMB,0,TYPES_TOKEN,1000,1000,1,RACE_PYRO,ATTRIBUTE_FIRE,POS_FACEUP,1-tp)`
fn token_data() -> duel::MonsterData {
    duel::MonsterData {
        setcodes: Some(vec![0]),
        type_: Some(TYPES_TOKEN),
        attack: Some(1000),
        defense: Some(1000),
        level: Some(1),
        race: Some(RACE_PYRO),
        attribute: Some(ATTRIBUTE_FIRE),
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.tgfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel);
    // if not (sc and Duel.SendtoGrave(sc,REASON_EFFECT)>0 and sc:IsLocation(LOCATION_GRAVE) and sc:IsSetCard(SET_VOLCANIC)) then return end
    let sc = match sc {
        Some(sc) => sc,
        None => return,
    };
    if duel::send_to_grave(d, sc, REASON_EFFECT, None, None) <= 0 {
        return;
    }
    if !card::is_location(d, sc, LOCATION_GRAVE) || !card::is_set_card(d, sc, &[SET_VOLCANIC as u16]) {
        return;
    }
    // local b1=sc:HasLevel()
    let b1 = card::has_level(d, sc);
    // local b2=Duel.GetLocationCount(1-tp,LOCATION_MZONE,tp)>0
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,TOKEN_BOMB,0,TYPES_TOKEN,1000,1000,1,RACE_PYRO,ATTRIBUTE_FIRE,POS_FACEUP,1-tp)
    let b2 = duel::get_location_count(d, 1 - a.tp, LOCATION_MZONE as u8, Some(a.tp), None, None).0 > 0
        && duel::is_player_can_special_summon_monster(d, a.tp, TOKEN_BOMB, token_data(), Some(POS_FACEUP as u8), Some(1 - a.tp), 0);
    // if (b1 or b2) and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if (b1 || b2) && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1)) {
        // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
        let op = duel::select_effect(d, a.tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if op==1 then
        if op == Some(1) {
            // Duel.Damage(1-tp,sc:GetLevel()*100,REASON_EFFECT)
            let amount = card::get_level(d, sc) as i64 * 100;
            duel::damage(d, 1 - a.tp, amount, REASON_EFFECT, false, None);
        // elseif op==2 then
        } else if op == Some(2) {
            // local token=Duel.CreateToken(tp,TOKEN_BOMB)
            let token = duel::create_token(d, a.tp, TOKEN_BOMB).nil();
            // if Duel.SpecialSummonStep(token,0,tp,1-tp,false,false,POS_FACEUP) then
            if duel::special_summon_step(d, token, 0, a.tp, 1 - a.tp, false, false, POS_FACEUP as u8, None) {
                // local e1=Effect.CreateEffect(e:GetHandler())
                let h = effect::get_handler(d, a.e).nil();
                let e1 = effect::create_effect(d, h);
                // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
                // e1:SetCode(EVENT_LEAVE_FIELD)
                effect::set_code(d, e1, EVENT_LEAVE_FIELD);
                // e1:SetOperation(s.damop)
                effect::set_operation(d, e1, Some(operation(damop)));
                // token:RegisterEffect(e1,true)
                card::register_effect(d, token, e1, true);
            }
            // Duel.SpecialSummonComplete()
            duel::special_summon_complete(d);
        }
    }
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsReason(REASON_DESTROY) then
    if card::is_reason(d, c, REASON_DESTROY) {
        // Duel.Damage(c:GetPreviousControler(),500,REASON_EFFECT)
        let p = card::get_previous_controler(d, c);
        duel::damage(d, p, 500, REASON_EFFECT, false, None);
    }
    // e:Reset()
    effect::reset(d, a.e);
}
