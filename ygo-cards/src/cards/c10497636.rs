//! War Rock Meteoragon
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 10497636;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_WAR_ROCK], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Cannot be destroyed by oppos card effects
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_INDESTRUCTABLE_EFFECT)
    effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_EFFECT);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetValue(aux.indoval)
    effect::set_value_fn(d, e1, aux::indoval());
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Negate Effects
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.negtg)
    effect::set_target(d, e2, Some(cost_target(negtg)));
    // e2:SetOperation(s.negop)
    effect::set_operation(d, e2, Some(operation(negop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // If your EARTH Warrior monster battles
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e3, CATEGORY_ATKCHANGE);
    // e3:SetProperty(EFFECT_FLAG_DAMAGE_STEP)
    effect::set_property(d, e3, EFFECT_FLAG_DAMAGE_STEP, 0);
    // e3:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e3, EFFECT_TYPE_QUICK_O);
    // e3:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e3, EVENT_FREE_CHAIN);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // local timing=TIMING_BATTLE_PHASE|TIMING_BATTLE_END|TIMING_DAMAGE_STEP|TIMING_ATTACK|TIMING_BATTLE_START
    let timing = (TIMING_BATTLE_PHASE | TIMING_BATTLE_END | TIMING_DAMAGE_STEP | TIMING_ATTACK | TIMING_BATTLE_START) as u32;
    // e3:SetHintTiming(timing,timing)
    effect::set_hint_timing(d, e3, timing, Some(timing));
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(condition(atkcon)));
    // e3:SetTarget(s.atktg)
    effect::set_target(d, e3, Some(cost_target(atktg)));
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Condition check for the Quick Effect
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_BATTLED)
        effect::set_code(d, ge1, EVENT_BATTLED);
        // ge1:SetOperation(s.checkop)
        effect::set_operation(d, ge1, Some(operation(checkop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
/// `s.checkfilter(c)`
fn checkfilter(d: &mut Duel, c: Option<CardId>) -> bool {
    // return c and c:IsFaceup() and c:IsAttribute(ATTRIBUTE_EARTH) and c:IsRace(RACE_WARRIOR)
    match c {
        Some(c) => card::is_faceup(d, c) && card::is_attribute(d, c, ATTRIBUTE_EARTH) && card::is_race(d, c, RACE_WARRIOR as u64),
        None => false,
    }
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, _a: &EffArgs) {
    // if not Duel.IsDamageCalculated() then return end
    if !duel::is_damage_calculated(d) {
        return;
    }
    // local bc0,bc1=Duel.GetBattleMonster(0)
    let (bc0, bc1) = duel::get_battle_monster(d, 0);
    // if s.checkfilter(bc0) then
    if checkfilter(d, bc0) {
        // Duel.RegisterFlagEffect(bc0:GetControler(),id,RESET_PHASE|PHASE_END,0,1)
        let p = card::get_controler(d, bc0.nil());
        duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
    // if s.checkfilter(bc1) then
    if checkfilter(d, bc1) {
        // Duel.RegisterFlagEffect(bc1:GetControler(),id,RESET_PHASE|PHASE_END,0,1)
        let p = card::get_controler(d, bc1.nil());
        duel::register_flag_effect(d, p, ID, RESET_PHASE | PHASE_END, 0, 1, 0);
    }
}
/// `s.negtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn negtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttackTarget()
    let at = duel::get_attack_target(d);
    let h = effect::get_handler(d, a.e);
    // if chk==0 then return ((at and at:IsFaceup() and Duel.GetAttacker()==e:GetHandler()) or at==e:GetHandler())
    //     and not e:GetHandler():IsStatus(STATUS_CHAINING) end
    if a.chk == Some(0) {
        let attacker_is_h = duel::get_attacker(d) == h;
        let first = match at {
            Some(at) => card::is_faceup(d, at) && attacker_is_h,
            None => false,
        };
        let cond = first || at == h;
        let hh = h.nil();
        return cond && !card::is_status(d, hh, STATUS_CHAINING);
    }
    // Duel.SetTargetCard(e:GetHandler():GetBattleTarget())
    let hh = h.nil();
    match card::get_battle_target(d, hh) {
        Some(bt) => duel::set_target_card(d, bt),
        None => duel::set_target_card(d, CardSet::new()),
    }
    true
}
/// `s.negop(e,tp,eg,ep,ev,re,r,rp)`
fn negop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local code=tc:GetOriginalCode()
    let _code = card::get_original_code(d, tc);
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) && !card::is_immune_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetCode(EFFECT_DISABLE)
        effect::set_code(d, e1, EFFECT_DISABLE);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
        // local e2=e1:Clone()
        let e2 = effect::clone(d, e1);
        // e2:SetCode(EFFECT_DISABLE_EFFECT)
        effect::set_code(d, e2, EFFECT_DISABLE_EFFECT);
        // e2:SetValue(RESET_TURN_SET)
        effect::set_value(d, e2, RESET_TURN_SET as i64);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
        // Duel.AdjustInstantly(c)
        duel::adjust_instantly(d, Some(c));
        // if not tc:IsImmuneToEffect(e) then
        if !card::is_immune_to_effect(d, tc, a.e) {
            // local e3=Effect.CreateEffect(c)
            let e3 = effect::create_effect(d, c);
            // e3:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e3, EFFECT_TYPE_FIELD);
            // e3:SetCode(EFFECT_DISABLE)
            effect::set_code(d, e3, EFFECT_DISABLE);
            // e3:SetTargetRange(LOCATION_MZONE,LOCATION_MZONE)
            effect::set_target_range(d, e3, LOCATION_MZONE, LOCATION_MZONE);
            // e3:SetTarget(s.distg)
            effect::set_target(d, e3, Some(effect_card_fn(distg)));
            // e3:SetLabel(tc:GetOriginalCodeRule())
            let label = card::get_original_code_rule(d, tc).first().copied().unwrap_or(0);
            effect::set_label(d, e3, &[label as i64]);
            // e3:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e3, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e3,tp)
            duel::register_effect(d, e3, a.tp);
            // local e4=Effect.CreateEffect(c)
            let e4 = effect::create_effect(d, c);
            // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
            effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
            // e4:SetCode(EVENT_CHAIN_SOLVING)
            effect::set_code(d, e4, EVENT_CHAIN_SOLVING);
            // e4:SetCondition(s.discon)
            effect::set_condition(d, e4, Some(condition(discon)));
            // e4:SetOperation(s.disop)
            effect::set_operation(d, e4, Some(operation(disop)));
            // e4:SetLabel(tc:GetOriginalCodeRule())
            let label = card::get_original_code_rule(d, tc).first().copied().unwrap_or(0);
            effect::set_label(d, e4, &[label as i64]);
            // e4:SetReset(RESET_PHASE|PHASE_END)
            effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
            // Duel.RegisterEffect(e4,tp)
            duel::register_effect(d, e4, a.tp);
        }
    }
}
/// `s.distg(e,c)`
fn distg(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // local code=e:GetLabel()
    let code = effect::get_label(d, e);
    // local code1,code2=c:GetOriginalCodeRule()
    let codes = card::get_original_code_rule(d, c);
    // return code1==code or code2==code
    codes.first().is_some_and(|x| *x as i64 == code) || codes.get(1).is_some_and(|x| *x as i64 == code)
}
/// `s.discon(e,tp,eg,ep,ev,re,r,rp)`
fn discon(d: &mut Duel, a: &EffArgs) -> bool {
    // local code=e:GetLabel()
    let code = effect::get_label(d, a.e);
    // local code1,code2=re:GetHandler():GetOriginalCodeRule()
    let re = a.re.nil();
    let rh = effect::get_handler(d, re).nil();
    let codes = card::get_original_code_rule(d, rh);
    // return re:IsMonsterEffect() and (code1==code or code2==code)
    effect::is_monster_effect(d, re)
        && (codes.first().is_some_and(|x| *x as i64 == code) || codes.get(1).is_some_and(|x| *x as i64 == code))
}
/// `s.disop(e,tp,eg,ep,ev,re,r,rp)`
fn disop(d: &mut Duel, a: &EffArgs) {
    // Duel.Hint(HINT_CARD,0,id)
    duel::hint(d, HINT_CARD as u8, 0, ID as u64);
    // Duel.NegateEffect(ev)
    duel::negate_effect(d, a.ev as u8);
}
/// `s.atkcon(e,tp,eg,ep,ev,re,r,rp)`
fn atkcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsBattlePhase() and Duel.GetFlagEffect(tp,id)>0 and aux.StatChangeDamageStepCondition()
    duel::is_battle_phase(d, None) && duel::get_flag_effect(d, a.tp, ID) > 0 && aux::stat_change_damage_step_condition(d)
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.FaceupFilter(Card.IsSetCard,SET_WAR_ROCK),tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_WAR_ROCK as u16]));
        return duel::is_existing_matching_card(d, Some(&f), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Can make 2 attacks on monsters
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
        effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
        // e1:SetRange(LOCATION_MZONE)
        effect::set_range(d, e1, LOCATION_MZONE);
        // e1:SetCode(EFFECT_EXTRA_ATTACK_MONSTER)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK_MONSTER);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // c:RegisterEffect(e1,true)
        card::register_effect(d, c, e1, true);
    }
    // local atkg=Duel.GetMatchingGroup(aux.FaceupFilter(Card.IsSetCard,SET_WAR_ROCK),tp,LOCATION_MZONE,0,nil)
    let f = aux::faceup_filter(|d, c| card::is_set_card(d, c, &[SET_WAR_ROCK as u16]));
    let atkg = duel::get_matching_group(d, Some(&f), a.tp, LOCATION_MZONE, 0, NO_EX);
    // for tc in atkg:Iter() do
    for tc in group::iter(d, &atkg) {
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
        // e2:SetCode(EFFECT_UPDATE_ATTACK)
        effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
        // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e2:SetReset(RESETS_STANDARD_PHASE_END|RESET_OPPO_TURN)
        effect::set_reset(d, e2, RESETS_STANDARD_PHASE_END | RESET_OPPO_TURN, 1);
        // e2:SetValue(200)
        effect::set_value(d, e2, 200);
        // tc:RegisterEffect(e2)
        card::register_effect(d, tc, e2, false);
    }
}
