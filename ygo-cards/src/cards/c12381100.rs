//! Dragon Master Magia
//! Scripted by Eerie Code
use super::prelude::*;

pub const ID: u32 = 12381100;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[23995346], listed_series: &[SET_BLUE_EYES, SET_CHAOS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion Summon procedure ("Blue-Eyes Ultimate Dragon")
    // local f0=Fusion.AddProcMix(c,true,true,23995346,s.ffilter)[1]
    let f0 = fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![fusion::Mat::Code(23995346), fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp)))],
    )
    .nil();
    // f0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, f0, aux::stringid(ID, 0));
    // Fusion Summon procedure (3 "Blue-Eyes" monsters)
    // local f1=Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_BLUE_EYES),3,s.ffilter,1)[1]
    let f1 = fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![
            (
                fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_BLUE_EYES as u16], Some(fc), st, Some(tp)))),
                3,
            ),
            (fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| ffilter(d, c, fc, st, tp))), 1),
        ],
    )
    .nil();
    // f1:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, f1, aux::stringid(ID, 1));
    // Must be Fusion Summoned
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_CONDITION)
    effect::set_code(d, e1, EFFECT_SPSUMMON_CONDITION);
    // e1:SetValue(aux.fuslimit)
    effect::set_value_fn(d, e1, aux::fuslimit());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate the activation of an opponent's card or effect
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e2, aux::stringid(ID, 2));
    // e2:SetCategory(CATEGORY_NEGATE+CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_NEGATE + CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DAMAGE_CAL)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DAMAGE_CAL, 0);
    // e2:SetCode(EVENT_CHAINING)
    effect::set_code(d, e2, EVENT_CHAINING);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCondition(s.negcon)
    effect::set_condition(d, e2, Some(condition(negcon)));
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Special Summon 1 "Blue-Eyes" monster, or 1 "Chaos" or "Black Luster Soldier" Ritual Monster, from your Extra Deck or GY
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e3, aux::stringid(ID, 3));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e3, EVENT_LEAVE_FIELD);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Register types
    // aux.GlobalCheck(s,function() s.type_list={} s.type_list[0]=0 s.type_list[1]=0 aux.AddValuesReset(...) end)
    aux::global_check(d, ID, |d| {
        // s.type_list={}
        // s.type_list[0]=0
        set_s_var(d, ID, 0, Value::Int(0));
        // s.type_list[1]=0
        set_s_var(d, ID, 1, Value::Int(0));
        // aux.AddValuesReset(function() s.type_list[0]=0 s.type_list[1]=0 end)
        aux::add_values_reset(d, reset_types);
    });
}
/// `aux.AddValuesReset(function() s.type_list[0]=0 s.type_list[1]=0 end)`
fn reset_types(d: &mut Duel) {
    set_s_var(d, ID, 0, Value::Int(0));
    set_s_var(d, ID, 1, Value::Int(0));
}
/// `s.ffilter(c,fc,sumtype,tp)`: `return c:IsSetCard(SET_CHAOS,fc,sumtype,tp) and c:IsType(TYPE_RITUAL,fc,sumtype,tp)`
fn ffilter(d: &mut Duel, c: CardId, fc: CardId, st: u64, tp: u8) -> bool {
    card::is_set_card_ctx(d, c, &[SET_CHAOS as u16], Some(fc), st, Some(tp)) && card::is_type_ctx(d, c, TYPE_RITUAL, Some(fc), st, Some(tp))
}
/// `s.negcon(e,tp,eg,ep,ev,re,r,rp)`
fn negcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return rp==1-tp and not e:GetHandler():IsStatus(STATUS_BATTLE_DESTROYED) and Duel.IsChainNegatable(ev)
    //   and s.type_list[tp]&re:GetActiveType()==0
    if a.rp != 1 - a.tp {
        return false;
    }
    let h = effect::get_handler(d, a.e).nil();
    if card::is_status(d, h, STATUS_BATTLE_DESTROYED) {
        return false;
    }
    if !duel::is_chain_negatable(d, a.ev as u8) {
        return false;
    }
    let re = a.re.nil();
    let list = s_var(d, ID, a.tp as u64).int() as u32;
    (list & effect::get_active_type(d, re)) == 0
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // s.type_list[tp]=s.type_list[tp]|(re:GetActiveType()&(TYPE_MONSTER|TYPE_SPELL|TYPE_TRAP))
    let re = a.re.nil();
    let list = s_var(d, ID, a.tp as u64).int() as u32;
    let active = effect::get_active_type(d, re) & (TYPE_MONSTER | TYPE_SPELL | TYPE_TRAP);
    set_s_var(d, ID, a.tp as u64, Value::Int((list | active) as i64));
    // Duel.SetOperationInfo(0,CATEGORY_NEGATE,eg,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_NEGATE, a.eg.clone(), 1, 0, 0);
    // local rc=re:GetHandler()
    let rc = effect::get_handler(d, re).nil();
    // if rc:IsRelateToEffect(re) and rc:IsDestructable() then
    if card::is_relate_to_effect(d, rc, re) && card::is_destructable(d, rc, None) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,eg,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, a.eg.clone(), 1, 0, 0);
    }
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // if Duel.NegateActivation(ev) and re:GetHandler():IsRelateToEffect(re) then
    if duel::negate_activation(d, a.ev as u8) {
        let re = a.re.nil();
        let rc = effect::get_handler(d, re).nil();
        if card::is_relate_to_effect(d, rc, re) {
            // Duel.Destroy(eg,REASON_EFFECT)
            duel::destroy(d, a.eg(), REASON_EFFECT, None, None);
        }
    }
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousPosition(POS_FACEUP) and c:IsPreviousControler(tp) and rp==1-tp
    card::is_previous_position(d, c, POS_FACEUP) && card::is_previous_controler(d, c, a.tp) && a.rp == 1 - a.tp
}
/// `s.spfilter(c,e,tp,mmz_chk)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, mmz_chk: bool) -> bool {
    // if not ((c:IsSetCard(SET_BLUE_EYES) or (c:IsSetCard(SET_CHAOS) and c:IsRitualMonster()))
    //   and c:IsCanBeSpecialSummoned(e,0,tp,false,false)) then return false end
    let ok = (card::is_set_card(d, c, &[SET_BLUE_EYES as u16]) || (card::is_set_card(d, c, &[SET_CHAOS as u16]) && card::is_ritual_monster(d, c)))
        && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None);
    if !ok {
        return false;
    }
    // if c:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, c, LOCATION_GRAVE) {
        // return mmz_chk
        mmz_chk
    } else {
        // return Duel.GetLocationCountFromEx(tp,tp,nil,c)>0
        duel::get_location_count_from_ex(d, tp, Some(tp), NO_EX, duel::ExtraCard::Card(c), None).0 > 0
    }
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    if a.chk == Some(0) {
        // return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_EXTRA|LOCATION_GRAVE,0,1,nil,e,tp,mmz_chk)
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, a.e, a.tp, mmz_chk)), a.tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_EXTRA|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, a.tp, (LOCATION_EXTRA | LOCATION_GRAVE) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local mmz_chk=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let mmz_chk = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_EXTRA|LOCATION_GRAVE,0,1,1,nil,e,tp,mmz_chk)
    let nv = aux::necro_valley_filter(|d, c| spfilter(d, c, a.e, a.tp, mmz_chk));
    let g = duel::select_matching_card(d, a.tp, Some(&nv), a.tp, LOCATION_EXTRA | LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
