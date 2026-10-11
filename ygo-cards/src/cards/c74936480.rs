//! Dinomorphia Stealthbergia
//! ダイノルフィア・ステルスベギア
use super::prelude::*;

pub const ID: u32 = 74936480;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DINOMORPHIA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // 2 "Dinoruffia" monsters with different names
    // Fusion.AddProcMixN(c,true,true,s.ffilter,2)
    fusion::add_proc_mix_n(d, c, true, true, vec![(fusion::Mat::F(fusion::fus_mat_full(|d, c, a| ffilter(d, c, a))), 2)]);
    // No LP Cost
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_LPCOST_CHANGE)
    effect::set_code(d, e1, EFFECT_LPCOST_CHANGE);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetCondition(function(e) return Duel.GetLP(e:GetHandlerPlayer())<=2000 end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        let p = card::get_controler(d, h);
        duel::get_lp(d, p) <= 2000
    })));
    // e1:SetValue(s.lpcostval)
    effect::set_value_fn(d, e1, lua_fn(lpcostval));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Damage
    // local e1=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e2, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e1:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e1:SetCondition(s.damcon)
    effect::set_condition(d, e2, Some(condition(damcon)));
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e2, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e2, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e2, false);
    // Special Summon
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e3, 1, ID, 1, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.ffilter(c,fc,sumtype,tp,sub,mg,sg)`
fn ffilter(d: &mut Duel, c: CardId, a: &fusion::MatArgs) -> bool {
    let (fc, sumtype, tp) = (a.fc, a.sumtype, a.tp);
    // return c:IsSetCard(SET_DINOMORPHIA,fc,sumtype,tp) and (not sg or not sg:IsExists(s.fusfilter,1,c,c:GetCode(fc,sumtype,tp),fc,sumtype,tp))
    if !card::is_set_card_ctx(d, c, &[SET_DINOMORPHIA as u16], Some(fc), sumtype, Some(tp)) {
        return false;
    }
    let Some(sg) = a.sg else { return true };
    let codes = card::get_code_ctx(d, c, Some(fc), sumtype, Some(tp));
    let code = codes.first().copied().nil();
    !group::is_exists(d, sg, &|d, x| fusfilter(d, x, code, fc, sumtype, tp), 1, c.into())
}
/// `s.fusfilter(c,code,fc,sumtype,tp)`
fn fusfilter(d: &mut Duel, c: CardId, code: u32, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsSummonCode(fc,sumtype,tp,code) and not c:IsHasEffect(511002961)
    card::is_summon_code(d, c, Some(fc), sumtype, tp, &[code]) && card::is_has_effect(d, c, 511002961, None).is_empty()
}
/// `s.lpcostval(e,re,rp,val)`
fn lpcostval(d: &mut Duel, a: Args) -> i64 {
    let val = a.int(3);
    // if not re then return val end
    let Some(re) = a.v(1).as_effect() else { return val };
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // if re:IsHasType(EFFECT_TYPE_ACTIVATE) and rc:IsTrap() then return 0 end
    if effect::is_has_type(d, re, EFFECT_TYPE_ACTIVATE) && card::is_trap(d, rc) {
        return 0;
    }
    // if re:IsMonsterEffect() and rc:IsSetCard(SET_DINOMORPHIA) then return 0 end
    if effect::is_monster_effect(d, re) && card::is_set_card(d, rc, &[SET_DINOMORPHIA as u16]) {
        return 0;
    }
    // return val
    val
}
/// `s.damcon(e,tp,eg,ep,ev,re,r,rp)`
fn damcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and re:IsMonsterEffect() and re:GetHandler():GetBaseAttack()>0
    a.rp == 1 - a.tp && effect::is_monster_effect(d, a.re.nil()) && {
        let h = effect::get_handler(d, a.re.nil()).nil();
        card::get_base_attack(d, h) > 0
    }
}
/// `s.damtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local atk=re:GetHandler():GetBaseAttack()
    let h = effect::get_handler(d, a.re.nil()).nil();
    let atk = card::get_base_attack(d, h);
    // Duel.SetTargetPlayer(1-tp)
    duel::set_target_player(d, 1 - tp);
    // Duel.SetTargetParam(atk)
    duel::set_target_param(d, atk as u32);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,atk)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, atk);
    true
}
/// `s.damop(e,tp,eg,ep,ev,re,r,rp)`
fn damop(d: &mut Duel, _a: &EffArgs) {
    // local p,d=Duel.GetChainInfo(0,CHAININFO_TARGET_PLAYER,CHAININFO_TARGET_PARAM)
    let info = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_PLAYER, CHAININFO_TARGET_PARAM]);
    // Duel.Damage(p,d,REASON_EFFECT)
    duel::damage(d, info[0].int() as u8, info[1].int() as i64, REASON_EFFECT, false, None);
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(_d: &mut Duel, a: &EffArgs) -> bool {
    // return (r&REASON_EFFECT+REASON_BATTLE)~=0
    (a.r as u32 & (REASON_EFFECT + REASON_BATTLE)) != 0
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsSetCard(SET_DINOMORPHIA) and c:IsLevelBelow(4) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_DINOMORPHIA as u16])
        && card::is_level_below(d, c, 4)
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // if Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return end
    if duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
