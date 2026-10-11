//! Synchro Panic
use super::prelude::*;

pub const ID: u32 = 83962752;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon 1 Tuner and any number of non-Tuners from your GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e1:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    // e1:SetCondition(function(e,tp,eg,ep) return tp==ep end)
    effect::set_condition(d, e1, Some(condition(|_d, a| a.tp == a.ep)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of the destroyed Synchro Monsters
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, move |d| {
        // s.desgroup={}
        // s.desgroup[0]=Group.CreateGroup()
        // s.desgroup[1]=Group.CreateGroup()
        // s.desgroup[0]:KeepAlive()
        // s.desgroup[1]:KeepAlive()
        set_s_var(d, ID, 0, Value::Group(CardSet::new()));
        set_s_var(d, ID, 1, Value::Group(CardSet::new()));
        // local e0=Effect.CreateEffect(c)
        let e0 = effect::create_effect(d, c);
        // e0:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e0, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e0:SetCode(EVENT_DESTROYED)
        effect::set_code(d, e0, EVENT_DESTROYED);
        // e0:SetOperation(s.desgroupregop)
        effect::set_operation(d, e0, Some(operation(desgroupregop)));
        // Duel.RegisterEffect(e0,0)
        duel::register_effect(d, e0, 0);
    });
    // Neither player can Synchro Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e2, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetTargetRange(1,1)
    effect::set_target_range(d, e2, 1, 1);
    // e2:SetTarget(function(e,c,tp,sumtp,sumpos) return (sumtp&SUMMON_TYPE_SYNCHRO)==SUMMON_TYPE_SYNCHRO end)
    effect::set_target(d, e2, Some(lua_fn(synchro_sum_target)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function(e,c,tp,sumtp,sumpos) return (sumtp&SUMMON_TYPE_SYNCHRO)==SUMMON_TYPE_SYNCHRO end
fn synchro_sum_target(_d: &mut Duel, a: Args) -> bool {
    let sumtp = a.int(3);
    (sumtp & SUMMON_TYPE_SYNCHRO as i64) == SUMMON_TYPE_SYNCHRO as i64
}
// The group `s.desgroup[p]`, kept in the duel.
fn desgroup(d: &Duel, p: u8) -> CardSet {
    s_var(d, ID, p as u64).as_group().cloned().nil()
}
// function s.cfilter(c,tp,e)
fn cfilter(d: &mut Duel, c: CardId, tp: u8, e: Option<EffectId>) -> bool {
    // return c:IsType(TYPE_SYNCHRO) and c:IsPreviousPosition(POS_FACEUP)
    //     and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp)
    //     and (c:IsReason(REASON_BATTLE) or (c:IsReason(REASON_EFFECT) and c:GetReasonPlayer()==1-tp))
    //     and (not e or c:IsCanBeEffectTarget(e))
    card::is_type(d, c, TYPE_SYNCHRO)
        && card::is_previous_position(d, c, POS_FACEUP)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, tp)
        && (card::is_reason(d, c, REASON_BATTLE)
            || (card::is_reason(d, c, REASON_EFFECT) && card::get_reason_player(d, c) == 1 - tp))
        && (e.is_none() || card::is_can_be_effect_target(d, c, e))
}
// function s.desgroupregop(e,tp,eg,ep,ev,re,r,rp)
fn desgroupregop(d: &mut Duel, a: &EffArgs) {
    // for p=0,1 do
    for p in 0..=1u8 {
        // local tg=eg:Filter(s.cfilter,nil,p)
        let eg = a.eg.clone().nil();
        let tg = group::filter(d, &eg, &|d, c| cfilter(d, c, p, None), NO_EX);
        // if #tg>0 then
        if !tg.is_empty() {
            // for tc in tg:Iter() do
            //     tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
            // end
            for tc in group::iter(d, &tg) {
                card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
            }
            let mut dg = desgroup(d, p);
            // if Duel.GetCurrentChain()==0 then s.desgroup[p]:Clear() end
            if duel::get_current_chain(d, false) == 0 {
                dg = CardSet::new();
            }
            // s.desgroup[p]:Merge(tg)
            group::add_card(&mut dg, &tg);
            // s.desgroup[p]:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
            group::remove(d, &mut dg, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
            set_s_var(d, ID, p as u64, Value::Group(dg.clone()));
            // Duel.RaiseEvent(s.desgroup[p],EVENT_CUSTOM+id,re,r,rp,p,0)
            duel::raise_event(d, dg, EVENT_CUSTOM + ID, a.re, a.r, a.rp, p, 0);
        }
    }
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:HasLevel() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::has_level(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.rescon(lvl,ft)
// return function(sg,e,tp,mg)
//     return ft>=#sg and sg:FilterCount(Card.IsType,nil,TYPE_TUNER)==1
//         and sg:IsExists(aux.NOT(Card.IsType),1,nil,TYPE_TUNER)
//         and sg:GetSum(Card.GetLevel)==lvl
//     end
fn rescon_for(lvl: i64, ft: i32) -> impl Fn(&mut Duel, &CardSet, Option<EffectId>, u8, &CardSet, Option<CardId>) -> (bool, bool) {
    move |d, sg, _e, _tp, _mg, _c| {
        let r = ft as usize >= sg.len()
            && group::filter_count(d, sg, &|d, c| card::is_type(d, c, TYPE_TUNER), NO_EX) == 1
            && group::is_exists(d, sg, &|d, c| !card::is_type(d, c, TYPE_TUNER), 1, NO_EX)
            && group::get_sum(d, sg, &|d, c| card::get_level(d, c) as i64) == lvl;
        (r, false)
    }
}
// function s.specialcheck(c,sumg,ft)
// return aux.SelectUnselectGroup(sumg,e,tp,2,ft,s.rescon(c:GetLevel(),ft),0)
// (e and tp are not parameters here: they are unset globals, nil in Lua)
fn specialcheck(d: &mut Duel, c: CardId, sumg: &CardSet, ft: i32) -> bool {
    let lvl = card::get_level(d, c) as i64;
    matches!(
        aux::select_unselect_group(d, sumg, None, 0, Some(2), Some(ft as usize), Some(&rescon_for(lvl, ft)), 0, 0, None, None, None, false),
        Err(true)
    )
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local synchg=s.desgroup[tp]:Filter(s.cfilter,nil,tp,e)
    let synchg_all = desgroup(d, tp);
    let synchg = group::filter(d, &synchg_all, &|d, c| cfilter(d, c, tp, Some(e)), NO_EX);
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // local sumg=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let sumg = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if chkc then return synchg:IsContains(chkc) and aux.SelectUnselectGroup(sumg,e,tp,2,ft,s.rescon(chkc:GetLevel(),ft),0) end
    if let Some(chkc) = a.chkc() {
        if !group::is_contains(&synchg, chkc) {
            return false;
        }
        let lvl = card::get_level(d, chkc) as i64;
        return matches!(
            aux::select_unselect_group(d, &sumg, Some(e), tp, Some(2), Some(ft as usize), Some(&rescon_for(lvl, ft)), 0, tp, None, None, None, false),
            Err(true)
        );
    }
    // if chk==0 then return ft>=2 and #sumg>=2 and not Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT)
    //     and synchg:IsExists(s.specialcheck,1,nil,sumg,ft)
    // end
    if a.chk == Some(0) {
        return ft >= 2
            && sumg.len() >= 2
            && duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_none()
            && group::is_exists(d, &synchg, &|d, c| specialcheck(d, c, &sumg, ft), 1, NO_EX);
    }
    // local tc=nil
    // if #synchg==1 then
    if synchg.len() == 1 {
        // tc=synchg:GetFirst()
        let tc = group::get_first(d, &synchg).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // tc=synchg:FilterSelect(tp,s.specialcheck,1,1,nil,sumg,ft)
        let tc = group::filter_select(d, &synchg, tp, &|d, c| specialcheck(d, c, &sumg, ft), 1, 1, false, NO_EX).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,2,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 2, tp, LOCATION_GRAVE as i32);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // --Destroy this card during your 3rd Standby Phase after activation
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EVENT_PHASE|PHASE_STANDBY)
    effect::set_code(d, e1, EVENT_PHASE | PHASE_STANDBY);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(function(e,tp) return Duel.IsTurnPlayer(tp) end)
    effect::set_condition(d, e1, Some(condition(|d, a| duel::is_turn_player(d, a.tp))));
    // e1:SetOperation(s.sdesop)
    effect::set_operation(d, e1, Some(operation(sdesop)));
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_STANDBY|RESET_SELF_TURN,3)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_STANDBY | RESET_SELF_TURN, 3);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // c:SetTurnCounter(0)
    card::set_turn_counter(d, c, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) or Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then return end
    if !card::is_relate_to_effect(d, tc, e) || duel::is_player_affected_by_effect(d, tp, CARD_BLUEEYES_SPIRIT).is_some() {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<2 then return end
    if ft < 2 {
        return;
    }
    // local g=Duel.GetMatchingGroup(s.spfilter,tp,LOCATION_GRAVE,0,nil,e,tp)
    let g = duel::get_matching_group(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, NO_EX);
    // if #g<2 then return end
    if g.len() < 2 {
        return;
    }
    // local lvl=tc:GetLevel()
    let lvl = card::get_level(d, tc) as i64;
    // local sg=aux.SelectUnselectGroup(g,e,tp,2,math.min(ft,#g),s.rescon(lvl,ft),1,tp,HINTMSG_SPSUMMON,s.rescon(lvl,ft))
    let maxc = (ft as usize).min(g.len());
    let sg = aux::select_unselect_group(
        d,
        &g,
        Some(e),
        tp,
        Some(2),
        Some(maxc),
        Some(&rescon_for(lvl, ft)),
        1,
        tp,
        Some(HINTMSG_SPSUMMON as u64),
        Some(&rescon_for(lvl, ft)),
        None,
        false,
    )
    .unwrap_or_default();
    // if #sg==0 then return end
    if sg.is_empty() {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // for tc in sg:Iter() do
    for sc in group::iter(d, &sg) {
        // if Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP) then
        if duel::special_summon_step(d, sc, 0, tp, tp, false, false, POS_FACEUP as u8, None) {
            // --Cannot be destroyed by battle or card effects this turn
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetDescription(3008)
            effect::set_description(d, e1, 3008);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1,true)
            card::register_effect(d, sc, e1, true);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
            effect::set_code(d, e2, EFFECT_INDESTRUCTABLE_EFFECT);
            // tc:RegisterEffect(e2,true)
            card::register_effect(d, sc, e2, true);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
// function s.sdesop(e,tp,eg,ep,ev,re,r,rp)
fn sdesop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local ct=c:GetTurnCounter()
    // ct=ct+1
    let ct = card::get_turn_counter(d, c) + 1;
    // c:SetTurnCounter(ct)
    card::set_turn_counter(d, c, ct as u16);
    // if ct==3 then
    if ct == 3 {
        // Duel.Destroy(c,REASON_RULE)
        duel::destroy(d, c, REASON_RULE, None, None);
    }
}
