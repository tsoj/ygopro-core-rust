//! Sacred Tree Beast, Hyperyton
//! Scripted by Hatter
use super::prelude::*;

pub const ID: u32 = 9349094;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Xyz summon procedure
    // Xyz.AddProcedure(c,nil,9,2)
    xyz::add_procedure(d, c, None, Some(9), 2, None, None, None, None, false, None);
    // Must be properly summoned before reviving
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Attach 1 card from your GY to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_LEAVE_GRAVE)
    effect::set_category(d, e1, CATEGORY_LEAVE_GRAVE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.attcon)
    effect::set_condition(d, e1, Some(condition(attcon)));
    // e1:SetTarget(s.atttg)
    effect::set_target(d, e1, Some(cost_target(atttg)));
    // e1:SetOperation(s.attop)
    effect::set_operation(d, e1, Some(operation(attop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate activation of a card/effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.DoubleSnareValidity(c,LOCATION_MZONE)
    aux::double_snare_validity(d, Some(c), LOCATION_MZONE, None);
}
/// `s.attcon(e,tp,eg,ep,ev,re,r,rp)`
fn attcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp) and rp==tp and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED)
    let h = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, a.tp) && a.rp == a.tp && !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
}
/// `s.atttg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atttg(d: &mut Duel, a: &EffArgs) -> bool {
    // local rt=re:GetActiveType()&(TYPE_SPELL|TYPE_MONSTER|TYPE_TRAP)
    let rt = effect::get_active_type(d, a.re.nil()) & (TYPE_SPELL | TYPE_MONSTER | TYPE_TRAP);
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and chkc:IsType(rt) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, a.tp) && card::is_type(d, chkc, rt);
    }
    // if chk==0 then return Duel.IsExistingTarget(Card.IsType,tp,LOCATION_GRAVE,0,1,nil,rt) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| card::is_type(d, c, rt)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // local g=Duel.SelectTarget(tp,Card.IsType,tp,LOCATION_GRAVE,0,1,1,nil,rt)
    let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_type(d, c, rt)), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, g, 1, 0, 0);
    true
}
/// `s.attop(e,tp,eg,ep,ev,re,r,rp)`
fn attop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // local g=Duel.GetTargetCards(e)
        let g = duel::get_target_cards(d, a.e).nil();
        // if #g>0 then Duel.Overlay(c,g) end
        if !g.is_empty() {
            duel::overlay(d, c, g, false);
        }
    }
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp) and Duel.IsChainNegatable(ev) and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED)
    let h = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, 1 - a.tp) && duel::is_chain_negatable(d, a.ev as u8) && !card::is_status(d, h, STATUS_BATTLE_DESTROYED)
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // local rt=re:GetActiveType()&(TYPE_SPELL|TYPE_MONSTER|TYPE_TRAP)
        let rt = effect::get_active_type(d, a.re.nil()) & (TYPE_SPELL | TYPE_MONSTER | TYPE_TRAP);
        // return e:GetHandler():GetOverlayGroup():IsExists(Card.IsType,1,nil,rt)
        let h = effect::get_handler(d, a.e).nil();
        let ov = card::get_overlay_group(d, h);
        return group::is_exists(d, &ov, &|d, c| card::is_type(d, c, rt), 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // if re:GetHandler():IsDestructable() and re:GetHandler():IsRelateToEffect(re) then
    let rh = effect::get_handler(d, a.re.nil()).nil();
    if card::is_destructable(d, rh, None) && card::is_relate_to_effect(d, rh, a.re.nil()) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local rt=re:GetActiveType()&(TYPE_SPELL|TYPE_MONSTER|TYPE_TRAP)
    let rt = effect::get_active_type(d, a.re.nil()) & (TYPE_SPELL | TYPE_MONSTER | TYPE_TRAP);
    // local ov=c:GetOverlayGroup():Filter(Card.IsType,nil,rt)
    let ov = group::filter(d, &card::get_overlay_group(d, c), &|d, cd| card::is_type(d, cd, rt), NO_EX);
    // if #ov<1 then return end
    if ov.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_XMATERIAL)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_XMATERIAL as u64);
    // local sg=ov:Select(tp,1,1,nil)
    let sg = group::select(d, &ov, a.tp, 1, 1, false, NO_EX).nil();
    // if #sg>0 and Duel.SendtoGrave(sg,REASON_EFFECT)>0 then
    if !sg.is_empty() && duel::send_to_grave(d, sg, REASON_EFFECT, None, None) > 0 {
        // Duel.RaiseSingleEvent(c,EVENT_DETACH_MATERIAL,e,0,0,0,0)
        duel::raise_single_event(d, c, EVENT_DETACH_MATERIAL, Some(a.e), 0, 0, 0, 0);
        // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
        if duel::negate_activation(d, a.ev as u8) {
            let rh = effect::get_handler(d, a.re.nil()).nil();
            if card::is_relate_to_effect(d, rh, a.re.nil()) {
                // Duel.Destroy(eg,REASON_EFFECT)
                duel::destroy(d, a.eg.clone().nil(), REASON_EFFECT, None, None);
            }
        }
    }
}
