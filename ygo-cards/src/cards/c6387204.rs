//! Number C6: Chronomaly Chaos Atlandis
use super::prelude::*;

pub const ID: u32 = 6387204;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMBER], xyz_number: Some(6), listed_names: &[9161357], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz.AddProcedure(c,nil,7,3)
    xyz::add_procedure(d, c, None, Some(7), 3, None, None, None, None, false, None);
    // Equip 1 monster your opponent controls to this card as an Equip Spell Card whose effect makes this monster gain 1000 ATK
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
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
    // aux.AddEREquipLimit(c,nil,function(ec,_,tp) return ec:IsControler(1-tp) end,s.equipop,e1)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| card::is_controler(d, a.c(0), 1 - a.p(2))),
        lua_fn(|d, a| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e1,
        None,
        None,
        None,
    );
    // Make your opponent's Life Points 100
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(function(e,tp) return Duel.GetLP(1-tp)~=100 and e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,9161357) end)
    effect::set_condition(d, e2, Some(condition(cond2)));
    // e2:SetCost(Cost.AND(Cost.DetachFromSelf(3),s.lpcost))
    effect::set_cost(d, e2, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(3), None, None), cost_target(lpcost)])));
    // e2:SetOperation(s.lpop)
    effect::set_operation(d, e2, Some(operation(lpop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.nodamage(e,tp)`
fn nodamage(d: &mut Duel, e: EffectId, tp: u8) {
    // if Duel.HasFlagEffect(tp,id) then return end
    if duel::has_flag_effect(d, tp, ID, None) {
        return;
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e1, EFFECT_CHANGE_DAMAGE);
    // e1:SetTargetRange(0,1)
    effect::set_target_range(d, e1, 0, 1);
    // e1:SetValue(0)
    effect::set_value(d, e1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
}
/// `s.eqtg(e,tp,...,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(1-tp) and chkc:IsAbleToChangeControler() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, 1 - tp) && card::is_able_to_change_controler(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(Card.IsAbleToChangeControler,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&card::is_able_to_change_controler), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,Card.IsAbleToChangeControler,tp,0,LOCATION_MZONE,1,1,nil)
    let g = duel::select_target(d, tp, Some(&card::is_able_to_change_controler), tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, 0, 0);
    true
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // if not c:EquipByEffectAndLimitRegister(e,tp,tc,id) then return end
    if !card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), None) {
        return;
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(1000)
    effect::set_value(d, e1, 1000);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
/// `s.eqop(e,tp,...)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        let tc = tc.nil();
        if card::is_relate_to_effect(d, tc, a.e) {
            // s.equipop(c,e,tp,tc)
            equipop(d, c, a.e, a.tp, tc);
        }
    }
    // s.nodamage(e,tp)
    nodamage(d, a.e, a.tp);
}
/// `s.lpcostfilter(c)`
fn lpcostfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:HasFlagEffect(id) and c:IsSetCard(SET_NUMBER) and c:IsFaceup() and c:IsAbleToGraveAsCost()
    card::has_flag_effect(d, c, ID, None)
        && card::is_set_card(d, c, &[SET_NUMBER as u16])
        && card::is_faceup(d, c)
        && card::is_able_to_grave_as_cost(d, c)
}
/// `s.lpcost(e,tp,...,chk)`
fn lpcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local eqg=e:GetHandler():GetEquipGroup():Filter(s.lpcostfilter,nil)
    let h = effect::get_handler(d, a.e).nil();
    let eq = card::get_equip_group(d, h);
    let eqg = group::filter(d, &eq, &|d, c| lpcostfilter(d, c), NO_EX);
    // if chk==0 then return #eqg>0 end
    if a.chk == Some(0) {
        return !eqg.is_empty();
    }
    // Duel.SendtoGrave(eqg,REASON_COST)
    duel::send_to_grave(d, eqg, REASON_COST, None, None);
    true
}
/// `s.lpop(e,tp,...)`
fn lpop(d: &mut Duel, a: &EffArgs) {
    // Duel.SetLP(1-tp,100)
    duel::set_lp(d, 1 - a.tp, 100);
    // s.nodamage(e,tp)
    nodamage(d, a.e, a.tp);
}
/// `function(e,tp) return Duel.GetLP(1-tp)~=100 and e:GetHandler():GetOverlayGroup():IsExists(Card.IsCode,1,nil,9161357) end`
fn cond2(d: &mut Duel, a: &EffArgs) -> bool {
    duel::get_lp(d, 1 - a.tp) != 100 && {
        let h = effect::get_handler(d, a.e).nil();
        let ov = card::get_overlay_group(d, h);
        group::is_exists(d, &ov, &|d, c| card::is_code(d, c, &[9161357]), 1, NO_EX)
    }
}
