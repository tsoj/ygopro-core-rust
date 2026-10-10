//! Qebehsenuef, Protection of Horus
use super::prelude::*;

pub const ID: u32 = 74725513;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_KING_SARCOPHAGUS], listed_series: &[SET_HORUS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon itself from the GY if you control "King's Sarcophagus"
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_GRAVE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Prevent "Horus" monsters from being targeted by attacks or opponent's effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY,EFFECT_FLAG2_CHECK_SIMULTANEOUS)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, EFFECT_FLAG2_CHECK_SIMULTANEOUS);
    // e2:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e2, EVENT_LEAVE_FIELD);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.protcon)
    effect::set_condition(d, e2, Some(condition(protcon)));
    // e2:SetOperation(s.protop)
    effect::set_operation(d, e2, Some(operation(protop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let c = match a.card(1) {
        None => return true,
        Some(c) => c,
    };
    // local eff={c:GetCardEffect(EFFECT_NECRO_VALLEY)}
    let eff = card::get_card_effect(d, c, EFFECT_NECRO_VALLEY);
    // for _,te in ipairs(eff) do
    for te in eff {
        // local op=te:GetOperation()
        let op = effect::get_operation(d, te);
        // if not op or op(e,c) then return false end
        match op {
            None => return false,
            Some(op) => {
                if op.call(d, &[Value::Effect(a.e(0)), Value::Card(c)]).truthy() {
                    return false;
                }
            }
        }
    }
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsCode,CARD_KING_SARCOPHAGUS),tp,LOCATION_ONFIELD,0,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(
            d,
            Some(&|d, x| card::is_faceup(d, x) && card::is_code(d, x, &[CARD_KING_SARCOPHAGUS])),
            tp,
            LOCATION_ONFIELD,
            0,
            1,
            NO_EX,
        )
}
/// `s.cfilter(c,tp)`
fn cfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsPreviousLocation(LOCATION_ONFIELD) and c:IsPreviousControler(tp)
    //   and c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp
    card::is_previous_location(d, c, LOCATION_ONFIELD)
        && card::is_previous_controler(d, c, tp)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::get_reason_player(d, c) == 1 - tp
}
/// `s.protcon(e,tp,eg,ep,ev,re,r,rp)`
fn protcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil,tp) and not Duel.HasFlagEffect(tp,id)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.tp), 1, NO_EX) && !duel::has_flag_effect(d, a.tp, ID, None)
}
/// `s.protop(e,tp,eg,ep,ev,re,r,rp)`
fn protop(d: &mut Duel, a: &EffArgs) {
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, a.tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
    // Your opponent monsters cannot select "Horus" monsters for attacks
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SELECT_BATTLE_TARGET)
    effect::set_code(d, e1, EFFECT_CANNOT_SELECT_BATTLE_TARGET);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(function(e,c) return c:IsSetCard(SET_HORUS) and c:IsFaceup() end)
    effect::set_value_fn(d, e1, lua_fn(horus_value));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // "Horus" monsters on the field cannot be targeted by your opponent's effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e2:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e2, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_HORUS))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_HORUS as u16]))));
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetValue(aux.tgoval)
    effect::set_value_fn(d, e2, aux::tgoval());
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
/// `function(e,c) return c:IsSetCard(SET_HORUS) and c:IsFaceup() end` (the value of `e1`)
fn horus_value(d: &mut Duel, a: Args) -> bool {
    let c = a.c(1);
    card::is_set_card(d, c, &[SET_HORUS as u16]) && card::is_faceup(d, c)
}
