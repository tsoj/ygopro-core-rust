//! Number 6: Chronomaly Atlandis
use super::prelude::*;

pub const ID: u32 = 9161357;

pub const INFO: ScriptInfo = ScriptInfo { xyz_number: Some(6), listed_series: &[SET_NUMBER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 2 Level 6 monsters
    // Xyz.AddProcedure(c,nil,6,2)
    xyz::add_procedure(d, c, None, Some(6), 2, None, None, None, None, false, None);
    // Equip 1 "Number" monster from your GY to this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_xyz_summoned(d, h)
    })));
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,function(ec,c,tp) return ec:IsSetCard(SET_NUMBER) and ec:IsControler(tp) end,function(c,e,tp,tc) c:EquipByEffectAndLimitRegister(e,tp,tc,id) end,e1)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equipop), e1, None, None, None);
    // This card gains ATK equal to half the ATK of that equipped monster
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(function(e,c) return c:GetEquipGroup():Match(Card.HasFlagEffect,nil,id):GetSum(Card.GetAttack)/2 end)
    effect::set_value_fn(d, e2, effect_card_value(|d, _e, c| {
        let mut g = card::get_equip_group(d, c);
        group::match_(d, &mut g, &|d, cc| card::has_flag_effect(d, cc, ID, None), NO_EX);
        let sum = group::get_sum(d, &g, &|d, cc| card::get_attack(d, cc) as i64);
        lua_int(sum as f64 / 2.0)
    }));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Halve your opponent's Life Points
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(Cost.AND(Cost.DetachFromSelf(1),s.halvelpcost))
    effect::set_cost(d, e3, Some(cost::and(vec![cost::detach_from_self(cost::Count::N(1), None, None), cost_target(halvelpcost)])));
    // e3:SetOperation(function(e,tp) local opp=1-tp Duel.SetLP(opp,Duel.GetLP(opp)/2) end)
    effect::set_operation(d, e3, Some(operation(|d, a| {
        let opp = 1 - a.tp;
        let lp = duel::get_lp(d, opp);
        duel::set_lp(d, opp, lua_int(lp as f64 / 2.0) as i32);
    })));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_NUMBER) and c:IsMonster() and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_set_card(d, c, &[SET_NUMBER as u16]) && card::is_monster(d, c) && !card::is_forbidden(d, c) && card::check_unique_on_field(d, c, tp, None, None)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_GRAVE) and s.eqfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_GRAVE) && eqfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0 and Duel.IsExistingTarget(s.eqfilter,tp,LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        let tp = a.tp;
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.eqfilter,tp,LOCATION_GRAVE,0,1,1,nil,tp)
    let tp = a.tp;
    let g = duel::select_target(d, tp, Some(&|d, c| eqfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, 1, tp, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) && card::is_relate_to_effect(d, tc.nil(), a.e) {
        // c:EquipByEffectAndLimitRegister(e,tp,tc,id)
        card::equip_by_effect_and_limit_register(d, c, a.e, a.tp, tc.nil(), Some(ID), None);
    }
}
/// `aux.AddEREquipLimit` ecval: `function(ec,c,tp) return ec:IsSetCard(SET_NUMBER) and ec:IsControler(tp) end`
fn eqval(d: &mut Duel, a: Args) -> bool {
    card::is_set_card(d, a.c(0), &[SET_NUMBER as u16]) && card::is_controler(d, a.c(0), a.p(2))
}
/// `aux.AddEREquipLimit` equipop: `function(c,e,tp,tc) c:EquipByEffectAndLimitRegister(e,tp,tc,id) end`
fn equipop(d: &mut Duel, a: Args) {
    card::equip_by_effect_and_limit_register(d, a.c(0), a.e(1), a.p(2), a.c(3), Some(ID), None);
}
/// `s.halvelpcostfilter(c,tp)`
fn halvelpcostfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:HasFlagEffect(id) and c:IsControler(tp) and c:IsAbleToGraveAsCost()
    card::has_flag_effect(d, c, ID, None) && card::is_controler(d, c, tp) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.halvelpcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn halvelpcost(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local eqg=c:GetEquipGroup():Match(s.halvelpcostfilter,nil,tp)
    let tp = a.tp;
    let mut eqg = card::get_equip_group(d, c);
    group::match_(d, &mut eqg, &|d, cc| halvelpcostfilter(d, cc, tp), NO_EX);
    // if chk==0 then return Duel.IsPhase(PHASE_MAIN1) and #eqg>0 end
    if a.chk == Some(0) {
        return duel::is_phase(d, PHASE_MAIN1) && eqg.len() > 0;
    }
    // local g=nil
    // if #eqg==1 then g=eqg
    let g = if eqg.len() == 1 {
        eqg
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
        // g=eqg:Select(tp,1,1,nil)
        group::select(d, &eqg, tp, 1, 1, false, NO_EX).nil()
    };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // You cannot conduct your Battle Phase the turn you activate this effect
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e1, aux::stringid(ID, 2));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_OATH+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_OATH + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_BP)
    effect::set_code(d, e1, EFFECT_CANNOT_BP);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    true
}
