//! Mermaid Archer
use super::prelude::*;

pub const ID: u32 = 4252828;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
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
    // e1:SetCondition(s.eqcon)
    effect::set_condition(d, e1, Some(condition(eqcon)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,s.eqcon,s.eqval,s.equipop,e1)
    aux::add_er_equip_limit(
        d,
        c,
        Some(condition(eqcon)),
        lua_fn(|d, a| eqval(d, a.c(0), a.p(2))),
        lua_fn(|d, a| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e1,
        None,
        None,
        None,
    );
    // Destroy replace
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetTarget(s.desreptg)
    effect::set_target(d, e2, Some(cost_target(desreptg)));
    // e2:SetOperation(s.desrepop)
    effect::set_operation(d, e2, Some(operation(desrepop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.eqval(ec,c,tp)`
fn eqval(d: &mut Duel, ec: CardId, tp: u8) -> bool {
    // return ec:IsControler(tp) and ec:IsLevelBelow(3)
    card::is_controler(d, ec, tp) && card::is_level_below(d, ec, 3)
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetFlagEffect(id)~=0
    card::get_flag_effect(d, c, ID) != 0
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsLevelBelow(3)
    card::is_faceup(d, c) && card::is_level_below(d, c, 3)
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetHandler():GetEquipGroup():Filter(s.eqfilter,nil)
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_equip_group(d, h);
    let g = group::filter(d, &g, &eqfilter, NO_EX);
    // return #g==0
    g.is_empty()
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local h=e:GetHandler() (the Lua reads e:GetHandler() in each branch)
    let h = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.filter(chkc) and chkc~=e:GetHandler() end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && filter(d, chkc) && chkc != h;
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_MZONE,0,1,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, h.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_MZONE,0,1,1,e:GetHandler())
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_MZONE, 0, 1, 1, false, h.into());
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
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e2, EFFECT_TYPE_EQUIP);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
    // e2:SetValue(800)
    effect::set_value(d, e2, 800);
    // tc:RegisterEffect(e2)
    card::register_effect(d, tc, e2, false);
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsFaceup() and tc:IsRelateToEffect(e) and tc:IsMonster() then
    if let Some(tc) = tc {
        if card::is_faceup(d, tc) && card::is_relate_to_effect(d, tc, a.e) && card::is_monster(d, tc) {
            // s.equipop(c,e,tp,tc)
            equipop(d, c, a.e, a.tp, tc);
        }
    }
}
/// `s.desreptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ec=c:GetEquipGroup():Filter(s.eqfilter,nil):GetFirst()
    let g = card::get_equip_group(d, c);
    let g = group::filter(d, &g, &eqfilter, NO_EX);
    let ec = group::get_first(d, &g);
    // if chk==0 then return ec and ec:IsHasCardTarget(c) and ec:IsDestructable(e) and not ec:IsStatus(STATUS_DESTROY_CONFIRMED)
    //     and not c:IsReason(REASON_REPLACE) end
    if a.chk == Some(0) {
        return match ec {
            Some(ec) => {
                card::is_has_card_target(d, ec, c)
                    && card::is_destructable(d, ec, Some(a.e))
                    && !card::is_status(d, ec, STATUS_DESTROY_CONFIRMED)
                    && !card::is_reason(d, c, REASON_REPLACE)
            }
            None => false,
        };
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, a.tp, c, Some(96))
}
/// `s.desrepop(e,tp,eg,ep,ev,re,r,rp)`
fn desrepop(d: &mut Duel, a: &EffArgs) {
    // local ec=e:GetHandler():GetEquipGroup():Filter(s.eqfilter,nil):GetFirst()
    let c = effect::get_handler(d, a.e).nil();
    let g = card::get_equip_group(d, c);
    let g = group::filter(d, &g, &eqfilter, NO_EX);
    let ec = group::get_first(d, &g);
    // Duel.Destroy(ec,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, ec.nil(), REASON_EFFECT | REASON_REPLACE, None, None);
}
