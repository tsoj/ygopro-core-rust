//! Trap Gatherer
//! トラップ・ギャザー
use super::prelude::*;

pub const ID: u32 = 24839398;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // aux.AddEquipProcedure(c)
    aux::add_equip_procedure(d, c, None, None, None, None, None, None, None, None);
    // The equipped monster gains 400 ATK for each Trap in your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_EQUIP)
    effect::set_type(d, e1, EFFECT_TYPE_EQUIP);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(function(e) return Duel.GetMatchingGroupCount(Card.IsTrap,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil)*400 end)
    effect::set_value_fn(d, e1, lua_fn(atk_value));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Set 1 Trap from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_LEAVE_GRAVE+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_LEAVE_GRAVE + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_BATTLED)
    effect::set_code(d, e2, EVENT_BATTLED);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetCondition(s.setcon)
    effect::set_condition(d, e2, Some(condition(setcon)));
    // e2:SetCost(Cost.SelfToGrave)
    effect::set_cost(d, e2, Some(cost::self_to_grave()));
    // e2:SetTarget(s.settg)
    effect::set_target(d, e2, Some(cost_target(settg)));
    // e2:SetOperation(s.setop)
    effect::set_operation(d, e2, Some(operation(setop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish this card you control instead of a face-up Trap(s) you control being destroyed
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e3, EFFECT_DESTROY_REPLACE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetTarget(s.reptg)
    effect::set_target(d, e3, Some(cost_target(reptg)));
    // e3:SetOperation(function(e) Duel.Remove(e:GetHandler(),POS_FACEUP,REASON_EFFECT|REASON_REPLACE) end)
    effect::set_operation(d, e3, Some(operation(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        duel::remove(d, CardSet::from_card(h), POS_FACEUP as u8, REASON_EFFECT | REASON_REPLACE, None, None);
    })));
    // e3:SetValue(function(e,c) return s.repfilter(c,e:GetHandlerPlayer()) end)
    effect::set_value_fn(d, e3, effect_card_fn(|d, e, c| {
        let p = effect::get_handler_player(d, e);
        repfilter(d, c, p)
    }));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return Duel.GetMatchingGroupCount(Card.IsTrap,e:GetHandlerPlayer(),LOCATION_GRAVE,0,nil)*400 end`
fn atk_value(d: &mut Duel, a: Args) -> i64 {
    let p = effect::get_handler_player(d, a.e(0));
    duel::get_matching_group_count(d, Some(&|d, c| card::is_trap(d, c)), p, LOCATION_GRAVE, 0, NO_EX) as i64 * 400
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local ec=e:GetHandler():GetEquipTarget()
    let h = effect::get_handler(d, a.e).nil();
    // if not ec then return false end
    let Some(ec) = card::get_equip_target(d, h) else {
        return false;
    };
    // local bc=ec:GetBattleTarget()
    let bc = card::get_battle_target(d, ec);
    // local ex,damg,damp=Duel.CheckEvent(EVENT_BATTLE_DAMAGE,true)
    let ev = duel::check_event(d, EVENT_BATTLE_DAMAGE);
    // return (bc and bc:IsStatus(STATUS_BATTLE_DESTROYED) and bc:IsControler(1-tp)) or (ex and damp==1-tp and damg:GetFirst()==ec)
    let first = match bc {
        Some(bc) => card::is_status(d, bc, STATUS_BATTLE_DESTROYED) && card::is_controler(d, bc, 1 - tp),
        None => false,
    };
    first
        || match &ev {
            Some(ev) => ev.event_player == 1 - tp && group::get_first(d, &ev.event_cards.clone().nil()) == Some(ec),
            None => false,
        }
}
/// `s.setfilter(c,ignore_zones)`
fn setfilter(d: &mut Duel, c: CardId, ignore_zones: bool) -> bool {
    // return c:IsTrap() and c:IsSSetable(ignore_zones)
    card::is_trap(d, c) && card::is_s_setable(d, c, ignore_zones, None)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.setfilter,tp,LOCATION_GRAVE,0,1,nil,true) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| setfilter(d, c, true)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, tp, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectMatchingCard(tp,s.setfilter,tp,LOCATION_GRAVE,0,1,1,nil,false)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| setfilter(d, c, false)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SSet(tp,g)
        duel::s_set(d, tp, g, None, true);
    }
}
/// `s.repfilter(c,tp)`
fn repfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(tp) and c:IsOnField() and not c:IsReason(REASON_REPLACE) and c:IsReason(REASON_EFFECT)
    //     and c:IsTrap() and c:IsFaceup()
    card::is_controler(d, c, tp)
        && card::is_on_field(d, c)
        && !card::is_reason(d, c, REASON_REPLACE)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_trap(d, c)
        && card::is_faceup(d, c)
}
/// `s.reptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn reptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsStatus(STATUS_DESTROY_CONFIRMED)
    //     and c:IsAbleToRemove() and eg:IsExists(s.repfilter,1,nil,tp) end
    if a.chk == Some(0) {
        return !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
            && card::is_able_to_remove(d, c, None, None, None)
            && group::is_exists(d, &a.eg.clone().nil(), &|d, x| repfilter(d, x, tp), 1, NO_EX);
    }
    // return Duel.SelectEffectYesNo(tp,c,96)
    duel::select_effect_yes_no(d, tp, c, Some(96))
}
