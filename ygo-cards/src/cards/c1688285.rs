//! Traptrix Pinguicula
use super::prelude::*;

pub const ID: u32 = 1688285;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_TRAPTRIX],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 4 monsters
    // Xyz.AddProcedure(c,nil,4,2)
    xyz::add_procedure(d, c, None, Some(4), 2, None, None, None, None, false, None);
    // Unaffected by Trap effects and effects of monsters with the same type as its materials
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
    // e1:SetCondition(function(e) return e:GetHandler():GetOverlayCount()>0 end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::get_overlay_count(d, h) > 0
    })));
    // e1:SetValue(s.immval)
    effect::set_value_fn(d, e1, lua_fn(immval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Search 1 "Traptrix" monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_TOHAND+CATEGORY_SEARCH)
    effect::set_category(d, e2, CATEGORY_TOHAND + CATEGORY_SEARCH);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCost(Cost.DetachFromSelf(1,1))
    effect::set_cost(d, e2, Some(cost::detach_from_self(cost::Count::N(1), Some(cost::Count::N(1)), None)));
    // e2:SetTarget(s.thtg)
    effect::set_target(d, e2, Some(cost_target(thtg)));
    // e2:SetOperation(s.thop)
    effect::set_operation(d, e2, Some(operation(thop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Attach 1 monster to this card
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_REMOVE)
    effect::set_code(d, e3, EVENT_REMOVE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.ovtg)
    effect::set_target(d, e3, Some(cost_target(ovtg)));
    // e3:SetOperation(s.ovop)
    effect::set_operation(d, e3, Some(operation(ovop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e4, CATEGORY_LEAVE_GRAVE);
    // e4:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e4, EVENT_TO_GRAVE);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.immval(e,te)`
fn immval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    // if te:IsTrapEffect() then return true end
    if effect::is_trap_effect(d, te) {
        return true;
    }
    // local tc=te:GetOwner()
    let tc = effect::get_owner(d, te).nil();
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // return c~=tc and te:IsActivated() and te:IsMonsterEffect()
    //     and c:GetOverlayGroup():IsExists(Card.IsRace,1,nil,tc:GetRace())
    if c == tc || !effect::is_activated(d, te) || !effect::is_monster_effect(d, te) {
        return false;
    }
    let race = card::get_race(d, tc);
    let og = card::get_overlay_group(d, c);
    group::is_exists(d, &og, &|d, c| card::is_race(d, c, race), 1, NO_EX)
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_TRAPTRIX) and c:IsMonster() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_TRAPTRIX as u16]) && card::is_monster(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&thfilter), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,s.thfilter,tp,LOCATION_DECK,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&thfilter), tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g.clone(), None, REASON_EFFECT, None);
        // Duel.ConfirmCards(1-tp,g)
        duel::confirm_cards(d, 1 - tp, g);
    }
}
/// `s.ovfilter(c,e,xc,tp)`
fn ovfilter(d: &mut Duel, c: CardId, e: EffectId, xc: CardId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsFaceup() and c:IsReason(REASON_EFFECT)
    //     and c:GetOwner()==1-tp and c:IsLocation(LOCATION_GRAVE|LOCATION_REMOVED)
    //     and not c:IsImmuneToEffect(e) and c:IsCanBeXyzMaterial(xc,tp,REASON_EFFECT)
    card::is_monster(d, c)
        && card::is_faceup(d, c)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::get_owner(d, c) == 1 - tp
        && card::is_location(d, c, LOCATION_GRAVE | LOCATION_REMOVED)
        && !card::is_immune_to_effect(d, c, e)
        && card::is_can_be_xyz_material(d, c, Some(xc), Some(tp), Some(REASON_EFFECT))
}
/// `s.ovtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ovtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return c:IsType(TYPE_XYZ) and eg:IsExists(s.ovfilter,1,nil,e,c,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return card::is_type(d, c, TYPE_XYZ) && group::is_exists(d, &eg, &|d, mc| ovfilter(d, mc, e, c, tp), 1, NO_EX);
    }
    // Duel.SetTargetCard(eg)
    duel::set_target_card(d, a.eg());
    // if e:GetCode()==EVENT_TO_GRAVE then
    if effect::get_code(d, e) == EVENT_TO_GRAVE {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,1-tp,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, 1 - tp, 0);
    }
    true
}
/// `s.ovop(e,tp,eg,ep,ev,re,r,rp)`
fn ovop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) or not c:IsType(TYPE_XYZ) or c:IsImmuneToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) || !card::is_type(d, c, TYPE_XYZ) || card::is_immune_to_effect(d, c, e) {
        return;
    }
    // local tg=Duel.GetTargetCards(e):Filter(Card.IsRelateToEffect,nil,e)
    let tgall = duel::get_target_cards(d, e).nil();
    let tg = group::filter(d, &tgall, &|d, mc| card::is_relate_to_effect(d, mc, e), NO_EX);
    // if #tg==0 then return end
    if tg.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
    // local g=tg:FilterSelect(tp,s.ovfilter,1,1,nil,e,c,tp)
    let g = group::filter_select(d, &tg, tp, &|d, mc| ovfilter(d, mc, e, c, tp), 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Overlay(c,g)
        duel::overlay(d, c, g, false);
    }
}
