//! Magikey Dragon - Andrabime
//! scripted by Rundas
use super::prelude::*;

pub const ID: u32 = 71159974;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_MAGIKEY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,s.matfilter1,s.matfilter2)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter1(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| matfilter2(d, c, fc, st, tp))),
        ],
    );
    // No Cards and Effects on Fusion Summon
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e0:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e0, EVENT_SPSUMMON_SUCCESS);
    // e0:SetCondition(s.limcon)
    effect::set_condition(d, e0, Some(condition(limcon)));
    // e0:SetOperation(s.limop)
    effect::set_operation(d, e0, Some(operation(limop)));
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCode(EVENT_CHAIN_END)
    effect::set_code(d, e1, EVENT_CHAIN_END);
    // e1:SetCondition(s.limcon)
    effect::set_condition(d, e1, Some(condition(limcon)));
    // e1:SetOperation(s.limop2)
    effect::set_operation(d, e1, Some(operation(limop2)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Draw
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DRAW)
    effect::set_category(d, e3, CATEGORY_DRAW);
    // e3:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY + EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.drcon)
    effect::set_condition(d, e3, Some(condition(drcon)));
    // e3:SetTarget(s.drtg)
    effect::set_target(d, e3, Some(cost_target(drtg)));
    // e3:SetOperation(s.drop)
    effect::set_operation(d, e3, Some(operation(drop_op)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.matfilter1(c,fc,sumtype,tp)`
fn matfilter1(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSetCard(SET_MAGIKEY) and c:IsType(TYPE_EFFECT,fc,sumtype,tp)
    card::is_set_card(d, c, &[SET_MAGIKEY as u16]) && card::is_type_ctx(d, c, TYPE_EFFECT, Some(fc), sumtype, Some(tp))
}
/// `s.matfilter2(c,fc,sumtype,tp)`
fn matfilter2(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsType(TYPE_NORMAL,fc,sumtype,tp) and not c:IsType(TYPE_TOKEN,fc,sumtype,tp)
    card::is_type_ctx(d, c, TYPE_NORMAL, Some(fc), sumtype, Some(tp))
        && !card::is_type_ctx(d, c, TYPE_TOKEN, Some(fc), sumtype, Some(tp))
}
// No Cards and Effects on Fusion Summon
/// `s.limcon(e,tp,eg,ep,ev,re,r,rp)`
fn limcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsFusionSummoned()
    let h = effect::get_handler(d, a.e).nil();
    card::is_fusion_summoned(d, h)
}
/// `s.limop(e,tp,eg,ep,ev,re,r,rp)`
fn limop(d: &mut Duel, a: &EffArgs) {
    // if Duel.GetCurrentChain()==0 then
    if duel::get_current_chain(d, false) == 0 {
        // Duel.SetChainLimitTillChainEnd(s.chainlm)
        duel::set_chain_limit_till_chain_end(d, chainlm_fn());
    // elseif Duel.GetCurrentChain()==1 then
    } else if duel::get_current_chain(d, false) == 1 {
        let h = effect::get_handler(d, a.e).nil();
        // e:GetHandler():RegisterFlagEffect(id,RESETS_STANDARD_PHASE_END,0,1)
        card::register_flag_effect(d, h, ID, RESETS_STANDARD_PHASE_END, 0, 1, 0, 0);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e1:SetCode(EVENT_CHAINING)
        effect::set_code(d, e1, EVENT_CHAINING);
        // e1:SetOperation(s.resetop)
        effect::set_operation(d, e1, Some(operation(resetop)));
        // Duel.RegisterEffect(e1,tp)
        duel::register_effect(d, e1, a.tp);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EVENT_BREAK_EFFECT)
        effect::set_code(d, e2, EVENT_BREAK_EFFECT);
        // e2:SetReset(RESET_CHAIN)
        effect::set_reset(d, e2, RESET_CHAIN, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
    }
}
/// `s.limop2(e,tp,eg,ep,ev,re,r,rp)`
fn limop2(d: &mut Duel, a: &EffArgs) {
    let h = effect::get_handler(d, a.e).nil();
    // if e:GetHandler():GetFlagEffect(id)~=0 then
    if card::get_flag_effect(d, h, ID) != 0 {
        // Duel.SetChainLimitTillChainEnd(s.chainlm)
        duel::set_chain_limit_till_chain_end(d, chainlm_fn());
    }
    // e:GetHandler():ResetFlagEffect(id)
    card::reset_flag_effect(d, h, ID);
}
/// `s.resetop(e,tp,eg,ep,ev,re,r,rp)`
fn resetop(d: &mut Duel, a: &EffArgs) {
    // e:GetHandler():ResetFlagEffect(id)
    let h = effect::get_handler(d, a.e).nil();
    card::reset_flag_effect(d, h, ID);
    // e:Reset()
    effect::reset(d, a.e);
}
/// `s.chainlm(e,rp,tp)`: `return tp==rp`
fn chainlm_fn() -> LuaFn {
    lua_fn(|_d, a| a.p(2) == a.p(1))
}
// Destroy
/// `s.attfilter(c,tp)`
fn attfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return ((c:IsSetCard(SET_MAGIKEY) or c:IsType(TYPE_NORMAL)) and c:IsMonster())
    //     and Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_MZONE,1,nil,c:GetAttribute())
    ((card::is_set_card(d, c, &[SET_MAGIKEY as u16]) || card::is_type(d, c, TYPE_NORMAL)) && card::is_monster(d, c)) && {
        let att = card::get_attribute(d, c);
        duel::is_existing_matching_card(d, Some(&|d, x| desfilter(d, x, att)), tp, 0, LOCATION_MZONE, 1, NO_EX)
    }
}
/// `s.desfilter(c,att)`
fn desfilter(d: &mut Duel, c: CardId, att: u32) -> bool {
    // return c:IsAttribute(att) and c:IsMonster() and c:IsFaceup()
    card::is_attribute(d, c, att) && card::is_monster(d, c) && card::is_faceup(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_GRAVE) and chkc:IsControler(tp) and s.attfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_GRAVE) && card::is_controler(d, chkc, tp) && attfilter(d, chkc, tp);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.attfilter,tp,LOCATION_GRAVE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| attfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,s.attfilter,tp,LOCATION_GRAVE,0,1,1,nil,tp):GetFirst()
    let g = duel::select_target(d, tp, Some(&|d, c| attfilter(d, c, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g).nil();
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,tc:GetAttribute())
    let att = card::get_attribute(d, tc);
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, att)), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,1-tp,LOCATION_MZONE)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 1 - tp, LOCATION_MZONE as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc and tc:IsRelateToEffect(e) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, a.e) {
            // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,tc:GetAttribute())
            let att = card::get_attribute(d, tc);
            let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, att)), tp, 0, LOCATION_MZONE, NO_EX);
            // if #g>0 then
            if !g.is_empty() {
                // Duel.Destroy(g,REASON_EFFECT)
                duel::destroy(d, g, REASON_EFFECT, None, None);
            }
        }
    }
}
// Draw
/// `s.drfilter(c,tp,att)`
fn drfilter(d: &mut Duel, c: CardId, tp: u8, att: u32) -> bool {
    // return c:IsPreviousControler(1-tp) and c:IsReason(REASON_BATTLE|REASON_EFFECT) and c:GetAttribute()&att>0
    card::is_previous_controler(d, c, 1 - tp)
        && card::is_reason(d, c, REASON_BATTLE | REASON_EFFECT)
        && (card::get_attribute(d, c) & att) > 0
}
/// `s.drcon(e,tp,eg,ep,ev,re,r,rp)`
fn drcon(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local att,mat=0,e:GetHandler():GetMaterial()
    let mut att: u32 = 0;
    let h = effect::get_handler(d, a.e).nil();
    let mat = card::get_material(d, h);
    // if e:GetHandler():IsFusionSummoned() and mat:GetClassCount(Card.GetAttribute)>1 then
    if card::is_fusion_summoned(d, h) && group::get_class_count(d, &mat, &|d, x| card::get_attribute(d, x) as i64) > 1 {
        // for tc in Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_GRAVE,0,nil):Iter() do
        let g = duel::get_matching_group(d, Some(&|d, x| card::is_monster(d, x)), tp, LOCATION_GRAVE, 0, NO_EX);
        for tc in group::iter(d, &g) {
            // att=att|tc:GetAttribute()
            att |= card::get_attribute(d, tc);
        }
        // return att>0 and eg:IsExists(s.drfilter,1,nil,tp,att)
        let eg = a.eg.clone().nil();
        return att > 0 && group::is_exists(d, &eg, &|d, c| drfilter(d, c, tp, att), 1, NO_EX);
    }
    // (falls off the end: nil, which is false)
    false
}
/// `s.drtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn drtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsPlayerCanDraw(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_draw(d, tp, 1);
    }
    // Duel.SetTargetPlayer(tp)
    duel::set_target_player(d, tp);
    // Duel.SetTargetParam(1)
    duel::set_target_param(d, 1);
    // Duel.SetOperationInfo(0,CATEGORY_DRAW,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_DRAW, None, 1, tp, LOCATION_DECK as i32);
    true
}
/// `s.drop(e,tp,eg,ep,ev,re,r,rp)`
fn drop_op(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Draw(p,d,REASON_EFFECT)
    duel::draw(d, info[0].int() as u8, info[1].int() as u32, REASON_EFFECT);
}
