//! DoomZ XII End - Drastrius
use super::prelude::*;

pub const ID: u32 = 95626382;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Xyz Summon procedure: 3 Level 8 monsters
    // Xyz.AddProcedure(c,nil,8,3)
    xyz::add_procedure(d, c, None, Some(8), 3, None, None, None, None, false, None);
    // The first time this card would be destroyed by battle each turn, it is not destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_COUNT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_COUNT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetValue(function(e,re,r) return (r&REASON_BATTLE|REASON_EFFECT)>0 end)
    effect::set_value_fn(d, e1, lua_fn(indestructable_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // You can equip 1 other face-up monster on the field to this card
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_SPSUMMON_SUCCESS);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsXyzSummoned() end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.AddEREquipLimit(c,nil,aux.TRUE,Card.EquipByEffectAndLimitRegister,e2)
    aux::add_er_equip_limit(d, c, None, aux_true(), lua_fn(equip_by_effect_op), e2, None, None, None);
    // Negate an opponent's monster effect activated on the field or in the GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DISABLE+CATEGORY_EQUIP)
    effect::set_category(d, e3, CATEGORY_DISABLE + CATEGORY_EQUIP);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.discon)
    effect::set_condition(d, e3, Some(condition(discon)));
    // e3:SetCost(Cost.DetachFromSelf(1))
    effect::set_cost(d, e3, Some(cost::detach_from_self(cost::Count::N(1), None, None)));
    // e3:SetTarget(s.distg)
    effect::set_target(d, e3, Some(cost_target(distg)));
    // e3:SetOperation(s.disop)
    effect::set_operation(d, e3, Some(operation(disop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // aux.AddEREquipLimit(c,nil,function(ec,c,tp) return ec:IsControler(1-tp) end,Card.EquipByEffectAndLimitRegister,e3)
    aux::add_er_equip_limit(d, c, None, lua_fn(opponent_equip_value), lua_fn(equip_by_effect_op), e3, None, None, None);
}
/// `function(e,re,r) return (r&REASON_BATTLE|REASON_EFFECT)>0 end` (EFFECT_INDESTRUCTABLE_COUNT)
/// Lua precedence: `|` binds tighter than `>`, so the whole left side is `(r&REASON_BATTLE)|REASON_EFFECT`.
fn indestructable_value(_d: &mut Duel, a: Args) -> bool {
    let r = a.int(2) as u32;
    ((r & REASON_BATTLE) | REASON_EFFECT) > 0
}
/// `function(e) return e:GetHandler():IsXyzSummoned() end` (e2)
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_xyz_summoned(d, c)
}
/// `s.eqfilter(c,tp)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and (c:IsControler(tp) or c:IsAbleToChangeControler())
    card::is_faceup(d, c) && (card::is_controler(d, c, tp) || card::is_able_to_change_controler(d, c))
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,e:GetHandler(),tp) end
    if a.chk == Some(0) {
        let c = effect::get_handler(d, a.e).nil();
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, c.into());
    }
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,1,PLAYER_EITHER,LOCATION_MZONE)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 1, PLAYER_EITHER as u8, LOCATION_MZONE as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFacedown() or not c:IsRelateToEffect(e) or Duel.GetLocationCount(tp,LOCATION_SZONE)==0 then return end
    if card::is_facedown(d, c) || !card::is_relate_to_effect(d, c, a.e) || duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local ec=Duel.SelectMatchingCard(tp,s.eqfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,c,tp):GetFirst()
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| eqfilter(d, x, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, c.into()).nil();
    let ec = group::get_first(d, &g);
    // if ec then
    if let Some(ec) = ec {
        // Duel.HintSelection(ec)
        duel::hint_selection(d, ec, true);
        // c:EquipByEffectAndLimitRegister(e,tp,ec)
        card::equip_by_effect_and_limit_register(d, c, a.e, a.tp, ec, None, None);
    }
}
/// `function(ec,c,tp) return ec:IsControler(1-tp) end` (equipval of e3)
fn opponent_equip_value(d: &mut Duel, a: Args) -> bool {
    card::is_controler(d, a.c(0), 1 - a.p(2))
}
/// `Card.EquipByEffectAndLimitRegister` as the equip operation of `aux.AddEREquipLimit`: `(c,e,tp,tc)`
fn equip_by_effect_op(d: &mut Duel, a: Args) {
    card::equip_by_effect_and_limit_register(d, a.c(0), a.e(1), a.p(2), a.c(3), None, None);
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp,chk)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local trig_loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_LOCATION)
    let trig_loc = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_LOCATION])[0].int();
    // return ep==1-tp and re:IsMonsterEffect() and trig_loc&(LOCATION_MZONE|LOCATION_GRAVE)>0 and Duel.IsChainDisablable(ev)
    if a.ep != 1 - a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_monster_effect(d, re) {
        return false;
    }
    if (trig_loc & (LOCATION_MZONE | LOCATION_GRAVE) as i64) <= 0 {
        return false;
    }
    duel::is_chain_disablable(d, a.ev as u8)
}
/// `s.distg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn distg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.SetOperationInfo(0,CATEGORY_DISABLE,eg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DISABLE, a.eg.clone(), 1, a.tp, 0);
    // Duel.SetPossibleOperationInfo(0,CATEGORY_EQUIP,nil,1,1-tp,LOCATION_MZONE)
    duel::set_possible_operation_info(d, 0, CATEGORY_EQUIP, None, 1, 1 - a.tp, LOCATION_MZONE as i32);
    true
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // if not Duel.NegateEffect(ev) then return end
    if !duel::negate_effect(d, a.ev as u8) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) or c:IsFacedown() then return end
    if !card::is_relate_to_effect(d, c, a.e) || card::is_facedown(d, c) {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.eqfilter,tp,0,LOCATION_MZONE,c,tp)
    let g = duel::get_matching_group(d, Some(&|d, x| eqfilter(d, x, a.tp)), a.tp, 0, LOCATION_MZONE, c.into());
    // if #g==0 or not Duel.SelectYesNo(tp,aux.Stringid(id,2)) then return end
    if g.is_empty() || !duel::select_yes_no(d, a.tp, aux::stringid(ID, 2)) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local ec=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    let ec = group::get_first(d, &sel);
    // if ec then
    if let Some(ec) = ec {
        // Duel.HintSelection(ec)
        duel::hint_selection(d, ec, true);
        // Duel.BreakEffect()
        duel::break_effect(d);
        // c:EquipByEffectAndLimitRegister(e,tp,ec)
        card::equip_by_effect_and_limit_register(d, c, a.e, a.tp, ec, None, None);
    }
}
