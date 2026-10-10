//! Evil Dragon Ananta
//! 邪龍アナンタ
use super::prelude::*;

pub const ID: u32 = 8400623;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // cannot special summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.FALSE)
    effect::set_value_fn(d, e1, aux_false());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e2, EFFECT_SPSUMMON_PROC);
    // e2:SetProperty(EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e2, EFFECT_FLAG_UNCOPYABLE, 0);
    // e2:SetRange(LOCATION_HAND)
    effect::set_range(d, e2, LOCATION_HAND);
    // e2:SetCondition(s.spcon)
    effect::set_condition(d, e2, Some(lua_fn(spcon)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(lua_fn(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE+PHASE_END)
    effect::set_code(d, e3, EVENT_PHASE + PHASE_END);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.descon)
    effect::set_condition(d, e3, Some(condition(descon)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_REPTILE) and c:IsAbleToRemoveAsCost()
    //     and (not c:IsLocation(LOCATION_MZONE) or c:IsFaceup())
    card::is_race(d, c, RACE_REPTILE as u64)
        && card::is_able_to_remove_as_cost(d, c, None)
        && (!card::is_location(d, c, LOCATION_MZONE) || card::is_faceup(d, c))
}
/// `s.mzfilter(c)`
fn mzfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsLocation(LOCATION_MZONE) and c:GetSequence()<5
    card::is_location(d, c, LOCATION_MZONE) && card::get_sequence(d, c) < 5
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local mg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let mg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local gg=not Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) and Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_GRAVE,0,nil)
    //     or Group.CreateGroup()
    let gg = if !duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_GRAVE, 0, NO_EX)
    } else {
        CardSet::new()
    };
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // return (#mg>0 and (ft>0 or (mg:IsExists(s.mzfilter,1,nil) and ft>-1)))
    //     or (#gg>0 and ft>0)
    (!mg.is_empty() && (ft > 0 || (group::is_exists(d, &mg, &mzfilter, 1, NO_EX) && ft > -1)))
        || (!gg.is_empty() && ft > 0)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let tp = a.p(1);
    let c = a.c(8);
    // local g
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_SPIRIT_ELIMINATION) then
    let g = if duel::is_player_affected_by_effect(d, tp, CARD_SPIRIT_ELIMINATION).is_some() {
        // g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
        duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX)
    } else {
        // g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
        duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX)
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g.clone(), POS_FACEUP as u8, REASON_COST, None, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_SET_ATTACK)
    effect::set_code(d, e1, EFFECT_SET_ATTACK);
    // e1:SetReset(RESET_EVENT|RESET_DISABLE|RESET_TURN_SET|RESET_TOGRAVE|RESET_REMOVE|RESET_TEMP_REMOVE|RESET_TOHAND|RESET_TODECK|RESET_LEAVE)
    effect::set_reset(d, e1, RESET_EVENT | RESET_DISABLE | RESET_TURN_SET | RESET_TOGRAVE | RESET_REMOVE | RESET_TEMP_REMOVE | RESET_TOHAND | RESET_TODECK | RESET_LEAVE, 1);
    // e1:SetValue(#g*600)
    effect::set_value(d, e1, g.len() as i64 * 600);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EFFECT_SET_DEFENSE)
    effect::set_code(d, e2, EFFECT_SET_DEFENSE);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsTurnPlayer(tp)
    duel::is_turn_player(d, a.tp)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsOnField() end
    if let Some(chkc) = a.chkc() {
        return card::is_on_field(d, chkc);
    }
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,aux.TRUE,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, tp, None, tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.clone().nil().len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, g, n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // Duel.Destroy(tc,REASON_EFFECT)
            duel::destroy(d, tc, REASON_EFFECT, None, None);
        }
    }
}
