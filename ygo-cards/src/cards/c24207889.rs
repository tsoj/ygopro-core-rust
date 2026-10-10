//! There Can Be Only One
use super::prelude::*;

pub const ID: u32 = 24207889;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetHintTiming(TIMING_STANDBY_PHASE,TIMING_STANDBY_PHASE|TIMINGS_CHECK_MONSTER)
    effect::set_hint_timing(d, e1, TIMING_STANDBY_PHASE, Some(TIMING_STANDBY_PHASE | TIMINGS_CHECK_MONSTER));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Adjust
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
    // e2:SetCode(id)
    effect::set_code(d, e2, ID);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCondition(function(e) return e:GetHandler():IsStatus(STATUS_EFFECT_ENABLED) end)
    effect::set_condition(d, e2, Some(condition(adjust_cond)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Cannot Normal Summon, Special Summon or Flip Summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetRange(LOCATION_SZONE)
    effect::set_range(d, e3, LOCATION_SZONE);
    // e3:SetCode(EFFECT_FORCE_SPSUMMON_POSITION)
    effect::set_code(d, e3, EFFECT_FORCE_SPSUMMON_POSITION);
    // e3:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e3, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e3:SetTargetRange(1,1)
    effect::set_target_range(d, e3, 1, 1);
    // e3:SetTarget(s.sumlimit)
    effect::set_target(d, e3, Some(lua_fn(sumlimit)));
    // e3:SetValue(POS_FACEDOWN)
    effect::set_value(d, e3, POS_FACEDOWN as i64);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=e3:Clone()
    let e4 = effect::clone(d, e3);
    // e4:SetCode(EFFECT_CANNOT_SUMMON)
    effect::set_code(d, e4, EFFECT_CANNOT_SUMMON);
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=e3:Clone()
    let e5 = effect::clone(d, e3);
    // e5:SetCode(EFFECT_CANNOT_FLIP_SUMMON)
    effect::set_code(d, e5, EFFECT_CANNOT_FLIP_SUMMON);
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // aux.GlobalCheck(s,function()
    //     s.lastFieldId={}
    //     s.lastFieldId[0]=nil
    //     s.lastFieldId[1]=nil
    //     local ge=Effect.GlobalEffect()
    //     ge:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    //     ge:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    //     ge:SetCode(EVENT_ADJUST)
    //     ge:SetOperation(s.adjustop)
    //     Duel.RegisterEffect(ge,0)
    // end)
    aux::global_check(d, ID, |d| {
        set_s_var(d, ID, 0, Value::Nil);
        set_s_var(d, ID, 1, Value::Nil);
        let ge = effect::global_effect(d);
        effect::set_type(d, ge, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        effect::set_property(d, ge, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        effect::set_code(d, ge, EVENT_ADJUST);
        effect::set_operation(d, ge, Some(operation(adjustop)));
        duel::register_effect(d, ge, 0);
    });
    // Cannot activate a monster's effect that flips itself face-up as cost if it's of a Type that player already controls
    // local e6=Effect.CreateEffect(c)
    let e6 = effect::create_effect(d, c);
    // e6:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e6, EFFECT_TYPE_FIELD);
    // e6:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e6, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e6:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e6, EFFECT_CANNOT_ACTIVATE);
    // e6:SetRange(LOCATION_SZONE)
    effect::set_range(d, e6, LOCATION_SZONE);
    // e6:SetTargetRange(1,1)
    effect::set_target_range(d, e6, 1, 1);
    // e6:SetValue(function(e,re,tp) ... end)
    effect::set_value_fn(d, e6, lua_fn(actlimit));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
}
/// `s.sumlimit(e,c,sump,sumtype,sumpos,targetp)`
fn sumlimit(d: &mut Duel, a: Args) -> bool {
    // local tp=sump
    // if targetp then tp=targetp end
    let tp = if a.v(5).truthy() { a.p(5) } else { a.p(2) };
    let c = a.c(1);
    // return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,c:GetRace()),tp,LOCATION_MZONE,0,1,c)
    let race = card::get_race(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_race(d, x, race)), tp, LOCATION_MZONE, 0, 1, c.into())
}
/// `function(e,re,tp) return re:HasSelfChangePositionCost() and Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsRace,re:GetHandler():GetRace()),tp,LOCATION_MZONE,0,1,nil) end`
fn actlimit(d: &mut Duel, a: Args) -> bool {
    let re = a.e(1);
    if !effect::has_self_change_position_cost(d, re) {
        return false;
    }
    let rc = effect::get_handler(d, re).nil();
    let race = card::get_race(d, rc);
    duel::is_existing_matching_card(d, Some(&|d, x| card::is_faceup(d, x) && card::is_race(d, x, race)), a.p(2), LOCATION_MZONE, 0, 1, NO_EX)
}
/// `function(e) return e:GetHandler():IsStatus(STATUS_EFFECT_ENABLED) end`
fn adjust_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_status(d, h, STATUS_EFFECT_ENABLED)
}
/// `faceupfil`: `aux.FaceupFilter(Card.IsHasEffect,id)`
fn faceupfil(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && !card::is_has_effect(d, c, ID, None).is_empty()
}
/// `s.adjustop(e,tp,eg,ep,ev,re,r,rp)`
fn adjustop(d: &mut Duel, a: &EffArgs) {
    // local phase=Duel.GetCurrentPhase()
    let phase = duel::get_current_phase(d);
    // if (phase==PHASE_DAMAGE and not Duel.IsDamageCalculated()) or phase==PHASE_DAMAGE_CAL then return end
    if (phase == PHASE_DAMAGE && !duel::is_damage_calculated(d)) || phase == PHASE_DAMAGE_CAL {
        return;
    }
    // if not Duel.IsExistingMatchingCard(faceupfil,0,LOCATION_SZONE,LOCATION_SZONE,1,nil) then
    if !duel::is_existing_matching_card(d, Some(&|d, c| faceupfil(d, c)), 0, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX) {
        // s.lastFieldId[0]=nil
        // s.lastFieldId[1]=nil
        set_s_var(d, ID, 0, Value::Nil);
        set_s_var(d, ID, 1, Value::Nil);
        return;
    }
    // local sg=Group.CreateGroup()
    let mut sg = CardSet::new();
    // for p=0,1 do
    for p in 0..2u8 {
        // local g=Duel.GetMatchingGroup(Card.IsFaceup,p,LOCATION_MZONE,0,nil)
        let mut g = duel::get_matching_group(d, Some(&|d, c| card::is_faceup(d, c)), p, LOCATION_MZONE, 0, NO_EX);
        // if #g==0 then
        if g.is_empty() {
            // s.lastFieldId[p]=nil
            set_s_var(d, ID, p as u64, Value::Nil);
        } else {
            // local race=1
            let mut race: u64 = 1;
            // local update_fid=false
            let mut update_fid = false;
            // while (RACE_ALL&race)~=0 do
            while (RACE_ALL & race) != 0 {
                // local rg=g:Filter(Card.IsRace,nil,race)
                let mut rg = group::filter(d, &g, &|d, c| card::is_race(d, c, race), NO_EX);
                // if s.lastFieldId[p] then
                if let Value::Int(fid) = s_var(d, ID, p as u64) {
                    // local forced
                    // forced,rg=rg:Split(s.fidfilter,nil,s.lastFieldId[p])
                    let (forced, rest) = group::split(d, &rg, &|d, c| card::get_field_id(d, c) as i64 > fid, NO_EX);
                    rg = rest;
                    // if #rg==0 then
                    if rg.is_empty() {
                        // rg=forced
                        rg = forced;
                        // update_fid=true
                        update_fid = true;
                    } else {
                        // sg:Merge(forced)
                        group::add_card(&mut sg, &forced);
                    }
                }
                // local rc=#rg
                let rc = rg.len();
                // if rc>1 then
                if rc > 1 {
                    // Duel.Hint(HINT_SELECTMSG,p,HINTMSG_TOGRAVE)
                    duel::hint(d, HINT_SELECTMSG as u8, p, HINTMSG_TOGRAVE as u64);
                    // sg:Merge(rg:Select(p,rc-1,rc-1,nil))
                    let sel = group::select(d, &rg, p, (rc - 1) as u16, (rc - 1) as u16, false, NO_EX).nil();
                    group::add_card(&mut sg, &sel);
                }
                // race=race<<1
                race <<= 1;
            }
            // if update_fid or not s.lastFieldId[p] then
            if update_fid || !s_var(d, ID, p as u64).truthy() {
                // local maxg,maxid=g:Sub(sg):GetMaxGroup(Card.GetFieldID)
                group::remove_card(&mut g, &sg);
                let maxid = group::get_max_group(d, &g, &|d, c| card::get_field_id(d, c) as i64).map(|(_, id)| id);
                // s.lastFieldId[p]=maxid
                set_s_var(d, ID, p as u64, maxid.map_or(Value::Nil, Value::Int));
            }
        }
    }
    // local p=e:GetHandlerPlayer()
    let p = effect::get_handler_player(d, a.e);
    // local g1,g2=Group.CreateGroup(),Group.CreateGroup()
    let mut g1 = CardSet::new();
    let mut g2 = CardSet::new();
    // local readjust=false
    let mut readjust = false;
    // if #sg>0 then g1,g2=sg:Split(Card.IsControler,nil,p) end
    if !sg.is_empty() {
        let (x, y) = group::split(d, &sg, &|d, c| card::is_controler(d, c, p), NO_EX);
        g1 = x;
        g2 = y;
    }
    // if #g1>0 then
    if !g1.is_empty() {
        // Duel.SendtoGrave(g1,REASON_RULE,PLAYER_NONE,p)
        duel::send_to_grave(d, g1, REASON_RULE, Some(PLAYER_NONE as u8), Some(p));
        // readjust=true
        readjust = true;
    }
    // if #g2>0 then
    if !g2.is_empty() {
        // Duel.SendtoGrave(g2,REASON_RULE,PLAYER_NONE,1-p)
        duel::send_to_grave(d, g2, REASON_RULE, Some(PLAYER_NONE as u8), Some(1 - p));
        // readjust=true
        readjust = true;
    }
    // if readjust then Duel.Readjust() end
    if readjust {
        duel::readjust(d);
    }
}
