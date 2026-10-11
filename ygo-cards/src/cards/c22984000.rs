//! Mercurium the Living Quicksilver
use super::prelude::*;

pub const ID: u32 = 22984000;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // --Xyz Summon procedure: 2+ Level 10 monsters
    // Xyz.AddProcedure(c,nil,10,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(10), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // --If this card is Xyz Summoned: You can attach 1 monster from your Deck to this card as material
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e1, Some(condition(e1_cond)));
    // e1:SetTarget(s.attachtg)
    effect::set_target(d, e1, Some(cost_target(attachtg)));
    // e1:SetOperation(s.attachop)
    effect::set_operation(d, e1, Some(operation(attachop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // --While this card has material, your opponent cannot activate the effects of monsters in their GY with the same Attribute as a monster in your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetCondition(function(e) return e:GetHandler():GetOverlayCount()>0 end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetValue(s.cannotactval)
    effect::set_value_fn(d, e2, lua_fn(cannotactval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // --Once per turn, during the End Phase: Detach 1 material from this card or take 3000 damage
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.detachtg)
    effect::set_target(d, e3, Some(cost_target(detachtg)));
    // e3:SetOperation(s.detachop)
    effect::set_operation(d, e3, Some(operation(detachop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():IsXyzSummoned() end`
fn e1_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, h)
}
/// `function(e) return e:GetHandler():GetOverlayCount()>0 end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::get_overlay_count(d, h) > 0
}
/// `s.attachfilter(c,xyzc,tp)`
fn attachfilter(d: &mut Duel, c: CardId, xyzc: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsCanBeXyzMaterial(xyzc,tp,REASON_EFFECT)
    card::is_monster(d, c) && card::is_can_be_xyz_material(d, c, Some(xyzc), Some(tp), Some(REASON_EFFECT))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.attachfilter,tp,LOCATION_DECK,0,1,nil,e:GetHandler(),tp) end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return duel::is_existing_matching_card(d, Some(&|d, c| attachfilter(d, c, h, tp)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, e) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_XMATERIAL as u64);
        // local sc=Duel.SelectMatchingCard(tp,s.attachfilter,tp,LOCATION_DECK,0,1,1,nil,c,tp):GetFirst()
        let g = duel::select_matching_card(d, tp, Some(&|d, x| attachfilter(d, x, c, tp)), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
        // if sc then Duel.Overlay(c,sc) end
        if let Some(sc) = group::get_first(d, &g) {
            duel::overlay(d, c, CardSet::from_card(sc), false);
        }
    }
}
/// `s.cannotactval(e,re,rp)`
fn cannotactval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let re = a.e(1);
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // return re:IsMonsterEffect() and rc:IsLocation(LOCATION_GRAVE)
    //   and Duel.IsExistingMatchingCard(Card.IsAttribute,e:GetHandlerPlayer(),LOCATION_GRAVE,0,1,nil,rc:GetAttribute())
    if !(effect::is_monster_effect(d, re) && card::is_location(d, rc, LOCATION_GRAVE)) {
        return false;
    }
    let tp = effect::get_handler_player(d, e);
    let attr = card::get_attribute(d, rc);
    duel::is_existing_matching_card(d, Some(&|d, c| card::is_attribute(d, c, attr)), tp, LOCATION_GRAVE, 0, 1, NO_EX)
}
/// `s.detachtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn detachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:GetOverlayCount()==0 then
    if card::get_overlay_count(d, c) == 0 {
        // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,3000)
        duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, tp, 3000);
    }
    // Duel.SetPossibleOperationInfo(0,CATEGORY_DAMAGE,nil,0,tp,3000)
    duel::set_possible_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, tp, 3000);
    true
}
/// `s.detachop(e,tp,eg,ep,ev,re,r,rp)`
fn detachop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) then return Duel.Damage(tp,3000,REASON_EFFECT) end
    if !card::is_relate_to_effect(d, c, e) {
        duel::damage(d, tp, 3000, REASON_EFFECT, false, None);
        return;
    }
    // local b1=c:CheckRemoveOverlayCard(tp,1,REASON_EFFECT)
    let b1 = card::check_remove_overlay_card(d, c, tp, 1, REASON_EFFECT);
    // local b2=true
    let b2 = true;
    // local op=b1 and Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)}) or 2
    let op = if b1 {
        duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))])
    } else {
        Some(2)
    };
    // if op==1 then
    if op == Some(1) {
        // c:RemoveOverlayCard(tp,1,1,REASON_EFFECT)
        card::remove_overlay_card(d, c, tp, 1, 1, REASON_EFFECT);
    } else if op == Some(2) {
        // Duel.Damage(tp,3000,REASON_EFFECT)
        duel::damage(d, tp, 3000, REASON_EFFECT, false, None);
    }
}
