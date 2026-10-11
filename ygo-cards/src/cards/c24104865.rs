//! Alien Mother
use super::prelude::*;

pub const ID: u32 = 24104865;
pub const INFO: ScriptInfo = ScriptInfo { counter_list: &[COUNTER_A], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // check
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EVENT_BATTLED)
    effect::set_code(d, e1, EVENT_BATTLED);
    // e1:SetOperation(s.checkop)
    effect::set_operation(d, e1, Some(operation(checkop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local g=Group.CreateGroup()
    // e1:SetLabelObject(g)
    effect::set_label_object(d, e1, Value::Group(CardSet::new()));
    // g:KeepAlive()  (groups are values: nothing to keep)
    // special summon
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e2, EVENT_BATTLE_DESTROYING);
    // e2:SetOperation(s.checkop2)
    effect::set_operation(d, e2, Some(operation(checkop2)));
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e3, CATEGORY_SPECIAL_SUMMON);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_F);
    // e3:SetCode(EVENT_PHASE|PHASE_BATTLE)
    effect::set_code(d, e3, EVENT_PHASE | PHASE_BATTLE);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCondition(s.spcon)
    effect::set_condition(d, e3, Some(condition(spcon)));
    // e3:SetTarget(s.sptg)
    effect::set_target(d, e3, Some(cost_target(sptg)));
    // e3:SetOperation(s.spop)
    effect::set_operation(d, e3, Some(operation(spop)));
    // e3:SetLabelObject(e2)
    effect::set_label_object(d, e3, Value::Effect(e2));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // destroy
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_CONTINUOUS);
    // e4:SetCode(EVENT_LEAVE_FIELD)
    effect::set_code(d, e4, EVENT_LEAVE_FIELD);
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.checkop(e,tp,eg,ep,ev,re,r,rp)`
fn checkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local t=Duel.GetAttackTarget()
    let t = duel::get_attack_target(d);
    // if t and t~=c and t:GetCounter(COUNTER_A)>0 then
    let ok = match t {
        Some(t) => t != c && card::get_counter(d, t, COUNTER_A as u16) > 0,
        None => false,
    };
    if ok {
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    } else {
        // else e:SetLabel(0) end
        effect::set_label(d, a.e, &[0]);
    }
}
/// `s.checkop2(e,tp,eg,ep,ev,re,r,rp)`
fn checkop2(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if e:GetLabelObject():GetLabel()==0 then return end
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    if effect::get_label(d, e1) == 0 {
        return;
    }
    // local t=c:GetBattleTarget()
    let t = card::get_battle_target(d, c);
    // local g=e:GetLabelObject():GetLabelObject()
    let mut g = effect::get_label_object(d, e1).as_group().cloned().nil();
    // if c:GetFieldID()~=e:GetLabel() then
    if card::get_field_id(d, c) as i64 != effect::get_label(d, a.e) {
        // g:Clear()
        g = CardSet::new();
        // e:SetLabel(c:GetFieldID())
        effect::set_label(d, a.e, &[card::get_field_id(d, c) as i64]);
    }
    // if aux.bdgcon(e,tp,eg,ep,ev,re,r,rp) then
    if aux::bdgcon().call(d, &eff_values(a)).truthy() {
        // g:AddCard(t)
        group::add_card(&mut g, &CardSet::from_card(t.nil()));
        // t:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_BATTLE,0,1)
        card::register_flag_effect(d, t.nil(), ID, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_BATTLE, 0, 1, 0, 0);
    }
    // store the group back (it is the label object of e1)
    effect::set_label_object(d, e1, Value::Group(g));
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetFieldID()==e:GetLabelObject():GetLabel()
    let c = effect::get_handler(d, a.e).nil();
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    card::get_field_id(d, c) as i64 == effect::get_label(d, e2)
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:GetFlagEffect(id)~=0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::get_flag_effect(d, c, ID) != 0 && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local g=e:GetLabelObject():GetLabelObject():GetLabelObject()
    let e2 = effect::get_label_object(d, a.e).as_effect().nil();
    let e1 = effect::get_label_object(d, e2).as_effect().nil();
    let mut g = effect::get_label_object(d, e1).as_group().cloned().nil();
    // if chk==0 then return g:IsExists(s.filter,1,nil,e,tp) end
    if a.chk == Some(0) {
        return group::is_exists(d, &g, &|d, c| filter(d, c, a.e, a.tp), 1, NO_EX);
    }
    // local dg=g:Filter(s.filter,nil,e,tp)
    let dg = group::filter(d, &g, &|d, c| filter(d, c, a.e, a.tp), NO_EX);
    // g:Clear()
    g = CardSet::new();
    effect::set_label_object(d, e1, Value::Group(g));
    // Duel.SetTargetCard(dg)
    duel::set_target_card(d, &dg);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,dg,#dg,0,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(dg.clone()), dg.len() as u8, 0, 0);
    true
}
/// `s.sfilter(c,e,tp)`
fn sfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsRelateToEffect(e) and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_relate_to_effect(d, c, e) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if ft<=0 then return end
    if ft <= 0 {
        return;
    }
    // if Duel.IsPlayerAffectedByEffect(tp,CARD_BLUEEYES_SPIRIT) then ft=1 end
    if duel::is_player_affected_by_effect(d, a.tp, CARD_BLUEEYES_SPIRIT).is_some() {
        ft = 1;
    }
    // local g=Duel.GetChainInfo(0,CHAININFO_TARGET_CARDS)
    let g = duel::get_chain_info(d, 0, &[CHAININFO_TARGET_CARDS]).first().and_then(|v| v.as_group().cloned()).nil();
    // local sg=g:Filter(s.sfilter,nil,e,tp)
    let mut sg = group::filter(d, &g, &|d, c| sfilter(d, c, a.e, a.tp), NO_EX);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_SPSUMMON as u64);
    // if #sg>ft then sg=sg:Select(tp,ft,ft,nil) end
    if sg.len() as i32 > ft {
        sg = group::select(d, &sg, a.tp, ft as u16, ft as u16, false, NO_EX).nil();
    }
    // local tc=sg:GetFirst()
    let _tc = group::get_first(d, &sg);
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // for tc in aux.Next(sg) do
    for tc in group::iter(d, &sg) {
        // Duel.SpecialSummonStep(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon_step(d, tc, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
        // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
        card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
        // c:CreateRelation(tc,RESET_EVENT|RESET_TURN_SET|RESET_TOFIELD)
        card::create_relation(d, c, tc, RESET_EVENT | RESET_TURN_SET | RESET_TOFIELD);
    }
    // Duel.SpecialSummonComplete()
    duel::special_summon_complete(d);
}
/// `s.desfilter(c,rc)`
fn desfilter(d: &mut Duel, c: CardId, rc: CardId) -> bool {
    // return c:GetFlagEffect(id)~=0 and rc:IsRelateToCard(c)
    card::get_flag_effect(d, c, ID) != 0 && card::is_relate_to_card(d, rc, c)
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local dg=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_MZONE,0,nil,e:GetHandler())
    let rc = effect::get_handler(d, a.e).nil();
    let dg = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, rc)), a.tp, LOCATION_MZONE, 0, NO_EX);
    // Duel.Destroy(dg,REASON_EFFECT)
    duel::destroy(d, dg, REASON_EFFECT, None, None);
}
/// `e,tp,eg,ep,ev,re,r,rp` as the core passes them to an operation or condition
fn eff_values(a: &EffArgs) -> Vec<Value> {
    vec![
        Value::Effect(a.e),
        Value::Int(a.tp as i64),
        a.eg.clone().map_or(Value::Nil, Value::Group),
        Value::Int(a.ep as i64),
        Value::Int(a.ev as i64),
        Value::effect(a.re),
        Value::Int(a.r as i64),
        Value::Int(a.rp as i64),
    ]
}
