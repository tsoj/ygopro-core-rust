//! 深淵の獣アルバ・ロス
//! The Bystial Alba Los
//! scripted by Naim
use super::prelude::*;

pub const ID: u32 = 69120785;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_BYSTIAL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special Summon condition
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e0, EFFECT_SPSUMMON_CONDITION);
    // e0:SetValue(aux.FALSE)
    effect::set_value_fn(d, e0, aux_false());
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Special Summon procedure from hand or GY
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // e1:SetValue(1)
    effect::set_value(d, e1, 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the effects of Ritual, Fusion, Synchro, Xyz, and Link Monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_DISABLE)
    effect::set_code(d, e2, EFFECT_DISABLE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
    effect::set_target_range(d, e2, LOCATION_MZONE, LOCATION_MZONE);
    // e2:SetCondition(function(e) return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+1) end)
    effect::set_condition(d, e2, Some(lua_fn(e2_cond)));
    // e2:SetTarget(function(e,c) return c:IsType(TYPE_RITUAL|TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK) and (c:IsType(TYPE_EFFECT) or c:IsOriginalType(TYPE_EFFECT)) end)
    effect::set_target(d, e2, Some(effect_card_fn(e2_target)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Banish all face-down cards in the Extra Decks
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e3, CATEGORY_REMOVE);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.rmvcond)
    effect::set_condition(d, e3, Some(condition(rmvcond)));
    // e3:SetTarget(s.rmvtg)
    effect::set_target(d, e3, Some(cost_target(rmvtg)));
    // e3:SetOperation(s.rmvop)
    effect::set_operation(d, e3, Some(operation(rmvop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_BYSTIAL) and c:IsReleasable()
    card::is_set_card(d, c, &[SET_BYSTIAL as u16]) && card::is_releasable(d, c, None, None)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, e);
    // local rg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // return #rg>0 and aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),0)
    if rg.is_empty() {
        return false;
    }
    let chk = aux::chkf_mmz(1);
    matches!(aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&chk), 0, tp, None, None, None, false), Err(true))
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)`
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_MZONE,0,nil)
    let rg = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_MZONE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),1,tp,HINTMSG_RELEASE,nil,nil,true)
    let chk = aux::chkf_mmz(1);
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&chk), 1, tp, Some(HINTMSG_RELEASE as u64), None, None, true).unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive() (groups are values in Rust)
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.Release(g,REASON_COST)
    duel::release(d, g, REASON_COST, None);
    // g:DeleteGroup() (groups are values in Rust)
}
/// `function(e) return e:GetHandler():IsSummonType(SUMMON_TYPE_SPECIAL+1) end`
fn e2_cond(d: &mut Duel, a: Args) -> bool {
    let h = effect::get_handler(d, a.e(0)).nil();
    card::is_summon_type(d, h, &[SUMMON_TYPE_SPECIAL + 1])
}
/// `function(e,c) return c:IsType(TYPE_RITUAL|TYPE_FUSION|TYPE_SYNCHRO|TYPE_XYZ|TYPE_LINK) and (c:IsType(TYPE_EFFECT) or c:IsOriginalType(TYPE_EFFECT)) end`
fn e2_target(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    card::is_type(d, c, TYPE_RITUAL | TYPE_FUSION | TYPE_SYNCHRO | TYPE_XYZ | TYPE_LINK)
        && (card::is_type(d, c, TYPE_EFFECT) || card::is_original_type(d, c, TYPE_EFFECT))
}
/// `s.rmvcond(e,tp,eg,ep,ev,re,r,rp)`
fn rmvcond(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // return rp==1-tp and c:IsReason(REASON_EFFECT) and c:IsPreviousControler(tp) and c:IsPreviousPosition(POS_FACEUP)
    //     and not c:IsLocation(LOCATION_DECK)
    a.rp == 1 - tp
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_previous_controler(d, c, tp)
        && card::is_previous_position(d, c, POS_FACEUP as u32)
        && !card::is_location(d, c, LOCATION_DECK)
}
/// `s.rmvfilter(c)`
fn rmvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFacedown() and c:IsAbleToRemove()
    card::is_facedown(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmvtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(s.rmvfilter,tp,LOCATION_EXTRA,LOCATION_EXTRA,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&rmvfilter), tp, LOCATION_EXTRA, LOCATION_EXTRA, 1, NO_EX);
    }
    // local g=Duel.GetMatchingGroup(s.rmvfilter,tp,LOCATION_EXTRA,LOCATION_EXTRA,nil)
    let g = duel::get_matching_group(d, Some(&rmvfilter), tp, LOCATION_EXTRA, LOCATION_EXTRA, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, 0, 0);
    true
}
/// `s.rmvop(e,tp,eg,ep,ev,re,r,rp)`
fn rmvop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local g=Duel.GetMatchingGroup(s.rmvfilter,tp,LOCATION_EXTRA,LOCATION_EXTRA,nil)
    let g = duel::get_matching_group(d, Some(&rmvfilter), tp, LOCATION_EXTRA, LOCATION_EXTRA, NO_EX);
    // if Duel.Remove(g,POS_FACEUP,REASON_EFFECT)>0 then
    if duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None) > 0 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, e).nil();
        // local fid=c:GetFieldID()
        let fid = card::get_field_id(d, c) as i64;
        // local og=Duel.GetOperatedGroup()
        let og = duel::get_operated_group(d);
        // for oc in og:Iter() do
        for oc in group::iter(d, &og) {
            // oc:RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,2,fid)
            card::register_flag_effect(d, oc, ID, RESETS_STANDARD_PHASE_END, 0, 2, fid, 0);
        }
        // og:KeepAlive() (groups are values in Rust)
        // --Return the banished cards to the Extra Deck
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,1))
        effect::set_description(d, e1, aux::stringid(ID, 1));
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetCode(EVENT_PHASE+PHASE_END)
        effect::set_code(d, e1, EVENT_PHASE + PHASE_END);
        // e1:SetCountLimit(1)
        effect::set_count_limit(d, e1, 1, 0, 0, 0);
        // e1:SetLabel(fid)
        effect::set_label(d, e1, &[fid]);
        // e1:SetLabelObject(og)
        effect::set_label_object(d, e1, Value::Group(og));
        // e1:SetCondition(s.rcond)
        effect::set_condition(d, e1, Some(condition(rcond)));
        // e1:SetOperation(s.roper)
        effect::set_operation(d, e1, Some(operation(roper)));
        // e1:SetReset(RESET_PHASE|PHASE_END|RESET_OPPO_TURN)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END | RESET_OPPO_TURN, 1);
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, tp);
    }
}
/// `s.rfilter(c,fid)`
fn rfilter(d: &mut Duel, c: CardId, fid: i64) -> bool {
    // return c:GetFlagEffectLabel(id)==fid
    card::get_flag_effect_label(d, c, ID).first().copied() == Some(fid)
}
/// `s.rcond(e,tp,eg,ep,ev,re,r,rp)`
fn rcond(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if Duel.IsTurnPlayer(tp) then return false end
    if duel::is_turn_player(d, tp) {
        return false;
    }
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned().nil();
    let fid = effect::get_label(d, e);
    // if not g:IsExists(s.rfilter,1,nil,e:GetLabel()) then
    if !group::is_exists(d, &g, &|d, c| rfilter(d, c, fid), 1, NO_EX) {
        // g:DeleteGroup() (groups are values in Rust)
        // e:Reset()
        effect::reset(d, e);
        // return false
        false
    } else {
        // else return true end
        true
    }
}
/// `s.roper(e,tp,eg,ep,ev,re,r,rp)`
fn roper(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, e).as_group().cloned().nil();
    let fid = effect::get_label(d, e);
    // local rg=g:Filter(s.rfilter,nil,e:GetLabel())
    let rg = group::filter(d, &g, &|d, c| rfilter(d, c, fid), NO_EX);
    // Duel.SendtoDeck(rg,nil,SEQ_DECKSHUFFLE,REASON_EFFECT|REASON_RETURN)
    duel::send_to_deck(d, rg, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT | REASON_RETURN, None);
}
