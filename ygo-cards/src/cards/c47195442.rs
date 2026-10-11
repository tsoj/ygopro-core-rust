//! Wonky Quartet
use super::prelude::*;

pub const ID: u32 = 47195442;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Xyz Summon Procedure
    // Xyz.AddProcedure(c,nil,4,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // --Unaffected by other cards' effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e1, EFFECT_IMMUNE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,te) return te:GetOwner()~=e:GetOwner() end)
    effect::set_value_fn(d, e1, lua_fn(immune_val));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --Attach the top 3 cards of your Deck to this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_CONTROL+CATEGORY_DAMAGE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_CONTROL + CATEGORY_DAMAGE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e2, Some(condition(xyz_cond)));
    // e2:SetTarget(s.atchtg)
    effect::set_target(d, e2, Some(cost_target(atchtg)));
    // e2:SetOperation(s.atchop)
    effect::set_operation(d, e2, Some(operation(atchop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_STANDBY);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e3, Some(condition(turn_cond)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e,te) return te:GetOwner()~=e:GetOwner() end`
fn immune_val(d: &mut Duel, a: Args) -> bool {
    let te = a.e(1);
    effect::get_owner(d, te) != effect::get_owner(d, a.e(0))
}
/// `function(e) return e:GetHandler():IsXyzSummoned() end`
fn xyz_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, c)
}
/// `function(e,tp) return Duel.IsTurnPlayer(tp) end`
fn turn_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
/// `s.atchtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atchtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsType(TYPE_XYZ) and Duel.GetFieldGroupCount(tp,LOCATION_DECK,0)>=3 end
    if a.chk == Some(0) {
        return card::is_type(d, c, TYPE_XYZ) && duel::get_field_group_count(d, tp, LOCATION_DECK, 0) >= 3;
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_CONTROL,c,1,1-tp,0)
    duel::set_possible_operation_info(d, 0, CATEGORY_CONTROL, Some(CardSet::from_card(c)), 1, 1 - tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,1,tp,400)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 1, tp, 400);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_DESTROY, None, 1, tp, LOCATION_MZONE as i32);
    true
}
/// `s.atchop(e,tp,eg,ep,ev,re,r,rp)`
fn atchop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetDecktopGroup(tp,3)
    let g = duel::get_decktop_group(d, tp, 3);
    // if not (c:IsRelateToEffect(e) and #g==3) then return end
    if !(card::is_relate_to_effect(d, c, a.e) && g.len() == 3) {
        return;
    }
    // Duel.DisableShuffleCheck()
    duel::disable_shuffle_check(d, true);
    // Duel.Overlay(c,g)
    duel::overlay(d, c, g, false);
    // local og=c:GetOverlayGroup()
    let og = card::get_overlay_group(d, c);
    // if og:FilterCount(Card.IsMonster,nil)<=4 then
    if group::filter_count(d, &og, &|d, x| card::is_monster(d, x), NO_EX) <= 4 {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // Duel.GetControl(c,1-tp)
        duel::get_control(d, c, 1 - tp, 0, 0, None, None);
    } else {
        // Duel.BreakEffect()
        duel::break_effect(d);
        // if Duel.Damage(tp,#og*400,REASON_EFFECT)>0 then
        if duel::damage(d, tp, og.len() as i64 * 400, REASON_EFFECT, false, None) > 0 {
            // local dg=Duel.GetFieldGroup(tp,LOCATION_MZONE,0)
            let dg = duel::get_field_group(d, tp, LOCATION_MZONE, 0);
            // if #dg==0 then return end
            if dg.is_empty() {
                return;
            }
            // Duel.Destroy(dg,REASON_EFFECT)
            duel::destroy(d, dg, REASON_EFFECT, None, None);
        }
    }
}
