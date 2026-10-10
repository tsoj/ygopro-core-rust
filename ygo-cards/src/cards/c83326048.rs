//! Dimensional Barrier
use super::prelude::*;

pub const ID: u32 = 83326048;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_DISABLE)
    effect::set_category(d, e1, CATEGORY_DISABLE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(0,TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, 0, Some(TIMINGS_CHECK_MONSTER));
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CARDTYPE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_CARDTYPE as u64);
    // Duel.SetTargetParam(Duel.SelectOption(tp,1057,1056,1063,1073,1074))
    let opt = duel::select_option(d, a.tp, true, &[1057, 1056, 1063, 1073, 1074]);
    duel::set_target_param(d, opt as u32);
    true
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local opt=Duel.GetChainInfo(0,CHAININFO_TARGET_PARAM)
    let opt = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PARAM])[0].int();
    // local ct=nil
    // if opt==0 then ct=TYPE_RITUAL end ... if opt==4 then ct=TYPE_PENDULUM end
    let ct: Option<u32> = match opt {
        0 => Some(TYPE_RITUAL),
        1 => Some(TYPE_FUSION),
        2 => Some(TYPE_SYNCHRO),
        3 => Some(TYPE_XYZ),
        4 => Some(TYPE_PENDULUM),
        _ => None,
    };
    // Cannot Special Summon monsters of the declared type
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,5))
    effect::set_description(d, e1, aux::stringid(ID, 5));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetTargetRange(1,1)
    effect::set_target_range(d, e1, 1, 1);
    // e1:SetTarget(s.sumlimit)
    effect::set_target(d, e1, Some(lua_fn(sumlimit)));
    // e1:SetLabel(ct)
    if let Some(ct) = ct {
        effect::set_label(d, e1, &[ct as i64]);
    }
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
    // Negate the effects of monsters of that type while on the field
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e2, EFFECT_DISABLE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetTarget(function(e,c) return c:IsType(e:GetLabel()) end)
    effect::set_target(
        d,
        e2,
        Some(effect_card_fn(|d, e, x| {
            let t = effect::get_label(d, e) as u32;
            card::is_type(d, x, t)
        })),
    );
    // e2:SetLabel(ct)
    if let Some(ct) = ct {
        effect::set_label(d, e2, &[ct as i64]);
    }
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let c = a.c(1);
    // local declared_type=e:GetLabel()
    let declared_type = effect::get_label(d, e) as u32;
    // if c:IsMonster() then return c:IsType(declared_type)
    if card::is_monster(d, c) {
        card::is_type(d, c, declared_type)
    } else {
        // else return c:IsOriginalType(declared_type) end
        card::is_original_type(d, c, declared_type)
    }
}
