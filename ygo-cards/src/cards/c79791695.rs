//! CXyz Chaos Barian Hope Dragon
use super::prelude::*;

pub const ID: u32 = 79791695;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER_C], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3+ Level 7 monsters OR 1 "Number C" monster you control with a number between "101" and "107" in its name
    // Xyz.AddProcedure(c,nil,7,3,s.ovfilter,aux.Stringid(id,0),Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(7), 3, Some(xyz::alter_filter(ovfilter)), Some(aux::stringid(ID, 0)), Some(xyz::INFINITE_MATS), None, false, None);
    // Gains 1000 ATK for each material attached to it
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return c:GetOverlayCount()*1000 end)
    effect::set_value_fn(d, e1, effect_card_value(|d, _e, x| card::get_overlay_count(d, x) as i64 * 1000));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // While this card has a "Number C" monster as material, your opponent cannot add cards from the Deck to the hand, except by drawing them
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_TO_HAND)
    effect::set_code(d, e2, EFFECT_CANNOT_TO_HAND);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCondition(function(e)
    //     return e:GetHandler():GetOverlayGroup():IsExists(Card.IsSetCard,1,nil,SET_NUMBER_C)
    // end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        let og = card::get_overlay_group(d, h);
        group::is_exists(d, &og, &|d, x| card::is_set_card(d, x, &[SET_NUMBER_C as u16]), 1, NO_EX)
    })));
    // e2:SetTarget(function(e,c,rp,re)
    //     return c:IsLocation(LOCATION_DECK)
    // end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, x| card::is_location(d, x, LOCATION_DECK))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Once per turn: You can detach 1 material from this card; attach the top 2 cards of your opponent's Deck to this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e3:SetTarget(function(e,tp,eg,ep,ev,re,r,rp,chk)
    //     if chk==0 then return e:GetHandler():IsXyzMonster() and Duel.GetFieldGroupCount(1-tp,LOCATION_DECK,0)>=2 end
    // end)
    effect::set_target(d, e3, Some(cost_target(e3_target)));
    // e3:SetOperation(function(e,tp,eg,ep,ev,re,r,rp)
    //     local c=e:GetHandler()
    //     local g=Duel.GetDecktopGroup(1-tp,2)
    //     if c:IsRelateToEffect(e) and #g==2 then
    //         Duel.DisableShuffleCheck()
    //         Duel.Overlay(c,g)
    //     end
    // end)
    effect::set_operation(d, e3, Some(operation(e3_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
// function s.ovfilter(c,tp,lc)
fn ovfilter(d: &mut Duel, c: CardId, tp: u8, lc: CardId) -> bool {
    // if c:IsFacedown() or not c:IsSetCard(SET_NUMBER_C,lc,SUMMON_TYPE_XYZ,tp) then return false end
    if card::is_facedown(d, c) || !card::is_set_card_ctx(d, c, &[SET_NUMBER_C as u16], Some(lc), SUMMON_TYPE_XYZ as u64, Some(tp)) {
        return false;
    }
    // local class=c:GetMetatable(true)
    // if class==nil then return false end
    // local no=class.xyz_number
    let no = crate::current_info(d, c).and_then(|m| m.xyz_number);
    // return no and no>=101 and no<=107
    matches!(no, Some(n) if (101..=107).contains(&n))
}
// function(e,tp,eg,ep,ev,re,r,rp,chk) (the target of e3)
fn e3_target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsXyzMonster() and Duel.GetFieldGroupCount(1-tp,LOCATION_DECK,0)>=2 end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_xyz_monster(d, h) && duel::get_field_group_count(d, 1 - a.tp, LOCATION_DECK, 0) >= 2;
    }
    true
}
// function(e,tp,eg,ep,ev,re,r,rp) (the operation of e3)
fn e3_op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetDecktopGroup(1-tp,2)
    let g = duel::get_decktop_group(d, 1 - a.tp, 2);
    // if c:IsRelateToEffect(e) and #g==2 then
    if card::is_relate_to_effect(d, c, a.e) && g.len() == 2 {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Overlay(c,g)
        duel::overlay(d, c, g, false);
    }
}
