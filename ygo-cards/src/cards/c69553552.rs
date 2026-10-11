//! F.A. Winners
use super::prelude::*;

pub const ID: u32 = 69553552;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_FA],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(s.indcon)
    effect::set_condition(d, e2, Some(condition(indcon)));
    // e2:SetValue(s.indval)
    effect::set_value_fn(d, e2, lua_fn(indval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_BATTLE_DAMAGE)
    effect::set_code(d, e3, EVENT_BATTLE_DAMAGE);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCountLimit(1,id)
    effect::set_count_limit(d, e3, 1, ID, 0, 0);
    // e3:SetCondition(s.rmcon)
    effect::set_condition(d, e3, Some(condition(rmcon)));
    // e3:SetTarget(s.rmtg)
    effect::set_target(d, e3, Some(cost_target(rmtg)));
    // e3:SetOperation(s.rmop)
    effect::set_operation(d, e3, Some(operation(rmop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.indcon(e)`
fn indcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_FA),e:GetHandlerPlayer(),LOCATION_MZONE,0,1,nil)
    let p = effect::get_handler_player(d, a.e);
    duel::is_existing_matching_card(d, Some(&aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_FA as u16]))), p, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.indval(e,re,tp)`
fn indval(d: &mut Duel, a: Args) -> bool {
    // return tp~=e:GetHandlerPlayer()
    let e = a.e(0);
    let tp = a.p(2);
    tp != effect::get_handler_player(d, e)
}
/// `s.rmcon(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let mut at = duel::get_attacker(d).nil();
    // local d=a:GetBattleTarget()
    let dt = card::get_battle_target(d, at);
    // if not d then return false end
    let Some(mut dcard) = dt else {
        return false;
    };
    // if not a:IsControler(tp) then a,d=d,a end
    if !card::is_controler(d, at, a.tp) {
        std::mem::swap(&mut at, &mut dcard);
    }
    // return a:IsControler(tp) and a:IsSetCard(SET_FA) and a:GetLevel()-a:GetOriginalLevel()>=5 and ep~=tp
    card::is_controler(d, at, a.tp)
        && card::is_set_card(d, at, &[SET_FA as u16])
        && card::get_level(d, at) as i64 - card::get_original_level(d, at) as i64 >= 5
        && a.ep != a.tp
}
/// `s.rmfilter(c)`
fn rmfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsAbleToRemove() and (c:IsLocation(LOCATION_ONFIELD) or aux.SpElimFilter(c,false,true))
    card::is_able_to_remove(d, c, None, None, None) && (card::is_location(d, c, LOCATION_ONFIELD) || aux::sp_elim_filter(d, c, false, true))
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmfilter,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmfilter), a.tp, LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, (LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE) as i32);
    true
}
/// `s.winfilter(c)`
fn winfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_FA) and c:IsType(TYPE_FIELD) and c:GetFlagEffect(id)~=0
    card::is_set_card(d, c, &[SET_FA as u16]) && card::is_type(d, c, TYPE_FIELD) && card::get_flag_effect(d, c, ID) != 0
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.rmfilter,tp,LOCATION_HAND|LOCATION_ONFIELD|LOCATION_GRAVE,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&rmfilter), a.tp, LOCATION_HAND | LOCATION_ONFIELD | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)~=0 then
    if let Some(tc) = tc {
        if duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 {
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, 0, 0);
            // local g=Duel.GetMatchingGroup(s.winfilter,tp,LOCATION_REMOVED,0,nil)
            let g = duel::get_matching_group(d, Some(&winfilter), a.tp, LOCATION_REMOVED, 0, NO_EX);
            // if g:GetClassCount(Card.GetCode)>=3 then
            if group::get_class_count(d, &g, &|d, c| card::get_code(d, c)[0] as i64) >= 3 {
                // Duel.Win(tp,WIN_REASON_FA_WINNERS)
                duel::win(d, a.tp, WIN_REASON_FA_WINNERS);
            }
        }
    }
}
