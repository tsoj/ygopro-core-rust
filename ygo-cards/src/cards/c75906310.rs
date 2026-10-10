//! Armed Dragon Catapult Cannon
use super::prelude::*;

pub const ID: u32 = 75906310;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[84243274, 73879377], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // fusion material
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,84243274,73879377)
    fusion::add_proc_mix(d, c, true, true, vec![fusion::Mat::Code(84243274), fusion::Mat::Code(73879377)]);
    // Fusion.AddContactProc(c,s.contactfil,s.contactop,s.splimit,s.spcon)
    fusion::add_contact_proc(
        d,
        c,
        fusion::Contact {
            material_filter: Some(std::sync::Arc::new(contactfil)),
            material_operation: Some(std::sync::Arc::new(contactop)),
            sumcon: Some(Some(lua_fn(splimit))),
            condition: Some(std::sync::Arc::new(spcon)),
            ..Default::default()
        },
    );
    // inactivate
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e3, EFFECT_CANNOT_ACTIVATE);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetTargetRange(0,1)
    effect::set_target_range(d, e3, 0, 1);
    // e3:SetValue(s.aclimit)
    effect::set_value_fn(d, e3, lua_fn(aclimit));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // banish
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e4, CATEGORY_REMOVE);
    // e4:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e4, EFFECT_TYPE_QUICK_O);
    // e4:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e4, EVENT_FREE_CHAIN);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCountLimit(1)
    effect::set_count_limit(d, e4, 1, 0, 0, 0);
    // e4:SetHintTiming(0,TIMINGS_CHECK_MONSTER_E)
    effect::set_hint_timing(d, e4, 0, Some(TIMINGS_CHECK_MONSTER_E));
    // e4:SetCondition(s.rmcon)
    effect::set_condition(d, e4, Some(condition(rmcon)));
    // e4:SetCost(s.rmcost)
    effect::set_cost(d, e4, Some(cost_target(rmcost)));
    // e4:SetTarget(s.rmtg)
    effect::set_target(d, e4, Some(cost_target(rmtg)));
    // e4:SetOperation(s.rmop)
    effect::set_operation(d, e4, Some(operation(rmop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // register summon
    // aux.GlobalCheck(s,function()
    //     local ge1=Effect.CreateEffect(c)
    //     ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    //     ge1:SetCode(EVENT_SPSUMMON_SUCCESS)
    //     ge1:SetOperation(s.regop)
    //     Duel.RegisterEffect(ge1,0)
    // end)
    aux::global_check(d, ID, move |d| {
        let ge1 = effect::create_effect(d, c);
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_code(d, ge1, EVENT_SPSUMMON_SUCCESS);
        effect::set_operation(d, ge1, Some(operation(regop)));
        duel::register_effect(d, ge1, 0);
    });
}
/// s.reg(c)
fn reg(d: &mut Duel, c: CardId) {
    // if c:IsCode(84243274) then Duel.RegisterFlagEffect(c:GetControler(),id,0,0,0)
    if card::is_code(d, c, &[84243274]) {
        let p = card::get_controler(d, c);
        duel::register_flag_effect(d, p, ID, 0, 0, 0, 0);
    // elseif c:IsCode(73879377) then Duel.RegisterFlagEffect(c:GetControler(),id+1,0,0,0)
    } else if card::is_code(d, c, &[73879377]) {
        let p = card::get_controler(d, c);
        duel::register_flag_effect(d, p, ID + 1, 0, 0, 0, 0);
    }
}
/// s.regop(e,tp,eg,ep,ev,re,r,rp)
fn regop(d: &mut Duel, a: &EffArgs) {
    // eg:ForEach(s.reg)
    let eg = a.eg.clone().nil();
    for tc in group::iter(d, &eg) {
        reg(d, tc);
    }
}
/// s.splimit(e,se,sp,st)
fn splimit(d: &mut Duel, a: Args) -> bool {
    // return e:GetHandler():GetLocation()~=LOCATION_EXTRA
    let h = effect::get_handler(d, a.e(0)).nil();
    card::get_location(d, h) != LOCATION_EXTRA
}
/// s.contactfil(tp)
fn contactfil(d: &mut Duel, tp: u8) -> CardSet {
    // return Duel.GetMatchingGroup(Card.IsAbleToRemoveAsCost,tp,LOCATION_ONFIELD|LOCATION_GRAVE,0,nil)
    duel::get_matching_group(d, Some(&|d, x| card::is_able_to_remove_as_cost(d, x, None)), tp, LOCATION_ONFIELD | LOCATION_GRAVE, 0, NO_EX)
}
/// s.contactop(g)
fn contactop(d: &mut Duel, g: &CardSet, _tp: u8, _c: CardId) {
    // Duel.Remove(g,POS_FACEUP,REASON_COST|REASON_MATERIAL)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST | REASON_MATERIAL, None, None);
}
/// s.spcon(tp)
fn spcon(d: &mut Duel, tp: u8) -> bool {
    // return Duel.GetFlagEffect(tp,id)~=0 and Duel.GetFlagEffect(tp,id+1)~=0
    duel::get_flag_effect(d, tp, ID) != 0 && duel::get_flag_effect(d, tp, ID + 1) != 0
}
/// s.acfilter(c,cd)
fn acfilter(d: &mut Duel, c: CardId, cd: u32) -> bool {
    // return c:IsFaceup() and c:IsCode(cd)
    card::is_faceup(d, c) && card::is_code(d, c, &[cd])
}
/// s.aclimit(e,re,tp)
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return Duel.IsExistingMatchingCard(s.acfilter,e:GetHandlerPlayer(),LOCATION_REMOVED,LOCATION_REMOVED,1,nil,re:GetHandler():GetCode())
    let p = effect::get_handler_player(d, a.e(0));
    let h = effect::get_handler(d, a.e(1)).nil();
    let cd = card::get_code(d, h)[0];
    duel::is_existing_matching_card(d, Some(&|d, x| acfilter(d, x, cd)), p, LOCATION_REMOVED, LOCATION_REMOVED, 1, NO_EX)
}
/// s.rmcon(e,tp,eg,ep,ev,re,r,rp)
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(1-tp)
    duel::is_turn_player(d, 1 - a.tp)
}
/// s.rmcost(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsAbleToRemoveAsCost,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| card::is_able_to_remove_as_cost(d, x, None)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=Duel.SelectMatchingCard(tp,Card.IsAbleToRemoveAsCost,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil)
    let g = duel::select_matching_card(d, a.tp, Some(&|d, x| card::is_able_to_remove_as_cost(d, x, None)), a.tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    true
}
/// s.rmfilter(c)
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and (c:IsLocation(LOCATION_SZONE) or aux.SpElimFilter(c,false,true))
    card::is_able_to_remove(d, c, None, None, None) && (card::is_location(d, c, LOCATION_SZONE) || aux::sp_elim_filter(d, c, false, true))
}
/// s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&rmfilter), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g.clone()), g.len() as u8, 0, 0);
    true
}
/// s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(s.rmfilter,tp,0,LOCATION_ONFIELD|LOCATION_GRAVE,nil)
    let g = duel::get_matching_group(d, Some(&rmfilter), a.tp, 0, LOCATION_ONFIELD | LOCATION_GRAVE, NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
