//! Liberator Eto
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 24643913;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your hand or GY) by paying half your LP ...
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e0, EFFECT_TYPE_FIELD);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SPSUMMON_PROC);
    // e0:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e0, LOCATION_HAND | LOCATION_GRAVE);
    // e0:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e0, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e0:SetCondition(s.spcon)
    effect::set_condition(d, e0, Some(lua_fn(spcon)));
    // e0:SetOperation(function(e,tp) Duel.PayLPCost(tp,math.floor(Duel.GetLP(tp)/2)) end)
    effect::set_operation(d, e0, Some(operation(|d, a| {
        let lp = duel::get_lp(d, a.tp);
        duel::pay_lp_cost(d, a.tp, lp.div_euclid(2) as u32);
    })));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // This card's Special Summon cannot be negated
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_CANNOT_DISABLE_SPSUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_DISABLE_SPSUMMON);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // While this card is in the Monster Zone, it cannot be used as material for a Fusion, Synchro, Xyz, or Link Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetCode(EFFECT_CANNOT_BE_MATERIAL)
    effect::set_code(d, e2, EFFECT_CANNOT_BE_MATERIAL);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetValue(aux.cannotmatfilter(SUMMON_TYPE_FUSION,SUMMON_TYPE_SYNCHRO,SUMMON_TYPE_XYZ,SUMMON_TYPE_LINK))
    effect::set_value_fn(
        d,
        e2,
        aux::cannotmatfilter(&[SUMMON_TYPE_FUSION, SUMMON_TYPE_SYNCHRO, SUMMON_TYPE_XYZ, SUMMON_TYPE_LINK]),
    );
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Unaffected by monster effects activated on your opponent's field during your turn only
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCondition(function(e) return Duel.IsTurnPlayer(e:GetHandlerPlayer()) end)
    effect::set_condition(d, e3, Some(condition(|d, a| {
        let p = effect::get_handler_player(d, a.e);
        duel::is_turn_player(d, p)
    })));
    // e3:SetValue(s.immval)
    effect::set_value_fn(d, e3, lua_fn(immval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // if not (c:IsMonsterCard() and c:IsFaceup()) then return false end
    if !(card::is_monster_card(d, c) && card::is_faceup(d, c)) {
        return false;
    }
    // local effs={c:GetOwnEffects()}
    let effs = card::get_own_effects(d, c);
    // for _,eff in ipairs(effs) do
    for eff in effs {
        // if eff:IsHasType(EFFECT_TYPE_QUICK_O|EFFECT_TYPE_QUICK_F) and eff:GetCode()==EVENT_CHAINING
        //     and (eff:GetRange()&(LOCATION_HAND|LOCATION_MZONE))>0 then
        if effect::is_has_type(d, eff, EFFECT_TYPE_QUICK_O | EFFECT_TYPE_QUICK_F)
            && effect::get_code(d, eff) == EVENT_CHAINING
            && (effect::get_range(d, eff) & (LOCATION_HAND | LOCATION_MZONE)) > 0
        {
            // return true
            return true;
        }
    }
    // return false
    false
}
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spconfilter,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,1,nil)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&spconfilter), tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, 1, NO_EX)
}
fn immval(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let te = a.e(1);
    // if not (te:IsMonsterEffect() and te:IsActivated()) then return false end
    if !(effect::is_monster_effect(d, te) && effect::is_activated(d, te)) {
        return false;
    }
    // local trig_loc,trig_ctrl=Duel.GetChainInfo(0,CHAININFO_TRIGGERING_LOCATION,CHAININFO_TRIGGERING_CONTROLER)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TRIGGERING_LOCATION, CHAININFO_TRIGGERING_CONTROLER]);
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // if not Duel.IsChainSolving() then
    if !duel::is_chain_solving(d) {
        // local tc=te:GetHandler()
        let tc = effect::get_handler(d, te).nil();
        // return tc:IsLocation(LOCATION_MZONE) and tc:IsControler(1-tp)
        card::is_location(d, tc, LOCATION_MZONE) && card::is_controler(d, tc, 1 - tp)
    } else {
        // return trig_loc==LOCATION_MZONE and trig_ctrl==1-tp
        matches!(info.first(), Some(Value::Int(x)) if *x == LOCATION_MZONE as i64)
            && matches!(info.get(1), Some(Value::Int(x)) if *x == (1 - tp) as i64)
    }
}
