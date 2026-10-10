//! Magical Trick Mirror
use super::prelude::*;

pub const ID: u32 = 85758066;

pub const INFO: ScriptInfo = ScriptInfo {
    listed_names: &[CARD_MONSTER_REBORN, CARD_OBELISK],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Set
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_LEAVE_GRAVE+CATEGORY_SET)
    effect::set_category(d, e1, CATEGORY_LEAVE_GRAVE + CATEGORY_SET);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EVENT_ATTACK_ANNOUNCE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCondition(s.setcon)
    effect::set_condition(d, e1, Some(condition(setcon)));
    // e1:SetTarget(s.settg)
    effect::set_target(d, e1, Some(cost_target(settg)));
    // e1:SetOperation(s.setop)
    effect::set_operation(d, e1, Some(operation(setop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Special Summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_GRAVE)
    effect::set_range(d, e2, LOCATION_GRAVE);
    // e2:SetCost(s.spcost)
    effect::set_cost(d, e2, Some(cost_target(spcost)));
    // e2:SetTarget(s.sptg)
    effect::set_target(d, e2, Some(cost_target(sptg)));
    // e2:SetOperation(s.spop)
    effect::set_operation(d, e2, Some(operation(spop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.setcon(e,tp,eg,ep,ev,re,r,rp)`
fn setcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local at=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // return at:IsControler(1-tp)
    card::is_controler(d, at, 1 - a.tp)
}
/// `s.setfilter(c,ft)`
fn setfilter(d: &mut Duel, c: CardId, ft: i32) -> bool {
    // return c:IsSpell() and c:IsSSetable(true) and (c:IsType(TYPE_FIELD) or ft>0)
    card::is_spell(d, c) && card::is_s_setable(d, c, true, None) && (card::is_type(d, c, TYPE_FIELD) || ft > 0)
}
/// `s.settg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn settg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
    let h = effect::get_handler(d, e).nil();
    if card::is_location(d, h, LOCATION_HAND) {
        ft -= 1;
    }
    // if chkc then return chkc:IsControler(1-tp) and chkc:IsLocation(LOCATION_GRAVE) and s.setfilter(chkc,ft) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, 1 - tp) && card::is_location(d, chkc, LOCATION_GRAVE) && setfilter(d, chkc, ft);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.setfilter,tp,0,LOCATION_GRAVE,1,nil,ft) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| setfilter(d, c, ft)), tp, 0, LOCATION_GRAVE, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SET as u64);
    // local g=Duel.SelectTarget(tp,s.setfilter,tp,0,LOCATION_GRAVE,1,1,nil,ft)
    let g = duel::select_target(d, tp, Some(&|d, c| setfilter(d, c, ft)), tp, 0, LOCATION_GRAVE, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, Some(g), 1, 0, 0);
    true
}
/// `s.setop(e,tp,eg,ep,ev,re,r,rp)`
fn setop(d: &mut Duel, a: &EffArgs) {
    let (e, tp) = (a.e, a.tp);
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // if tc and tc:IsRelateToEffect(e) and s.setfilter(tc,ft) then
    if let Some(tc) = tc {
        if card::is_relate_to_effect(d, tc, e) && setfilter(d, tc, ft) {
            // Duel.SSet(tp,tc)
            duel::s_set(d, tp, tc, None, true);
        }
    }
}
/// `s.spcfilter(c,tp)`
fn spcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsCode(CARD_MONSTER_REBORN) and c:IsAbleToGraveAsCost() and (c:IsLocation(LOCATION_HAND) or c:IsFacedown())
    //     and Duel.GetMZoneCount(tp,c)>0
    card::is_code(d, c, &[CARD_MONSTER_REBORN])
        && card::is_able_to_grave_as_cost(d, c)
        && (card::is_location(d, c, LOCATION_HAND) || card::is_facedown(d, c))
        && duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
/// `aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)`: `Cost.SelfBanish` with the arguments of the cost
fn bfgcost(d: &mut Duel, a: &EffArgs, chk: i64) -> bool {
    let args = [
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
        Value::Int(chk),
    ];
    aux::bfgcost().call(d, &args).truthy()
}
/// `s.spcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn spcost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)
    //     and Duel.IsExistingMatchingCard(s.spcfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,nil,tp) end
    if a.chk == Some(0) {
        return bfgcost(d, a, 0) && duel::is_existing_matching_card(d, Some(&|d, c| spcfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, NO_EX);
    }
    // aux.bfgcost(e,tp,eg,ep,ev,re,r,rp,chk)
    bfgcost(d, a, a.chk.unwrap_or(0));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=Duel.SelectMatchingCard(tp,s.spcfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,0,1,1,nil,tp)
    let g = duel::select_matching_card(d, tp, Some(&|d, c| spcfilter(d, c, tp)), tp, LOCATION_HAND | LOCATION_ONFIELD, 0, 1, 1, false, NO_EX).nil();
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, &g, REASON_COST, None, None);
    true
}
/// `s.spfilter(c,e,tp)`
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsCode(CARD_OBELISK) and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP_DEFENSE)
    card::is_code(d, c, &[CARD_OBELISK]) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, Some(POS_FACEUP_DEFENSE as u8), None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // if chk==0 then return Duel.IsExistingMatchingCard(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| spfilter(d, c, e, tp)), tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,1,tp,LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 1, tp, LOCATION_GRAVE as i32);
    true
}
/// `s.effcon(e)`
fn effcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():IsControler(e:GetOwnerPlayer())
    let h = effect::get_handler(d, a.e).nil();
    card::is_controler(d, h, effect::get_owner_player(d, a.e))
}
/// `s.atklimit(e,c)`
fn atklimit(d: &mut Duel, a: Args) -> bool {
    // return c==e:GetHandler()
    let (e, c) = (a.e(0), a.c(1));
    let h = effect::get_handler(d, e).nil();
    c == h
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
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.spfilter),tp,LOCATION_GRAVE,0,1,1,nil,e,tp):GetFirst()
    let g = duel::select_matching_card(d, tp, Some(&aux::necro_valley_filter(|d, c| spfilter(d, c, e, tp))), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &g);
    // if tc and Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP_DEFENSE) and Duel.IsTurnPlayer(1-tp) then
    if let Some(tc) = tc {
        if duel::special_summon_step(d, tc, 0, tp, tp, false, false, POS_FACEUP_DEFENSE as u8, None) && duel::is_turn_player(d, 1 - tp) {
            // Must attack
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetType(EFFECT_TYPE_FIELD)
            effect::set_type(d, e1, EFFECT_TYPE_FIELD);
            // e1:SetCode(EFFECT_MUST_ATTACK)
            effect::set_code(d, e1, EFFECT_MUST_ATTACK);
            // e1:SetRange(LOCATION_MZONE)
            effect::set_range(d, e1, LOCATION_MZONE);
            // e1:SetTargetRange(0,LOCATION_MZONE)
            effect::set_target_range(d, e1, 0, LOCATION_MZONE);
            // e1:SetCondition(s.effcon)
            effect::set_condition(d, e1, Some(condition(effcon)));
            // e1:SetOwnerPlayer(tp)
            effect::set_owner_player(d, e1, tp);
            // e1:SetReset(RESETS_STANDARD_PHASE_END)
            effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
            // local e2=e1:Clone()
            let e2 = effect::clone(d, e1);
            // e2:SetCode(EFFECT_MUST_ATTACK_MONSTER)
            effect::set_code(d, e2, EFFECT_MUST_ATTACK_MONSTER);
            // e2:SetValue(s.atklimit)
            effect::set_value_fn(d, e2, lua_fn(atklimit));
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc, e2, false);
        }
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
