//! The Zombie Vampire
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 73082255;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,8,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(8), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // lv change
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_XYZ_LEVEL)
    effect::set_code(d, e1, EFFECT_XYZ_LEVEL);
    // e1:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e1, LOCATION_EXTRA);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(s.lvtg)
    effect::set_target(d, e1, Some(effect_card_fn(lvtg)));
    // e1:SetValue(s.lvval)
    effect::set_value_fn(d, e1, lua_fn(lvval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Cannot be targeted by effects of monsters, except summoned from the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(s.tgval)
    effect::set_value_fn(d, e2, lua_fn(tgval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Mill 4 cards and special summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_TOGRAVE+CATEGORY_DECKDES+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_TOGRAVE + CATEGORY_DECKDES + CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCost(Cost.DetachFromSelf(1,1,nil))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e3:SetTarget(s.gytg)
    effect::set_target(d, e3, Some(cost_target(gytg)));
    // e3:SetOperation(s.gyop)
    effect::set_operation(d, e3, Some(operation(gyop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.lvtg(e,c)
fn lvtg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c:IsLevelAbove(1) and c:GetOwner()~=e:GetHandlerPlayer()
    card::is_level_above(d, c, 1) && card::get_owner(d, c) != effect::get_handler_player(d, e)
}
// function s.lvval(e,c,rc)
fn lvval(d: &mut Duel, a: Args) -> i64 {
    // local lv=c:GetLevel()
    let c = a.c(1);
    let lv = card::get_level(d, c);
    // if rc:IsCode(id) then
    let rc = a.card(2).nil();
    if card::is_code(d, rc, &[ID]) {
        // return 8
        8
    } else {
        // return lv
        lv as i64
    }
}
// function s.tgval(e,re,rp)
fn tgval(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // local loc=re:GetActivateLocation()
    let loc = effect::get_activate_location(d, re);
    // return re:IsMonsterEffect() and loc==LOCATION_MZONE and not rc:IsSummonLocation(LOCATION_GRAVE) and rc:IsSpecialSummoned()
    effect::is_monster_effect(d, re) && loc == LOCATION_MZONE && !card::is_summon_location(d, rc, LOCATION_GRAVE) && card::is_special_summoned(d, rc)
}
// function s.gytg(e,tp,eg,ep,ev,re,r,rp,chk)
fn gytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeck(tp,4) and Duel.IsPlayerCanDiscardDeck(1-tp,4) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck(d, a.tp, 4) && duel::is_player_can_discard_deck(d, 1 - a.tp, 4);
    }
    // Duel.SetOperationInfo(0,CATEGORY_DECKDES,0,0,PLAYER_ALL,4)
    duel::set_operation_info(d, 0, CATEGORY_DECKDES, None, 0, PLAYER_ALL as u8, 4);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,PLAYER_EITHER,LOCATION_GRAVE)
    duel::set_possible_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, PLAYER_EITHER as u8, LOCATION_GRAVE as i32);
    true
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsLocation(LOCATION_GRAVE) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_monster(d, c) && card::is_location(d, c, LOCATION_GRAVE) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.gyop(e,tp,eg,ep,ev,re,r,rp)
fn gyop(d: &mut Duel, a: &EffArgs) {
    // local gy1=Duel.DiscardDeck(tp,4,REASON_EFFECT)
    let gy1 = duel::discard_deck(d, a.tp, 4, REASON_EFFECT);
    // local og=Duel.GetOperatedGroup()
    let mut og = duel::get_operated_group(d);
    // local gy2=Duel.DiscardDeck(1-tp,4,REASON_EFFECT)
    let gy2 = duel::discard_deck(d, 1 - a.tp, 4, REASON_EFFECT);
    // if gy1==0 and gy2==0 then return end
    if gy1 == 0 && gy2 == 0 {
        return;
    }
    // og:Merge(Duel.GetOperatedGroup())
    let operated = duel::get_operated_group(d);
    group::add_card(&mut og, &operated);
    // if #og>0 and Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and og:IsExists(aux.NecroValleyFilter(s.spfilter),1,nil,e,tp)
    //     and Duel.SelectYesNo(tp,aux.Stringid(id,1)) then
    if !og.is_empty()
        && duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && group::is_exists(d, &og, &aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp)), 1, NO_EX)
        && duel::select_yes_no(d, a.tp, aux::stringid(ID, 1))
    {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
        // local sg=og:FilterSelect(tp,s.spfilter,1,1,nil,e,tp)
        let sg = group::filter_select(d, &og, a.tp, &|d, c| spfilter(d, c, a.e, a.tp), 1, 1, false, NO_EX).nil();
        // Duel.SpecialSummon(sg,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, sg, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
