//! Zenna's Deceiving Doll Maidens
use super::prelude::*;

pub const ID: u32 = 7594154;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link Summon procedure: 2+ Effect Monsters
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsType,TYPE_EFFECT),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_type_ctx(d, c, TYPE_EFFECT, sc, st, Some(tp)))), 2, None, None, None, None);
    // You can also use 1 monster in your hand as material to Link Summon this card
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e0, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e0:SetCode(EFFECT_EXTRA_MATERIAL)
    effect::set_code(d, e0, EFFECT_EXTRA_MATERIAL);
    // e0:SetRange(LOCATION_EXTRA)
    effect::set_range(d, e0, LOCATION_EXTRA);
    // e0:SetTargetRange(1,0)
    effect::set_target_range(d, e0, 1, 0);
    // e0:SetOperation(function(c,e,tp,sg,mg,lc,og,chk) return sg:FilterCount(Card.HasFlagEffect,nil,id)<=1 end)
    effect::set_operation(
        d,
        e0,
        Some(lua_fn(|d, a| {
            let sg = a.g(3);
            let n = group::filter_count(d, &sg, &|d, c| card::has_flag_effect(d, c, ID, None), NO_EX);
            n <= 1
        })),
    );
    // e0:SetValue(s.extraval)
    effect::set_value_fn(d, e0, lua_fn(extraval));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // If this card is Link Summoned: You can send 1 monster from your Deck to the GY, or if this card was Link Summoned using only monsters you control, you can send 1 monster from your Extra Deck to the GY instead
    // local e1a=Effect.CreateEffect(c)
    let e1a = effect::create_effect(d, c);
    // e1a:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1a, aux::stringid(ID, 0));
    // e1a:SetCategory(CATEGORY_TOGRAVE)
    effect::set_category(d, e1a, CATEGORY_TOGRAVE);
    // e1a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e1a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e1a, EFFECT_FLAG_DELAY, 0);
    // e1a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1a, EVENT_SPSUMMON_SUCCESS);
    // e1a:SetCountLimit(1,{id,0})
    effect::set_count_limit(d, e1a, 1, ID, 0, 0);
    // e1a:SetCondition(function(e) return e:GetHandler():IsLinkSummoned() end)
    effect::set_condition(
        d,
        e1a,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_link_summoned(d, h)
        })),
    );
    // e1a:SetTarget(s.tgtg)
    effect::set_target(d, e1a, Some(cost_target(tgtg)));
    // e1a:SetOperation(s.tgop)
    effect::set_operation(d, e1a, Some(operation(tgop)));
    // c:RegisterEffect(e1a)
    card::register_effect(d, c, e1a, false);
    // local e1b=Effect.CreateEffect(c)
    let e1b = effect::create_effect(d, c);
    // e1b:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1b, EFFECT_TYPE_SINGLE);
    // e1b:SetCode(EFFECT_MATERIAL_CHECK)
    effect::set_code(d, e1b, EFFECT_MATERIAL_CHECK);
    // e1b:SetValue(s.matcheck)
    effect::set_value_fn(d, e1b, lua_fn(matcheck));
    // e1b:SetLabelObject(e1a)
    effect::set_label_object(d, e1b, Value::Effect(e1a));
    // c:RegisterEffect(e1b)
    card::register_effect(d, c, e1b, false);
    // During your next Standby Phase after this card was sent from the field to the GY: You can add 1 monster from your GY to your hand
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2a, CATEGORY_TOHAND);
    // e2a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetCode(EVENT_PHASE+PHASE_STANDBY)
    effect::set_code(d, e2a, EVENT_PHASE + PHASE_STANDBY);
    // e2a:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2a, LOCATION_GRAVE);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetCondition(function(e,tp) local c=e:GetHandler() return Duel.IsTurnPlayer(tp) and c:HasFlagEffect(id) and (c:GetFlagEffectLabel(id)==1 or c:GetTurnID()~=Duel.GetTurnCount()) end)
    effect::set_condition(d, e2a, Some(condition(e2a_condition)));
    // e2a:SetTarget(s.thtg)
    effect::set_target(d, e2a, Some(cost_target(thtg)));
    // e2a:SetOperation(s.thop)
    effect::set_operation(d, e2a, Some(operation(thop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=Effect.CreateEffect(c)
    let e2b = effect::create_effect(d, c);
    // e2b:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2b, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2b:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2b, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2b:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2b, EVENT_TO_GRAVE);
    // e2b:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(
        d,
        e2b,
        Some(condition(|d, a| {
            let h = effect::get_handler(d, a.e).nil();
            card::is_previous_location(d, h, LOCATION_ONFIELD)
        })),
    );
    // e2b:SetOperation(function(e,tp) local ct=(Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_STANDBY)) and 2 or 1 e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,0,ct,ct) end)
    effect::set_operation(d, e2b, Some(operation(e2b_operation)));
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `s.extraval(chk,summon_type,e,...)`
fn extraval(d: &mut Duel, a: Args) -> Value {
    let chk = a.int(0);
    if chk == 0 {
        // local tp,sc=...
        let tp = a.p(3);
        let sc = a.card(4);
        let summon_type = a.int(1) as u32;
        let h = effect::get_handler(d, a.e(2)).nil();
        // if summon_type~=SUMMON_TYPE_LINK or sc~=e:GetHandler() then
        if summon_type != SUMMON_TYPE_LINK || sc != Some(h) {
            // return Group.CreateGroup()
            return Value::Group(group::create_group(&[]));
        }
        // local g=Duel.GetMatchingGroup(aux.NOT(Card.HasFlagEffect),tp,LOCATION_HAND,0,nil,id)
        let g = duel::get_matching_group(d, Some(&|d, c| !card::has_flag_effect(d, c, ID, None)), tp, LOCATION_HAND, 0, NO_EX);
        // for mc in g:Iter() do mc:RegisterFlagEffect(id,0,0,1) end
        for mc in group::iter(d, &g) {
            card::register_flag_effect(d, mc, ID, 0, 0, 1, 0, 0);
        }
        // return g
        Value::Group(g)
    } else if chk == 2 {
        let p = effect::get_handler_player(d, a.e(2));
        // local g=Duel.GetMatchingGroup(Card.HasFlagEffect,e:GetHandlerPlayer(),LOCATION_HAND,0,nil,id)
        let g = duel::get_matching_group(d, Some(&|d, c| card::has_flag_effect(d, c, ID, None)), p, LOCATION_HAND, 0, NO_EX);
        // for mc in g:Iter() do mc:ResetFlagEffect(id) end
        for mc in group::iter(d, &g) {
            card::reset_flag_effect(d, mc, ID);
        }
        Value::Nil
    } else {
        Value::Nil
    }
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local locations=e:GetLabel()==0 and LOCATION_DECK or LOCATION_DECK|LOCATION_EXTRA
    let locations = if effect::get_label(d, a.e) == 0 { LOCATION_DECK } else { LOCATION_DECK | LOCATION_EXTRA };
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.AND(Card.IsMonster,Card.IsAbleToGrave),tp,locations,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None)), tp, locations, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,locations)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, tp, locations as i32);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local locations=e:GetLabel()==0 and LOCATION_DECK or LOCATION_DECK|LOCATION_EXTRA
    let locations = if effect::get_label(d, a.e) == 0 { LOCATION_DECK } else { LOCATION_DECK | LOCATION_EXTRA };
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.AND(Card.IsMonster,Card.IsAbleToGrave),tp,locations,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_grave(d, c, None)), tp, locations, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_EFFECT)
        duel::send_to_grave(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.matcheckfilter(c,tp)`
fn matcheckfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:IsControler(tp)
    card::is_location(d, c, LOCATION_MZONE) && card::is_controler(d, c, tp)
}
/// `s.matcheck(e,c)`
fn matcheck(d: &mut Duel, a: Args) -> Value {
    let e = a.e(0);
    let c = a.c(1);
    // local mg=c:GetMaterial()
    let mg = card::get_material(d, c);
    // e:GetLabelObject():SetLabel(mg:FilterCount(s.matcheckfilter,nil,e:GetHandlerPlayer())==#mg and 1 or 0)
    let p = effect::get_handler_player(d, e);
    let count = group::filter_count(d, &mg, &|d, x| matcheckfilter(d, x, p), NO_EX);
    let label = if count == mg.len() { 1 } else { 0 };
    let target = effect::get_label_object(d, e).as_effect().nil();
    effect::set_label(d, target, &[label]);
    Value::Nil
}
/// The condition of `e2a`: `function(e,tp) local c=e:GetHandler() return Duel.IsTurnPlayer(tp) and c:HasFlagEffect(id) and (c:GetFlagEffectLabel(id)==1 or c:GetTurnID()~=Duel.GetTurnCount()) end`
fn e2a_condition(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    duel::is_turn_player(d, a.tp)
        && card::has_flag_effect(d, c, ID, None)
        && (card::get_flag_effect_label(d, c, ID).first().copied() == Some(1) || card::get_turn_id(d, c) as i32 != duel::get_turn_count(d, None))
}
/// `s.thtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.AND(Card.IsMonster,Card.IsAbleToHand),tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_hand(d, c, None)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_ATOHAND)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_ATOHAND as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.AND(Card.IsMonster,Card.IsAbleToHand),tp,LOCATION_GRAVE,0,1,1,nil)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| card::is_monster(d, c) && card::is_able_to_hand(d, c, None)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.HintSelection(g)
        duel::hint_selection(d, g.clone(), true);
        // Duel.SendtoHand(g,nil,REASON_EFFECT)
        duel::send_to_hand(d, g, None, REASON_EFFECT, None);
    }
}
/// The operation of `e2b`: `function(e,tp) local ct=(Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_STANDBY)) and 2 or 1 e:GetHandler():RegisterFlagEffect(...) end`
fn e2b_operation(d: &mut Duel, a: &EffArgs) {
    // local ct=(Duel.IsTurnPlayer(tp) and Duel.IsPhase(PHASE_STANDBY)) and 2 or 1
    let ct: u16 = if duel::is_turn_player(d, a.tp) && duel::is_phase(d, PHASE_STANDBY) { 2 } else { 1 };
    // e:GetHandler():RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,0,ct,ct)
    let h = effect::get_handler(d, a.e).nil();
    card::register_flag_effect(d, h, ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 0, ct, ct as i64, 0);
}
