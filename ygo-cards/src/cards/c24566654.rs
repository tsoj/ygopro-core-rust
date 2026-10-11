//! Crimson Fire
use super::prelude::*;

pub const ID: u32 = 24566654;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_RED_DRAGON_ARCHFIEND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Your opponent takes twice the effect damage you would have taken
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e1, EVENT_CHAINING);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.condition(e,tp,eg,ep,ev,re,r,rp)
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_RED_DRAGON_ARCHFIEND),tp,LOCATION_ONFIELD,0,1,nil)
    //   and ep==1-tp and re:IsHasType(EFFECT_TYPE_ACTIVATE) and aux.damcon1(e,tp,eg,ep,ev,re,r,rp)
    if !duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_code(d, c, &[CARD_RED_DRAGON_ARCHFIEND])),
        a.tp,
        LOCATION_ONFIELD,
        0,
        1,
        NO_EX,
    ) {
        return false;
    }
    if a.ep != 1 - a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) {
        return false;
    }
    // aux.damcon1(e,tp,eg,ep,ev,re,r,rp)
    let args = [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::Effect(re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    aux::damcon1().call(d, &args).truthy()
}
// function s.operation(e,tp,eg,ep,ev,re,r,rp)
fn op(d: &mut Duel, a: &EffArgs) {
    // local cid=Duel.GetChainInfo(ev,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_CHAIN_ID]).first().map_or(0, |v| v.int());
    // local e1=Effect.CreateEffect(e:GetHandler())
    let h = effect::get_handler(d, a.e).nil();
    let e1 = effect::create_effect(d, h);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_REFLECT_DAMAGE)
    effect::set_code(d, e1, EFFECT_REFLECT_DAMAGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetLabel(cid)
    effect::set_label(d, e1, &[cid]);
    // e1:SetValue(s.refcon)
    effect::set_value_fn(d, e1, lua_fn(refcon));
    // e1:SetReset(RESET_CHAIN)
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // local e2=Effect.CreateEffect(e:GetHandler())
    let e2 = effect::create_effect(d, h);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CHANGE_DAMAGE)
    effect::set_code(d, e2, EFFECT_CHANGE_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetLabel(cid)
    effect::set_label(d, e2, &[cid]);
    // e2:SetValue(s.dammul)
    effect::set_value_fn(d, e2, lua_fn(dammul));
    // e2:SetReset(RESET_CHAIN)
    effect::set_reset(d, e2, RESET_CHAIN, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
// function s.refcon(e,re,val,r,rp,rc)
fn refcon(d: &mut Duel, a: Args) -> Value {
    let e = a.e(0);
    let r = a.int(3) as u32;
    // local cc=Duel.GetCurrentChain()
    let cc = duel::get_current_chain(d, false);
    // if cc==0 or (r&REASON_EFFECT)==0 then return end
    if cc == 0 || (r & REASON_EFFECT) == 0 {
        return Value::Nil;
    }
    // local cid=Duel.GetChainInfo(0,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID]).first().map_or(0, |v| v.int());
    // return cid==e:GetLabel()
    Value::Bool(cid == effect::get_label(d, e))
}
// function s.dammul(e,re,val,r,rp,rc)
fn dammul(d: &mut Duel, a: Args) -> Value {
    let e = a.e(0);
    let val = a.int(2);
    let r = a.int(3) as u32;
    // local cc=Duel.GetCurrentChain()
    let cc = duel::get_current_chain(d, false);
    // if cc==0 or (r&REASON_EFFECT)==0 then return end
    if cc == 0 || (r & REASON_EFFECT) == 0 {
        return Value::Nil;
    }
    // local cid=Duel.GetChainInfo(0,CHAININFO_CHAIN_ID)
    let cid = duel::get_chain_info(d, 0, &[CHAININFO_CHAIN_ID]).first().map_or(0, |v| v.int());
    // return cid==e:GetLabel() and val*2 or val
    if cid == effect::get_label(d, e) {
        Value::Int(val * 2)
    } else {
        Value::Int(val)
    }
}
