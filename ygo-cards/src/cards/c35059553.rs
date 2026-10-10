//! Kaiser Colosseum
use super::prelude::*;

pub const ID: u32 = 35059553;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // mzone limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCode(EFFECT_MAX_MZONE)
    effect::set_code(d, e2, EFFECT_MAX_MZONE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetValue(s.value)
    effect::set_value_fn(d, e2, lua_fn(value));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // advance summon limit
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EFFECT_UNRELEASABLE_SUM)
    effect::set_code(d, e3, EFFECT_UNRELEASABLE_SUM);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetProperty(EFFECT_FLAG_SET_AVAILABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e3, EFFECT_FLAG_SET_AVAILABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e3:SetValue(s.sumlimit)
    effect::set_value_fn(d, e3, effect_card_fn(sumlimit));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.value(e,fp,rp,r)`
fn value(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let rp = a.p(2);
    let r = a.int(3);
    // local limit=Duel.GetFieldGroupCount(e:GetHandlerPlayer(),LOCATION_MZONE,0)
    let hp = effect::get_handler_player(d, e);
    let limit = duel::get_field_group_count(d, hp, LOCATION_MZONE, 0);
    // if rp==e:GetHandlerPlayer() or r~=LOCATION_REASON_TOFIELD or limit==0 then return 99 end
    if rp == hp || r != LOCATION_REASON_TOFIELD as i64 || limit == 0 {
        return 99;
    }
    // return limit
    limit as i64
}
/// `s.sumlimit(e,c)`
fn sumlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // if c:IsControler(1-tp) then
    if card::is_controler(d, c, 1 - tp) {
        // local mint,maxt=c:GetTributeRequirement()
        let (_mint, maxt) = card::get_tribute_requirement(d, c);
        // local x=Duel.GetFieldGroupCount(tp,LOCATION_MZONE,0)
        let x = duel::get_field_group_count(d, tp, LOCATION_MZONE, 0);
        // local y=Duel.GetFieldGroupCount(tp,0,LOCATION_MZONE)
        let y = duel::get_field_group_count(d, tp, 0, LOCATION_MZONE);
        // local ex=Duel.GetMatchingGroupCount(Card.IsHasEffect,tp,LOCATION_MZONE,0,nil,EFFECT_EXTRA_RELEASE)
        let mut ex = duel::get_matching_group_count(d, Some(&|d, c| !card::is_has_effect(d, c, EFFECT_EXTRA_RELEASE, None).is_empty()), tp, LOCATION_MZONE, 0, NO_EX) as i32;
        // local exs=Duel.GetMatchingGroupCount(Card.IsHasEffect,tp,LOCATION_MZONE,0,nil,EFFECT_EXTRA_RELEASE_SUM)
        let exs = duel::get_matching_group_count(d, Some(&|d, c| !card::is_has_effect(d, c, EFFECT_EXTRA_RELEASE_SUM, None).is_empty()), tp, LOCATION_MZONE, 0, NO_EX) as i32;
        // if ex==0 and exs>0 then ex=1 end
        if ex == 0 && exs > 0 {
            ex = 1;
        }
        // return y-maxt+ex+1 > x-ex
        return y - maxt + ex + 1 > x - ex;
    }
    // return false
    false
}
