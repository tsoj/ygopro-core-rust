//! Card Blocker
use super::prelude::*;

pub const ID: u32 = 42256406;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // to defense
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_POSITION)
    effect::set_category(d, e1, CATEGORY_POSITION);
    // e1:SetType(EFFECT_TYPE_TRIGGER_F+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_TRIGGER_F + EFFECT_TYPE_SINGLE);
    // e1:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e1, EVENT_SUMMON_SUCCESS);
    // e1:SetTarget(s.potg)
    effect::set_target(d, e1, Some(cost_target(potg)));
    // e1:SetOperation(s.poop)
    effect::set_operation(d, e1, Some(operation(poop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // local e2=e1:Clone()
    let e2 = effect::clone(d, e1);
    // e2:SetCode(EVENT_FLIP_SUMMON_SUCCESS)
    effect::set_code(d, e2, EVENT_FLIP_SUMMON_SUCCESS);
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // local e3=e1:Clone()
    let e3 = effect::clone(d, e1);
    // e3:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e3, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // change battle target
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e4, EVENT_BE_BATTLE_TARGET);
    // e4:SetRange(LOCATION_MZONE)
    effect::set_range(d, e4, LOCATION_MZONE);
    // e4:SetCondition(s.cbcon)
    effect::set_condition(d, e4, Some(condition(cbcon)));
    // e4:SetTarget(s.cbtg)
    effect::set_target(d, e4, Some(cost_target(cbtg)));
    // e4:SetOperation(s.cbop)
    effect::set_operation(d, e4, Some(operation(cbop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // def
    // local e5=Effect.CreateEffect(c)
    let e5 = effect::create_effect(d, c);
    // e5:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e5, aux::stringid(ID, 2));
    // e5:SetCategory(CATEGORY_DEFCHANGE)
    effect::set_category(d, e5, CATEGORY_DEFCHANGE);
    // e5:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e5, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e5:SetCode(EVENT_BE_BATTLE_TARGET)
    effect::set_code(d, e5, EVENT_BE_BATTLE_TARGET);
    // e5:SetCost(s.defcost)
    effect::set_cost(d, e5, Some(cost_target(defcost)));
    // e5:SetOperation(s.defop)
    effect::set_operation(d, e5, Some(operation(defop)));
    // c:RegisterEffect(e5)
    card::register_effect(d, c, e5, false);
}
/// `s.potg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn potg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return e:GetHandler():IsAttackPos() end
    if a.chk == Some(0) {
        let h = effect::get_handler(d, a.e).nil();
        return card::is_attack_pos(d, h);
    }
    // Duel.SetOperationInfo(0,CATEGORY_POSITION,e:GetHandler(),1,0,0)
    let h = effect::get_handler(d, a.e).nil();
    duel::set_operation_info(d, 0, CATEGORY_POSITION, Some(CardSet::from_card(h)), 1, 0, 0);
    true
}
/// `s.poop(e,tp,eg,ep,ev,re,r,rp)`
fn poop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsAttackPos() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_attack_pos(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // Duel.ChangePosition(c,POS_FACEUP_DEFENSE)
        duel::change_position(d, c, POS_FACEUP_DEFENSE as u8, None, None, None, false);
    }
}
/// `s.cbcon(e,tp,eg,ep,ev,re,r,rp)`
fn cbcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local bt=eg:GetFirst()
    let bt = group::get_first(d, &a.eg.clone().nil());
    // return r~=REASON_REPLACE and c~=bt and bt:IsFaceup() and bt:GetControler()==c:GetControler()
    if a.r == REASON_REPLACE {
        return false;
    }
    if bt == Some(c) {
        return false;
    }
    let bt = bt.nil();
    card::is_faceup(d, bt) && card::get_controler(d, bt) == card::get_controler(d, c)
}
/// `s.cbtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cbtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetAttacker():GetAttackableTarget():IsContains(e:GetHandler()) end
    if a.chk == Some(0) {
        let att = duel::get_attacker(d).nil();
        let (ag, _) = card::get_attackable_target(d, att);
        let h = effect::get_handler(d, a.e).nil();
        return group::is_contains(&ag, h);
    }
    true
}
/// `s.cbop(e,tp,eg,ep,ev,re,r,rp)`
fn cbop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) and not Duel.GetAttacker():IsImmuneToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        let att = duel::get_attacker(d).nil();
        if !card::is_immune_to_effect(d, att, a.e) {
            // Duel.ChangeAttackTarget(c)
            duel::change_attack_target(d, Some(c), false);
        }
    }
}
/// `s.defcost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn defcost(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsPlayerCanDiscardDeckAsCost(tp,1) end
    if a.chk == Some(0) {
        return duel::is_player_can_discard_deck_as_cost(d, a.tp, 1);
    }
    // local ct={}
    let mut ct: Vec<u64> = Vec::new();
    // for i=3,1,-1 do
    for i in (1..=3u32).rev() {
        // if Duel.IsPlayerCanDiscardDeckAsCost(tp,i) then table.insert(ct,i) end
        if duel::is_player_can_discard_deck_as_cost(d, a.tp, i) {
            ct.push(i as u64);
        }
    }
    // if #ct==1 then
    if ct.len() == 1 {
        // Duel.DiscardDeck(tp,ct[1],REASON_COST)
        duel::discard_deck(d, a.tp, ct[0] as u16, REASON_COST);
        // e:SetLabel(1)
        effect::set_label(d, a.e, &[1]);
    } else {
        // Duel.Hint(HINT_SELECTMSG,tp,aux.Stringid(id,3))
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, aux::stringid(ID, 3));
        // local ac=Duel.AnnounceNumber(tp,table.unpack(ct))
        let (ac, _) = duel::announce_number(d, a.tp, &ct).nil();
        // Duel.DiscardDeck(tp,ac,REASON_COST)
        duel::discard_deck(d, a.tp, ac as u16, REASON_COST);
        // e:SetLabel(ac)
        effect::set_label(d, a.e, &[ac as i64]);
    }
    true
}
/// `s.defop(e,tp,eg,ep,ev,re,r,rp)`
fn defop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsFaceup() and c:IsRelateToEffect(e) then
    if card::is_faceup(d, c) && card::is_relate_to_effect(d, c, a.e) {
        // local ct=e:GetLabel()
        let ct = effect::get_label(d, a.e);
        // local e1=Effect.CreateEffect(c)
        let e1 = effect::create_effect(d, c);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_UPDATE_DEFENSE)
        effect::set_code(d, e1, EFFECT_UPDATE_DEFENSE);
        // e1:SetReset(RESETS_STANDARD_DISABLE_PHASE_END)
        effect::set_reset(d, e1, RESETS_STANDARD_DISABLE_PHASE_END, 1);
        // e1:SetValue(ct*500)
        effect::set_value(d, e1, ct * 500);
        // c:RegisterEffect(e1)
        card::register_effect(d, c, e1, false);
    }
}
