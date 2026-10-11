//! Tainted of the Tistina
use super::prelude::*;

pub const ID: u32 = 50281477;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TISTINA], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Tribute Summon face-up by tributing 1 monster the opponent controls
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e0, aux::stringid(ID, 0));
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_SUMMON_PROC)
    effect::set_code(d, e0, EFFECT_SUMMON_PROC);
    // e0:SetCondition(s.sumcon)
    effect::set_condition(d, e0, Some(lua_fn(sumcon)));
    // e0:SetTarget(s.sumtg)
    effect::set_target(d, e0, Some(lua_fn(sumtg)));
    // e0:SetOperation(s.sumop)
    effect::set_operation(d, e0, Some(lua_fn(sumop)));
    // e0:SetValue(SUMMON_TYPE_TRIBUTE)
    effect::set_value(d, e0, SUMMON_TYPE_TRIBUTE as i64);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Becomes Level 10
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_LEVEL)
    effect::set_code(d, e1, EFFECT_CHANGE_LEVEL);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(function(e) return e:GetHandler():IsNormalSummoned() end)
    effect::set_condition(d, e1, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        card::is_normal_summoned(d, h)
    })));
    // e1:SetValue(10)
    effect::set_value(d, e1, 10);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // "Tistina" monster can make a second attack
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2:SetProperty(EFFECT_FLAG_DELAY+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_DELAY + EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetCondition(function(e) return Duel.IsAbleToEnterBP() and not e:GetHandler():IsPreviousLocation(LOCATION_ONFIELD) end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let h = effect::get_handler(d, a.e).nil();
        duel::is_able_to_enter_bp(d) && !card::is_previous_location(d, h, LOCATION_ONFIELD)
    })));
    // e2:SetTarget(s.atktg)
    effect::set_target(d, e2, Some(cost_target(atktg)));
    // e2:SetOperation(s.atkop)
    effect::set_operation(d, e2, Some(operation(atkop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.sumcon(e,c,minc,zone,relzone,exeff)`
fn sumcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    let minc = a.int(2);
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // if minc>1 or c:IsLevelBelow(4) or Duel.GetLocationCount(tp,LOCATION_MZONE)==0 then return false end
    if minc > 1 || card::is_level_below(d, c, 4) || duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 == 0 {
        return false;
    }
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsFacedown,Card.IsReleasable),tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c) && card::is_releasable(d, c, None, None)), tp, 0, LOCATION_MZONE, NO_EX);
    // local must_g=Duel.GetMatchingGroup(Card.IsHasEffect,tp,LOCATION_MZONE,LOCATION_MZONE,nil,EFFECT_EXTRA_RELEASE)
    let must_g = duel::get_matching_group(d, Some(&|d, c| !card::is_has_effect(d, c, EFFECT_EXTRA_RELEASE, None).is_empty()), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // return #g>0 and (#must_g==0 or #(g&must_g)>0)
    if g.is_empty() {
        return false;
    }
    if must_g.is_empty() {
        return true;
    }
    let both = group::iter(d, &g).into_iter().filter(|x| must_g.contains(*x)).count();
    both > 0
}
/// `s.sumtg(e,tp,eg,ep,ev,re,r,rp,chk,c,minc,zone,relzone,exeff)`
fn sumtg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsFacedown,Card.IsReleasable),tp,0,LOCATION_MZONE,nil)
    let mut g = duel::get_matching_group(d, Some(&|d, c| card::is_facedown(d, c) && card::is_releasable(d, c, None, None)), tp, 0, LOCATION_MZONE, NO_EX);
    // local must_g=Duel.GetMatchingGroup(Card.IsHasEffect,tp,LOCATION_MZONE,LOCATION_MZONE,nil,EFFECT_EXTRA_RELEASE)
    let must_g = duel::get_matching_group(d, Some(&|d, c| !card::is_has_effect(d, c, EFFECT_EXTRA_RELEASE, None).is_empty()), tp, LOCATION_MZONE, LOCATION_MZONE, NO_EX);
    // if #must_g>0 then g=g&must_g end
    if !must_g.is_empty() {
        g = group::filter(d, &g, &|_d, x| must_g.contains(x), NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_RELEASE as u64);
    // local mc=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
    let Some(mc) = group::get_first(d, &sel) else {
        // if not mc then return false end
        return false;
    };
    // e:SetLabelObject(mc)
    effect::set_label_object(d, e, Value::Card(mc));
    true
}
/// `s.sumop(e,tp,eg,ep,ev,re,r,rp,c,minc,zone,relzone,exeff)`
fn sumop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(8);
    // local mc=e:GetLabelObject()
    let mc = effect::get_label_object(d, e).as_card().nil();
    // c:SetMaterial(mc)
    card::set_material(d, c, CardSet::from_card(mc));
    // Duel.Release(mc,REASON_SUMMON|REASON_MATERIAL)
    duel::release(d, CardSet::from_card(mc), REASON_SUMMON | REASON_MATERIAL, None);
}
/// `s.atkfilter(c)`
fn atkfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsSetCard(SET_TISTINA) and not c:IsHasEffect(EFFECT_EXTRA_ATTACK)
    card::is_faceup(d, c) && card::is_set_card(d, c, &[SET_TISTINA as u16]) && card::is_has_effect(d, c, EFFECT_EXTRA_ATTACK, None).is_empty()
}
/// `s.atktg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn atktg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsLocation(LOCATION_MZONE) and chkc:IsControler(tp) and s.atkfilter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_MZONE) && card::is_controler(d, chkc, tp) && atkfilter(d, chkc);
    }
    // if chk==0 then return Duel.IsExistingTarget(s.atkfilter,tp,LOCATION_MZONE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&atkfilter), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_FACEUP as u64);
    // Duel.SelectTarget(tp,s.atkfilter,tp,LOCATION_MZONE,0,1,1,nil)
    duel::select_target(d, tp, Some(&atkfilter), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, tc, a.e) {
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetDescription(3201)
        effect::set_description(d, e1, 3201);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
        // e1:SetCode(EFFECT_EXTRA_ATTACK)
        effect::set_code(d, e1, EFFECT_EXTRA_ATTACK);
        // e1:SetValue(1)
        effect::set_value(d, e1, 1);
        // e1:SetReset(RESETS_STANDARD_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
        // tc:RegisterEffect(e1)
        card::register_effect(d, tc, e1, false);
    }
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_IGNORE_IMMUNE)
    effect::set_property(d, e1, EFFECT_FLAG_IGNORE_IMMUNE, 0);
    // e1:SetCode(EFFECT_CANNOT_ATTACK_ANNOUNCE)
    effect::set_code(d, e1, EFFECT_CANNOT_ATTACK_ANNOUNCE);
    // e1:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e1, LOCATION_MZONE, 0);
    // e1:SetCondition(function(e) return e:GetLabel()~=0 end)
    effect::set_condition(d, e1, Some(condition(|d, a| effect::get_label(d, a.e) != 0)));
    // e1:SetTarget(function(e,c) return c:GetFieldID()~=e:GetLabel() end)
    effect::set_target(d, e1, Some(effect_card_fn(|d, e, c| (card::get_field_id(d, c) as i64) != effect::get_label(d, e))));
    // e1:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e1, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e1,tp)
    duel::register_effect(d, e1, tp);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e2:SetCode(EVENT_ATTACK_ANNOUNCE)
    effect::set_code(d, e2, EVENT_ATTACK_ANNOUNCE);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // e2:SetOperation(function(e,_,eg) e:GetLabelObject():SetLabel(eg:GetFirst():GetFieldID()) end)
    effect::set_operation(d, e2, Some(lua_fn(atkop_e2)));
    // e2:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, tp);
    // aux.RegisterClientHint(c,0,tp,1,0,aux.Stringid(id,2))
    aux::register_client_hint(d, c, 0, tp, 1, 0, Some(aux::stringid(ID, 2)), 0, None);
}
/// `function(e,_,eg) e:GetLabelObject():SetLabel(eg:GetFirst():GetFieldID()) end` (attack announce of the second attack)
fn atkop_e2(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let eg = a.g(2);
    let lo = effect::get_label_object(d, e).as_effect().nil();
    let first = group::get_first(d, &eg).nil();
    let fid = card::get_field_id(d, first) as i64;
    effect::set_label(d, lo, &[fid]);
}
