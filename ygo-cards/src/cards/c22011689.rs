//! Predaplant Moray Nepenthes
//! 捕食植物モーレイ・ネペンテス
use super::prelude::*;

pub const ID: u32 = 22011689;

pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[COUNTER_PREDATOR], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Gains 200 ATK for each Predator Counter on the field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return Duel.GetCounter(0,1,1,COUNTER_PREDATOR)*200 end)
    effect::set_value_fn(d, e1, effect_card_value(|d, _e, _c| duel::get_counter(d, 0, 1, 1, COUNTER_PREDATOR as u16) as i64 * 200));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equip an opponent's monster that was destroyed by battle with this card to this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetCondition(aux.bdocon)
    effect::set_condition(d, e2, Some(aux::bdocon()));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.AddEREquipLimit(c,nil,aux.FilterBoolFunction(Card.IsMonster),function(c,e,tp,bc) c:EquipByEffectAndLimitRegister(e,tp,bc,id) end,e2)
    aux::add_er_equip_limit(
        d,
        c,
        None,
        lua_fn(|d, a| card::is_monster(d, a.c(0))),
        lua_fn(|d, a| card::equip_by_effect_and_limit_register(d, a.c(0), a.e(1), a.p(2), a.c(3), Some(ID), None)),
        e2,
        None,
        None,
        None,
    );
    // Destroy 1 Monster Card equipped to this card by this card's effect and gain LP equal to its ATK
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY+CATEGORY_RECOVER)
    effect::set_category(d, e3, CATEGORY_DESTROY + CATEGORY_RECOVER);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local bc=e:GetHandler():GetBattleTarget()
    let h = effect::get_handler(d, a.e).nil();
    let bc = card::get_battle_target(d, h);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and bc:IsMonster() and bc:IsFaceup() end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0 && card::is_monster(d, bc.nil()) && card::is_faceup(d, bc.nil());
    }
    // Duel.SetTargetCard(bc)
    duel::set_target_card(d, CardSet::from_card(bc.nil()));
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,bc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, Some(CardSet::from_card(bc.nil())), 1, tp, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local bc=Duel.GetFirstTarget()
    let bc = duel::first_target(d).nil();
    // if bc:IsRelateToEffect(e) and bc:IsMonster() and bc:IsFaceup() then
    if card::is_relate_to_effect(d, bc, a.e) && card::is_monster(d, bc) && card::is_faceup(d, bc) {
        // e:GetHandler():EquipByEffectAndLimitRegister(e,tp,bc,id)
        let h = effect::get_handler(d, a.e).nil();
        card::equip_by_effect_and_limit_register(d, h, a.e, a.tp, bc, Some(ID), None);
    }
}
/// `s.desfilter(c,ec)`
fn desfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:HasFlagEffect(id) and c:GetEquipTarget()==ec and c:IsMonsterCard()
    card::has_flag_effect(d, c, ID, None) && card::get_equip_target(d, c) == Some(ec) && card::is_monster_card(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return chkc:IsLocation(LOCATION_SZONE) and chkc:IsControler(tp) and s.desfilter(chkc,c) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_SZONE) && card::is_controler(d, chkc, tp) && desfilter(d, chkc, c);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_SZONE,0,1,nil,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, x| desfilter(d, x, c)), tp, LOCATION_SZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_SZONE,0,1,1,nil,c)
    let g = duel::select_target(d, tp, Some(&|d, x| desfilter(d, x, c)), tp, LOCATION_SZONE, 0, 1, 1, false, NO_EX).nil();
    // local atk=g:GetFirst():GetTextAttack()
    let first = group::get_first(d, &g).nil();
    let atk = card::get_text_attack(d, first) as i64;
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    // if atk>0 then
    if atk > 0 {
        // Duel.SetOperationInfo(0,CATEGORY_RECOVER,nil,0,tp,atk)
        duel::set_operation_info(d, 0, CATEGORY_RECOVER, None, 0, tp, atk as i32);
    }
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and Duel.Destroy(tc,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && duel::destroy(d, CardSet::from_card(tc), REASON_EFFECT, None, None) > 0 {
        // Duel.Recover(tp,tc:GetTextAttack(),REASON_EFFECT)
        let atk = card::get_text_attack(d, tc) as i64;
        duel::recover(d, tp, atk, REASON_EFFECT, false, None);
    }
}
