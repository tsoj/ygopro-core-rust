//! Abyss Shark
//! アビス・シャーク
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 61496006;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[ID], listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself and search
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON+CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON + CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Can be treated as Level 3 or 4 for a "Number" Xyz
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_XYZ_LEVEL)
    effect::set_code(d, e2, EFFECT_XYZ_LEVEL);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.xyzlv)
    effect::set_value_fn(d, e2, lua_fn(xyzlv));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.cfilter(c)
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return not (c:IsFaceup() and c:IsAttribute(ATTRIBUTE_WATER))
    !(card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_WATER))
}
// function s.spcon(e,tp,eg,ep,ev,re,r,rp)
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
    let g = duel::get_field_group(d, a.tp, LOCATION_MZONE, 0);
    // return #g>0 and not g:IsExists(s.cfilter,1,nil)
    !g.is_empty() && !group::is_exists(d, &g, &cfilter, 1, NO_EX)
}
// function s.thfilter(c)
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_FISH) and c:IsLevel(3,4,5) and not c:IsCode(id) and c:IsAbleToHand()
    card::is_race(d, c, RACE_FISH) && card::is_level(d, c, &[3, 4, 5]) && !card::is_code(d, c, &[ID]) && card::is_able_to_hand(d, c, None)
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    //     and Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None)
            && duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, LOCATION_HAND as i32);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e1, aux::stringid(ID, 1));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(lua_fn(splimit)));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_BATTLE_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(lua_fn(damcon)));
    // e2:SetValue(DOUBLE_DAMAGE)
    effect::set_value(d, e2, DOUBLE_DAMAGE as i64);
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // if not c:IsRelateToEffect(e) or Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if !card::is_relate_to_effect(d, c, a.e) || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // if Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)>0 then
    if duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, None) > 0 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
        let sc = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if #sc>0 and Duel.SendtoHand(sc,tp,REASON_EFFECT)>0 then
        if !sc.is_empty() && duel::send_to_hand(d, sc.clone(), Some(tp), REASON_EFFECT, None) > 0 {
            // Duel.ConfirmCards(1-tp,sc)
            duel::confirm_cards(d, 1 - tp, sc);
        }
    }
}
// function s.splimit(e,c,sump,sumtype,sumpos,targetp,se)
fn splimit(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    // return not c:IsAttribute(ATTRIBUTE_WATER)
    !card::is_attribute(d, c, ATTRIBUTE_WATER)
}
// function s.damcon(e)
fn damcon(d: &mut Duel, a: Args) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // local bc0,bc1=Duel.GetBattleMonster(tp)
    let (bc0, bc1) = duel::get_battle_monster(d, tp);
    // if Duel.GetFlagEffect(tp,id)==0 and bc0 and bc1 and bc0:IsSetCard(SET_NUMBER) and bc0:IsStatus(STATUS_OPPO_BATTLE) then
    if duel::get_flag_effect(d, tp, ID) == 0
        && bc0.is_some()
        && bc1.is_some()
        && card::is_set_card(d, bc0.nil(), &[SET_NUMBER as u16])
        && card::is_status(d, bc0.nil(), STATUS_OPPO_BATTLE)
    {
        // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
        duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
        // return true
        return true;
    }
    // return false
    false
}
// function s.xyzlv(e,c,rc)
fn xyzlv(d: &mut Duel, a: Args) -> Value {
    // if rc:IsSetCard(SET_NUMBER) then
    let rc = a.card(2).nil();
    if card::is_set_card(d, rc, &[SET_NUMBER as u16]) {
        // return 4,3,e:GetHandler():GetLevel()
        let h = effect::get_handler(d, a.e(0)).nil();
        Value::Multi(vec![Value::Int(4), Value::Int(3), Value::Int(card::get_level(d, h) as i64)])
    } else {
        // return e:GetHandler():GetLevel()
        let h = effect::get_handler(d, a.e(0)).nil();
        Value::Int(card::get_level(d, h) as i64)
    }
}
