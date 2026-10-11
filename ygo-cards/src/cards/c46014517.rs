//! Goblin Bikers Gone Bonkers
use super::prelude::*;

pub const ID: u32 = 46014517;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_GOBLIN],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2+ Level 6 monsters
    // Xyz.AddProcedure(c,nil,6,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(6), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Attach face-up monsters your opponent controls up to the number of "Goblin" monsters you control to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.attachtg)
    effect::set_target(d, e1, Some(cost_target(attachtg)));
    // e1:SetOperation(s.attachop)
    effect::set_operation(d, e1, Some(operation(attachop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Send cards your opponent controls up to the number of "Goblin" Xyz Monsters you control to the GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e2, CATEGORY_TOGRAVE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMING_STANDBY_PHASE|TIMING_MAIN_END|TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e,tp) return Duel.IsTurnPlayer(1-tp) end)
    effect::set_condition(d, e2, Some(condition(|d, a| duel::is_turn_player(d, 1 - a.tp))));
    // e2:SetTarget(s.tgtg)
    effect::set_target(d, e2, Some(cost_target(tgtg)));
    // e2:SetOperation(s.tgop)
    effect::set_operation(d, e2, Some(operation(tgop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT) and chkc:IsFaceup() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE)
            && card::is_controler(d, chkc, 1 - tp)
            && card::is_can_be_xyz_material(d, chkc, Some(c), Some(tp), Some(REASON_EFFECT))
            && card::is_faceup(d, chkc);
    }
    // local ct=Duel.GetMatchingGroupCount(aux.FaceupFilter(Card.IsSetCard,SET_GOBLIN),tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&aux::faceup_filter(|d, mc| card::is_set_card(d, mc, &[SET_GOBLIN as u16]))), tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return c:IsType(TYPE_XYZ) and ct>0 and Duel.CheckRemoveOverlayCard(tp,1,0,1,REASON_EFFECT)
    //     and Duel.IsExistingTarget(aux.FaceupFilter(Card.IsCanBeXyzMaterial,c,tp,REASON_EFFECT),tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return card::is_type(d, c, TYPE_XYZ)
            && ct > 0
            && duel::check_remove_overlay_card(d, tp, 1, 0, 1, REASON_EFFECT, None)
            && duel::is_existing_target(d, Some(&|d, mc| attach_target_filter(d, mc, c, tp)), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATTACH)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATTACH as u64);
    // Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsCanBeXyzMaterial,c,tp,REASON_EFFECT),tp,0,LOCATION_MZONE,1,ct,nil)
    duel::select_target(d, tp, Some(&|d, mc| attach_target_filter(d, mc, c, tp)), tp, 0, LOCATION_MZONE, 1, ct as u16, false, NO_EX);
    true
}
/// `aux.FaceupFilter(Card.IsCanBeXyzMaterial,c,tp,REASON_EFFECT)`
fn attach_target_filter(d: &mut Duel, mc: CardId, c: CardId, tp: u8) -> bool {
    card::is_faceup(d, mc) && card::is_can_be_xyz_material(d, mc, Some(c), Some(tp), Some(REASON_EFFECT))
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.RemoveOverlayCard(tp,1,0,1,1,REASON_EFFECT)~=1 then return end
    if duel::remove_overlay_card(d, tp, 1, 0, 1, 1, REASON_EFFECT, None) != 1 {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if not c:IsRelateToEffect(e) or c:IsImmuneToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, e) || card::is_immune_to_effect(d, c, e) {
        return;
    }
    // local tg=Duel.GetTargetCards(e):Remove(Card.IsImmuneToEffect,nil,e)
    let mut tg = duel::get_target_cards(d, e).nil();
    group::remove(d, &mut tg, &|d, mc| card::is_immune_to_effect(d, mc, e), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.Overlay(c,tg,true)
        duel::overlay(d, c, tg, true);
    }
}
/// `s.tgctfilter(c)`
fn tgctfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_GOBLIN) and c:IsType(TYPE_XYZ) and c:IsFaceup()
    card::is_set_card(d, c, &[SET_GOBLIN as u16]) && card::is_type(d, c, TYPE_XYZ) && card::is_faceup(d, c)
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() and chkc:IsControler(1-tp) and chkc:IsAbleToGrave() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc) && card::is_controler(d, chkc, 1 - tp) && card::is_able_to_grave(d, chkc, None);
    }
    // local ct=Duel.GetMatchingGroupCount(s.tgctfilter,tp,LOCATION_MZONE,0,nil)
    let ct = duel::get_matching_group_count(d, Some(&tgctfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // if chk==0 then return ct>0 and Duel.CheckRemoveOverlayCard(tp,1,0,3,REASON_EFFECT)
    //     and Duel.IsExistingTarget(Card.IsAbleToGrave,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return ct > 0
            && duel::check_remove_overlay_card(d, tp, 1, 0, 3, REASON_EFFECT, None)
            && duel::is_existing_target(d, Some(&|d, mc| card::is_able_to_grave(d, mc, None)), tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToGrave,tp,0,LOCATION_ONFIELD,1,ct,nil)
    let g = duel::select_target(d, tp, Some(&|d, mc| card::is_able_to_grave(d, mc, None)), tp, 0, LOCATION_ONFIELD, 1, ct as u16, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, Some(g), n, tp, 0);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.RemoveOverlayCard(tp,1,0,3,3,REASON_EFFECT)~=3 then return end
    if duel::remove_overlay_card(d, tp, 1, 0, 3, 3, REASON_EFFECT, None) != 3 {
        return;
    }
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, e).nil();
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoGrave(tg,REASON_EFFECT)
        duel::send_to_grave(d, tg, REASON_EFFECT, None, None);
    }
}
