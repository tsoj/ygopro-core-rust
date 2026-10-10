//! Swordsoul Sinister Sovereign - Qixing Longyuan
use super::prelude::*;

pub const ID: u32 = 47710198;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Synchro.AddProcedure(c,nil,1,1,Synchro.NonTunerEx(Card.IsRace,RACE_WYRM),1,99)
    synchro::add_procedure(
        d,
        c,
        None,
        1,
        1,
        Some(synchro::non_tuner_ex(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_WYRM, sc, st, Some(tp)))),
        1,
        99,
        None,
        None,
        None,
        None,
        None,
    );
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e1, CATEGORY_DRAW);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SPSUMMON_SUCCESS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.drcon)
    effect::set_condition(d, e1, Some(condition(drcon)));
    // e1:SetTarget(s.drtg)
    effect::set_target(d, e1, Some(cost_target(drtg)));
    // e1:SetOperation(s.drop)
    effect::set_operation(d, e1, Some(operation(drop_op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_REMOVE+CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_REMOVE + CATEGORY_DAMAGE);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY, 0);
    // e2:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e2, EVENT_CUSTOM + ID);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.GetCurrentPhase()~=PHASE_DAMAGE and Duel.GetCurrentPhase()~=PHASE_DAMAGE_CAL end)
    effect::set_condition(d, e2, Some(condition(phase_cond)));
    // e2:SetTarget(s.rmtg)
    effect::set_target(d, e2, Some(cost_target(rmtg)));
    // e2:SetOperation(s.rmop)
    effect::set_operation(d, e2, Some(operation(rmop)));
    // local g=Group.CreateGroup()
    // g:KeepAlive()
    // e2:SetLabelObject(g)
    effect::set_label_object(d, e2, Value::Group(CardSet::new()));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2a, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2a:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SPSUMMON_SUCCESS);
    // e2a:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2a, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2a:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2a, LOCATION_MZONE);
    // e2a:SetLabelObject(e2)
    effect::set_label_object(d, e2a, Value::Effect(e2));
    // e2a:SetOperation(s.regop)
    effect::set_operation(d, e2a, Some(operation(regop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetCategory(CATEGORY_REMOVE+CATEGORY_DAMAGE)
    effect::set_category(d, e3, CATEGORY_REMOVE + CATEGORY_DAMAGE);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_CHAINING)
    effect::set_code(d, e3, EVENT_CHAINING);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.rmcon2)
    effect::set_condition(d, e3, Some(condition(rmcon2)));
    // e3:SetTarget(s.rmtg2)
    effect::set_target(d, e3, Some(cost_target(rmtg2)));
    // e3:SetOperation(s.rmop2)
    effect::set_operation(d, e3, Some(operation(rmop2)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c,e,tp)`
fn cfilter(d: &mut Duel, c: CardId, _e: EffectId, tp: u8) -> bool {
    // return c:IsFaceup() and c:IsType(TYPE_SYNCHRO) and c:IsSynchroSummoned() and c:IsRace(RACE_WYRM) and c:IsSummonPlayer(tp)
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_SYNCHRO) && card::is_synchro_summoned(d, c) && card::is_race(d, c, RACE_WYRM) && card::is_summon_player(d, c, tp)
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,e:GetHandler(),e,tp)
    let eg = a.eg.clone().nil();
    let h = effect::get_handler(d, a.e).nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c, a.e, a.tp), 1, h.into())
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, a.tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, a.tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,0,tp,1)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 0, a.tp, 1);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    let p = info[0].int() as u8;
    let dr = info[1].int() as u32;
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, p, dr, REASON_EFFECT);
}
/// `s.filter(c,tp,e)`: `e` is nil in the `Filter` calls of `s.regop` and `s.rmtg`
fn filter(d: &mut Duel, c: CardId, tp: u8, e: Option<EffectId>) -> bool {
    // return c:IsSummonPlayer(1-tp) and c:IsLocation(LOCATION_MZONE) and c:IsAbleToRemove()
    //   and (not e or c:IsRelateToEffect(e))
    card::is_summon_player(d, c, 1 - tp)
        && card::is_location(d, c, LOCATION_MZONE)
        && card::is_able_to_remove(d, c, None, None, None)
        && match e {
            None => true,
            Some(e) => card::is_relate_to_effect(d, c, e),
        }
}
/// `function() return Duel.GetCurrentPhase()~=PHASE_DAMAGE and Duel.GetCurrentPhase()~=PHASE_DAMAGE_CAL end`
fn phase_cond(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::get_current_phase(d) != PHASE_DAMAGE && duel::get_current_phase(d) != PHASE_DAMAGE_CAL
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject():Filter(s.filter,nil,tp,nil)
    let lg = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &lg, &|d, c| filter(d, c, a.tp, None), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.RegisterFlagEffect(tp,id,RESET_PHASE|PHASE_END,0,1)
    duel::register_flag_effect(d, a.tp, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    // Duel.SetTargetCard(g)
    duel::set_target_card(d, g.clone());
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1200)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1200);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned().nil();
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local bg=g:FilterSelect(tp,s.filter,1,1,nil,tp,e)
    let bg = group::filter_select(d, &g, a.tp, &|d, c| filter(d, c, a.tp, Some(a.e)), 1, 1, false, NO_EX).nil();
    // if #bg>0 and Duel.Remove(bg,POS_FACEUP,REASON_EFFECT)>0 then
    if !bg.is_empty() && duel::remove(d, bg, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // Duel.Damage(1-tp,1200,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, 1200, REASON_EFFECT, false, None);
    }
}
/// `s.regop(e,tp,eg,ep,ev,re,r,rp)`
fn regop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetFlagEffect(tp,id)>0 then return end
    if duel::get_flag_effect(d, a.tp, ID) > 0 {
        return;
    }
    // local c=e:GetHandler()
    // local tg=eg:Filter(s.filter,nil,tp)
    let eg = a.eg.clone().nil();
    let tg = group::filter(d, &eg, &|d, c| filter(d, c, a.tp, None), NO_EX);
    // if #tg>0 then
    if !tg.is_empty() {
        // for tc in tg:Iter() do tc:RegisterFlagEffect(id,RESET_CHAIN,0,1) end
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        // local g=e:GetLabelObject():GetLabelObject()
        let e2 = effect::get_label_object(d, a.e).as_effect().nil();
        let mut g = effect::get_label_object(d, e2).as_group().cloned().nil();
        // if Duel.GetCurrentChain()==0 then g:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            g = CardSet::new();
        }
        // g:Merge(tg)
        group::add_card(&mut g, &tg);
        // g:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut g, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        // e:GetLabelObject():SetLabelObject(g)
        effect::set_label_object(d, e2, Value::Group(g));
        // Duel.RaiseSingleEvent(e:GetHandler(),EVENT_CUSTOM+id,e,0,tp,tp,0)
        let h = effect::get_handler(d, a.e).nil();
        duel::raise_single_event(d, h, EVENT_CUSTOM + ID, Some(a.e), 0, a.tp, a.tp, 0);
    }
}
/// `s.rmcon2(e,tp,eg,ep,ev,re,r,rp)`
fn rmcon2(d: &mut Duel, a: &EffArgs) -> bool {
    // return ep==1-tp and re:IsSpellTrapEffect() and re:GetHandler():IsRelateToEffect(re)
    if a.ep != 1 - a.tp {
        return false;
    }
    let re = a.re.nil();
    if !effect::is_spell_trap_effect(d, re) {
        return false;
    }
    let h = effect::get_handler(d, re).nil();
    card::is_relate_to_effect(d, h, re)
}
/// `s.rmtg2(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg2(d: &mut Duel, a: &EffArgs) -> bool {
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    // if chk==0 then return re:GetHandler():IsAbleToRemove() end
    if a.chk == Some(0) {
        return card::is_able_to_remove(d, h, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,re:GetHandler(),1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(h.into()), 1, 0, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1200)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, 1200);
    true
}
/// `s.rmop2(e,tp,eg,ep,ev,re,r,rp)`
fn rmop2(d: &mut Duel, a: &EffArgs) {
    let re = a.re.nil();
    let h = effect::get_handler(d, re).nil();
    // if not re:GetHandler():IsRelateToEffect(re) then return end
    if !card::is_relate_to_effect(d, h, re) {
        return;
    }
    // if Duel.Remove(eg,POS_FACEUP,REASON_EFFECT)>0 then
    if duel::remove(d, a.eg(), POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // Duel.Damage(1-tp,1200,REASON_EFFECT)
        duel::damage(d, 1 - a.tp, 1200, REASON_EFFECT, false, None);
    }
}
