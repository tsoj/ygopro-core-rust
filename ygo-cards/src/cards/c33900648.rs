//! Clear World
use super::prelude::*;

pub const ID: u32 = 33900648;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local fid=c:GetFieldID()
    let fid = card::get_field_id(d, c);
    // local copying=c:IsStatus(STATUS_COPYING_EFFECT)
    let copying = card::is_status(d, c, STATUS_COPYING_EFFECT);
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Maintenance cost: Pay 500 LP or destroy this card
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e1, Some(condition(turn_cond)));
    // e1:SetOperation(s.maintop)
    effect::set_operation(d, e1, Some(operation(maintop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // ● LIGHT: Play with your hand revealed at all times
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_PUBLIC)
    effect::set_code(d, e2, EFFECT_PUBLIC);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_HAND,LOCATION_HAND)
    effect::set_target_range(d, e2, LOCATION_HAND, LOCATION_HAND);
    // e2:SetTarget(function(e,c) return s.PlayerIsAffectedByClearWorld(c:GetControler(),ATTRIBUTE_LIGHT) end)
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| affected_by_clear_world(d, card::get_controler(d, c), ATTRIBUTE_LIGHT))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // ● DARK: If you control 2 or more monsters, you cannot declare an attack
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e3, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(1,0)
    effect::set_target_range(d, e3, 1, 0);
    // e3:SetCondition(s.darkconyou)
    effect::set_condition(d, e3, Some(condition(darkconyou)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCondition(s.darkconopp)
    effect::set_condition(d, e4, Some(condition(darkconopp)));
    // e4:SetTargetRange(0,1)
    effect::set_target_range(d, e4, 0, 1);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // ● EARTH: Destroy 1 face-up Defense Position monster you control
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e5, aux::stringid(ID, 1));
    // e5:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e5, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e5:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e5, EVENT_PHASE | PHASE_STANDBY);
    // e5:SetRange(LOCATION_FZONE)
    effect::set_range(d, e5, LOCATION_FZONE);
    // e5:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) and s.PlayerControlsAttributeOrIsAffectedByClearWall(tp,ATTRIBUTE_EARTH) and c:IsHasEffect(id) and not c:HasFlagEffect(id) and (not copying or c:IsFieldID(fid)) end)
    effect::set_condition(d, e5, Some(condition(move |d, a| phase_cond(d, a.tp, c, copying, fid, ATTRIBUTE_EARTH, ID))));
    // e5:SetOperation(s.desop)
    effect::set_operation(d, e5, Some(operation(desop)));
    // if copying then e5:SetReset(RESET_PHASE|PHASE_END) end
    if copying {
        effect::set_reset(d, e5, RESET_PHASE | PHASE_END, 1);
    }
    // Duel.RegisterEffect(e5,0)
    duel::register_effect(d, e5, 0);
    // local e5b=e5:Clone()
    let e5b = effect::clone(d, e5);
    // Duel.RegisterEffect(e5b,1)
    duel::register_effect(d, e5b, 1);
    // ● WATER: Discard 1 card
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e6, aux::stringid(ID, 2));
    // e6:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e6:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e6, EVENT_PHASE + PHASE_END);
    // e6:SetRange(LOCATION_FZONE)
    effect::set_range(d, e6, LOCATION_FZONE);
    // e6:SetCountLimit(1)
    effect::set_count_limit(d, e6, 1, 0, 0, 0);
    // e6:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) and s.PlayerControlsAttributeOrIsAffectedByClearWall(tp,ATTRIBUTE_WATER) and c:IsHasEffect(id) and not c:HasFlagEffect(id+1) and (not copying or c:IsFieldID(fid)) end)
    effect::set_condition(d, e6, Some(condition(move |d, a| phase_cond(d, a.tp, c, copying, fid, ATTRIBUTE_WATER, ID + 1))));
    // e6:SetOperation(s.discardop)
    effect::set_operation(d, e6, Some(operation(discardop)));
    // if copying then e6:SetReset(RESET_PHASE|PHASE_END) end
    if copying {
        effect::set_reset(d, e6, RESET_PHASE | PHASE_END, 1);
    }
    // Duel.RegisterEffect(e6,0)
    duel::register_effect(d, e6, 0);
    // local e6b=e6:Clone()
    let e6b = effect::clone(d, e6);
    // Duel.RegisterEffect(e6b,1)
    duel::register_effect(d, e6b, 1);
    // ● FIRE: Take 1000 damage
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e7, aux::stringid(ID, 3));
    // e7:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e7, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e7:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e7, EVENT_PHASE + PHASE_END);
    // e7:SetRange(LOCATION_FZONE)
    effect::set_range(d, e7, LOCATION_FZONE);
    // e7:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) and s.PlayerControlsAttributeOrIsAffectedByClearWall(tp,ATTRIBUTE_FIRE) and c:IsHasEffect(id) and not c:HasFlagEffect(id+2) and (not copying or c:IsFieldID(fid)) end)
    effect::set_condition(d, e7, Some(condition(move |d, a| phase_cond(d, a.tp, c, copying, fid, ATTRIBUTE_FIRE, ID + 2))));
    // e7:SetOperation(s.damop)
    effect::set_operation(d, e7, Some(operation(damop)));
    // if copying then e7:SetReset(RESET_PHASE|PHASE_END) end
    if copying {
        effect::set_reset(d, e7, RESET_PHASE | PHASE_END, 1);
    }
    // Duel.RegisterEffect(e7,0)
    duel::register_effect(d, e7, 0);
    // local e7b=e7:Clone()
    let e7b = effect::clone(d, e7);
    // Duel.RegisterEffect(e7b,1)
    duel::register_effect(d, e7b, 1);
    // ● WIND: You must pay 500 Life Points to activate a Spell Card
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e8, EFFECT_TYPE_FIELD);
    // e8:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e8, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e8:SetCode(EFFECT_ACTIVATE_COST)
    effect::set_code(d, e8, EFFECT_ACTIVATE_COST);
    // e8:SetRange(LOCATION_FZONE)
    effect::set_range(d, e8, LOCATION_FZONE);
    // e8:SetTargetRange(1,0)
    effect::set_target_range(d, e8, 1, 0);
    // e8:SetCondition(function(e) return s.PlayerIsAffectedByClearWorld(e:GetHandlerPlayer(),ATTRIBUTE_WIND) end)
    effect::set_condition(d, e8, Some(condition(|d, a| affected_by_clear_world(d, effect::get_handler_player(d, a.e), ATTRIBUTE_WIND))));
    // e8:SetTarget(function(e,te,tp) return te:IsSpellEffect() and te:IsHasType(EFFECT_TYPE_ACTIVATE) end)
    effect::set_target(d, e8, Some(lua_fn(|d, a| {
        let te = a.e(1);
        effect::is_spell_effect(d, te) && effect::is_has_type(d, te, EFFECT_TYPE_ACTIVATE)
    })));
    // e8:SetCost(function(e,te_or_c,tp) return Duel.CheckLPCost(tp,500) end)
    effect::set_cost(d, e8, Some(lua_fn(|d, a| duel::check_lp_cost(d, a.p(2), 500))));
    // e8:SetOperation(function(e,tp) Duel.PayLPCost(tp,500) end)
    effect::set_operation(d, e8, Some(lua_fn(|d, a| duel::pay_lp_cost(d, a.p(1), 500))));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
    // local e9=e8:Clone()
    let e9 = effect::clone(d, e8);
    // e9:SetTargetRange(0,1)
    effect::set_target_range(d, e9, 0, 1);
    // e9:SetCondition(function(e) return s.PlayerIsAffectedByClearWorld(1-e:GetHandlerPlayer(),ATTRIBUTE_WIND) end)
    effect::set_condition(d, e9, Some(condition(|d, a| affected_by_clear_world(d, 1 - effect::get_handler_player(d, a.e), ATTRIBUTE_WIND))));
    // c:RegisterEffect(e9)
    card::register_effect(d, c, e9, false);
    // Apply a dummy effect on itself to track whether the card's effects are currently active or not
    // local e10=Effect.CreateEffect(c)
    let e10 = effect::create_effect(d, c);
    // e10:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e10, EFFECT_TYPE_SINGLE);
    // e10:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e10, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e10:SetCode(id)
    effect::set_code(d, e10, ID);
    // e10:SetRange(LOCATION_FZONE)
    effect::set_range(d, e10, LOCATION_FZONE);
    // c:RegisterEffect(e10)
    card::register_effect(d, c, e10, false);
}
/// `function(e,tp) return Duel.IsTurnPlayer(tp) end`
fn turn_cond(d: &mut Duel, a: &EffArgs) -> bool {
    duel::is_turn_player(d, a.tp)
}
/// `s.maintop(e,tp,eg,ep,ev,re,r,rp)`: "Pay 500 LP or destroy this card"
fn maintop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, a.e));
    // local b1=Duel.CheckLPCost(tp,500)
    let b1 = duel::check_lp_cost(d, tp, 500);
    // local b2=true
    let b2 = true;
    // local op=b1 and Duel.SelectEffect(tp, {b1,aux.Stringid(id,4)}, {b2,aux.Stringid(id,5)}) or 2
    let op = if b1 {
        duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 4)), (b2, aux::stringid(ID, 5))]).unwrap_or(2)
    } else {
        2
    };
    if op == 1 {
        // Duel.PayLPCost(tp,500)
        duel::pay_lp_cost(d, tp, 500);
    } else if op == 2 {
        // Duel.Destroy(e:GetHandler(),REASON_COST)
        let h = effect::get_handler(d, a.e).nil();
        duel::destroy(d, h, REASON_COST, None, None);
    }
}
/// `s.PlayerControlsAttributeOrIsAffectedByClearWall(player,attribute)`
fn controls_or_clear_wall(d: &mut Duel, player: u8, attribute: u32) -> bool {
    // return Duel.IsPlayerAffectedByEffect(1-player,EFFECT_CLEAR_WALL)
    //     or Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsAttribute,attribute),player,LOCATION_MZONE,0,1,nil)
    duel::is_player_affected_by_effect(d, 1 - player, EFFECT_CLEAR_WALL).is_some()
        || duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_faceup(d, c) && card::is_attribute(d, c, attribute)),
            player,
            LOCATION_MZONE,
            0,
            1,
            NO_EX,
        )
}
/// `s.PlayerIsAffectedByClearWorld(player,attribute)`
fn affected_by_clear_world(d: &mut Duel, player: u8, attribute: u32) -> bool {
    // return not Duel.IsPlayerAffectedByEffect(player,EFFECT_CLEAR_WORLD_IMMUNE)
    //     and s.PlayerControlsAttributeOrIsAffectedByClearWall(player,attribute)
    duel::is_player_affected_by_effect(d, player, EFFECT_CLEAR_WORLD_IMMUNE).is_none() && controls_or_clear_wall(d, player, attribute)
}
/// `s.darkconyou(e)`
fn darkconyou(d: &mut Duel, a: &EffArgs) -> bool {
    // local affected_player=e:GetHandlerPlayer()
    let affected_player = effect::get_handler_player(d, a.e);
    // return s.PlayerIsAffectedByClearWorld(affected_player,ATTRIBUTE_DARK) and Duel.GetFieldGroupCount(affected_player,LOCATION_MZONE,0)>=2
    affected_by_clear_world(d, affected_player, ATTRIBUTE_DARK) && duel::get_field_group_count(d, affected_player, LOCATION_MZONE, 0) >= 2
}
/// `s.darkconopp(e)`
fn darkconopp(d: &mut Duel, a: &EffArgs) -> bool {
    // local affected_player=1-e:GetHandlerPlayer()
    let affected_player = 1 - effect::get_handler_player(d, a.e);
    // return s.PlayerIsAffectedByClearWorld(affected_player,ATTRIBUTE_DARK) and Duel.GetFieldGroupCount(affected_player,LOCATION_MZONE,0)>=2
    affected_by_clear_world(d, affected_player, ATTRIBUTE_DARK) && duel::get_field_group_count(d, affected_player, LOCATION_MZONE, 0) >= 2
}
/// The condition of the three turn-start/end triggers (EARTH, WATER, FIRE):
/// `Duel.IsTurnPlayer(tp) and s.PlayerControlsAttributeOrIsAffectedByClearWall(tp,attr) and c:IsHasEffect(id)
///  and not c:HasFlagEffect(flag) and (not copying or c:IsFieldID(fid))`
fn phase_cond(d: &mut Duel, tp: u8, c: CardId, copying: bool, fid: u32, attr: u32, flag: u32) -> bool {
    duel::is_turn_player(d, tp)
        && controls_or_clear_wall(d, tp, attr)
        && !card::is_has_effect(d, c, ID, None).is_empty()
        && !card::has_flag_effect(d, c, flag, None)
        && (!copying || card::is_field_id(d, c, fid))
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`: EARTH
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // if not s.PlayerIsAffectedByClearWorld(tp,ATTRIBUTE_EARTH)
    //     or not Duel.IsExistingMatchingCard(Card.IsPosition,tp,LOCATION_MZONE,0,1,nil,POS_FACEUP_DEFENSE) then return end
    if !affected_by_clear_world(d, tp, ATTRIBUTE_EARTH)
        || !duel::is_existing_matching_card(d, Some(&|d, c| card::is_position(d, c, POS_FACEUP_DEFENSE)), tp, LOCATION_MZONE, 0, 1, NO_EX)
    {
        return;
    }
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, a.e));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsPosition,tp,LOCATION_MZONE,0,1,1,nil,POS_FACEUP_DEFENSE)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_position(d, c, POS_FACEUP_DEFENSE)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
/// `s.discardop(e,tp,eg,ep,ev,re,r,rp)`: WATER
fn discardop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // e:GetHandler():RegisterFlagEffect(id+1,RESETS_STANDARD_PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID + 1, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // if not s.PlayerIsAffectedByClearWorld(tp,ATTRIBUTE_WATER)
    //     or Duel.GetFieldGroupCount(tp,LOCATION_HAND,0)==0 then return end
    if !affected_by_clear_world(d, tp, ATTRIBUTE_WATER) || duel::get_field_group_count(d, tp, LOCATION_HAND, 0) == 0 {
        return;
    }
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, a.e));
    // Duel.DiscardHand(tp,nil,1,1,REASON_EFFECT|REASON_DISCARD)
    duel::discard_hand(d, tp, None, 1, 1, REASON_EFFECT | REASON_DISCARD, NO_EX);
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`: FIRE
fn damop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // e:GetHandler():RegisterFlagEffect(id+2,RESETS_STANDARD_PHASE_END,0,1)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID + 2, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
    // if not s.PlayerIsAffectedByClearWorld(tp,ATTRIBUTE_FIRE) then return end
    if !affected_by_clear_world(d, tp, ATTRIBUTE_FIRE) {
        return;
    }
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.Hint(HINT_OPSELECTED,1-tp,e:GetDescription())
    duel::hint(d, HINT_OPSELECTED as u8, 1 - tp, effect::get_description(d, a.e));
    // Duel.Damage(tp,1000,REASON_EFFECT)
    duel::damage(d, tp, 1000, REASON_EFFECT, false, None);
}
