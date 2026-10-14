//! Encode Talker
use super::prelude::*;

pub const ID: u32 = 6622715;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // Link.AddProcedure(c,aux.FilterBoolFunctionEx(Card.IsRace,RACE_CYBERSE),2)
    link::add_procedure(d, c, Some(mat_filter(|d, c, sc, st, tp| card::is_race_ctx(d, c, RACE_CYBERSE, sc, st, Some(tp)))), 2, None, None, None, None);
    // activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_CONFIRM)
    effect::set_code(d, e1, EVENT_BATTLE_CONFIRM);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.operation)
    effect::set_operation(d, e1, Some(operation(op)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local lg=e:GetHandler():GetLinkedGroup()
    let h = effect::get_handler(d, a.e).nil();
    let lg = card::get_linked_group(d, h);
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // local b=a:GetBattleTarget()
    // if not b then return false end
    let Some(bt) = card::get_battle_target(d, at) else {
        return false;
    };
    // if a:IsControler(1-tp) then a,b=b,a end
    let (at, b) = if card::is_controler(d, at, 1 - a.tp) { (bt, at) } else { (at, bt) };
    // return a:GetControler()~=b:GetControler()
    //     and lg:IsContains(a) and a:IsFaceup() and b:IsFaceup()
    //     and b:GetAttack()>a:GetAttack()
    card::get_controler(d, at) != card::get_controler(d, b)
        && group::is_contains(&lg, at)
        && card::is_faceup(d, at)
        && card::is_faceup(d, b)
        && card::get_attack(d, b) > card::get_attack(d, at)
}
/// `s.operation(e,tp,eg,ep,ev,re,r,rp)`
fn op(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local a=Duel.GetAttacker()
    let at = duel::get_attacker(d).nil();
    // local b=a:GetBattleTarget()
    let bt = card::get_battle_target(d, at);
    // if a:IsControler(1-tp) then a,b=b,a end
    let (at, b) = if card::is_controler(d, at, 1 - a.tp) { (bt, Some(at)) } else { (Some(at), bt) };
    // if a and a:IsRelateToBattle() then
    // (ygoai override lua_fixes/c6622715.lua: the original indexes a nil a when the attacker is the
    // opponent's and its battle target has left; the fix skips the indestructibility then)
    if let Some(at) = at {
        if card::is_relate_to_battle(d, at) {
            // local e1=Effect.CreateEffect(c)
            let e1 = effect::create_effect(d, c);
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
            effect::set_code(d, e1, EFFECT_INDESTRUCTABLE_BATTLE);
            // e1:SetValue(1)
            effect::set_value(d, e1, 1);
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
            // a:RegisterEffect(e1)
            card::register_effect(d, at, e1, false);
        }
    }
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
    effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
    // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
    // e2:SetTargetRange(1,0)
    effect::set_target_range(d, e2, 1, 0);
    // e2:SetReset(RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
    // Duel.RegisterEffect(e2,tp)
    duel::register_effect(d, e2, a.tp);
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
    // e3:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e3, EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e3:SetCode(EVENT_BATTLED)
    effect::set_code(d, e3, EVENT_BATTLED);
    // e3:SetLabelObject(b)
    effect::set_label_object(d, e3, Value::card(b));
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetOperation(s.atkop)
    effect::set_operation(d, e3, Some(operation(atkop)));
    // e3:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE)
    effect::set_reset(d, e3, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE, 1);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.atkop(e,tp,eg,ep,ev,re,r,rp)`
fn atkop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local b=e:GetLabelObject()
    let b = effect::get_label_object(d, a.e).as_card().nil();
    // if b:IsFacedown() or not b:IsRelateToBattle() then return end
    if card::is_facedown(d, b) || !card::is_relate_to_battle(d, b) {
        return;
    }
    // local lg=c:GetLinkedGroup()
    let mut lg = card::get_linked_group(d, c);
    // lg:AddCard(c)
    group::add_card(&mut lg, &CardSet::from_card(c));
    // local tc=nil
    // if #lg==1 then
    let tc = if lg.len() == 1 {
        // tc=lg:GetFirst()
        group::get_first(d, &lg).nil()
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_FACEUP)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_FACEUP as u64);
        // tc=lg:Select(tp,1,1,nil):GetFirst()
        let sel = group::select(d, &lg, a.tp, 1, 1, false, NO_EX).nil();
        group::get_first(d, &sel).nil()
    };
    // Duel.HintSelection(Group.FromCards(tc))
    duel::hint_selection(d, CardSet::from_card(tc), true);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(b:GetAttack())
    let batk = card::get_attack(d, b) as i64;
    effect::set_value(d, e1, batk);
    // e1:SetReset(RESETS_STANDARD_PHASE_END)
    effect::set_reset(d, e1, RESETS_STANDARD_PHASE_END, 1);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
