//! Disablaster the Negation Fortress
//! 無孔砲塔－ディセイブラスター
use super::prelude::*;

pub const ID: u32 = 58707981;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // Negate all cards and effects activated in this card's column
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetCode(EVENT_CHAIN_SOLVING)
    effect::set_code(d, e1, EVENT_CHAIN_SOLVING);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetOperation(s.negop)
    effect::set_operation(d, e1, Some(operation(negop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon this card (from your hand)
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e3, EFFECT_FLAG_UNCOPYABLE, 0);
    // e3:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e3, EFFECT_SPSUMMON_PROC);
    // e3:SetRange(LOCATION_HAND)
    effect::set_range(d, e3, LOCATION_HAND);
    // e3:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e3, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e3:SetValue(s.hspval)
    effect::set_value_fn(d, e3, lua_fn(hspval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Place this card in your Pendulum Zone
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e4, EFFECT_FLAG_DELAY, 0);
    // e4:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e4, EVENT_DESTROYED);
    // e4:SetCondition(function(e) return e:GetHandler():IsPreviousLocation(LOCATION_MZONE) end)
    effect::set_condition(
        d,
        e4,
        Some(lua_fn(|d, a: Args| -> bool {
            let h = effect::get_handler(d, a.e(0)).nil();
            card::is_previous_location(d, h, LOCATION_MZONE)
        })),
    );
    // e4:SetTarget(s.pentg)
    effect::set_target(d, e4, Some(cost_target(pentg)));
    // e4:SetOperation(s.penop)
    effect::set_operation(d, e4, Some(operation(penop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local seq,p,loc=Duel.GetChainInfo(ev,CHAININFO_TRIGGERING_SEQUENCE,CHAININFO_TRIGGERING_CONTROLER,CHAININFO_TRIGGERING_LOCATION)
    let info = duel::get_chain_info(d, a.ev as u8, &[CHAININFO_TRIGGERING_SEQUENCE, CHAININFO_TRIGGERING_CONTROLER, CHAININFO_TRIGGERING_LOCATION]);
    let mut seq = info[0].int();
    let p = info[1].int();
    let loc = info[2].int();
    // if loc&(LOCATION_MZONE|LOCATION_SZONE)==0 or (loc&LOCATION_FZONE)>0 then return end --triggering outside the field or in the Field Zone
    if loc & (LOCATION_MZONE | LOCATION_SZONE) as i64 == 0 || (loc & LOCATION_FZONE as i64) > 0 {
        return;
    }
    // if (seq==5 or seq==6) then --First, correct effects triggering in the Extra Monster Zone
    if seq == 5 || seq == 6 {
        // if seq==5 then
        if seq == 5 {
            // seq=1
            seq = 1;
        // elseif seq==6 then
        } else if seq == 6 {
            // seq=3
            seq = 3;
        }
    }
    // if p==1-tp then --Then correct effects triggering in the opponent's (main) sequences, by mirroring them to your sequences
    if p == (1 - tp) as i64 {
        // seq=4-seq
        seq = 4 - seq;
    }
    // local dis_seq=e:GetHandler():GetSequence()
    let h = effect::get_handler(d, a.e).nil();
    let mut dis_seq = card::get_sequence(d, h) as i64;
    // if (dis_seq==5 or dis_seq==6) then --Then correct this card's sequence, if it is in the EMZ
    if dis_seq == 5 || dis_seq == 6 {
        // if dis_seq==5 then
        if dis_seq == 5 {
            // dis_seq=1
            dis_seq = 1;
        // elseif dis_seq==6 then
        } else if dis_seq == 6 {
            // dis_seq=3
            dis_seq = 3;
        }
    }
    // if dis_seq==seq then
    if dis_seq == seq {
        // Duel.NegateEffect(ev)
        duel::negate_effect(d, a.ev as u8);
    }
}
/// `s.hspval(e,c)`
fn hspval(d: &mut Duel, a: Args<'_>) -> Value {
    // local tp=c:GetControler()
    let c = a.c(1);
    let tp = card::get_controler(d, c);
    // local oz=0
    let mut oz: u32 = 0;
    // local fg=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,LOCATION_ONFIELD)
    let fg = duel::get_field_group(d, tp, LOCATION_ONFIELD, LOCATION_ONFIELD);
    // for tc in fg:Iter() do
    for tc in group::iter(d, &fg) {
        // oz=oz|tc:GetColumnZone(LOCATION_MZONE,0,0,tp)
        oz |= card::get_column_zone(d, tc, LOCATION_MZONE, 0, 0, Some(tp));
    }
    // return 0,ZONES_MMZ&~oz
    Value::Multi(vec![Value::Int(0), Value::Int((ZONES_MMZ & !oz) as i64)])
}
/// `s.pentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, tp);
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, c, LOCATION_GRAVE) {
        // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(CardSet::from_card(c)), 1, tp, 0);
    }
    true
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
