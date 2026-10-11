//! Psi-Curse
use super::prelude::*;

pub const ID: u32 = 16678947;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_DAMAGE)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_DAMAGE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_BATTLE_DESTROYED)
    effect::set_code(d, e1, EVENT_BATTLE_DESTROYED);
    // e1:SetCondition(s.condition)
    effect::set_condition(d, e1, Some(condition(cond)));
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.condition(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cond(d: &mut Duel, a: &EffArgs) -> bool {
    // local dc=eg:GetFirst()
    let dc = group::get_first(d, &a.eg.clone().nil()).nil();
    // local bc=dc:GetBattleTarget()
    let bc = card::get_battle_target(d, dc);
    // return dc:IsPreviousControler(tp) and dc:IsLocation(LOCATION_GRAVE) and dc:IsRace(RACE_PSYCHIC)
    //     and dc:GetPreviousRaceOnField()&RACE_PSYCHIC>0 and dc:GetPreviousLevelOnField()>0
    //     and bc:IsRelateToBattle() and bc:IsControler(1-tp) and bc==Duel.GetAttacker()
    card::is_previous_controler(d, dc, a.tp)
        && card::is_location(d, dc, LOCATION_GRAVE)
        && card::is_race(d, dc, RACE_PSYCHIC)
        && (card::get_previous_race_on_field(d, dc) & RACE_PSYCHIC) > 0
        && card::get_previous_level_on_field(d, dc) > 0
        && {
            let bc = bc.nil();
            card::is_relate_to_battle(d, bc) && card::is_controler(d, bc, 1 - a.tp) && Some(bc) == duel::get_attacker(d)
        }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // local dc=eg:GetFirst()
    let dc = group::get_first(d, &a.eg.clone().nil());
    // local bc=Duel.GetAttacker()
    let bc = duel::get_attacker(d);
    // if chk==0 then return bc:IsCanBeEffectTarget(e) end
    if a.chk == Some(0) {
        return card::is_can_be_effect_target(d, bc.nil(), Some(a.e));
    }
    // local lv=dc:GetPreviousLevelOnField()
    let lv = card::get_previous_level_on_field(d, dc.nil());
    // e:SetLabel(lv)
    effect::set_label(d, a.e, &[lv as i64]);
    // Duel.SetTargetCard(bc)
    let bc = bc.nil();
    duel::set_target_card(d, bc);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,bc,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(CardSet::from_card(bc)), 1, a.tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_DAMAGE,nil,0,1-tp,lv*300)
    duel::set_operation_info(d, 0, CATEGORY_DAMAGE, None, 0, 1 - a.tp, lv as i32 * 300);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsControler(1-tp) and Duel.Destroy(tc,REASON_EFFECT)>0 then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_controler(d, tc, 1 - a.tp) && duel::destroy(d, tc, REASON_EFFECT, None, None) > 0 {
        // Duel.Damage(1-tp,e:GetLabel()*300,REASON_EFFECT)
        let label = effect::get_label(d, a.e);
        duel::damage(d, 1 - a.tp, label * 300, REASON_EFFECT, false, None);
    }
}
