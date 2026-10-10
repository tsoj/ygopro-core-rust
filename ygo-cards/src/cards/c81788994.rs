//! Curse of the Shadow Prison
use super::prelude::*;

pub const ID: u32 = 81788994;

pub const INFO: ScriptInfo = ScriptInfo { counter_place_list: &[0x16], listed_series: &[SET_SHADDOLL], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableCounterPermit(0x16)
    card::enable_counter_permit(d, c, 0x16, None, None);
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // counter
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e2:SetCode(EVENT_TO_GRAVE)
    effect::set_code(d, e2, EVENT_TO_GRAVE);
    // e2:SetProperty(EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetCondition(s.ctcon)
    effect::set_condition(d, e2, Some(condition(ctcon)));
    // e2:SetOperation(s.ctop)
    effect::set_operation(d, e2, Some(operation(ctop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atkdown
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e3, 0, LOCATION_MZONE);
    // e3:SetCondition(s.atkcon)
    effect::set_condition(d, e3, Some(lua_fn(atkcon)));
    // e3:SetValue(s.atkval)
    effect::set_value_fn(d, e3, effect_card_value(atkval));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // chain material
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e4, aux::stringid(ID, 0));
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetCode(EFFECT_CHAIN_MATERIAL)
    effect::set_code(d, e4, EFFECT_CHAIN_MATERIAL);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // e4:SetCondition(s.chcon)
    effect::set_condition(d, e4, Some(condition(chcon)));
    // e4:SetTarget(s.chtg)
    effect::set_target(d, e4, Some(lua_fn(chtg)));
    // e4:SetOperation(s.chop)
    effect::set_operation(d, e4, Some(lua_fn(chop)));
    // e4:SetValue(aux.FilterBoolFunction(Card.IsSetCard,SET_SHADDOLL))
    effect::set_value_fn(d, e4, lua_fn(|d, a| card::is_set_card(d, a.c(0), &[SET_SHADDOLL as u16])));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetOperation(s.chk)
    effect::set_operation(d, e5, Some(lua_fn(chk)));
    // e4:SetLabelObject(e5)
    effect::set_label_object(d, e4, Value::Effect(e5));
}
/// `s.cfilter(c)`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_SHADDOLL) and c:IsMonster() and c:IsReason(REASON_EFFECT)
    card::is_set_card(d, c, &[SET_SHADDOLL as u16]) && card::is_monster(d, c) && card::is_reason(d, c, REASON_EFFECT)
}
/// `s.ctcon(e,tp,eg,ep,ev,re,r,rp)`
fn ctcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.cfilter,1,nil)
    let eg = a.eg.clone().nil();
    group::is_exists(d, &eg, &|d, c| cfilter(d, c), 1, NO_EX)
}
/// `s.ctop(e,tp,eg,ep,ev,re,r,rp)`
fn ctop(d: &mut Duel, a: &EffArgs) {
    // local ct=eg:FilterCount(s.cfilter,nil)
    let eg = a.eg.clone().nil();
    let ct = group::filter_count(d, &eg, &|d, c| cfilter(d, c), NO_EX);
    // e:GetHandler():AddCounter(0x16,ct)
    let h = effect::get_handler(d, a.e).nil();
    card::add_counter(d, h, 0x16, ct as u16, false);
}
/// `s.atkcon(e)`
fn atkcon(d: &mut Duel, a: Args) -> bool {
    // return Duel.GetTurnPlayer()~=e:GetHandlerPlayer()
    let turn = duel::get_turn_player(d);
    let p = effect::get_handler_player(d, a.e(0));
    turn != p
}
/// `s.atkval(e,c)`
fn atkval(d: &mut Duel, e: EffectId, _c: CardId) -> i64 {
    // return e:GetHandler():GetCounter(0x16)*-100
    let h = effect::get_handler(d, e).nil();
    card::get_counter(d, h, 0x16) as i64 * -100
}
/// `s.chcon(e,...)`
fn chcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return e:GetHandler():GetCounter(0x16)>=3
    let h = effect::get_handler(d, a.e).nil();
    card::get_counter(d, h, 0x16) >= 3
}
/// `s.chfilter(c,e,tp)`
fn chfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and (c:IsFaceup() or c:IsControler(tp)) and c:IsCanBeFusionMaterial() and not c:IsImmuneToEffect(e)
    card::is_monster(d, c)
        && (card::is_faceup(d, c) || card::is_controler(d, c, tp))
        && card::is_can_be_fusion_material(d, c, None, Some(SUMMON_TYPE_FUSION as u64), None)
        && !card::is_immune_to_effect(d, c, e)
}
/// `s.chtg(e,te,tp,value)`
fn chtg(d: &mut Duel, a: Args) -> Value {
    let te = a.e(1);
    let tp = a.p(2);
    let value = a.int(3);
    // if value&SUMMON_TYPE_FUSION==0 then return Group.CreateGroup() end
    if (value & SUMMON_TYPE_FUSION as i64) == 0 {
        return Value::Group(CardSet::new());
    }
    // return Duel.GetMatchingGroup(s.chfilter,tp,LOCATION_MZONE|LOCATION_HAND,LOCATION_MZONE,nil,te,tp)
    Value::Group(duel::get_matching_group(d, Some(&|d, c| chfilter(d, c, te, tp)), tp, LOCATION_MZONE | LOCATION_HAND, LOCATION_MZONE, NO_EX))
}
/// `s.chop(e,te,tp,tc,mat,sumtype,sg,sumpos)`
fn chop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let tp = a.p(2);
    let tc = a.c(3);
    let mat = a.g(4);
    // if not sumtype then sumtype=SUMMON_TYPE_FUSION end
    let sumtype = match a.v(5) {
        Value::Nil | Value::Bool(false) => SUMMON_TYPE_FUSION as i64,
        v => v.int(),
    };
    // tc:SetMaterial(mat)
    card::set_material(d, tc, mat.clone());
    // Duel.SendtoGrave(mat,REASON_EFFECT+REASON_MATERIAL+REASON_FUSION)
    duel::send_to_grave(d, mat.clone(), REASON_EFFECT + REASON_MATERIAL + REASON_FUSION, None, None);
    // if mat:IsExists(Card.IsControler,1,nil,1-tp) then
    if group::is_exists(d, &mat, &|d, c| card::is_controler(d, c, 1 - tp), 1, NO_EX) {
        // e:GetHandler():RemoveCounter(tp,0x16,3,REASON_EFFECT)
        let h = effect::get_handler(d, e).nil();
        card::remove_counter(d, h, tp, 0x16, 3, REASON_EFFECT);
    }
    // Duel.BreakEffect()
    duel::break_effect(d);
    // if sg then sg:AddCard(tc) else Duel.SpecialSummon(tc,sumtype,tp,tp,false,false,sumpos) end
    match a.v(6) {
        Value::Nil | Value::Bool(false) => {
            let sumpos = a.int(7);
            duel::special_summon(d, tc, sumtype as u32, tp, tp, false, false, sumpos as u8, None);
        }
        // UNSURE: sg:AddCard(tc) on a group argument cannot write back to the caller's group
        // (Rust groups are values). The fusion procedure always passes nil here.
        _ => crate::lua_error("sg:AddCard in s.chop: group argument not supported"),
    }
}
/// `s.chk(tp,sg,fc)`
fn chk(d: &mut Duel, a: Args) -> bool {
    // return sg:FilterCount(Card.IsControler,nil,1-tp)<=1
    let tp = a.p(0);
    let sg = a.g(1);
    group::filter_count(d, &sg, &|d, c| card::is_controler(d, c, 1 - tp), NO_EX) <= 1
}
