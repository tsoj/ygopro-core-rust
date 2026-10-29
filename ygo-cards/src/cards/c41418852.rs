//! Numeron Network
use super::prelude::*;

pub const ID: u32 = 41418852;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_NUMERON], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // This effect becomes the effect of 1 "Numeron" Normal Spell when that card is activated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_FZONE)
    effect::set_range(d, e1, LOCATION_FZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCost(s.applycost)
    effect::set_cost(d, e1, Some(cost_target(applycost)));
    // e1:SetTarget(s.applytg)
    effect::set_target(d, e1, Some(cost_target(applytg)));
    // e1:SetOperation(s.applyop)
    effect::set_operation(d, e1, Some(operation(applyop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // "Numeron" Xyz Monsters you control can activate effects without detaching material(s)
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EFFECT_OVERLAY_REMOVE_REPLACE)
    effect::set_code(d, e2, EFFECT_OVERLAY_REMOVE_REPLACE);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(function(e,tp,eg,ep,ev,re,r,rp) ... end)
    effect::set_condition(d, e2, Some(condition(replace_cond)));
    // e2:SetOperation(function() Duel.Hint(HINT_CARD,0,id) return true end)
    effect::set_operation(d, e2, Some(condition(replace_op)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.applycost(e,...,chk)`
fn applycost(d: &mut Duel, a: &EffArgs) -> bool {
    // e:SetLabel(1)
    effect::set_label(d, a.e, &[1]);
    // return true
    true
}
/// `s.applyfilter(c)`
fn applyfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_NUMERON) and c:IsNormalSpell() and c:IsAbleToGraveAsCost()
    //     and c:CheckActivateEffect(false,true,false)~=nil
    card::is_set_card(d, c, &[SET_NUMERON as u16])
        && card::is_normal_spell(d, c)
        && card::is_able_to_grave_as_cost(d, c)
        && card::check_activate_effect(d, c, false, true, false).is_some()
}
/// `s.applytg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn applytg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then
    if a.chk == Some(0) {
        // if e:GetLabel()==0 then return false end
        if effect::get_label(d, a.e) == 0 {
            return false;
        }
        // e:SetLabel(0)
        effect::set_label(d, a.e, &[0]);
        // return Duel.IsExistingMatchingCard(s.applyfilter,tp,LOCATION_DECK,0,1,nil)
        return duel::is_existing_matching_card(d, Some(&applyfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOGRAVE as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.applyfilter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&applyfilter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let sc = group::get_first(d, &sel).nil();
    // local te,ceg,cep,cev,cre,cr,crp=sc:CheckActivateEffect(false,true,true)
    let (te, info) = card::check_activate_effect(d, sc, false, true, true).nil();
    // Duel.SendtoGrave(sc,REASON_COST)
    duel::send_to_grave(d, sc, REASON_COST, None, None);
    // e:SetLabel(te:GetLabel())
    let label = effect::get_labels(d, te);
    effect::set_label(d, a.e, &label);
    // e:SetLabelObject(te:GetLabelObject())
    let lo = effect::get_label_object(d, te);
    effect::set_label_object(d, a.e, lo);
    // e:SetProperty(te:GetProperty())
    let (p1, p2) = effect::get_property(d, te);
    effect::set_property(d, a.e, p1, p2);
    // local tg=te:GetTarget()
    let tg = effect::get_target(d, te);
    // if tg then tg(e,tp,ceg,cep,cev,cre,cr,crp,1) end
    if let Some(tg) = tg {
        let mut args = vec![Value::Effect(a.e), Value::Int(a.tp as i64)];
        match info {
            Some(ev) => args.extend([
                ev.event_cards.clone().map_or(Value::Nil, Value::Group),
                Value::Int(ev.event_player as i64),
                Value::Int(ev.event_value as i64),
                Value::effect(ev.reason_effect),
                Value::Int(ev.reason as i64),
                Value::Int(ev.reason_player as i64),
            ]),
            None => args.extend([Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil, Value::Nil]),
        }
        args.push(Value::Int(1));
        tg.call(d, &args);
    }
    // te:SetLabel(e:GetLabel())
    let label = effect::get_labels(d, a.e);
    effect::set_label(d, te, &label);
    // te:SetLabelObject(e:GetLabelObject())
    let lo = effect::get_label_object(d, a.e);
    effect::set_label_object(d, te, lo);
    // e:SetLabelObject(te)
    effect::set_label_object(d, a.e, Value::Effect(te));
    // Duel.ClearOperationInfo(0)
    duel::clear_operation_info(d, 0);
    true
}
/// `s.applyop(e,tp,eg,ep,ev,re,r,rp)`
fn applyop(d: &mut Duel, a: &EffArgs) {
    // local te=e:GetLabelObject()
    let te = effect::get_label_object(d, a.e);
    // if not te then return end
    let Some(te) = te.as_effect() else { return };
    // e:SetLabel(te:GetLabel())
    let label = effect::get_labels(d, te);
    effect::set_label(d, a.e, &label);
    // e:SetLabelObject(te:GetLabelObject())
    let lo = effect::get_label_object(d, te);
    effect::set_label_object(d, a.e, lo);
    // local op=te:GetOperation()
    let op = effect::get_operation(d, te);
    // if op then op(e,tp,Group.CreateGroup(),PLAYER_NONE,0,e,REASON_EFFECT,PLAYER_NONE) end
    if let Some(op) = op {
        op.call(
            d,
            &[
                Value::Effect(a.e),
                Value::Int(a.tp as i64),
                Value::Group(CardSet::new()),
                Value::Int(PLAYER_NONE as i64),
                Value::Int(0),
                Value::Effect(a.e),
                Value::Int(REASON_EFFECT as i64),
                Value::Int(PLAYER_NONE as i64),
            ],
        );
    }
    // e:SetLabel(0)
    effect::set_label(d, a.e, &[0]);
    // e:SetLabelObject(nil)
    effect::set_label_object(d, a.e, Value::Nil);
}
/// The condition of the replacement effect `e2`.
fn replace_cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local rc=re:GetHandler()
    let re = a.re.nil();
    let rc = effect::get_handler(d, re).nil();
    // return (r&REASON_COST)>0 and re:IsActivated() and re:IsActiveType(TYPE_XYZ) and rc:IsSetCard(SET_NUMERON)
    //     and ep==e:GetOwnerPlayer() and ev>=1 and rc:GetOverlayCount()>=ev-1
    (a.r & REASON_COST) > 0
        && effect::is_activated(d, re)
        && effect::is_active_type(d, re, TYPE_XYZ)
        && card::is_set_card(d, rc, &[SET_NUMERON as u16])
        && a.ep == effect::get_owner_player(d, a.e)
        && a.ev >= 1
        && card::get_overlay_count(d, rc) as i64 >= a.ev as i64 - 1
}
/// The operation of the replacement effect `e2`.
fn replace_op(d: &mut Duel, _a: &EffArgs) -> bool {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // return true
    true
}
