//! Ultimate Conductor Tyranno
use super::prelude::*;

pub const ID: u32 = 18940556;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Special summon procedure
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCondition(s.sprcon)
    effect::set_condition(d, e1, Some(lua_fn(sprcon)));
    // e1:SetTarget(s.sprtg)
    effect::set_target(d, e1, Some(lua_fn(sprtg)));
    // e1:SetOperation(s.sprop)
    effect::set_operation(d, e1, Some(lua_fn(sprop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Destroy 1 monster and change monsters to face-down
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_POSITION+CATEGORY_SET)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_POSITION + CATEGORY_SET);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetHintTiming(0,TIMINGS_CHECK_MONSTER|TIMING_MAIN_END)
    effect::set_hint_timing(d, e2, 0, Some(TIMINGS_CHECK_MONSTER | TIMING_MAIN_END));
    // e2:SetCondition(s.descon)
    effect::set_condition(d, e2, Some(condition(descon)));
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Can attack all monsters
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetCode(EFFECT_ATTACK_ALL)
    effect::set_code(d, e3, EFFECT_ATTACK_ALL);
    // e3:SetValue(1)
    effect::set_value(d, e3, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Send defense position monster to the GY
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetCategory(CATEGORY_DAMAGE+CATEGORY_TOGRAVE)
    effect::set_category(d, e4, CATEGORY_DAMAGE + CATEGORY_TOGRAVE);
    // e4:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e4, EVENT_BATTLE_START);
    // e4:SetTarget(s.tgtg)
    effect::set_target(d, e4, Some(cost_target(tgtg)));
    // e4:SetOperation(s.tgop)
    effect::set_operation(d, e4, Some(operation(tgop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
}
/// `s.sprfilter(c)`
fn sprfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRace(RACE_DINOSAUR) and c:IsAbleToRemoveAsCost() and aux.SpElimFilter(c,true)
    card::is_race(d, c, RACE_DINOSAUR) && card::is_able_to_remove_as_cost(d, c, None) && aux::sp_elim_filter(d, c, true, false)
}
/// `s.sprcon(e,c)`
fn sprcon(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    // if c==nil then return true end
    let Some(c) = a.card(1) else {
        return true;
    };
    // local tp=c:GetControler()
    let tp = card::get_controler(d, c);
    // local rg=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>-2 and #rg>1 and aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),0)
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > -2
        && rg.len() > 1
        && matches!(
            aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 0, tp, None, None, None, false),
            Err(true)
        )
}
/// `s.sprtg(e,tp,eg,ep,ev,re,r,rp,c)`
fn sprtg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    // local rg=Duel.GetMatchingGroup(s.sprfilter,tp,LOCATION_MZONE|LOCATION_GRAVE,0,nil)
    let rg = duel::get_matching_group(d, Some(&sprfilter), tp, LOCATION_MZONE | LOCATION_GRAVE, 0, NO_EX);
    // local g=aux.SelectUnselectGroup(rg,e,tp,2,2,aux.ChkfMMZ(1),1,tp,HINTMSG_REMOVE)
    let g = aux::select_unselect_group(d, &rg, Some(e), tp, Some(2), Some(2), Some(&aux::chkf_mmz(1)), 1, tp, Some(HINTMSG_REMOVE as u64), None, None, false)
        .unwrap_or_default();
    // if #g>0 then
    if !g.is_empty() {
        // g:KeepAlive()
        // e:SetLabelObject(g)
        effect::set_label_object(d, e, Value::Group(g));
        // return true
        return true;
    }
    // return false
    false
}
/// `s.sprop(e,tp,eg,ep,ev,re,r,rp,c)`
fn sprop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.Remove(g,POS_FACEUP,REASON_COST)
    duel::remove(d, g, POS_FACEUP as u8, REASON_COST, None, None);
    // g:DeleteGroup()
}
/// `s.descon()`
fn descon(d: &mut Duel, _a: &EffArgs) -> bool {
    // return Duel.IsMainPhase()
    duel::is_main_phase(d, None)
}
/// `s.posfilter(c)`
fn posfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsCanTurnSet()
    card::is_faceup(d, c) && card::is_can_turn_set(d, c)
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return not c:IsPublic() or c:IsMonster()
    !card::is_public(d, c) || card::is_monster(d, c)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return Duel.IsExistingMatchingCard(Card.IsMonster,tp,LOCATION_HAND|LOCATION_MZONE,0,1,nil)
    //     and Duel.IsExistingMatchingCard(s.posfilter,tp,0,LOCATION_MZONE,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&|d, c| card::is_monster(d, c)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&posfilter), tp, 0, LOCATION_MZONE, 1, NO_EX);
    }
    // local dg=Duel.GetMatchingGroup(Card.IsMonster,tp,LOCATION_HAND|LOCATION_MZONE,0,nil)
    let dg = duel::get_matching_group(d, Some(&|d, c| card::is_monster(d, c)), tp, LOCATION_HAND | LOCATION_MZONE, 0, NO_EX);
    // local g=Duel.GetMatchingGroup(s.posfilter,tp,0,LOCATION_MZONE,nil)
    let g = duel::get_matching_group(d, Some(&posfilter), tp, 0, LOCATION_MZONE, NO_EX);
    // if not Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_HAND,0,1,nil) then
    if !duel::is_existing_matching_card(d, Some(&filter), tp, LOCATION_HAND, 0, 1, NO_EX) {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,dg,1,tp,LOCATION_HAND|LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(dg), 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_HAND|LOCATION_MZONE)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, tp, (LOCATION_HAND | LOCATION_MZONE) as i32);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,g,#g,tp,POS_FACEDOWN_DEFENSE)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(g), n, tp, POS_FACEDOWN_DEFENSE as i32);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local g1=Duel.SelectMatchingCard(tp,Card.IsMonster,tp,LOCATION_HAND|LOCATION_MZONE,0,1,1,nil)
    let g1 = duel::select_matching_card(d, tp, Some(&|d, c| card::is_monster(d, c)), tp, LOCATION_HAND | LOCATION_MZONE, 0, 1, 1, false, NO_EX).nil();
    // if #g1>0 and Duel.Destroy(g1,REASON_EFFECT)~=0 then
    if !g1.is_empty() && duel::destroy(d, g1, REASON_EFFECT, None, None) != 0 {
        // local g2=Duel.GetMatchingGroup(s.posfilter,tp,0,LOCATION_MZONE,nil)
        let g2 = duel::get_matching_group(d, Some(&posfilter), tp, 0, LOCATION_MZONE, NO_EX);
        // if #g2>0 then
        if !g2.is_empty() {
            // Duel.ChangePosition(g2,POS_FACEDOWN_DEFENSE)
            duel::change_position(d, g2, POS_FACEDOWN_DEFENSE as u8, None, None, None, false);
        }
    }
}
/// `s.tgtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    let (e, tp) = (a.e, a.tp);
    // local d=Duel.GetAttackTarget()
    let df = duel::get_attack_target(d);
    // if chk ==0 then return Duel.GetAttacker()==e:GetHandler() and d and d:IsDefensePos() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, e).nil();
        return duel::get_attacker(d) == Some(h) && df.is_some_and(|x| card::is_defense_pos(d, x));
    }
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,1000)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - tp, 1000);
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,d,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, df.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // if Duel.Damage(1-tp,1000,REASON_EFFECT)~=0 then
    if duel::damage(d, 1 - tp, 1000, REASON_EFFECT, false, None) != 0 {
        // local d=Duel.GetAttackTarget()
        let df = duel::get_attack_target(d).nil();
        // if d:IsRelateToBattle() and d:IsDefensePos() then
        if card::is_relate_to_battle(d, df) && card::is_defense_pos(d, df) {
            // Duel.SendtoGrave(d,REASON_EFFECT)
            duel::send_to_grave(d, df, REASON_EFFECT, None, None);
        }
    }
}
