//! Vaylantz Buster Baron
use super::prelude::*;

pub const ID: u32 = 14418464;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_series: &[SET_VAYLANTZ],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Pendulum.AddProcedure(c)
    pendulum::add_procedure(d, c, None, None);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_PZONE)
    effect::set_range(d, e1, LOCATION_PZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(function(e) return e:GetHandler():IsSpecialSummoned() end)
    effect::set_condition(d, e2, Some(condition(e2_cond)));
    // e2:SetTarget(s.mmvtg)
    effect::set_target(d, e2, Some(cost_target(mmvtg)));
    // e2:SetOperation(s.mmvop)
    effect::set_operation(d, e2, Some(operation(mmvop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e3, aux::stringid(ID, 2));
    // e3:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e3:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DELAY)
    effect::set_property(d, e3, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DELAY, 0);
    // e3:SetCode(EVENT_MOVE)
    effect::set_code(d, e3, EVENT_MOVE);
    // e3:SetCountLimit(1,{id,2})
    effect::set_count_limit(d, e3, 1, ID, 2, 0);
    // e3:SetCondition(s.smvcon)
    effect::set_condition(d, e3, Some(condition(smvcon)));
    // e3:SetTarget(s.smvtg)
    effect::set_target(d, e3, Some(cost_target(smvtg)));
    // e3:SetOperation(s.smvop)
    effect::set_operation(d, e3, Some(operation(smvop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `function(e) return e:GetHandler():IsSpecialSummoned() end`
fn e2_cond(d: &mut Duel, a: &EffArgs) -> bool {
    let h = effect::get_handler(d, a.e).nil();
    card::is_special_summoned(d, h)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then
    if a.chk == Some(0) {
        // local zone=(1<<c:GetSequence())&ZONES_MMZ
        let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
        // return zone~=0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,tp,zone)
        return zone != 0 && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, Some(POS_FACEUP as u8), Some(a.tp), Some(zone));
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(c.into()), 1, 0, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if not c:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, c, a.e) {
        return;
    }
    // local zone=(1<<c:GetSequence())&ZONES_MMZ
    let zone = (1u32 << card::get_sequence(d, c)) & ZONES_MMZ;
    // if zone~=0 then
    if zone != 0 {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP,zone)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, Some(zone));
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,3))
    effect::set_description(d, e1, aux::stringid(ID, 3));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e1, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e1:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e1, EFFECT_FLAG_PLAYER_TARGET + EFFECT_FLAG_CLIENT_HINT, 0);
    // e1:SetTargetRange(1,0)
    effect::set_target_range(d, e1, 1, 0);
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // e1:SetTarget(s.splimit)
    effect::set_target(d, e1, Some(effect_card_fn(splimit)));
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, a.tp);
}
/// `s.splimit(e,c)`
fn splimit(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return not c:IsSetCard(SET_VAYLANTZ) and not c:IsLocation(LOCATION_EXTRA)
    !card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && !card::is_location(d, c, LOCATION_EXTRA)
}
/// `s.mmvfilter(c)`
fn mmvfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_VAYLANTZ) and c:CheckAdjacent()
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_VAYLANTZ as u16]) && card::check_adjacent(d, c)
}
/// `s.mmvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn mmvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chkc then return c~=chkc and chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.mmvfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return c != chkc && card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, a.tp) && mmvfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.mmvfilter,tp,LOCATION_MZONE,0,1,c) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&mmvfilter), a.tp, LOCATION_MZONE, 0, 1, c.into());
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.mmvfilter,tp,LOCATION_MZONE,0,1,1,c)
    duel::select_target(d, a.tp, Some(&mmvfilter), a.tp, LOCATION_MZONE, 0, 1, 1, false, c.into());
    true
}
/// `s.mmvop(e,tp,eg,ep,ev,re,r,rp)`
fn mmvop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and not tc:IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) && !card::is_immune_to_effect(d, tc, a.e) {
        // tc:MoveAdjacent()
        card::move_adjacent(d, tc, None);
    }
}
/// `s.smvcon(e,tp,eg,ep,ev,re,r,rp)`
fn smvcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // return c:IsLocation(LOCATION_MZONE) and c:IsPreviousLocation(LOCATION_MZONE)
    card::is_location(d, c, LOCATION_MZONE) && card::is_previous_location(d, c, LOCATION_MZONE)
}
/// `s.smvfilter(c,tp)`
fn smvfilter(d: &mut Duel, c: CardId, tp: Option<u8>) -> bool {
    // local z=1<<c:GetSequence()
    let z = 1u32 << card::get_sequence(d, c);
    // return Duel.GetLocationCount(tp,LOCATION_SZONE,tp,LOCATION_REASON_TOFIELD,(z<<1|z>>1)&ZONES_MMZ)>0
    // (Lua `tp` nil in the chkc call: the player reads as 0 and the reason player as the default)
    duel::get_location_count(d, tp.unwrap_or(0), LOCATION_SZONE as u8, tp, Some(LOCATION_REASON_TOFIELD), Some(((z << 1) | (z >> 1)) & ZONES_MMZ)).0 > 0
}
/// `s.smvtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn smvtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_PZONE) and s.smvfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_PZONE) && smvfilter(d, chkc, None);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.smvfilter,tp,LOCATION_PZONE,LOCATION_PZONE,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| smvfilter(d, c, Some(a.tp))), a.tp, LOCATION_PZONE, LOCATION_PZONE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.smvfilter,tp,LOCATION_PZONE,LOCATION_PZONE,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, c| smvfilter(d, c, Some(a.tp))), a.tp, LOCATION_PZONE, LOCATION_PZONE, 1, 1, false, NO_EX);
    true
}
/// `s.smvop(e,tp,eg,ep,ev,re,r,rp)`
fn smvop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if not tc:IsRelateToEffect(e) then return end
    if !card::is_relate_to_effect(d, tc, a.e) {
        return;
    }
    // local seq=tc:GetSequence()
    let seq = card::get_sequence(d, tc);
    // if seq>4 then return end
    if seq > 4 {
        return;
    }
    // local flag=0
    let mut flag: u32 = 0;
    // if seq>0 and Duel.CheckLocation(tp,LOCATION_SZONE,seq-1) then flag=flag|(0x1<<seq-1) end
    if seq > 0 && duel::check_location(d, a.tp, LOCATION_SZONE, seq - 1) {
        flag |= 0x1 << (seq - 1);
    }
    // if seq<4 and Duel.CheckLocation(tp,LOCATION_SZONE,seq+1) then flag=flag|(0x1<<seq+1) end
    if seq < 4 && duel::check_location(d, a.tp, LOCATION_SZONE, seq + 1) {
        flag |= 0x1 << (seq + 1);
    }
    // if flag==0 then return end
    if flag == 0 {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOZONE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TOZONE as u64);
    // local zone=math.log(Duel.SelectDisableField(tp,1,LOCATION_SZONE,0,~(flag<<8)),2)-8
    let sel = duel::select_disable_field(d, a.tp, 1, LOCATION_SZONE, 0, Some(!(flag << 8)), false);
    // (Duel.SelectDisableField returns 0 where the Lua gives nil; math.log(nil,2) is an error)
    if sel == 0 {
        crate::lua_error("bad argument #1 to 'log' (number expected, got nil)");
    }
    let zone = sel.trailing_zeros() as i64 - 8;
    // if Duel.MoveSequence(tc,zone,LOCATION_SZONE) then
    if duel::move_sequence(d, tc, zone as u8, Some(LOCATION_SZONE)) {
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetCode(EFFECT_CHANGE_TYPE)
        effect::set_code(d, e1, EFFECT_CHANGE_TYPE);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD&~RESET_TURN_SET)
        effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD & !RESET_TURN_SET), 1);
        // e1:SetValue(TYPE_SPELL+TYPE_CONTINUOUS)
        effect::set_value(d, e1, (TYPE_SPELL + TYPE_CONTINUOUS) as i64);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
}
