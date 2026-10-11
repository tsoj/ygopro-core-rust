//! Supreme King Z-ARC
//! (覇王龍ズァーク)
use super::prelude::*;

pub const ID: u32 = 13331639;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_SUPREME_KING_DRAGON], miracle_synchro_fusion: true, ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Fusion.AddProcMix(c,true,true,s.fusfilter1,s.fusfilter2,s.fusfilter3,s.fusfilter4)
    fusion::add_proc_mix(
        d,
        c,
        true,
        true,
        vec![
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| fusfilter1(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| fusfilter2(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| fusfilter3(d, c, fc, st, tp))),
            fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| fusfilter4(d, c, fc, st, tp))),
        ],
    );
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // spsummon condition
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
    // act limit
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_CANNOT_ACTIVATE)
    effect::set_code(d, e2, EFFECT_CANNOT_ACTIVATE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetRange(LOCATION_PZONE)
    effect::set_range(d, e2, LOCATION_PZONE);
    // e2:SetTargetRange(0,1)
    effect::set_target_range(d, e2, 0, 1);
    // e2:SetValue(s.limval)
    effect::set_value_fn(d, e2, lua_fn(limval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // destroy drawn
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e3:SetCode(EVENT_TO_HAND)
    effect::set_code(d, e3, EVENT_TO_HAND);
    // e3:SetRange(LOCATION_PZONE)
    effect::set_range(d, e3, LOCATION_PZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.ddcon)
    effect::set_condition(d, e3, Some(condition(ddcon)));
    // e3:SetTarget(s.ddtg)
    effect::set_target(d, e3, Some(cost_target(ddtg)));
    // e3:SetOperation(s.ddop)
    effect::set_operation(d, e3, Some(operation(ddop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // destroy all
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e4, CATEGORY_DESTROY);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Immune
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE);
    // e5:SetCode(EFFECT_CANNOT_BE_EFFECT_TARGET)
    effect::set_code(d, e5, EFFECT_CANNOT_BE_EFFECT_TARGET);
    // e5:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e5, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e5:SetRange(LOCATION_MZONE)
    effect::set_range(d, e5, LOCATION_MZONE);
    // e5:SetValue(aux.tgoval)
    effect::set_value_fn(d, e5, aux::tgoval());
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
    // local e6=e5:Clone()
    let e6 = effect::clone(d, e5);
    // e6:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e6, EFFECT_INDESTRUCTABLE_EFFECT);
    // e6:SetValue(s.tgvalue)
    effect::set_value_fn(d, e6, lua_fn(tgvalue));
    // c:RegisterEffect(e6)
    card::register_effect(d, c, e6, false);
    // special summon
    // local e7=Effect.CreateEffect(c)
    let e7 = effect::create_effect(d, c);
    // e7:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e7, aux::stringid(ID, 2));
    // e7:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e7, CATEGORY_SPECIAL_SUMMON);
    // e7:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e7, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e7:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e7, EVENT_BATTLE_DESTROYING);
    // e7:SetCondition(aux.bdocon)
    effect::set_condition(d, e7, Some(aux::bdocon()));
    // e7:SetTarget(s.sptg)
    effect::set_target(d, e7, Some(cost_target(sptg)));
    // e7:SetOperation(s.spop)
    effect::set_operation(d, e7, Some(operation(spop)));
    // c:RegisterEffect(e7)
    card::register_effect(d, c, e7, false);
    // pendulum
    // local e8=Effect.CreateEffect(c)
    let e8 = effect::create_effect(d, c);
    // e8:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e8, aux::stringid(ID, 3));
    // e8:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e8, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e8:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e8, EVENT_DESTROYED);
    // e8:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e8, EFFECT_FLAG_DELAY, 0);
    // e8:SetCondition(s.pencon)
    effect::set_condition(d, e8, Some(condition(pencon)));
    // e8:SetTarget(s.pentg)
    effect::set_target(d, e8, Some(cost_target(pentg)));
    // e8:SetOperation(s.penop)
    effect::set_operation(d, e8, Some(operation(penop)));
    // c:RegisterEffect(e8)
    card::register_effect(d, c, e8, false);
}
/// `s.fusfilter1(c,fc,sumtype,tp)`
fn fusfilter1(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON,fc,sumtype,tp) and c:IsType(TYPE_FUSION,fc,sumtype,tp)
    card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), sumtype, Some(tp)) && card::is_type_ctx(d, c, TYPE_FUSION, Some(fc), sumtype, Some(tp))
}
/// `s.fusfilter2(c,fc,sumtype,tp)`
fn fusfilter2(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON,fc,sumtype,tp) and c:IsType(TYPE_SYNCHRO,fc,sumtype,tp)
    card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), sumtype, Some(tp)) && card::is_type_ctx(d, c, TYPE_SYNCHRO, Some(fc), sumtype, Some(tp))
}
/// `s.fusfilter3(c,fc,sumtype,tp)`
fn fusfilter3(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON,fc,sumtype,tp) and c:IsType(TYPE_XYZ,fc,sumtype,tp)
    card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), sumtype, Some(tp)) && card::is_type_ctx(d, c, TYPE_XYZ, Some(fc), sumtype, Some(tp))
}
/// `s.fusfilter4(c,fc,sumtype,tp)`
fn fusfilter4(d: &mut Duel, c: CardId, fc: CardId, sumtype: u64, tp: u8) -> bool {
    // return c:IsRace(RACE_DRAGON,fc,sumtype,tp) and c:IsType(TYPE_PENDULUM,fc,sumtype,tp)
    card::is_race_ctx(d, c, RACE_DRAGON, Some(fc), sumtype, Some(tp)) && card::is_type_ctx(d, c, TYPE_PENDULUM, Some(fc), sumtype, Some(tp))
}
/// `s.limval(e,re,rp)`
fn limval(d: &mut Duel, a: Args) -> bool {
    // local rc=re:GetHandler()
    let re = a.e(1);
    let rc = effect::get_handler(d, re).nil();
    // return rc:IsLocation(LOCATION_MZONE) and re:IsMonsterEffect()
    //     and rc:IsType(TYPE_FUSION+TYPE_SYNCHRO+TYPE_XYZ)
    card::is_location(d, rc, LOCATION_MZONE) && effect::is_monster_effect(d, re) && card::is_type(d, rc, TYPE_FUSION + TYPE_SYNCHRO + TYPE_XYZ)
}
/// `s.ddcon(e,tp,eg,ep,ev,re,r,rp)`
fn ddcon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.GetCurrentPhase()~=PHASE_DRAW
    duel::get_current_phase(d) != PHASE_DRAW
}
/// `s.ddfilter(c,tp)`
fn ddfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsControler(1-tp) and c:IsPreviousLocation(LOCATION_DECK)
    card::is_controler(d, c, 1 - tp) && card::is_previous_location(d, c, LOCATION_DECK)
}
/// `s.ddtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn ddtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local g=eg:Filter(s.ddfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| ddfilter(d, c, tp), NO_EX);
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.ddop(e,tp,eg,ep,ev,re,r,rp)`
fn ddop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // local g=eg:Filter(s.ddfilter,nil,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| ddfilter(d, c, tp), NO_EX);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
    let g = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.Destroy(g,REASON_EFFECT)
        duel::destroy(d, g, REASON_EFFECT, None, None);
    }
}
/// `s.tgvalue(e,re,rp)`
fn tgvalue(d: &mut Duel, a: Args) -> bool {
    // return rp~=e:GetHandlerPlayer()
    let e = a.e(0);
    a.p(2) != effect::get_handler_player(d, e)
}
/// `s.spfilter(c,e,tp,rp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, rp: u8) -> bool {
    // if c:IsLocation(LOCATION_DECK) and Duel.GetLocationCount(tp,LOCATION_MZONE)<=0 then return false end
    if card::is_location(d, c, LOCATION_DECK) && duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 <= 0 {
        return false;
    }
    // if c:IsLocation(LOCATION_EXTRA) and Duel.GetLocationCountFromEx(tp,rp,nil,c)<=0 then return false end
    if card::is_location(d, c, LOCATION_EXTRA) && duel::get_location_count_from_ex(d, tp, Some(rp), NO_EX, duel::ExtraCard::Card(c), None).0 <= 0 {
        return false;
    }
    // return c:IsSetCard(SET_SUPREME_KING_DRAGON) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_set_card(d, c, &[SET_SUPREME_KING_DRAGON as u16]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    let e = a.e;
    let rp = a.rp;
    // if chk==0 then
    if a.chk == Some(0) {
        // return loc~=0 and Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,nil,e,tp,rp)
        // (`loc` is never assigned in the script, so it is nil and `loc~=0` is true)
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp, rp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_DECK|LOCATION_EXTRA)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, (LOCATION_DECK | LOCATION_EXTRA) as i32);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    let rp = a.rp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spfilter,tp,LOCATION_DECK|LOCATION_EXTRA,0,1,1,nil,e,tp,rp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spfilter(d, c, e, tp, rp)), tp, LOCATION_DECK | LOCATION_EXTRA, 0, 1, 1, false, NO_EX).nil();
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SpecialSummon(g,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, g, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.pencon(e,tp,eg,ep,ev,re,r,rp)`
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsFaceup()
    card::is_previous_location(d, c, LOCATION_MZONE) && card::is_faceup(d, c)
}
/// `s.pentg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn pentg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.CheckPendulumZones(tp) end
    if a.chk == Some(0) {
        return duel::check_pendulum_zones(d, a.tp);
    }
    true
}
/// `s.penop(e,tp,eg,ep,ev,re,r,rp)`
fn penop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if not Duel.CheckPendulumZones(tp) then return false end
    if !duel::check_pendulum_zones(d, tp) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
