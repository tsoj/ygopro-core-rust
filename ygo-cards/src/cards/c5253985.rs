//! Soul Crossing
use super::prelude::*;

pub const ID: u32 = 5253985;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    let e1 = effect::create_effect(d, c);
    effect::set_category(d, e1, CATEGORY_SUMMON);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    effect::set_condition(d, e1, Some(condition(main_phase)));
    effect::set_target(d, e1, Some(cost_target(target)));
    effect::set_operation(d, e1, Some(operation(activate)));
    card::register_effect(d, c, e1, false);
    // Duel.AddCustomActivityCounter(id,ACTIVITY_CHAIN,s.chainfilter)
    duel::add_custom_activity_counter(d, ID, ACTIVITY_CHAIN, lua_fn(chainfilter));
}
/// `function() return Duel.IsMainPhase() end`
fn main_phase(d: &mut Duel, _a: &EffArgs) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.chainfilter(re,tp,cid)`
fn chainfilter(d: &mut Duel, a: Args) -> bool {
    let (re, tp) = (a.e(0), a.p(1));
    // if Duel.GetFlagEffect(tp,id)==0 then return true end
    if duel::get_flag_effect(d, tp, ID) == 0 {
        return true;
    }
    // return not (re:IsSpellTrapEffect() or not re:GetHandler():IsRace(RACE_DIVINE))
    let rc = effect::get_handler(d, re).nil();
    !(effect::is_spell_trap_effect(d, re) || !card::is_race(d, rc, RACE_DIVINE))
}
/// `s.filter(c,e,ec)`
fn filter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // if not c:IsRace(RACE_DIVINE) then return false end
    if !card::is_race(d, c, RACE_DIVINE) {
        return false;
    }
    // local e1=Effect.CreateEffect(ec)
    let e1 = effect::create_effect(d, ec);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_ADD_EXTRA_TRIBUTE)
    effect::set_code(d, e1, EFFECT_ADD_EXTRA_TRIBUTE);
    // e1:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e1, 0, LOCATION_MZONE);
    // e1:SetValue(POS_FACEUP)
    effect::set_value(d, e1, POS_FACEUP as i64);
    // e1:SetReset(RESET_CHAIN)
    effect::set_reset(d, e1, RESET_CHAIN, 1);
    // c:RegisterEffect(e1,true)
    card::register_effect(d, c, e1, true);
    // local res=c:IsSummonable(true,nil,1) or c:IsMSetable(true,nil,1)
    let res = card::is_summonable(d, c, true, None, 1, None) || card::is_m_setable(d, c, true, None, 1, None);
    // e1:Reset()
    effect::reset(d, e1);
    // return res
    res
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    let c = effect::get_handler(d, e).nil();
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil,e,e:GetHandler()) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, x| filter(d, x, c)), tp, LOCATION_HAND, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SUMMON,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SUMMON, None, 1, 0, 0);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SUMMON as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_HAND,0,1,1,nil,e,c):GetFirst()
    let sel = duel::select_matching_card(d, tp, Some(&|d, x| filter(d, x, c)), tp, LOCATION_HAND, 0, 1, 1, false, NO_EX).nil();
    // if tc then
    if let Some(tc) = group::get_first(d, &sel) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_ADD_EXTRA_TRIBUTE)
        effect::set_code(d, e1, EFFECT_ADD_EXTRA_TRIBUTE);
        // e1:SetTargetRange(0,LOCATION_MZONE)
        effect::set_target_range(d, e1, 0, LOCATION_MZONE);
        // e1:SetValue(POS_FACEUP)
        effect::set_value(d, e1, POS_FACEUP as i64);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // tc:RegisterEffect(e1,true)
        card::register_effect(d, tc, e1, true);
        // local s1=tc:IsSummonable(true,nil,1)
        let s1 = card::is_summonable(d, tc, true, None, 1, None);
        // local s2=tc:IsMSetable(true,nil,1)
        let s2 = card::is_m_setable(d, tc, true, None, 1, None);
        // if (s1 and s2 and Duel.SelectPosition(tp,tc,POS_FACEUP_ATTACK+POS_FACEDOWN_DEFENSE)==POS_FACEUP_ATTACK) or not s2 then
        if (s1 && s2 && duel::select_position(d, tp, tc, (POS_FACEUP_ATTACK + POS_FACEDOWN_DEFENSE) as u8) == POS_FACEUP_ATTACK as u32) || !s2 {
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e2:SetCode(EVENT_SUMMON_SUCCESS)
            effect::set_code(d, e2, EVENT_SUMMON_SUCCESS);
            // e2:SetOperation(s.sumsuc)
            effect::set_operation(d, e2, Some(operation(sumsuc)));
            // e2:SetLabel(tp)
            effect::set_label(d, e2, &[tp as i64]);
            // e2:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
            effect::set_reset(d, e2, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
            // tc:RegisterEffect(e2,true)
            card::register_effect(d, tc, e2, true);
            // Duel.Summon(tp,tc,true,nil,1)
            duel::summon(d, tp, tc, true, None, 1, None);
        } else {
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
            // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e3:SetCode(EVENT_MSET)
            effect::set_code(d, e3, EVENT_MSET);
            // e3:SetReset(RESET_EVENT|(RESETS_STANDARD&~RESET_TOFIELD))
            effect::set_reset(d, e3, RESET_EVENT | (RESETS_STANDARD & !RESET_TOFIELD), 1);
            // e3:SetOperation(s.sumsuc)
            effect::set_operation(d, e3, Some(operation(sumsuc)));
            // e3:SetLabel(tp)
            effect::set_label(d, e3, &[tp as i64]);
            // tc:RegisterEffect(e3,true)
            card::register_effect(d, tc, e3, true);
            // Duel.MSet(tp,tc,true,nil,1)
            duel::m_set(d, tp, tc, true, None, 1, None);
        }
    }
}
/// `s.sumsuc(e,tp,eg,ep,ev,re,r,rp)`
fn sumsuc(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    // local player=e:GetLabel()
    let player = effect::get_label(d, e) as u8;
    let c = effect::get_handler(d, e).nil();
    // if e:GetHandler():GetMaterial():IsExists(Card.IsPreviousControler,1,nil,1-player) then
    let mat = card::get_material(d, c);
    if group::is_exists(d, &mat, &|d, x| card::is_previous_controler(d, x, 1 - player), 1, NO_EX) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(aux.Stringid(id,0))
        effect::set_description(d, e1, aux::stringid(ID, 0));
        // e1:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e1, EFFECT_TYPE_FIELD);
        // e1:SetCode(EFFECT_CANNOT_ACTIVATE)
        effect::set_code(d, e1, EFFECT_CANNOT_ACTIVATE);
        // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetTargetRange(1,0)
        effect::set_target_range(d, e1, 1, 0);
        // e1:SetCondition(s.accon)
        effect::set_condition(d, e1, Some(condition(accon)));
        // e1:SetValue(s.aclimit)
        effect::set_value_fn(d, e1, lua_fn(aclimit));
        // e1:SetReset(RESET_PHASE|PHASE_END,2)
        effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 2);
        // Duel.RegisterEffect(e1,player)
        duel::register_effect(d, e1, player);
        // Duel.RegisterFlagEffect(player,id,RESET_PHASE|PHASE_END,0,2)
        duel::register_flag_effect(d, player, ID, RESET_PHASE | PHASE_END, 0, 2, 0);
    }
}
/// `s.accon(e)`
fn accon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.GetCustomActivityCount(id,e:GetHandlerPlayer(),ACTIVITY_CHAIN)>0
    let c = effect::get_handler(d, a.e).nil();
    let p = card::get_controler(d, c);
    duel::get_custom_activity_count(d, ID, p, ACTIVITY_CHAIN as u32) > 0
}
/// `s.aclimit(e,re,tp)`
fn aclimit(d: &mut Duel, a: Args) -> bool {
    // return re:IsSpellTrapEffect() or not re:GetHandler():IsRace(RACE_DIVINE)
    let re = a.e(1);
    if effect::is_spell_trap_effect(d, re) {
        return true;
    }
    let rc = effect::get_handler(d, re).nil();
    !card::is_race(d, rc, RACE_DIVINE)
}
