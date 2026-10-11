//! Beast King Unleashed
use super::prelude::*;

pub const ID: u32 = 50675040;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e0, EFFECT_TYPE_ACTIVATE);
    // e0:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e0, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e1, CATEGORY_TOHAND);
    // e1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e1:SetCode(EVENT_BATTLE_START)
    effect::set_code(d, e1, EVENT_BATTLE_START);
    // e1:SetRange(LOCATION_SZONE)
    effect::set_range(d, e1, LOCATION_SZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetCondition(s.thcon)
    effect::set_condition(d, e1, Some(condition(thcon)));
    // e1:SetCost(Cost.HardOncePerChain(id))
    effect::set_cost(d, e1, Some(cost::hard_once_per_chain(ID, None)));
    // e1:SetTarget(s.thtg)
    effect::set_target(d, e1, Some(cost_target(thtg)));
    // e1:SetOperation(s.thop)
    effect::set_operation(d, e1, Some(operation(thop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.thcon(e,tp,...)`
fn thcon(d: &mut Duel, a: &EffArgs) -> bool {
    // local bc1,bc2=Duel.GetBattleMonster(tp)
    let (bc1, bc2) = duel::get_battle_monster(d, a.tp);
    // return bc1 and bc2 and bc1:IsColumn(bc2:GetSequence(),1-tp,LOCATION_MZONE)
    match (bc1, bc2) {
        (Some(bc1), Some(bc2)) => {
            let seq = card::get_sequence(d, bc2);
            card::is_column(d, bc1, seq, Some(1 - a.tp), Some(LOCATION_MZONE))
        }
        _ => false,
    }
}
/// `s.thtg(e,tp,...,chk)`
fn thtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local bc=Duel.GetBattleMonster(tp)
    let bc = duel::get_battle_monster(d, tp).0.nil();
    // local colg=bc:GetColumnGroup():Match(Card.IsAbleToHand,nil)
    let cg = card::get_column_group(d, bc, 0, 0);
    let colg = group::filter(d, &cg, &|d, c| card::is_able_to_hand(d, c, None), NO_EX);
    // if chk==0 then return bc:IsAbleToHand() and #colg>0 end
    if a.chk == Some(0) {
        return card::is_able_to_hand(d, bc, None) && !colg.is_empty();
    }
    // e:SetLabelObject(bc)
    effect::set_label_object(d, a.e, Value::Card(bc));
    // bc:CreateEffectRelation(e)
    card::create_effect_relation(d, bc, a.e);
    // Duel.SetOperationInfo(0,CATEGORY_TOHAND,colg+bc,#colg,tp,0)
    let mut colg_bc = colg.clone();
    group::add_card(&mut colg_bc, &CardSet::from_card(bc));
    duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(colg_bc), colg.len() as u8, tp, 0);
    true
}
/// `s.thop(e,tp,...)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local bc=e:GetLabelObject()
    let bc = effect::get_label_object(d, a.e).as_card().nil();
    // if bc:IsRelateToEffect(e) and bc:IsControler(tp) then
    if card::is_relate_to_effect(d, bc, a.e) && card::is_controler(d, bc, a.tp) {
        // local colg=bc:GetColumnGroup()+bc
        let mut colg = card::get_column_group(d, bc, 0, 0);
        group::add_card(&mut colg, &CardSet::from_card(bc));
        // if #colg>0 then Duel.SendtoHand(colg,nil,REASON_EFFECT) end
        if !colg.is_empty() {
            duel::send_to_hand(d, colg, None, REASON_EFFECT, None);
        }
    }
}
