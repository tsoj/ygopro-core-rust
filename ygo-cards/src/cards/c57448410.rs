//! Prima Materiactor
use super::prelude::*;

pub const ID: u32 = 57448410;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MATERIACTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Xyz Monsters you control gain 100 ATK for each Xyz Material on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetTarget(function(e,c) return c:IsXyzMonster() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, _e, c| card::is_xyz_monster(d, c))));
    // e1:SetValue(function(e,c) return Duel.GetOverlayCount(0,1,1)*100 end)
    effect::set_value_fn(d, e1, effect_value(|d, _e| duel::get_overlay_count(d, 0, 1, 1, None) as i64 * 100));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can target 1 "Materiactor" Xyz Monster you control; attach this card to it as material, then draw 1 card. You can only use this effect of "Prima Materiactor" once per turn
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e2, CATEGORY_DRAW);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.attachtg)
    effect::set_target(d, e2, Some(cost_target(attachtg)));
    // e2:SetOperation(s.attachop)
    effect::set_operation(d, e2, Some(operation(attachop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.xyzfilter(c,tp,hc)`
fn xyzfilter(d: &mut Duel, c: CardId, tp: u8, hc: CardId) -> bool {
    // return c:IsSetCard(SET_MATERIACTOR) and c:IsXyzMonster() and c:IsFaceup() and hc:IsCanBeXyzMaterial(c,tp,REASON_EFFECT)
    card::is_set_card(d, c, &[SET_MATERIACTOR as u16])
        && card::is_xyz_monster(d, c)
        && card::is_faceup(d, c)
        && card::is_can_be_xyz_material(d, hc, Some(c), Some(tp), Some(REASON_EFFECT))
}
/// `s.attachtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn attachtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.xyzfilter(chkc,tp,c) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && xyzfilter(d, chkc, tp, c);
    }
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1)
    //     and Duel.IsExistingTarget(s.xyzfilter,tp,LOCATION_MZONE,0,1,nil,tp,c) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1)
            && duel::is_existing_target(d, Some(&|d, x| xyzfilter(d, x, tp, c)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.xyzfilter,tp,LOCATION_MZONE,0,1,1,nil,tp,c)
    duel::select_target(d, tp, Some(&|d, x| xyzfilter(d, x, tp, c)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, tp, 1);
    true
}
/// `s.attachop(e,tp,eg,ep,ev,re,r,rp)`
fn attachop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, c, e) && card::is_relate_to_effect(d, tc.nil(), e) && !card::is_immune_to_effect(d, tc.nil(), e) {
        // Duel.Overlay(tc,c)
        duel::overlay(d, tc.nil(), c, false);
        // if Duel.IsPlayerCanDraw(tp) then
        if duel::is_player_can_draw(d, tp, 0) {
            // Duel.BreakEffect()
            duel::break_effect(d);
            // Duel.Draw(tp,1,REASON_EFFECT)
            duel::draw(d, tp, 1, REASON_EFFECT);
        }
    }
}
