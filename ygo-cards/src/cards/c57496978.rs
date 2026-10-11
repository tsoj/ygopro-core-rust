//! Clashing Souls
use super::prelude::*;

pub const ID: u32 = 57496978;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_ATKCHANGE)
    effect::set_category(d, e1, CATEGORY_ATKCHANGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_PRE_DAMAGE_CALCULATE)
    effect::set_code(d, e1, EVENT_PRE_DAMAGE_CALCULATE);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local a=Duel.GetAttacker()
    let mut x: Option<CardId> = duel::get_attacker(d);
    // local d=Duel.GetAttackTarget()
    let mut y: Option<CardId> = duel::get_attack_target(d);
    // if a:IsControler(1-tp) then a=Duel.GetAttackTarget() d=Duel.GetAttacker() end
    if card::is_controler(d, x.nil(), 1 - a.tp) {
        std::mem::swap(&mut x, &mut y);
    }
    // return a and d and a:IsAttackPos() and d:IsAttackPos() and a:GetAttack()<d:GetAttack()
    match (x, y) {
        (Some(x), Some(y)) => {
            card::is_attack_pos(d, x) && card::is_attack_pos(d, y) && card::get_attack(d, x) < card::get_attack(d, y)
        }
        _ => false,
    }
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local a=Duel.GetAttacker()
    let x = duel::get_attacker(d).nil();
    // local d=Duel.GetAttackTarget()
    let y = duel::get_attack_target(d).nil();
    // if a:IsFaceup() and a:IsRelateToBattle() and d:IsFaceup() and d:IsRelateToBattle() then
    if card::is_faceup(d, x) && card::is_relate_to_battle(d, x) && card::is_faceup(d, y) && card::is_relate_to_battle(d, y) {
        // local g=Group.FromCards(a,d)
        let mut g = CardSet::from_card(x);
        group::add_card(&mut g, &CardSet::from_card(y));
        // local chk=true
        // while chk do
        loop {
            // local tg=g:GetMinGroup(Card.GetAttack)
            let tg = group::get_min_group(d, &g, &|d, c| card::get_attack(d, c) as i64).map(|r| r.0).unwrap_or_default();
            // local tc=tg:GetFirst()
            // if #tg==1 and Duel.CheckLPCost(tc:GetControler(),500)
            //   and not tc:IsImmuneToEffect(e)
            //   and Duel.SelectYesNo(tc:GetControler(),aux.Stringid(id,0)) then
            let mut paid = false;
            if tg.len() == 1 {
                let tc = group::get_first(d, &tg).nil();
                let p = card::get_controler(d, tc);
                if duel::check_lp_cost(d, p, 500)
                    && !card::is_immune_to_effect(d, tc, a.e)
                    && duel::select_yes_no(d, p, aux::stringid(ID, 0))
                {
                    // Duel.PayLPCost(tc:GetControler(),500)
                    duel::pay_lp_cost(d, p, 500);
                    // local e1=Effect.CreateEffect(c)
                    let e1 = effect::create_effect(d, c);
                    // e1:SetType(EFFECT_TYPE_SINGLE)
                    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                    // e1:SetCode(EFFECT_UPDATE_ATTACK)
                    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
                    // e1:SetValue(500)
                    effect::set_value(d, e1, 500);
                    // e1:SetReset(RESET_EVENT|RESETS_STANDARD|RESET_PHASE|PHASE_DAMAGE_CAL)
                    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD | RESET_PHASE | PHASE_DAMAGE_CAL, 1);
                    // tg:GetFirst():RegisterEffect(e1)
                    card::register_effect(d, tc, e1, false);
                    paid = true;
                }
            }
            if !paid {
                break;
            }
        }
        // g:KeepAlive() (groups are values: nothing to do)
        // local e2=Effect.CreateEffect(c)
        let e2 = effect::create_effect(d, c);
        // e2:SetType(EFFECT_TYPE_FIELD)
        effect::set_type(d, e2, EFFECT_TYPE_FIELD);
        // e2:SetCode(EFFECT_AVOID_BATTLE_DAMAGE)
        effect::set_code(d, e2, EFFECT_AVOID_BATTLE_DAMAGE);
        // e2:SetProperty(EFFECT_FLAG_PLAYER_TARGET)
        effect::set_property(d, e2, EFFECT_FLAG_PLAYER_TARGET, 0);
        // e2:SetTargetRange(1,1)
        effect::set_target_range(d, e2, 1, 1);
        // e2:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e2, RESET_PHASE | PHASE_DAMAGE, 1);
        // Duel.RegisterEffect(e2,tp)
        duel::register_effect(d, e2, a.tp);
        // local e3=Effect.CreateEffect(c)
        let e3 = effect::create_effect(d, c);
        // e3:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_CONTINUOUS)
        effect::set_type(d, e3, EFFECT_TYPE_FIELD + EFFECT_TYPE_CONTINUOUS);
        // e3:SetCode(EVENT_BATTLED)
        effect::set_code(d, e3, EVENT_BATTLED);
        // e3:SetLabelObject(g)
        effect::set_label_object(d, e3, Value::Group(g));
        // e3:SetOperation(s.tgop)
        effect::set_operation(d, e3, Some(operation(tgop)));
        // e3:SetReset(RESET_PHASE|PHASE_DAMAGE)
        effect::set_reset(d, e3, RESET_PHASE | PHASE_DAMAGE, 1);
        // Duel.RegisterEffect(e3,tp)
        duel::register_effect(d, e3, a.tp);
    }
}
/// `s.tgop(e,tp,eg,ep,ev,re,r,rp)`
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject():Filter(Card.IsStatus,nil,STATUS_BATTLE_DESTROYED)
    let lo = effect::get_label_object(d, a.e).as_group().cloned().nil();
    let g = group::filter(d, &lo, &|d, c| card::is_status(d, c, STATUS_BATTLE_DESTROYED), NO_EX);
    // local tg=Group.CreateGroup()
    let mut tg = CardSet::new();
    // if g:IsExists(Card.IsControler,1,nil,tp) then
    if group::is_exists(d, &g, &|d, c| card::is_controler(d, c, a.tp), 1, NO_EX) {
        // local g1=Duel.GetFieldGroup(tp,LOCATION_ONFIELD,0)
        let g1 = duel::get_field_group(d, a.tp, LOCATION_ONFIELD, 0);
        // tg:Merge(g1)
        group::add_card(&mut tg, &g1);
    }
    // if g:IsExists(Card.IsControler,1,nil,1-tp) then
    if group::is_exists(d, &g, &|d, c| card::is_controler(d, c, 1 - a.tp), 1, NO_EX) {
        // local g2=Duel.GetFieldGroup(tp,0,LOCATION_ONFIELD)
        let g2 = duel::get_field_group(d, a.tp, 0, LOCATION_ONFIELD);
        // tg:Merge(g2)
        group::add_card(&mut tg, &g2);
    }
    // if #tg>0 then
    if !tg.is_empty() {
        // Duel.SendtoGrave(tg,REASON_EFFECT)
        duel::send_to_grave(d, tg, REASON_EFFECT, None, None);
    }
}
