//! Number 75: Bamboozling Gossip Shadow
use super::prelude::*;

pub const ID: u32 = 71166481;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_NUMBER],
    xyz_number: Some(75),
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,3,2,nil,nil,Xyz.InfiniteMats)
    xyz::add_procedure(d, c, None, Some(3), 2, None, None, Some(xyz::INFINITE_MATS), None, false, None);
    // Change effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e1, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.chcon)
    effect::set_condition(d, e1, Some(condition(chcon)));
    // e1:SetCost(Cost.DetachFromSelf(2))
    effect::set_cost(d, e1, Some(cost::detach_from_self(cost::Count::N(2), Some(cost::Count::N(2)), None)));
    // e1:SetTarget(s.chtg)
    effect::set_target(d, e1, Some(cost_target(chtg)));
    // e1:SetOperation(s.chop)
    effect::set_operation(d, e1, Some(operation(chop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Attach a card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetTarget(s.xtg)
    effect::set_target(d, e2, Some(cost_target(xtg)));
    // e2:SetOperation(s.xop)
    effect::set_operation(d, e2, Some(operation(xop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.chcon(e,tp,eg,ep,ev,re,r,rp)`
fn chcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return re:IsMonsterEffect() and rp==1-tp
    let re = a.re.nil();
    effect::is_monster_effect(d, re) && a.rp == 1 - a.tp
}
/// `s.chtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn chtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) and Duel.IsPlayerCanDraw(1-tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1) && duel::is_player_can_draw(d, 1 - a.tp, 1);
    }
    true
}
/// `s.chop(e,tp,eg,ep,ev,re,r,rp)`
fn chop(d: &mut Duel, a: &EffArgs) {
    // local g=Group.CreateGroup()
    let g = CardSet::new();
    // Duel.ChangeTargetCard(ev,g)
    duel::change_target_card(d, a.ev as u8, &g);
    // Duel.ChangeChainOperation(ev,s.repop)
    duel::change_chain_operation(d, a.ev as u8, operation(repop));
}
/// `s.repop(e,tp,eg,ep,ev,re,r,rp)`
fn repop(d: &mut Duel, a: &EffArgs) {
    // Duel.Draw(tp,1,REASON_EFFECT)
    duel::draw(d, a.tp, 1, REASON_EFFECT);
    // Duel.Draw(1-tp,1,REASON_EFFECT)
    duel::draw(d, 1 - a.tp, 1, REASON_EFFECT);
}
/// `s.xfilter(c)`
fn xfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_XYZ) and c:IsSetCard(SET_NUMBER)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_XYZ) && card::is_set_card(d, c, &[SET_NUMBER as u16])
}
/// `s.xtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn xtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.xfilter(chkc) and chkc~=c end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && xfilter(d, chkc) && chkc != c;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.xfilter,tp,LOCATION_MZONE,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&xfilter), a.tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.xfilter,tp,LOCATION_MZONE,0,1,1,c)
    duel::select_target(d, a.tp, Some(&xfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    true
}
/// `s.xop(e,tp,eg,ep,ev,re,r,rp)`
fn xop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc and tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && !card::is_immune_to_effect(d, tc, a.e) {
                // Duel.Overlay(tc,c)
                duel::overlay(d, tc, CardSet::from_card(c), false);
            }
        }
    }
}
