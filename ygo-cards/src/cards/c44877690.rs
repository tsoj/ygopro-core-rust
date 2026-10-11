//! Sacred Spirit of the Ice Barrier
use super::prelude::*;

pub const ID: u32 = 44877690;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_ICE_BARRIER as u32], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local sme,soe=Spirit.AddProcedure(c,EVENT_SUMMON_SUCCESS,EVENT_FLIP)
    let procs = spirit::add_procedure(d, c, &[EVENT_SUMMON_SUCCESS, EVENT_FLIP]);
    let (sme, soe) = (procs[0], procs[1]);
    // Mandatory return
    // sme:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, sme, aux::stringid(ID, 0));
    // sme:SetCondition(s.mretcon)
    effect::set_condition(d, sme, Some(condition(mretcon)));
    // sme:SetTarget(s.mrettg)
    effect::set_target(d, sme, Some(cost_target(mrettg)));
    // sme:SetOperation(s.mretop)
    effect::set_operation(d, sme, Some(operation(mretop)));
    // Optional return
    // soe:SetCondition(aux.AND(aux.NOT(s.icecon),Spirit.OptionalReturnCondition))
    effect::set_condition(d, soe, Some(condition(optional_return_cond)));
    // Cannot be Special Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// Calls a library function (a `LuaFn` of the Spirit procedure) with the Lua arguments of the calling
/// function; `chk` is the ninth argument where the Lua passes one
fn call_with(d: &mut Duel, f: LuaFn, a: &EffArgs, chk: Option<i64>) -> Value {
    let mut args = vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ];
    if let Some(chk) = chk {
        args.push(Value::Int(chk));
    }
    f.call(d, &args)
}
/// `s.icecon(e,tp)`
fn icecon(d: &mut Duel, e: EffectId, tp: u8) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_ICE_BARRIER),tp,LOCATION_MZONE,0,1,e:GetHandler())
    let h = effect::get_handler(d, e).nil();
    duel::is_existing_matching_card(
        d,
        Some(&|d, c| card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_ICE_BARRIER as u16])),
        tp,
        LOCATION_MZONE,
        0,
        1,
        h.into(),
    )
}
/// `s.mretcon(e,tp,eg,ep,ev,re,r,rp)`
fn mretcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Spirit.CommonCondition(e) and (s.icecon(e,tp) or Spirit.MandatoryReturnCondition(e))
    spirit::common_condition(d, a.e) && (icecon(d, a.e, a.tp) || call_with(d, spirit::mandatory_return_condition(), a, None).truthy())
}
/// `aux.AND(aux.NOT(s.icecon),Spirit.OptionalReturnCondition)`
fn optional_return_cond(d: &mut Duel, a: &EffArgs) -> bool {
    !icecon(d, a.e, a.tp) && call_with(d, spirit::optional_return_condition(), a, None).truthy()
}
/// `s.mrettg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn mrettg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    if icecon(d, a.e, a.tp) {
        // e:SetProperty(EFFECT_FLAG_CARD_TARGET)
        effect::set_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0);
        // e:GetHandler():ResetFlagEffect(FLAG_SPIRIT_RETURN)
        let h = effect::get_handler(d, a.e).nil();
        card::reset_flag_effect(d, h, spirit::FLAG_SPIRIT_RETURN);
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RTOHAND)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RTOHAND as u64);
        // local g=Duel.SelectTarget(tp,Card.IsAbleToHand,tp,0,LOCATION_MZONE,1,1,nil)
        let g = duel::select_target(d, a.tp, Some(&|d, c| card::is_able_to_hand(d, c, None)), a.tp, 0, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,g,#g,0,0)
        let n = g.len() as u8;
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(g), n, 0, 0);
    } else {
        // e:SetProperty(0)
        effect::set_property(d, a.e, 0, 0);
        // Spirit.MandatoryReturnTarget(e,tp,eg,ep,ev,re,r,rp,1)
        call_with(d, spirit::mandatory_return_target_fn(), a, Some(1));
    }
    true
}
/// `s.mretop(e,tp,eg,ep,ev,re,r,rp)`
fn mretop(d: &mut Duel, a: &EffArgs) {
    // if not e:IsHasProperty(EFFECT_FLAG_CARD_TARGET) then return Spirit.ReturnOperation(e,tp,eg,ep,ev,re,r,rp) end
    if !effect::is_has_property(d, a.e, EFFECT_FLAG_CARD_TARGET, 0) {
        call_with(d, spirit::return_operation(), a, None);
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.SendtoHand(tc,nil,REASON_EFFECT)
            duel::send_to_hand(d, tc, None, REASON_EFFECT, None);
        }
    }
}
