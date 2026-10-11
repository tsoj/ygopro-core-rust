//! Inzektor Firefly
use super::prelude::*;

pub const ID: u32 = 66066482;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_INZEKTOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e1)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| eqval(d, a.c(0), a.p(2))),
        lua_fn(|d, a| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e1,
        None,
        None,
        None,
    );
    // equip effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_LEVEL)
    effect::set_code(d, e2, EFFECT_UPDATE_LEVEL);
    // e2:SetValue(1)
    effect::set_value(d, e2, 1);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e3, EFFECT_TYPE_EQUIP);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetValue(100)
    effect::set_value(d, e3, 100);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e4, EFFECT_TYPE_EQUIP);
    // e4:SetCode(EFFECT_UPDATE_DEFENSE)
    effect::set_code(d, e4, EFFECT_UPDATE_DEFENSE);
    // e4:SetValue(100)
    effect::set_value(d, e4, 100);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // confirm
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_EQUIP)
    effect::set_code(d, e3, EVENT_EQUIP);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(s.cfcon)
    effect::set_condition(d, e3, Some(condition(cfcon)));
    // e3:SetTarget(s.cftg)
    effect::set_target(d, e3, Some(cost_target(cftg)));
    // e3:SetOperation(s.cfop)
    effect::set_operation(d, e3, Some(operation(cfop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, ec: CardId, tp: u8) -> bool {
    // return ec:IsControler(tp) and ec:IsSetCard(SET_INZEKTOR)
    card::is_controler(d, ec, tp) && card::is_set_card(d, ec, &[SET_INZEKTOR as u16])
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_INZEKTOR) and c:IsMonster() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_INZEKTOR as u16]) && card::is_monster(d, c) && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //   and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE|LOCATION_HAND,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,tp,LOCATION_GRAVE|LOCATION_HAND)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, a.tp, (LOCATION_GRAVE | LOCATION_HAND) as i32);
    true
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // c:EquipByEffectAndLimitRegister(e,tp,tc,nil,true)
    card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, Some(true));
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if Duel.GetLocationCount(tp,LOCATION_SZONE)<=0 then return end
    if duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // if c:IsFacedown() or not c:IsRelateToEffect(e) then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_GRAVE|LOCATION_HAND,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(filter)), a.tp, LOCATION_GRAVE | LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    let tc = group::get_first(d, &g);
    // if tc then
    if let Some(tc) = tc {
        // s.equipop(c,e,tp,tc)
        equipop(d, c, a.e, a.tp, tc);
    }
}
/// `s.eqfilter(c,ec)`
fn eqfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:IsFaceup() and c:GetEquipTarget()==ec and c:IsSetCard(SET_INZEKTOR)
    card::is_faceup(d, c) && card::get_equip_target(d, c) == Some(ec) && card::is_set_card(d, c, &[SET_INZEKTOR as u16])
}
/// `s.cfcon(e,tp,eg,ep,ev,re,r,rp)`
fn cfcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.eqfilter,1,nil,e:GetHandler())
    let c = effect::get_handler(d, a.e).nil();
    let eg = a.eg();
    group::is_exists(d, &eg, &|d, x| eqfilter(d, x, c), 1, NO_EX)
}
/// `s.cftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cftg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsFacedown,tp,0,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_facedown(d, x)), a.tp, 0, LOCATION_ONFIELD, 1, NO_EX);
    }
    true
}
/// `s.cfop(e,tp,eg,ep,ev,re,r,rp)`
fn cfop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(Card.IsFacedown,tp,0,LOCATION_ONFIELD,nil)
    let g = duel::get_matching_group(d, Some(&|d, x| card::is_facedown(d, x)), a.tp, 0, LOCATION_ONFIELD, NO_EX);
    // if #g>0 then
    if g.len() > 0 {
        // Duel.ConfirmCards(tp,g)
        duel::confirm_cards(d, a.tp, g);
    }
}
