//! Volcanic Doomfire
use super::prelude::*;

pub const ID: u32 = 32543380;

pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[21420702], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_MUST_ATTACK)
    effect::set_code(d, e2, EFFECT_MUST_ATTACK);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTargetRange(0,LOCATION_MZONE)
    effect::set_target_range(d, e2, 0, LOCATION_MZONE);
    // e2:SetCondition(function(e) return Duel.IsTurnPlayer(1-e:GetHandlerPlayer()) and Duel.IsBattlePhase() end)
    effect::set_condition(d, e2, Some(condition(|d, a| {
        let hp = effect::get_handler_player(d, a.e);
        duel::is_turn_player(d, 1 - hp) && duel::is_battle_phase(d, None)
    })));
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsAttackPos))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_attack_pos(d, c))));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e2:Clone()
    let e3 = effect::clone(d, e2);
    // e3:SetCode(EFFECT_MUST_ATTACK_MONSTER)
    effect::set_code(d, e3, EFFECT_MUST_ATTACK_MONSTER);
    // e3:SetValue(function(e,_c) return _c==e:GetHandler() end)
    effect::set_value_fn(d, e3, lua_fn(|d, a: Args| {
        let h = effect::get_handler(d, a.e(0)).nil();
        a.card(1) == Some(h)
    }));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e4, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e4:SetCode(EVENT_BATTLE_DESTROYING)
    effect::set_code(d, e4, EVENT_BATTLE_DESTROYING);
    // e4:SetCondition(s.descon)
    effect::set_condition(d, e4, Some(condition(descon)));
    // e4:SetTarget(s.destg)
    effect::set_target(d, e4, Some(cost_target(destg)));
    // e4:SetOperation(s.desop)
    effect::set_operation(d, e4, Some(operation(desop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.cfilter(c)`: `return c:IsFaceup() and c:IsCode(21420702) and c:IsAbleToGraveAsCost()`
fn cfilter(d: &mut Duel, c: CardId) -> bool {
    card::is_faceup(d, c) && card::is_code(d, c, &[21420702]) && card::is_able_to_grave_as_cost(d, c)
}
/// `s.spcon(e,c)`
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_ONFIELD,0,nil,ft)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_ONFIELD, 0, NO_EX);
    // return #g>0 and Duel.GetMZoneCount(tp,g)>0
    !g.is_empty() && duel::get_m_zone_count(d, tp, (&g).into(), None, None, None).0 > 0
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,c)`: the card is argument 8 (no `chk`)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let (e, tp) = (a.e(0), a.p(1));
    // local g=Duel.GetMatchingGroup(s.cfilter,tp,LOCATION_ONFIELD,0,nil)
    let g = duel::get_matching_group(d, Some(&cfilter), tp, LOCATION_ONFIELD, 0, NO_EX);
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,1,aux.ChkfMMZ(1),1,tp,HINTMSG_TOGRAVE,nil,nil,true)
    let rc = aux::chkf_mmz(1);
    let sg = aux::select_unselect_group(d, &g, Some(e), tp, Some(1), Some(1), Some(&rc), 1, tp, Some(HINTMSG_TOGRAVE as u64), None, None, true)
        .unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // sg:KeepAlive()
        // e:SetLabelObject(sg)
        effect::set_label_object(d, e, Value::Group(sg));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp,c)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_group().cloned();
    // if not g then return end
    let Some(g) = g else {
        return;
    };
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    // g:DeleteGroup()
}
/// `s.descon(e,tp,eg,ep,ev,re,r,rp)`
fn descon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bc=c:GetBattleTarget()
    let bc = card::get_battle_target(d, c).nil();
    // return c:IsRelateToBattle() and bc:IsLocation(LOCATION_GRAVE) and bc:IsMonster()
    card::is_relate_to_battle(d, c) && card::is_location(d, bc, LOCATION_GRAVE) && card::is_monster(d, bc)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g.clone()), n, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,#g*500)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, (g.len() * 500) as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(nil,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, None, a.tp, 0, LOCATION_MZONE, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local ct=Duel.Destroy(g,REASON_EFFECT)
    let ct = duel::destroy(d, g, REASON_EFFECT, None, None);
    // if ct>0 then Duel.Damage(1-tp,ct*500,REASON_EFFECT) end
    if ct > 0 {
        duel::damage(d, 1 - a.tp, (ct * 500) as i64, REASON_EFFECT, false, None);
    }
}
