//! Borreload Savage Dragon
//! Scripted by AlphaKretin
use super::prelude::*;

pub const ID: u32 = 27548199;

pub const INFO: ScriptInfo = ScriptInfo {
    counter_place_list: &[0x147],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x147)
    card::enable_counter_permit(d, c, 0x147, None, None);
    // synchro summon
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTuner(nil),1,99)
    synchro::add_procedure(d, c, None, 1, 1, Some(synchro::non_tuner(None)), 1, 99, None, None, None, None, None);
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY, 0);
    // e1:SetCondition(s.eqcon)
    effect::set_condition(d, e1, Some(condition(eqcon)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e1)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equipop_lua), e1, None, None, None);
    // Negate activation
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_NEGATE)
    effect::set_category(d, e2, CATEGORY_NEGATE);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.discon)
    effect::set_condition(d, e2, Some(condition(discon)));
    // e2:SetCost(s.discost)
    effect::set_cost(d, e2, Some(cost_target(discost)));
    // e2:SetTarget(s.distg)
    effect::set_target(d, e2, Some(cost_target(distg)));
    // e2:SetOperation(s.disop)
    effect::set_operation(d, e2, Some(operation(disop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, a: Args) -> bool {
    let ec = a.c(0);
    let tp = a.p(2);
    // return ec:IsControler(tp) and (ec:GetOriginalType()&TYPE_LINK==TYPE_LINK)
    card::is_controler(d, ec, tp) && (card::get_original_type(d, ec) & TYPE_LINK) == TYPE_LINK
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLinkMonster() and not c:IsForbidden()
    card::is_link_monster(d, c) && !card::is_forbidden(d, c)
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsSynchroSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_synchro_summoned(d, h)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, a.tp, LOCATION_GRAVE as i32);
    true
}
/// `s.equipop(c,e,tp,tc)`, the `equipop` of `aux.AddEREquipLimit`
fn equipop_lua(d: &mut Duel, a: Args) {
    equipop(d, a.c(0), a.e(1), a.p(2), a.c(3));
}
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc,id,true) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), Some(true)) {
        return;
    }
    // local lk=tc:GetLink()
    let lk = card::get_link(d, tc);
    // local atk=tc:GetTextAttack()/2
    let mut atk = card::get_text_attack(d, tc) as f64 / 2.0;
    // if atk<0 then atk=0 end
    if atk < 0.0 {
        atk = 0.0;
    }
    // if lk>0 then
    if lk > 0 {
        // c:AddCounter(0x147,lk)
        card::add_counter(d, c, 0x147, lk as u16, false);
    }
    // if atk>0 then
    if atk > 0.0 {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_EQUIP)
        effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
        // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE+EFFECT_FLAG_OWNER_RELATE)
        effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE + EFFECT_FLAG_OWNER_RELATE, 0);
        // e2:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
        // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
        // e2:SetValue(atk)
        effect::set_value(d, e2, lua_int(atk));
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
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
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.filter),tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&aux::necro_valley_filter(|d, x| filter(d, x))), a.tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // local tc=g:GetFirst()
    // if tc then s.equipop(c,e,tp,tc) end
    if let Some(tc) = group::get_first(d, &g) {
        equipop(d, c, a.e, a.tp, tc);
    }
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp~=tp and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)
    let h = effect::get_handler(d, a.e).nil();
    a.rp != a.tp && !card::is_status(d, h, STATUS_BATTLE_DESTROYED) && duel::is_chain_negatable(d, a.ev as u8)
}
/// `s.discost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discost(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():IsCanRemoveCounter(tp,0x147,1,REASON_COST) end
    if a.chk == Some(0) {
        return card::is_can_remove_counter(d, h, a.tp, 0x147, 1, REASON_COST);
    }
    // e:GetHandler():RemoveCounter(tp,0x147,1,REASON_COST)
    card::remove_counter(d, h, a.tp, 0x147, 1, REASON_COST);
    true
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:IsHasType(EFFECT_TYPE_ACTIVATE) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) && effect::is_has_type(d, a.re.nil(), EFFECT_TYPE_ACTIVATE) {
        let rh = effect::get_handler(d, a.re.nil()).nil();
        if card::is_relate_to_effect(d, rh, a.re.nil()) {
            // Duel.SendtoGrave(eg,REASON_EFFECT)
            duel::send_to_grave(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
