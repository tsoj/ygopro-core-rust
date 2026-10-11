//! Libromancer Realized
//! リブロマンサー・リアライズ
use super::prelude::*;

pub const ID: u32 = 15590355;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[15590356], listed_series: &[SET_LIBROMANCER], ..ScriptInfo::DEFAULT };

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
    // Special Summon 1 "Fire Token"
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
    // e2:SetCost(s.tkncost)
    effect::set_cost(d, e2, Some(cost_target(tkncost)));
    // e2:SetTarget(s.tkntg)
    effect::set_target(d, e2, Some(cost_target(tkntg)));
    // e2:SetOperation(s.tknop)
    effect::set_operation(d, e2, Some(operation(tknop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.rvlfilter(c,tp)`
fn rvlfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsRitualMonster() and c:IsSetCard(SET_LIBROMANCER) and not c:IsPublic()
    //     and Duel.IsPlayerCanSpecialSummonMonster(tp,id+1,0,TYPES_TOKEN,0,0,c:GetLevel(),RACE_CYBERSE,ATTRIBUTE_FIRE)
    card::is_ritual_monster(d, c)
        && card::is_set_card(d, c, &[SET_LIBROMANCER as u16])
        && !card::is_public(d, c)
        && {
            let level = card::get_level(d, c);
            duel::is_player_can_special_summon_monster(
                d,
                tp,
                ID + 1,
                duel::MonsterData {
                    setcodes: Some(vec![0]),
                    type_: Some(TYPES_TOKEN),
                    attack: Some(0),
                    defense: Some(0),
                    level: Some(level),
                    race: Some(RACE_CYBERSE as u64),
                    attribute: Some(ATTRIBUTE_FIRE),
                },
                None,
                None,
                0,
            )
        }
}
/// `s.tkncost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tkncost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rvlfilter,tp,LOCATION_HAND,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| rvlfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local g=Duel.SelectMatchingCard(tp,s.rvlfilter,tp,LOCATION_HAND,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| rvlfilter(d, c, tp)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // Duel.ConfirmCards(1-tp,g)
    duel::confirm_cards(d, 1 - tp, g.clone());
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g);
    true
}
/// `s.tkntg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tkntg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOKEN,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_TOKEN, None, 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, 0);
    true
}
/// `s.tknop(e,tp,eg,ep,ev,re,r,rp)`
fn tknop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<1 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 < 1 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local token=Duel.CreateToken(tp,id+1)
    let token = duel::create_token(d, tp, ID + 1).nil();
    // if Duel.SpecialSummonStep(token,0,tp,tp,false,false,POS_FACEUP) then
    if duel::special_summon_step(d, token, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // --Cannot Special Summon, except "Libromancer" monsters
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
        effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e1:SetRange(LOCATION_ONFIELD)
        effect::set_range(d, e1, LOCATION_ONFIELD);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetTarget(aux.NOT(aux.TargetBoolFunction(Card.IsSetCard,SET_LIBROMANCER)))
        effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| !card::is_set_card(d, c, &[SET_LIBROMANCER as u16]))));
        // token:RegisterEffect(e1,true)
        card::register_effect(d, token, e1, true);
        // --Change its Level
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_CHANGE_LEVEL)
        effect::set_code(d, e2, EFFECT_CHANGE_LEVEL);
        // e2:SetValue(tc:GetLevel())
        let lv = card::get_level(d, tc);
        effect::set_value(d, e2, lv as i64);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // token:RegisterEffect(e2,true)
        card::register_effect(d, token, e2, true);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
