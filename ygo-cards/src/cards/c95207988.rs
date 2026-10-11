//! Earthbound Resonance
use super::prelude::*;

pub const ID: u32 = 95207988;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_EARTHBOUND], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Inflict damage equal to half of the ATK of the monster destroyed
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_CUSTOM+id)
    effect::set_code(d, e1, EVENT_CUSTOM + ID);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.damtg)
    effect::set_target(d, e1, Some(cost_target(damtg)));
    // e1:SetOperation(s.damop)
    effect::set_operation(d, e1, Some(operation(damop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Keep track of the destroyed monsters
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // s.desgroup=Group.CreateGroup()
        set_s_var(d, ID, 0, Value::Group(CardSet::new()));
        // s.desgroup:KeepAlive()
        // local ge1=Effect.CreateEffect(c)
        let ge1 = effect::create_effect(d, c);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // ge1:SetCode(EVENT_DESTROYED)
        effect::set_code(d, ge1, EVENT_DESTROYED);
        // ge1:SetOperation(s.desgroupregop)
        effect::set_operation(d, ge1, Some(operation(desgroupregop)));
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
    // Destroy 1 card on the field
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
    // e2:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e2, EVENT_BE_BATTLE_TARGET);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2, 1, ID, 1, 0);
    // e2:SetCondition(s.descond)
    effect::set_condition(d, e2, Some(condition(descond)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.desgroup` (the script variable, kept in the duel)
fn desgroup(d: &Duel) -> CardSet {
    match s_var(d, ID, 0) {
        Value::Group(g) => g,
        _ => CardSet::new(),
    }
}
fn cfilter(d: &mut Duel, c: CardId, e: Option<EffectId>) -> bool {
    // return c:IsReason(REASON_EFFECT) and c:IsPreviousLocation(LOCATION_MZONE) and c:IsSummonLocation(LOCATION_EXTRA)
    //   and c:GetAttack()>0 and (not e or c:IsCanBeEffectTarget(e))
    card::is_reason(d, c, REASON_EFFECT)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_summon_location(d, c, LOCATION_EXTRA)
        && card::get_attack(d, c) > 0
        && e.map_or(true, |e| card::is_can_be_effect_target(d, c, Some(e)))
}
fn desgroupregop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=eg:Filter(s.cfilter,nil)
    let eg = a.eg();
    let tg = group::filter(d, &eg, &|d, c| cfilter(d, c, None), NO_EX);
    // if #tg>0 then
    if group::get_count(&tg) > 0 {
        // for tc in tg:Iter() do
        //   tc:RegisterFlagEffect(id,RESET_CHAIN,0,1)
        // end
        for tc in group::iter(d, &tg) {
            card::register_flag_effect(d, tc, ID, RESET_CHAIN, 0, 1, 0, 0);
        }
        let mut dg = desgroup(d);
        // if Duel.GetCurrentChain()==0 then s.desgroup:Clear() end
        if duel::get_current_chain(d, false) == 0 {
            dg.clear();
        }
        // s.desgroup:Merge(tg)
        group::add_card(&mut dg, &tg);
        // s.desgroup:Remove(function(c) return c:GetFlagEffect(id)==0 end,nil)
        group::remove(d, &mut dg, &|d, c| card::get_flag_effect(d, c, ID) == 0, NO_EX);
        set_s_var(d, ID, 0, Value::Group(dg.clone()));
        // Duel.RaiseEvent(s.desgroup,EVENT_CUSTOM+id,e,0,tp,tp,0)
        duel::raise_event(d, dg, EVENT_CUSTOM + ID, Some(a.e), 0, tp, tp, 0);
    }
}
fn damtg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=s.desgroup:Filter(s.cfilter,nil,e)
    let dg = desgroup(d);
    let g = group::filter(d, &dg, &|d, c| cfilter(d, c, Some(a.e)), NO_EX);
    // if chkc then return g:IsContains(chkc) and s.cfilter(chkc,e) end
    if let Some(chkc) = a.chkc() {
        return group::is_contains(&g, chkc) && cfilter(d, chkc, Some(a.e));
    }
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return group::get_count(&g) > 0;
    }
    // local tc=nil
    // if #g==1 then
    let tc = if group::get_count(&g) == 1 {
        // tc=g:GetFirst()
        // Duel.SetTargetCard(tc)
        let tc = group::get_first(d, &g).nil();
        duel::set_target_card(d, tc);
        tc
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
        // tc=g:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel).nil();
        // Duel.SetTargetCard(tc)
        duel::set_target_card(d, tc);
        tc
    };
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,tc,1,0,tc:GetAttack())
    let atk = card::get_attack(d, tc);
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, Some(CardSet::from_card(tc)), 1, 0, atk);
    true
}
fn damop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // local val=tc:GetAttack()//2
    let val = card::get_attack(d, tc).div_euclid(2);
    // if tc:IsRelateToEffect(e) and val>0 then
    if card::is_relate_to_effect(d, tc, a.e) && val > 0 {
        // Duel.Damage(tp,val,REASON_EFFECT,true)
        duel::damage(d, a.tp, val as i64, REASON_EFFECT, true, None);
        // Duel.Damage(1-tp,val,REASON_EFFECT,true)
        duel::damage(d, 1 - a.tp, val as i64, REASON_EFFECT, true, None);
        // Duel.RDComplete()
        duel::rd_complete(d);
    }
}
fn descond(d: &mut Duel, a: &EffArgs) -> bool {
    // local d=Duel.GetAttackTarget()
    // return d and d:IsControler(tp) and d:IsFaceup() and d:IsAttribute(ATTRIBUTE_DARK) and d:IsType(TYPE_SYNCHRO)
    //   and d:IsSummonLocation(LOCATION_EXTRA)
    let Some(at) = duel::get_attack_target(d) else { return false };
    card::is_controler(d, at, a.tp)
        && card::is_faceup(d, at)
        && card::is_attribute(d, at, ATTRIBUTE_DARK)
        && card::is_type(d, at, TYPE_SYNCHRO)
        && card::is_summon_location(d, at, LOCATION_EXTRA)
}
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_ONFIELD) end
    // (not run: `chkc` is an undefined global in the Lua script, always nil)
    // if chk==0 then return Duel.IsExistingTarget(nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESTROY as u64);
    // local g=Duel.SelectTarget(tp,nil,tp,LOCATION_ONFIELD,LOCATION_ONFIELD,1,1,nil)
    let g = duel::select_target(d, a.tp, None, a.tp, LOCATION_ONFIELD, LOCATION_ONFIELD, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, 0, 0);
    true
}
fn desop(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // Duel.Destroy(tc,REASON_EFFECT)
        duel::destroy(d, tc, REASON_EFFECT, None, None);
    }
}
