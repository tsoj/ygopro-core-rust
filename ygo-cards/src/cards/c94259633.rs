//! Relinquished Anima
use super::prelude::*;

pub const ID: u32 = 94259633;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,s.matfilter,1,1)
    link::add_procedure(d, c, Some(mat_filter(matfilter)), 1, Some(1), None, None, None);
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
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.eqcon)
    effect::set_condition(d, e1, Some(condition(eqcon)));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,s.eqcon,function(ec,_,tp) return ec:IsControler(tp,1-tp) end,s.equipop,e1)
    aux::add_er_equip_limit(
        d,
        c,
        Some(condition(eqcon)),
        lua_fn(|d, a| card::is_controler(d, a.c(0), a.p(2))),
        lua_fn(|d, a| equipop(d, a.c(0), a.e(1), a.p(2), a.c(3))),
        e1,
        None,
        None,
        None,
    );
    // atk/def
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetCondition(s.adcon)
    effect::set_condition(d, e2, Some(condition(adcon)));
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.matfilter(c,lc,sumtype,tp)`
fn matfilter(d: &mut Duel, c: CardId, lc: Option<CardId>, sumtype: u64, tp: u8) -> bool {
    // return c:GetLevel()==1 and not c:IsType(TYPE_TOKEN,lc,sumtype,tp)
    card::get_level(d, c) == 1 && !card::is_type_ctx(d, c, TYPE_TOKEN, lc, sumtype, Some(tp))
}
/// `s.eqcon(e,tp,eg,ep,ev,re,r,rp)`
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetHandler():GetEquipGroup():Filter(s.eqfilter,nil)
    // return #g==0
    let h = effect::get_handler(d, a.e).nil();
    let g = card::get_equip_group(d, h);
    group::filter(d, &g, &|d, c| eqfilter(d, c), NO_EX).is_empty()
}
/// `s.eqfilter(c)`
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:GetFlagEffect(id)~=0
    card::get_flag_effect(d, c, ID) != 0
}
/// `s.eqfilter2(c,e,tp,lg)`
fn eqfilter2(d: &mut Duel, c: CardId, tp: u8, lg: &CardSet) -> bool {
    // return c:IsFaceup() and (c:IsControler(tp) or c:IsAbleToChangeControler()) and lg:IsContains(c)
    card::is_faceup(d, c) && (card::is_controler(d, c, tp) || card::is_able_to_change_controler(d, c)) && lg.contains(c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local lg=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, h);
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and s.eqfilter2(chkc,e,tp,lg) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && eqfilter2(d, chkc, a.tp, &lg);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.eqfilter2,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,e,tp,lg) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| eqfilter2(d, c, a.tp, &lg)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.eqfilter2,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,e,tp,lg)
    let g = duel::select_target(d, a.tp, Some(&|d, c| eqfilter2(d, c, a.tp, &lg)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, 0, 0);
    true
}
/// `s.equipop(c,e,tp,tc)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId) {
    // c:EquipByEffectAndLimitRegister(e,tp,tc,id)
    card::equip_by_effect_and_limit_register(d, c, e, tp, tc, Some(ID), None);
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsFacedown() then return end
    if card::is_facedown(d, tc) {
        return;
    }
    // if tc and tc:IsRelateToEffect(e) and tc:IsMonster() and s.eqcon(e,tp,eg,ep,ev,re,r,rp) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_monster(d, tc) && eqcon(d, a) {
        // s.equipop(c,e,tp,tc)
        equipop(d, c, a.e, a.tp, tc);
    }
}
/// `s.adcon(e,tp,eg,ep,ev,re,r,rp)`
fn adcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=c:GetEquipGroup():Filter(s.eqfilter,nil)
    // return #g>0
    let g = card::get_equip_group(d, c);
    !group::filter(d, &g, &|d, x| eqfilter(d, x), NO_EX).is_empty()
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // local g=c:GetEquipGroup():Filter(s.eqfilter,nil)
    let eq = card::get_equip_group(d, c);
    let g = group::filter(d, &eq, &|d, x| eqfilter(d, x), NO_EX);
    // local atk=g:GetFirst():GetTextAttack()
    let first = group::get_first(d, &g).nil();
    let atk = card::get_text_attack(d, first) as i64;
    // if g:GetFirst():GetOriginalType()&TYPE_MONSTER==0 or atk<0 then
    if (card::get_original_type(d, first) & TYPE_MONSTER) == 0 || atk < 0 {
        // return 0
        0
    } else {
        // return atk
        atk
    }
}
