//! Mamonaka the Vaylantz United
//! ヴァリアンツの聚－幻中
use super::prelude::*;

pub const ID: u32 = 40680521;
pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_VAYLANTZ], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Pendulum.AddProcedure(c,false)
    pendulum::add_procedure(d, c, Some(false), None);
    // Fusion Materials: 3 "Vaylantz" monsters
    // Fusion.AddProcMixN(c,true,true,aux.FilterBoolFunctionEx(Card.IsSetCard,SET_VAYLANTZ),3)
    fusion::add_proc_mix_n(
        d,
        c,
        true,
        true,
        vec![(fusion::Mat::F(fusion::fus_mat(|d, c, fc, st, tp| card::is_set_card_ctx(d, c, &[SET_VAYLANTZ as u16], Some(fc), st, Some(tp)))), 3)],
    );
    // Activate 1 of these effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.efftg)
    effect::set_target(d, e1, Some(cost_target(efftg)));
    // e1:SetOperation(s.effop)
    effect::set_operation(d, e1, Some(operation(effop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Place 1 Effect Monster in your opponent's Main Monster Zone face-up as a Continuous Spell in their Spell & Trap Zone in its same column
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e2, CATEGORY_DESTROY);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function() return Duel.IsMainPhase() end)
    effect::set_condition(d, e2, Some(lua_fn(cond_main_phase)));
    // e2:SetTarget(s.pltg)
    effect::set_target(d, e2, Some(cost_target(pltg)));
    // e2:SetOperation(s.plop)
    effect::set_operation(d, e2, Some(operation(plop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Place this card in your Pendulum Zone
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e3, EVENT_DESTROYED);
    // e3:SetCondition(s.pencon)
    effect::set_condition(d, e3, Some(condition(pencon)));
    // e3:SetTarget(s.pentg)
    effect::set_target(d, e3, Some(cost_target(pentg)));
    // e3:SetOperation(s.penop)
    effect::set_operation(d, e3, Some(operation(penop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function() return Duel.IsMainPhase() end`
fn cond_main_phase(d: &mut Duel, _a: Args) -> bool {
    duel::is_main_phase(d, None)
}
/// `s.efftg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn efftg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local zone=(1<<c:GetSequence())&ZONES_MMZ
    let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
    // local b1=zone>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
    let b1 = zone > 0 && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(tp), Some(zone));
    // local b2=Duel.IsExistingMatchingCard(Card.CheckAdjacent,tp,LOCATION_MMZONE,0,1,nil)
    let b2 = duel::is_existing_matching_card(d, Some(&|d, cc| card::check_adjacent(d, cc)), tp, LOCATION_MMZONE, 0, 1, NO_EX);
    // if chk==0 then return sp or mv end
    // (sp and mv are never assigned in the script: the Lua value is nil, which is false)
    if a.chk == Some(0) {
        return false;
    }
    // local op=Duel.SelectEffect(tp,
    //     {b1,aux.Stringid(id,3)},
    //     {b2,aux.Stringid(id,4)})
    let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 3)), (b2, aux::stringid(ID, 4))]);
    // if op==1 then
    if op == Some(1) {
        // e:SetCategory(CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_SPECIAL_SUMMON);
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    // elseif op==2 then
    } else if op == Some(2) {
        // e:SetCategory(0)
        effect::set_category(d, a.e, 0);
    }
    true
}
/// `s.effop(e,tp,eg,ep,ev,re,r,rp)`
fn effop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local op=e:GetLabel()
    let op = effect::get_label(d, a.e);
    // if op==1 then
    if op == 1 {
        // local c=e:GetHandler()
        let c = effect::get_handler(d, a.e).nil();
        // if not c:IsRelateToEffect(e) then return end
        if !card::is_relate_to_effect(d, c, a.e) {
            return;
        }
        // local zone=(1<<c:GetSequence())&ZONES_MMZ
        let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
        // if zone>0 then
        if zone > 0 {
            // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP,zone)
            duel::special_summon(d, c, 0, tp, tp, false, false, POS_FACEUP as u8, Some(zone));
        }
    // elseif op==2 then
    } else if op == 2 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SELECT)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SELECT as u64);
        // local tc=Duel.SelectMatchingCard(tp,Card.CheckAdjacent,tp,LOCATION_MMZONE,0,1,1,nil):GetFirst()
        let sg = duel::select_matching_card(d, tp, Some(&|d, cc| card::check_adjacent(d, cc)), tp, LOCATION_MMZONE, 0, 1, 1, false, NO_EX).nil();
        // if tc then
        if let Some(tc) = group::get_first(d, &sg) {
            // tc:MoveAdjacent()
            card::move_adjacent(d, tc, None);
        }
    }
}
/// `s.pltg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn pltg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsInMainMZone(1-tp) and chkc:IsFaceup() and chkc:IsType(TYPE_EFFECT) end
    if let Some(chkc) = a.chkc() {
        return card::is_in_main_m_zone(d, chkc, Some(1 - tp)) && card::is_faceup(d, chkc) && card::is_type(d, chkc, TYPE_EFFECT);
    }
    // if chk==0 then return Duel.IsExistingTarget(aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,0,LOCATION_MMZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&faceup_effect), tp, 0, LOCATION_MMZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // local tc=Duel.SelectTarget(tp,aux.FaceupFilter(Card.IsType,TYPE_EFFECT),tp,0,LOCATION_MMZONE,1,1,nil):GetFirst()
    let sg = duel::select_target(d, tp, Some(&faceup_effect), tp, 0, LOCATION_MMZONE, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sg).nil();
    // local dc=Duel.GetFieldCard(1-tp,LOCATION_SZONE,tc:GetSequence())
    let dc = duel::get_field_card(d, 1 - tp, LOCATION_SZONE, card::get_sequence(d, tc));
    // if dc then
    if let Some(dc) = dc {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dc,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(dc)), 1, 0, 0);
    }
    true
}
/// `aux.FaceupFilter(Card.IsType,TYPE_EFFECT)`
fn faceup_effect(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_type(d, c, TYPE_EFFECT)
}
/// `s.plop(e,tp,eg,ep,ev,re,r,rp)`
fn plop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not (tc:IsRelateToEffect(e) and tc:IsControler(1-tp)) or tc:IsImmuneToEffect(e) then return end
    if !(card::is_relate_to_effect(d, tc, a.e) && card::is_controler(d, tc, 1 - tp)) || card::is_immune_to_effect(d, tc, a.e) {
        return;
    }
    // local seq=tc:GetSequence()
    let seq = card::get_sequence(d, tc);
    // local dc=Duel.GetFieldCard(1-tp,LOCATION_SZONE,seq)
    let dc = duel::get_field_card(d, 1 - tp, LOCATION_SZONE, seq);
    // if dc and Duel.Destroy(dc,REASON_RULE)>0 and dc:IsMonsterCard() and dc:GetBaseAttack()>0 then
    if let Some(dc) = dc {
        if duel::destroy(d, dc, REASON_RULE, None, None) > 0 && card::is_monster_card(d, dc) && card::get_base_attack(d, dc) > 0 {
            // Duel.SetLP(1-tp,Duel.GetLP(1-tp)-dc:GetBaseAttack())
            let lp = duel::get_lp(d, 1 - tp);
            let base = card::get_base_attack(d, dc);
            duel::set_lp(d, 1 - tp, lp - base);
        }
    }
    // if Duel.CheckLocation(1-tp,LOCATION_SZONE,seq)
    //     and Duel.MoveToField(tc,tp,1-tp,LOCATION_SZONE,POS_FACEUP,tc:IsMonsterCard(),1<<seq) then
    if duel::check_location(d, 1 - tp, LOCATION_SZONE, seq) {
        let is_monster = card::is_monster_card(d, tc);
        if duel::move_to_field(d, tc, tp, 1 - tp, LOCATION_SZONE as u16, POS_FACEUP as u8, is_monster, Some(1 << seq)) {
            // Treated as a Continuous Spell
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
            // e1:SetCode(EFFECT_CHANGE_TYPE)
            effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
            // e1:SetValue(TYPE_SPELL|TYPE_CONTINUOUS)
            effect::set_value(d, e1, (TYPE_SPELL | TYPE_CONTINUOUS) as i64);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
            effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
/// `s.pencon(e,tp,eg,ep,ev,re,r,rp)`
fn pencon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(tp) and c:IsSpecialSummoned()
    //     and c:IsReason(REASON_EFFECT) and c:IsReasonPlayer(1-tp)
    card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, a.tp)
        && card::is_special_summoned(d, c)
        && card::is_reason(d, c, REASON_EFFECT)
        && card::is_reason_player(d, c, 1 - a.tp)
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
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.MoveToField(c,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, c, a.tp, a.tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    }
}
